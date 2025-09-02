/* history.rs

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

#[derive(Debug)]
pub struct History {
    stack: Vec<String>,
    current_index: usize,
}

impl Default for History {
    fn default() -> Self {
        let initial_url = "https://mr.katebzadeh.xyz";
        Self {
            stack: vec![initial_url.to_string()],
            current_index: 0,
        }
    }
}
impl History {
    pub fn new(initial_url: &str) -> Self {
        Self {
            stack: vec![initial_url.to_string()],
            current_index: 0,
        }
    }

    pub fn push(&mut self, url: &str) {
        self.stack.truncate(self.current_index + 1);
        self.stack.push(url.to_string());
        self.current_index += 1;
    }

    pub fn back(&mut self) -> Option<&str> {
        if self.current_index > 0 {
            self.current_index -= 1;
            self.stack.get(self.current_index).map(|s| s.as_str())
        } else {
            None
        }
    }

    pub fn forward(&mut self) -> Option<&str> {
        if self.current_index + 1 < self.stack.len() {
            self.current_index += 1;
            self.stack.get(self.current_index).map(|s| s.as_str())
        } else {
            None
        }
    }

    pub fn current(&self) -> &str {
        &self.stack[self.current_index]
    }
}

/* history.rs ends here */
