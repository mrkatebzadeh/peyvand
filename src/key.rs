/* key.rs

*
* Author: M.R.Siavash Katebzadeh <mr@katebzadeh.xyz>
* Keywords: Rust
* Version: 0.0.1
*
* This program is free software; you can redistribute it and/or modify
* it under the terms of the GNU General Public License as published by
* the Free Software Foundation, either version 3 of the License, or
* (at your option) any later version.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU General Public License for more details.
*
* You should have received a copy of the GNU General Public License
* along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum KeyMode {
    #[default]
    Normal,
    Insert,
    Search,
    Command,
}

impl Display for KeyMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            KeyMode::Normal => "Normal",
            KeyMode::Insert => "Insert",
            KeyMode::Search => "Search",
            KeyMode::Command => "Command",
        };
        write!(f, "{}", s)
    }
}

pub type Command = String;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeySequence(pub Vec<String>);

impl KeySequence {
    pub fn from_str(s: &str) -> Self {
        Self(s.chars().map(|c| c.to_string()).collect())
    }

    pub fn is_prefix_of(&self, other: &KeySequence) -> bool {
        if self.0.len() >= other.0.len() {
            return false;
        }
        self.0.iter().zip(other.0.iter()).all(|(a, b)| a == b)
    }
}

#[derive(Debug, Deserialize)]
pub struct KeybindingConfig {
    pub bindings: HashMap<String, HashMap<String, String>>,
}

pub struct KeybindingManager {
    bindings: HashMap<KeyMode, HashMap<String, KeySequence>>,
}

impl KeybindingManager {
    pub fn with_defaults() -> Self {
        let mut bindings: HashMap<KeyMode, HashMap<String, KeySequence>> = HashMap::new();

        let mut normal = HashMap::new();
        normal.insert("scroll-down".to_string(), KeySequence::from_str("j"));
        normal.insert("scroll-up".to_string(), KeySequence::from_str("k"));
        normal.insert("scroll-top".to_string(), KeySequence::from_str("gt"));
        normal.insert("scroll-bottom".to_string(), KeySequence::from_str("gb"));
        normal.insert("scroll-half-down".to_string(), KeySequence::from_str("C-d"));
        normal.insert("scroll-half-up".to_string(), KeySequence::from_str("C-u"));
        normal.insert("go-back".to_string(), KeySequence::from_str("h"));
        normal.insert("go-forward".to_string(), KeySequence::from_str("l"));
        normal.insert("mode-insert".to_string(), KeySequence::from_str("i"));
        normal.insert("mode-command".to_string(), KeySequence::from_str(":"));

        bindings.insert(KeyMode::Normal, normal);

        Self { bindings }
    }
    pub fn new(config: Option<&KeybindingConfig>) -> Result<Self, String> {
        let mut manager = KeybindingManager::with_defaults();

        if let Some(cfg) = config {
            for (mode_str, map) in &cfg.bindings {
                let mode = match mode_str.as_str() {
                    "normal" => KeyMode::Normal,
                    "insert" => KeyMode::Insert,
                    "search" => KeyMode::Search,
                    "command" => KeyMode::Command,
                    _ => return Err(format!("Unknown mode: {}", mode_str)),
                };

                let mode_map = manager.bindings.entry(mode).or_default();

                for (seq_str, cmd) in map {
                    let seq = KeySequence::from_str(seq_str);

                    for other_seq in mode_map.values() {
                        if seq.is_prefix_of(other_seq) || other_seq.is_prefix_of(&seq) {
                            return Err(format!(
                                "Invalid binding: sequence {:?} for command '{}' conflicts with {:?}",
                                seq, cmd, other_seq
                            ));
                        }
                    }

                    mode_map.insert(cmd.clone(), seq);
                }
            }
        }

        Ok(manager)
    }
}

impl KeybindingManager {
    pub fn export_full_js(&self) -> String {
        let mut js = String::new();

        js.push_str(
            r#"window.appState = {
  mode: "Normal",
  commandBuffer: "",
};
"#,
        );

        js.push_str(
            r#"
const sendCommand = (cmd) => window.ipc.postMessage(cmd);
"#,
        );

        js.push_str(
            r#"
class KeyNode {
  constructor() {
    this.command = null;
    this.children = new Map();
  }
}

class KeyTrie {
  constructor() {
    this.root = new KeyNode();
    this.currentNode = this.root;
  }

  insert(sequence, command) {
    let node = this.root;
    for (const key of sequence) {
      if (!node.children.has(key)) node.children.set(key, new KeyNode());
      node = node.children.get(key);
    }
    node.command = command;
  }

  processKey(key) {
    const nextNode = this.currentNode.children.get(key);
    if (!nextNode) {
      this.currentNode = this.root;
      return null; // invalid sequence
    }
    this.currentNode = nextNode;
    if (nextNode.command) {
      const cmd = nextNode.command;
      this.currentNode = this.root;
      return cmd;
    }
    return undefined; // waiting for next key
  }

  reset() {
    this.currentNode = this.root;
  }
}
"#,
        );

        js.push_str("window.keyTries = {};\n");
        for (mode, map) in &self.bindings {
            let mode_str = match mode {
                KeyMode::Normal => "Normal",
                KeyMode::Insert => "Insert",
                KeyMode::Search => "Search",
                KeyMode::Command => "Command",
            };
            js.push_str(&format!(
                "window.keyTries['{}'] = new KeyTrie();\n",
                mode_str
            ));
            for (cmd, seq) in map {
                let keys: Vec<String> = seq.0.iter().map(|s| format!("\"{}\"", s)).collect();
                js.push_str(&format!(
                    "window.keyTries['{}'].insert([{}], \"{}\");\n",
                    mode_str,
                    keys.join(", "),
                    cmd
                ));
            }
        }

        js.push_str(
            r#"
document.addEventListener("keydown", (e) => {
  e.stopPropagation();

  if (e.key === "Escape" && window.appState.mode !== "Normal") {
    window.appState.mode = "Normal";
    sendCommand("mode-normal");
    if (window.keyTries[window.appState.mode])
      window.keyTries[window.appState.mode].reset();
    e.preventDefault();
    return;
  }

  let key = e.key;
  if (e.ctrlKey) key = "C-" + key;

  const trie = window.keyTries[window.appState.mode];
  if (!trie) return;

  if (window.appState.mode === "Command") {
    if (e.key === "Enter") {
      sendCommand("command:" + window.appState.commandBuffer);
      window.appState.commandBuffer = "";
      window.appState.mode = "Normal";
      sendCommand("mode-normal");
      trie.reset();
      e.preventDefault();
    } else if (e.key.length === 1 && !e.ctrlKey && !e.metaKey) {
      window.appState.commandBuffer += e.key;
      e.preventDefault();
    }
    return;
  }

  const cmd = trie.processKey(key);
  if (cmd) {
    sendCommand(cmd);
    e.preventDefault();
    if (cmd.startsWith("mode-")) {
      window.appState.mode = cmd.split("-")[1][0].toUpperCase() + cmd.split("-")[1].slice(1);
      if (window.appState.mode === "Command") window.appState.commandBuffer = "";
    }
  } else if (cmd === null) {
    trie.reset(); // invalid sequence
  }
});
"#,
        );

        js
    }
}

/* key.rs ends here */
