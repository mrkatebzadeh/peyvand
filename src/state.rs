/* state.rs

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

use crate::history::{self, History};
use dpi::{LogicalPosition, LogicalSize};
use spdlog::debug;
use tao::{
    event::WindowEvent,
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

use wry::WebViewBuilder;

#[derive(Default)]
pub struct State {
    window: Option<Window>,
    webview: Option<wry::WebView>,
    history: History,
}

impl State {
    pub fn new<T, S: AsRef<str>>(event_loop: &EventLoop<T>, url: S) -> anyhow::Result<Self> {
        let window = WindowBuilder::new().build(event_loop)?;

        let builder = WebViewBuilder::new()
            .with_url(url.as_ref())
            .with_new_window_req_handler(|url, features| {
                debug!("new window req: {url} {features:?}");
                wry::NewWindowResponse::Allow
            });

        #[cfg(feature = "drag-drop")]
        let builder = builder.with_drag_drop_handler(|e| {
            match e {
                wry::DragDropEvent::Enter { paths, position } => {
                    println!("DragEnter: {position:?} {paths:?} ")
                }
                wry::DragDropEvent::Over { position } => println!("DragOver: {position:?} "),
                wry::DragDropEvent::Drop { paths, position } => {
                    println!("DragDrop: {position:?} {paths:?} ")
                }
                wry::DragDropEvent::Leave => println!("DragLeave"),
                _ => {}
            }

            true
        });

        #[cfg(any(
            target_os = "windows",
            target_os = "macos",
            target_os = "ios",
            target_os = "android"
        ))]
        let webview = builder.build(&window)?;
        #[cfg(not(any(
            target_os = "windows",
            target_os = "macos",
            target_os = "ios",
            target_os = "android"
        )))]
        let webview = {
            use tao::platform::unix::WindowExtUnix;
            use wry::WebViewBuilderExtUnix;
            let vbox = window.default_vbox().unwrap();
            builder.build_gtk(vbox)?
        };

        let history = History::new(url.as_ref());
        Ok(Self {
            webview: Some(webview),
            window: Some(window),
            history,
        })
    }
}

impl State {
    pub fn set_url<S: AsRef<str>>(&mut self, url: S) {
        self.history.push(url.as_ref());
        if let Some(webview) = &self.webview {
            webview.load_url(url.as_ref()).unwrap();
        }
    }

    pub fn go_back(&mut self) {
        if let Some(previous_url) = self.history.back() {
            if let Some(webview) = &self.webview {
                webview.load_url(previous_url).unwrap();
            }
        }
    }

    pub fn go_forward(&mut self) {
        if let Some(next_url) = self.history.forward() {
            if let Some(webview) = &self.webview {
                webview.load_url(next_url).unwrap();
            }
        }
    }
}

/* state.rs ends here */
