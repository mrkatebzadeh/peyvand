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

use std::fmt::Display;

#[derive(Default, Clone, Copy, Debug)]
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

/* key.rs ends here */
