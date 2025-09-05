/* url.rs

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

const URL_JS: &str = r#"
window.showUrlBar = function(initialUrl = "") {
    if (window.overlays.urlBar) window.overlays.urlBar.remove();

    console.log("Entered input mode");
    const input = document.createElement("input");
    input.id = "url-bar-overlay";
    input.type = "text";
    input.value = initialUrl;
    input.style.position = "fixed";
    input.style.top = "0";
    input.style.left = "50%";
    input.style.transform = "translateX(-50%)";
    input.style.width = "80%";
    input.style.padding = "0.5em";
    input.style.fontSize = "1rem";
    input.style.zIndex = 10000;
    input.style.background = "rgba(255,255,255,0.85)";
    input.style.color = '#000';
    input.style.border = "1px solid #ccc";
    input.style.outline = "none";

    document.body.appendChild(input);
    input.focus();
    window.overlays.urlBar = input;

    function remove() { input.remove(); window.overlays.urlBar = null; document.removeEventListener("keydown", escHandler); }
    function escHandler(e) { if (e.key === "Escape") remove(); }

    input.addEventListener("keydown", (e) => {
        if (e.key === "Enter") {
            window.ipc.postMessage("change-url:" + input.value);
            remove();
        } else if (e.key === "Escape") remove();
    });

    document.addEventListener("keydown", escHandler);
};
"#;

pub struct Url {}

impl Url {
    pub fn get_url(&self) -> &str {
        URL_JS
    }
    pub fn new() -> Self {
        Self {}
    }
}
/* url.rs ends here */
