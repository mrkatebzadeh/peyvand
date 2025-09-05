/* search.rs

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

const SEARCH_JS: &str = r#"
window.searchState = {
  matches: [],
  index: -1
};

window.clearHighlights = function() {
  document.querySelectorAll(".pey-search-highlight").forEach(el => {
    el.replaceWith(...el.childNodes);
  });
  window.searchState.matches = [];
  window.searchState.index = -1;
};

window.searchHighlight = function(needle) {
  window.clearHighlights();
  if (!needle) return;

  const regex = new RegExp(needle, "gi");
  const walker = document.createTreeWalker(document.body, NodeFilter.SHOW_TEXT);
  let node;

  while ((node = walker.nextNode())) {
    if (regex.test(node.nodeValue)) {
      const span = document.createElement("span");
      span.innerHTML = node.nodeValue.replace(regex, m => `<mark class="pey-search-highlight">${m}</mark>`);
      node.replaceWith(span);
    }
  }
  window.searchState.matches = Array.from(document.querySelectorAll(".pey-search-highlight"));
  window.searchState.index = -1;
};

window.searchNext = function() {
  if (!window.searchState.matches.length) return;
  window.searchState.index = (window.searchState.index + 1) % window.searchState.matches.length;
  const el = window.searchState.matches[window.searchState.index];
  el.scrollIntoView({ behavior: "smooth", block: "center" });
  el.style.background = "orange";
};

window.searchPrev = function() {
  if (!window.searchState.matches.length) return;
  window.searchState.index = (window.searchState.index - 1 + window.searchState.matches.length) % window.searchState.matches.length;
  const el = window.searchState.matches[window.searchState.index];
  el.scrollIntoView({ behavior: "smooth", block: "center" });
  el.style.background = "orange";
};

"#;

#[derive(Default)]
pub struct Search {}

impl Search {
    pub fn get_js() -> &'static str {
        SEARCH_JS
    }
}

/* search.rs ends here */
