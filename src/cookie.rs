/* cookie.rs

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

use spdlog::error;
use std::path::PathBuf;
use wry::{WebView, cookie::Cookie};

pub struct CookieManager {
    file: Option<PathBuf>,
    policy: CookiePolicy,
}

#[derive(Clone, Copy)]
pub enum CookiePolicy {
    All,
    None,
    SessionOnly,
}

impl CookieManager {
    pub fn new(file: Option<String>, policy: Option<String>) -> Self {
        let policy = match policy.as_deref() {
            Some("all") => CookiePolicy::All,
            Some("none") => CookiePolicy::None,
            Some("session") => CookiePolicy::SessionOnly,
            _ => CookiePolicy::All,
        };

        Self {
            file: file.map(PathBuf::from),
            policy,
        }
    }

    pub fn load_cookies(&self, webview: &WebView) -> anyhow::Result<()> {
        if let Some(path) = &self.file {
            if path.exists() {
                let data = std::fs::read_to_string(path)?;
                for line in data.lines() {
                    if let Some((name, value)) = line.split_once('=') {
                        let cookie = Cookie::new(name.trim(), value.trim());
                        if self.accept(&cookie) {
                            webview.set_cookie(&cookie)?;
                        }
                    }
                }
            } else {
                error!("Path not found {}", path.display());
            }
        }
        Ok(())
    }

    pub fn save_cookies(&self, webview: &WebView) -> anyhow::Result<()> {
        if let Some(path) = &self.file {
            let cookies = webview.cookies()?;
            let mut content = String::new();
            for cookie in cookies {
                if self.accept(&cookie) {
                    content.push_str(&format!("{}={}\n", cookie.name(), cookie.value()));
                }
            }
            std::fs::write(path, content)?;
        }
        Ok(())
    }

    fn accept(&self, cookie: &Cookie) -> bool {
        match self.policy {
            CookiePolicy::All => true,
            CookiePolicy::None => false,
            CookiePolicy::SessionOnly => cookie.expires().is_none(),
        }
    }
}

/* cookie.rs ends here */
