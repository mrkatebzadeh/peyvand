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
    #[arg(short = 'a')]
    pub cookie_policies: Option<String>,

    /// Scrollbars off
    #[arg(short = 'b', conflicts_with = "scrollbars_on")]
    pub scrollbars_off: bool,

    /// Scrollbars on
    #[arg(short = 'B', conflicts_with = "scrollbars_off")]
    pub scrollbars_on: bool,

    /// Cookie file path
    #[arg(short = 'c')]
    pub cookiefile: Option<String>,

    /// Style file path
    #[arg(short = 'C')]
    pub stylefile: Option<String>,

    /// Disk cache off
    #[arg(short = 'd', conflicts_with = "diskcache_on")]
    pub diskcache_off: bool,

    /// Disk cache on
    #[arg(short = 'D', conflicts_with = "diskcache_off")]
    pub diskcache_on: bool,

    /// Embed window id
    #[arg(short = 'e')]
    pub embed: Option<u32>,

    /// Run in fullscreen off
    #[arg(short = 'f', conflicts_with = "fullscreen_on")]
    pub fullscreen_off: bool,

    /// Run in fullscreen on
    #[arg(short = 'F', conflicts_with = "fullscreen_off")]
    pub fullscreen_on: bool,

    /// Geolocation off
    #[arg(short = 'g', conflicts_with = "geolocation_on")]
    pub geolocation_off: bool,

    /// Geolocation on
    #[arg(short = 'G', conflicts_with = "geolocation_off")]
    pub geolocation_on: bool,

    /// Load images off
    #[arg(short = 'i', conflicts_with = "loadimages_on")]
    pub loadimages_off: bool,

    /// Load images on
    #[arg(short = 'I', conflicts_with = "loadimages_off")]
    pub loadimages_on: bool,

    /// Kiosk mode off
    #[arg(short = 'k', conflicts_with = "kiosk_on")]
    pub kiosk_off: bool,

    /// Kiosk mode on
    #[arg(short = 'K', conflicts_with = "kiosk_off")]
    pub kiosk_on: bool,

    /// Style light
    #[arg(short = 'm', conflicts_with = "style_dark")]
    pub style_light: bool,

    /// Style dark
    #[arg(short = 'M', conflicts_with = "style_light")]
    pub style_dark: bool,

    /// Inspector off
    #[arg(short = 'n', conflicts_with = "inspector_on")]
    pub inspector_off: bool,

    /// Inspector on
    #[arg(short = 'N', conflicts_with = "inspector_off")]
    pub inspector_on: bool,

    /// Plugins off
    #[arg(short = 'p', conflicts_with = "plugins_on")]
    pub plugins_off: bool,

    /// Plugins on
    #[arg(short = 'P', conflicts_with = "plugins_off")]
    pub plugins_on: bool,

    /// Script file
    #[arg(short = 'r')]
    pub scriptfile: Option<String>,

    /// JavaScript off
    #[arg(short = 's', conflicts_with = "javascript_on")]
    pub javascript_off: bool,

    /// JavaScript on
    #[arg(short = 'S', conflicts_with = "javascript_off")]
    pub javascript_on: bool,

    /// Strict TLS off
    #[arg(short = 't', conflicts_with = "stricttls_on")]
    pub stricttls_off: bool,

    /// Strict TLS on
    #[arg(short = 'T', conflicts_with = "stricttls_off")]
    pub stricttls_on: bool,

    /// User agent string
    #[arg(short = 'u')]
    pub user_agent: Option<String>,

    #[arg(short, long, action = clap::ArgAction::Count)]
    #[arg(help = "Increment verbosity level (repeat for more detail, e.g., -vvv)")]
    pub verbose: u8,

    /// Show XID
    #[arg(short = 'w')]
    pub show_xid: bool,

    /// Certificate off
    #[arg(short = 'x', conflicts_with = "certificate_on")]
    pub certificate_off: bool,

    /// Certificate on
    #[arg(short = 'X', conflicts_with = "certificate_off")]
    pub certificate_on: bool,

    /// Zoom level
    #[arg(short = 'z')]
    pub zoom: Option<f32>,

    /// Positional URL
    #[arg(default_value = "about:blank")]
    pub url: String,
}

#[allow(unused)]
pub(crate) fn parse() -> Args {
    Args::parse()
}
/* args.rs ends here */
