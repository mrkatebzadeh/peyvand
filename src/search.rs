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

window.clearHighlights = function () {
  const marks = document.querySelectorAll('mark.pey-search-highlight');
  marks.forEach(mark => {
    const parent = mark.parentNode;
    if (!parent) return;
    while (mark.firstChild) parent.insertBefore(mark.firstChild, mark);
    parent.removeChild(mark);
    parent.normalize();
  });
  window.searchState.matches = [];
  window.searchState.index = -1;
};

function escapeRegExp(s) {
  return s.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}

window.searchHighlight = function (needle) {
  window.clearHighlights();
  if (!needle) return;

  const root = document.body || document.documentElement;
  if (!root) return;

  const safe = escapeRegExp(needle);
  const re = new RegExp(safe, 'gi');

  const walker = document.createTreeWalker(
    root,
    NodeFilter.SHOW_TEXT,
    {
      acceptNode(node) {
        const text = node.nodeValue;
        if (!text || !text.trim()) return NodeFilter.FILTER_REJECT;

        const p = node.parentNode;
        if (!p) return NodeFilter.FILTER_REJECT;

        const tag = p.nodeName;
        if (tag === 'SCRIPT' || tag === 'STYLE' || tag === 'NOSCRIPT') {
          return NodeFilter.FILTER_REJECT;
        }
        if (
          p.closest(
            '.pey-search-highlight, #pey-status, #help-overlay, #url-bar-overlay'
          )
        ) {
          return NodeFilter.FILTER_REJECT;
        }
        return NodeFilter.FILTER_ACCEPT;
      }
    },
    false
  );

  const textNodes = [];
  let n;
  while ((n = walker.nextNode())) {
    if (n.nodeValue.match(re)) textNodes.push(n);
  }

  textNodes.forEach(node => {
    const text = node.nodeValue;
    let lastIndex = 0;
    const frag = document.createDocumentFragment();

    text.replace(re, (match, offset) => {
      if (offset > lastIndex) {
        frag.appendChild(
          document.createTextNode(text.slice(lastIndex, offset))
        );
      }
      const mark = document.createElement('mark');
      mark.className = 'pey-search-highlight';
      mark.textContent = match;
      frag.appendChild(mark);

      lastIndex = offset + match.length;
      return match;
    });

    if (lastIndex < text.length) {
      frag.appendChild(document.createTextNode(text.slice(lastIndex)));
    }

    node.parentNode.replaceChild(frag, node);
  });

  window.searchState.matches = Array.from(
    document.querySelectorAll('mark.pey-search-highlight')
  );
  window.searchState.index = -1;

   if (window.searchState.matches.length) {
     window.searchState.matches[0].scrollIntoView({ behavior: 'instant', block: 'center' });
   }
};
window.searchNext = function () {
  if (!window.searchState.matches.length) return;

  window.searchState.matches.forEach(m => {
    m.style.background = "yellow";
  });

  window.searchState.index =
    (window.searchState.index + 1) % window.searchState.matches.length;

  const el = window.searchState.matches[window.searchState.index];
  el.scrollIntoView({ behavior: "smooth", block: "center" });
  el.style.background = "red";
};

window.searchPrev = function () {
  if (!window.searchState.matches.length) return;

  window.searchState.matches.forEach(m => {
    m.style.background = "yellow";
  });

  window.searchState.index =
    (window.searchState.index - 1 + window.searchState.matches.length) %
    window.searchState.matches.length;

  const el = window.searchState.matches[window.searchState.index];
  el.scrollIntoView({ behavior: "smooth", block: "center" });
  el.style.background = "red";
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
