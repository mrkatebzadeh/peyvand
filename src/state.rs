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
    agent, args::Args, command::Command, cookie::CookieManager, history::History, key::KeyMode,
};
use spdlog::{debug, error};
use std::sync::mpsc::Sender;
use tao::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};
use wry::WebViewBuilder;

const SCROLL_STEP: i32 = 40;
const KEYBINDING_JS: &str = include_str!("./keybindings.js");

fn make_ipc_handler(tx: Sender<Command>) -> impl Fn(Request<String>) + 'static {
    move |req: Request<String>| {
        if let Some(cmd) = req.body().strip_prefix("command:") {
            tx.send(Command::NormalMode).ok();
            match cmd {
                "q" => {
                    tx.send(Command::Exit).ok();
                }
                _ => {
                    error!("Unknown command: {}", cmd);
                }
            };
        }
        match req.body().as_ref() {
            "go-back" => {
                tx.send(Command::GoBack).ok();
            }
            "go-forward" => {
                tx.send(Command::GoForward).ok();
            }
            "mode-normal" => {
                tx.send(Command::NormalMode).ok();
            }
            "mode-insert" => {
                tx.send(Command::InsertMode).ok();
            }
            "mode-command" => {
                tx.send(Command::CmdMode).ok();
            }
            "scroll-down" => {
                tx.send(Command::ScrollDown).ok();
            }
            "scroll-up" => {
                tx.send(Command::ScrollUp).ok();
            }
            "scroll-top" => {
                tx.send(Command::ScrollTop).ok();
            }
            "scroll-bottom" => {
                tx.send(Command::ScrollBottom).ok();
            }
            "scroll-half-down" => {
                tx.send(Command::ScrollHalfDown).ok();
            }
            "scroll-half-up" => {
                tx.send(Command::ScrollHalfUp).ok();
            }
            _ => {}
        }
    }
}

fn make_navigation_handler(nav_tx: Sender<String>) -> impl Fn(String) -> bool + 'static {
    move |url: String| {
        nav_tx.send(url.to_string()).ok();
        true
    }
}

pub struct State {
    pub window: Window,
    pub webview: wry::WebView,
    pub history: History,
    pub key_mode: KeyMode,
    pub cookie_mgr: CookieManager,
}

impl State {
    pub fn new<T, S: AsRef<str>>(
        args: &Args,
        event_loop: &EventLoop<T>,
        url: S,
    ) -> anyhow::Result<(Self, mpsc::Receiver<Command>, mpsc::Receiver<String>)> {
        let (cmd_tx, cmd_rx) = mpsc::channel::<Command>();
        let ipc_handler = make_ipc_handler(cmd_tx.clone());

        let (nav_tx, nav_rx) = mpsc::channel::<String>();
        let nav_handler = make_navigation_handler(nav_tx.clone());

        let window = WindowBuilder::new()
            .with_title(url.as_ref())
            .build(event_loop)?;

        let agent = match &args.user_agent {
            Some(agent) => agent.as_str(),
            None => agent::default_user_agent(),
        };

        let builder = WebViewBuilder::new()
            .with_url(url.as_ref())
            .with_user_agent(agent)
            .with_ipc_handler(ipc_handler)
            .with_initialization_script(KEYBINDING_JS)
            .with_navigation_handler(nav_handler);

        let webview = builder.build(&window)?;

        let cookie_mgr = CookieManager::new(args.cookiefile.clone(), args.cookie_policies.clone());
        cookie_mgr.load_cookies(&webview)?;

        let history = History::new(url.as_ref());
        Ok((
            Self {
                webview,
                window,
                history,
                key_mode: KeyMode::Normal,
                cookie_mgr,
            },
            cmd_rx,
            nav_rx,
        ))
    }
}

impl State {
    pub fn set_url<S: AsRef<str>>(&mut self, url: S) {
        self.history.push(url.as_ref());
        self.window.set_title(self.history.current());
    }

    pub fn go_back(&mut self) {
        self.history.back();
        self.window.set_title(self.history.current());
        let _ = self.webview.evaluate_script("history.back();");
    }

    pub fn go_forward(&mut self) {
        self.history.forward();
        self.window.set_title(self.history.current());
        let _ = self.webview.evaluate_script("history.forward();");
    }

    pub fn get_key_mode(&self) -> KeyMode {
        self.key_mode
    }

    pub fn set_key_mode(&mut self, mode: KeyMode) {
        self.key_mode = mode;

        let script = format!(
            "window.appState = {{ mode: '{mode}' }};",
            mode = match mode {
                KeyMode::Normal => "Normal",
                KeyMode::Insert => "Insert",
                KeyMode::Search => "Search",
                KeyMode::Command => "Command",
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

    pub fn scroll_top(&self) {
        let _ = self.webview.evaluate_script("window.scrollTo(0, 0);");
    }

    pub fn scroll_bottom(&self) {
        let _ = self
            .webview
            .evaluate_script("window.scrollTo(0, document.body.scrollHeight);");
    }

    pub fn scroll_half_down(&self) {
        let script = format!("window.scrollBy(0, {});", SCROLL_STEP * 3);
        let _ = self.webview.evaluate_script(&script);
    }

    pub fn scroll_half_up(&self) {
        let script = format!("window.scrollBy(0, -{});", SCROLL_STEP * 3);
        let _ = self.webview.evaluate_script(&script);
    }

    pub fn exit(&self) {
        let _ = self.cookie_mgr.save_cookies(&self.webview);
    }
}

/* state.rs ends here */
