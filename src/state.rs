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

use std::sync::mpsc;
use wry::http::Request;

use crate::{
    command::Command,
    history::{self, History},
    key::KeyMode,
};
use spdlog::debug;
use tao::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};
use wry::WebViewBuilder;

const SCROLL_STEP: i32 = 40;

pub struct State {
    window: Window,
    webview: wry::WebView,
    history: History,
    key_mode: KeyMode,
}

impl State {
    pub fn new<T, S: AsRef<str>>(
        event_loop: &EventLoop<T>,
        url: S,
    ) -> anyhow::Result<(Self, mpsc::Receiver<Command>)> {
        let (tx, rx) = mpsc::channel::<Command>();
        let window = WindowBuilder::new()
            .with_title(url.as_ref())
            .build(event_loop)?;

        let builder = WebViewBuilder::new()
            .with_url(url.as_ref())
            .with_ipc_handler(move |req: Request<String>| match req.body().as_ref() {
                "go-back" => {
                    tx.send(Command::GoBack).ok();
                }
                "go-forward" => {
                    tx.send(Command::GoForward).ok();
                }
                "mode-normal" => {
                    tx.send(Command::ModeNormal).ok();
                }
                "mode-insert" => {
                    tx.send(Command::ModeInsert).ok();
                }
                "scroll-down" => {
                    tx.send(Command::ScrollDown).ok();
                }
                "scroll-up" => {
                    tx.send(Command::ScrollUp).ok();
                }
                _ => {}
            })
            .with_initialization_script(
                r#"
                window.appState = { mode: 'Normal' };
            document.addEventListener('keydown', (e) => {
                e.stopPropagation();

                const mode = window.appState.mode;
            if (mode === 'Normal') {
                if (e.key === 'h') {
                    window.ipc.postMessage('go-back');
                    e.preventDefault();
                } else if (e.key === 'l') {
                    window.ipc.postMessage('go-forward');
                    e.preventDefault();
                } else if (e.key === 'j') {
                    window.ipc.postMessage('scroll-down');
                    e.preventDefault();
                } else if (e.key === 'k') {
                    window.ipc.postMessage('scroll-up');
                    e.preventDefault();
                } else if (e.key === 'i') {
                    window.ipc.postMessage('mode-insert');
                    e.preventDefault();
                }
            } else if (mode === 'Insert') {
                if (e.key === 'Escape') {
                    window.ipc.postMessage('mode-normal');
                    e.preventDefault();
                }
            }
            });
        "#,
            );

        let webview = builder.build(&window)?;

        let history = History::new(url.as_ref());
        Ok((
            Self {
                webview,
                window,
                history,
                key_mode: KeyMode::Normal,
            },
            rx,
        ))
    }
}

impl State {
    pub fn set_url<S: AsRef<str>>(&mut self, url: S) {
        self.history.push(url.as_ref());
        self.webview.load_url(url.as_ref()).unwrap();
    }

    pub fn go_back(&self) {
        let _ = self.webview.evaluate_script("history.back();");
    }

    pub fn go_forward(&self) {
        let _ = self.webview.evaluate_script("history.forward();");
    }

    pub fn get_key_mode(&self) -> KeyMode {
        self.key_mode
    }

    pub fn set_key_mode(&mut self, mode: KeyMode) {
        self.key_mode = mode;

        let script = format!(
            "window.appState = {{ mode: '{}' }};",
            match mode {
                KeyMode::Normal => "Normal",
                KeyMode::Insert => "Insert",
                KeyMode::Search => "Search",
            }
        );

        debug!("Mode: {:#?}", mode);
        let _ = self.webview.evaluate_script(&script);
    }

    pub fn scroll_down(&self) {
        let script = format!("window.scrollBy(0, {});", SCROLL_STEP);
        let _ = self.webview.evaluate_script(&script);
    }

    pub fn scroll_up(&self) {
        let script = format!("window.scrollBy(0, -{});", SCROLL_STEP);
        let _ = self.webview.evaluate_script(&script);
    }
}

/* state.rs ends here */
