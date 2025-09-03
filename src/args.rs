/* args.rs

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

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, term_width = 80)]
pub struct Args {
    /// Cookie policies
    #[arg(short = 'a', default_value = "all")]
    pub cookie_policies: Option<String>,

    /// Scrollbars
    #[arg(short = 'b')]
    pub scrollbars: bool,

    /// Cookie file path
    #[arg(short = 'c')]
    pub cookiefile: Option<String>,

    /// Style file path
    #[arg(short = 'C')]
    pub stylefile: Option<String>,

    /// Disk cache
    #[arg(short = 'd')]
    pub diskcache: bool,

    /// Embed window id
    #[arg(short = 'e')]
    pub embed: Option<u32>,

    /// Run in fullscreen
    #[arg(short = 'f')]
    pub fullscreen: bool,

    /// Geolocation
    #[arg(short = 'g')]
    pub geolocation: bool,

    /// Load images
    #[arg(short = 'i')]
    pub loadimages: bool,

    /// Style dark
    #[arg(short = 'm')]
    pub dark_mode: bool,

    /// Inspector
    #[arg(short = 'n')]
    pub inspector: bool,

    /// JavaScript
    #[arg(short = 's')]
    pub javascript: bool,

    /// Strict TLS
    #[arg(short = 't')]
    pub stricttls: bool,

    /// User agent string
    #[arg(short = 'u')]
    pub user_agent: Option<String>,

    #[arg(short, long, action = clap::ArgAction::Count)]
    #[arg(help = "Increment verbosity level (repeat for more detail, e.g., -vvv)")]
    pub verbose: u8,

    /// Positional URL
    #[arg(default_value = "about:blank")]
    pub url: String,
}

#[allow(unused)]
pub(crate) fn parse() -> Args {
    Args::parse()
}
/* args.rs ends here */
