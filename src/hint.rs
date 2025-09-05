/* hint.rs

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

const HINT_JS: &str = r#"

window.hintState = {
  active: false,
  hints: [],
  buffer: ""
};

window.showHints = function() {
  window.clearHints();
  window.hintState.active = true;
  window.hintState.buffer = "";

  const links = document.querySelectorAll("a, button, [role='link']");
  let index = 0;

  links.forEach(link => {
    const label = index.toString(36); // base36 (0-9, a-z) short labels
    index++;

    const rect = link.getBoundingClientRect();
    if (rect.width === 0 && rect.height === 0) return;

    const hint = document.createElement("div");
    hint.className = "pey-hint";
    hint.innerText = label.toUpperCase();
    hint.style.position = "absolute";
    hint.style.left = `${window.scrollX + rect.left}px`;
    hint.style.top = `${window.scrollY + rect.top}px`;
    hint.style.background = "red";
    hint.style.color = "white";
    hint.style.fontSize = "12px";
    hint.style.fontWeight = "bold";
    hint.style.padding = "2px 4px";
    hint.style.borderRadius = "3px";
    hint.style.zIndex = 99999;

    document.body.appendChild(hint);

    window.hintState.hints.push({ label: label.toUpperCase(), link, hint });
  });
};

window.clearHints = function() {
  window.hintState.hints.forEach(h => h.hint.remove());
  window.hintState.hints = [];
  window.hintState.active = false;
  window.hintState.buffer = "";
};
"#;

pub struct Hint {}

impl Hint {
    pub fn get_js() -> &'static str {
        HINT_JS
    }
}

/* hint.rs ends here */
