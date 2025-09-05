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

use crate::action::Action;
use serde::Deserialize;
use std::collections::HashMap;
use std::str::FromStr;
use strum::Display;
use strum::{AsRefStr, EnumIter, EnumString};

#[derive(
    AsRefStr, Default, Clone, Copy, Debug, EnumIter, EnumString, Display, PartialEq, Eq, Hash,
)]
pub enum KeyMode {
    #[default]
    Normal,
    Insert,
    Search,
    Cmd,
}

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
        normal.insert(Action::ScrollDown.to_string(), KeySequence::from_str("j"));
        normal.insert(Action::ScrollUp.to_string(), KeySequence::from_str("k"));
        normal.insert(Action::ScrollTop.to_string(), KeySequence::from_str("gt"));
        normal.insert(
            Action::ScrollBottom.to_string(),
            KeySequence::from_str("gb"),
        );
        normal.insert(
            Action::ScrollHalfDown.to_string(),
            KeySequence::from_str("C-d"),
        );
        normal.insert(
            Action::ScrollHalfUp.to_string(),
            KeySequence::from_str("C-u"),
        );
        normal.insert(Action::GoBack.to_string(), KeySequence::from_str("h"));
        normal.insert(Action::GoForward.to_string(), KeySequence::from_str("l"));
        normal.insert(Action::InsertMode.to_string(), KeySequence::from_str("i"));
        normal.insert(Action::CmdMode.to_string(), KeySequence::from_str(":"));

        normal.insert(Action::ShowHelp.to_string(), KeySequence::from_str("?"));
        normal.insert(Action::ShowURL.to_string(), KeySequence::from_str("go"));
        normal.insert(
            Action::HardRefreshURL.to_string(),
            KeySequence::from_str("R"),
        );
        normal.insert(
            Action::SoftRefreshURL.to_string(),
            KeySequence::from_str("r"),
        );
        normal.insert(Action::CopyURL.to_string(), KeySequence::from_str("uy"));
        normal.insert(Action::PasteURL.to_string(), KeySequence::from_str("up"));

        normal.insert(Action::SearchNext.to_string(), KeySequence::from_str("n"));
        normal.insert(Action::SearchPrev.to_string(), KeySequence::from_str("N"));
        normal.insert(Action::SearchMode.to_string(), KeySequence::from_str("/"));
        bindings.insert(KeyMode::Normal, normal);

        Self { bindings }
    }
    pub fn new(config: Option<&KeybindingConfig>) -> Result<Self, String> {
        let mut manager = KeybindingManager::with_defaults();

        if let Some(cfg) = config {
            for (mode_str, map) in &cfg.bindings {
                let mode = KeyMode::from_str(&mode_str.to_lowercase())
                    .map_err(|_| format!("Unknown mode: {}", mode_str))?;

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
    pub fn get_help_map(&self, mode: KeyMode) -> HashMap<String, String> {
        self.bindings
            .get(&mode)
            .map(|map| {
                map.iter()
                    .map(|(key, seq)| (key.clone(), seq.0.join("")))
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn export_full_js(&self) -> String {
        let mut js = String::new();

        js.push_str(
            r#"window.appState = {
  mode: "Normal",
  commandBuffer: "",
  searchBuffer: "",
};
"#,
        );

        js.push_str(
            r#"
const sendAction = (cmd) => window.ipc.postMessage(cmd);
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
            let mode_str = mode.to_string();

            if mode_str != "Cmd" {
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
        }

        js.push_str(
            r#"
document.addEventListener("keydown", (e) => {
  e.stopPropagation();

  if (e.key === "Escape" && window.appState.mode !== "Normal") {
    window.appState.mode = "Normal";
    sendAction("normal-mode");
    window.updateStatus(window.appState.mode);
    if (window.keyTries[window.appState.mode])
      window.keyTries[window.appState.mode].reset();
    e.preventDefault();
    return;
  }

  let key = e.key;
  if (e.ctrlKey) key = "C-" + key;

  if (window.appState.mode === "Cmd") {
    if (key === "Enter") {
      sendAction("command:" + window.appState.commandBuffer);
      window.appState.commandBuffer = "";
      window.appState.mode = "Normal";
      sendAction("normal-mode");

      window.updateStatus(window.appState.mode);
      e.preventDefault();
    } else if (key === "Backspace") {
        if (window.appState.commandBuffer && window.appState.commandBuffer.length > 0) {
            window.appState.commandBuffer = window.appState.commandBuffer.slice(0, -1);
            window.updateStatus(":" + window.appState.commandBuffer);
        }
        e.preventDefault();
     }else if (key.length === 1 && !e.ctrlKey && !e.metaKey) {
      window.appState.commandBuffer = window.appState.commandBuffer || "";
      window.appState.commandBuffer += key;

      window.updateStatus(":" + window.appState.commandBuffer);
      e.preventDefault();
    }
    return;
  } else if (window.appState.mode === "Search") {
    if (key === "Enter") {
      window.appState.mode = "Normal";
      sendAction("search:" + window.appState.searchBuffer);
      sendAction("normal-mode");
      window.updateStatus(window.appState.mode);
      e.preventDefault();
    } else if (key === "Backspace") {
        if (window.appState.searchBuffer && window.appState.searchBuffer.length > 0) {
            window.appState.searchBuffer = window.appState.searchBuffer.slice(0, -1);
            window.updateStatus("/" + window.appState.searchBuffer);
        }
        e.preventDefault();
     }else if (key.length === 1 && !e.ctrlKey && !e.metaKey) {
      window.appState.searchBuffer = window.appState.searchBuffer || "";
      window.appState.searchBuffer += key;
      window.updateStatus("/" + window.appState.searchBuffer);
      e.preventDefault();
    }
    return;
  }
  else {

  const trie = window.keyTries[window.appState.mode];
  if (!trie) return;

  const cmd = trie.processKey(key);
  if (cmd) {
    sendAction(cmd);
    e.preventDefault();

  const modeCommands = ["normal-mode", "insert-mode", "cmd-mode", "search-mode"];

  if (modeCommands.includes(cmd)) {
    let displayMode = "";
    switch (cmd) {
        case "normal-mode":
            displayMode = "Normal";
            window.updateStatus(displayMode);
            break;
        case "insert-mode":
            displayMode = "Insert";
            window.updateStatus(displayMode);
            break;
        case "cmd-mode":
            displayMode = "Cmd";
            window.appState.commandBuffer = "";
            window.updateStatus(":");
            break;
        case "search-mode":
            displayMode = "Search";

            window.appState.searchBuffer = "";
            window.updateStatus("/");
            break;
    }

    window.appState.mode = displayMode;

    const newTrie = window.keyTries[displayMode];
    if (newTrie) newTrie.reset();
}
    } else if (cmd === null) {
    trie.reset(); // invalid sequence
   }
  }
});
"#,
        );

        js.push_str(
r#"
window.overlays = window.overlays || {};

window.showHelp = function(bindings) {
    if (window.overlays.help) window.overlays.help.remove();

    const overlay = document.createElement("div");
    overlay.id = "help-overlay";
    overlay.style.position = "fixed";
    overlay.style.bottom = "0";
    overlay.style.left = "0";
    overlay.style.width = "100%";
    overlay.style.maxHeight = "40%";
    overlay.style.background = "rgba(0,0,0,0.85)";
    overlay.style.color = '#eee';
    overlay.style.zIndex = 9999;
    overlay.style.fontFamily = "monospace";
    overlay.style.overflowY = "auto";
    overlay.style.padding = "1em";
    overlay.style.borderTop = "2px solid #ccc";

    let content = "<h3>Key Bindings</h3><ul>";
    for (const key in bindings) {
        content += `<li><b>${key}</b> — ${bindings[key]}</li>`;
    }
    content += "</ul><p>Press ESC to close</p>";
    overlay.innerHTML = content;

    document.body.appendChild(overlay);
    window.overlays.help = overlay;

    function remove() { overlay.remove(); window.overlays.help = null; document.removeEventListener("keydown", escHandler); }
    function escHandler(e) { if (e.key === "Escape") remove(); }
    overlay.addEventListener("click", remove);
    document.addEventListener("keydown", escHandler);
};
"#
);

        js
    }
}

/* key.rs ends here */
