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
  keyBuffer: [],
  keyTimeout: null,
};
"#,
        );

        js.push_str(
            r#"
const sendCommand = (cmd) => window.ipc.postMessage(cmd);

const resetKeyBuffer = () => {
  window.appState.keyBuffer = [];
  if (window.appState.keyTimeout) {
    clearTimeout(window.appState.keyTimeout);
    window.appState.keyTimeout = null;
  }
};
"#,
        );

        js.push_str("window.keybindings = {\n");
        for (mode, map) in &self.bindings {
            let mode_str = match mode {
                KeyMode::Normal => "Normal",
                KeyMode::Insert => "Insert",
                KeyMode::Search => "Search",
                KeyMode::Command => "Command",
            };
            js.push_str(&format!("  \"{}\": {{\n", mode_str));
            for (cmd, seq) in map {
                let seq_str = seq.0.join("");
                js.push_str(&format!("    \"{}\": \"{}\",\n", seq_str, cmd));
            }
            js.push_str("  },\n");
        }
        js.push_str("};\n");

        js.push_str(
            r#"
function handleKey(e) {
  e.stopPropagation();

  if (e.key === "Escape" && window.appState.mode !== "Normal") {
    window.appState.mode = "Normal";
    sendCommand("mode-normal");
    window.appState.commandBuffer = "";
    resetKeyBuffer();
    e.preventDefault();
    return;
  }

  let modeBindings = window.keybindings[window.appState.mode];
  let key = e.key;
  if (e.ctrlKey) key = "C-" + key;

  if (window.appState.mode === "Command") {
    if (e.key === "Enter") {
      sendCommand("command:" + window.appState.commandBuffer);
      window.appState.commandBuffer = "";
      window.appState.mode = "Normal";
      sendCommand("mode-normal");
      resetKeyBuffer();
      e.preventDefault();
    } else if (e.key.length === 1 && !e.ctrlKey && !e.metaKey) {
      window.appState.commandBuffer += e.key;
      e.preventDefault();
    }
    return;
  }

  window.appState.keyBuffer.push(key);
  if (window.appState.keyTimeout) clearTimeout(window.appState.keyTimeout);
  window.appState.keyTimeout = setTimeout(resetKeyBuffer, 300);

  const seq = window.appState.keyBuffer.join("");
  const cmd = modeBindings[seq];
  if (cmd) {
    sendCommand(cmd);
    resetKeyBuffer();
    e.preventDefault();

    if (cmd.startsWith("mode-")) {
      window.appState.mode = cmd.split("-")[1][0].toUpperCase() + cmd.split("-")[1].slice(1);
      if (window.appState.mode === "Command") {
        window.appState.commandBuffer = "";
      }
    }
  }
}

document.addEventListener("keydown", handleKey);
"#,
        );

        js
    }
}

/* key.rs ends here */
