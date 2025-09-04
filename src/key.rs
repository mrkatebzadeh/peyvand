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
    bindings: HashMap<KeyMode, HashMap<KeySequence, Command>>,
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
            for (seq, cmd) in map {
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
  window.appState.keyTimeout = setTimeout(resetKeyBuffer, 1500);

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
impl KeybindingManager {
    pub fn with_defaults() -> Self {
        let mut bindings: HashMap<KeyMode, HashMap<KeySequence, Command>> = HashMap::new();

        let mut normal = HashMap::new();
        normal.insert(KeySequence::from_str("j"), "scroll-down".to_string());
        normal.insert(KeySequence::from_str("k"), "scroll-up".to_string());
        normal.insert(KeySequence::from_str("gt"), "scroll-top".to_string());
        normal.insert(KeySequence::from_str("gb"), "scroll-bottom".to_string());
        normal.insert(KeySequence::from_str("C-d"), "scroll-half-down".to_string());
        normal.insert(KeySequence::from_str("C-u"), "scroll-half-up".to_string());
        normal.insert(KeySequence::from_str("h"), "go-back".to_string());
        normal.insert(KeySequence::from_str("l"), "go-forward".to_string());
        normal.insert(KeySequence::from_str("i"), "mode-insert".to_string());
        normal.insert(KeySequence::from_str(":"), "mode-command".to_string());

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

                    for existing_seq in mode_map.keys() {
                        if seq.is_prefix_of(existing_seq) || existing_seq.is_prefix_of(&seq) {
                            return Err(format!(
                                "Invalid binding: {:?} conflicts with prefix {:?} in mode {:?}",
                                seq, existing_seq, mode
                            ));
                        }
                    }

                    mode_map.insert(seq, cmd.clone());
                }
            }
        }

        Ok(manager)
    }

    pub fn get_command(&self, mode: KeyMode, input: &str) -> Option<&Command> {
        let seq = KeySequence::from_str(input);
        self.bindings.get(&mode)?.get(&seq)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keybindings() {
        let toml_str = r#"
            [bindings.normal]
            j = "scroll_down"
            gk = "scroll_top"
            gnl = "scroll_bottom"
        "#;

        let config: KeybindingConfig = toml::from_str(toml_str).unwrap();
        let manager = KeybindingManager::new(Some(&config)).unwrap();

        assert_eq!(
            manager.get_command(KeyMode::Normal, "j"),
            Some(&"scroll_down".to_string())
        );
        assert_eq!(
            manager.get_command(KeyMode::Normal, "gk"),
            Some(&"scroll_top".to_string())
        );
        assert_eq!(
            manager.get_command(KeyMode::Normal, "gnl"),
            Some(&"scroll_bottom".to_string())
        );
        assert_eq!(manager.get_command(KeyMode::Normal, "gf"), None);
    }
}

/* key.rs ends here */
