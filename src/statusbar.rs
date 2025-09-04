/* statusbar.rs

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

const STATUSBAR_JS: &str = r#"
const initStatusBar = () => {
  const statusBar = document.createElement("div");
  statusBar.id = "peyvand-statusbar";
  Object.assign(statusBar.style, {
    position: "fixed",
    bottom: "0px",
    left: "0px",
    width: "100%",
    height: "24px",
    backgroundColor: "rgba(0,0,0,0.8)",
    color: "white",
    fontFamily: "monospace",
    fontSize: "14px",
    paddingLeft: "4px",
    lineHeight: "24px",
    zIndex: "999999",
  });
  statusBar.innerText = "Normal";
  document.body.appendChild(statusBar);

  window.updateStatus = (text) => {
    statusBar.innerText = text;
  };
};

if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", initStatusBar);
} else {
  initStatusBar();
}"#;

pub struct Statusbar {}

impl Statusbar {
    pub fn get_statusbar(&self) -> &str {
        STATUSBAR_JS
    }
    pub fn new() -> Self {
        Self {}
    }
}

/* statusbar.rs ends here */
