/* app.rs

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

use std::sync::Mutex;

use crate::command::Command;
use crate::key::KeyMode;
use crate::{args::Args, state::State};
use spdlog::{debug, info};
use tao::event::DeviceEvent::Key;
use tao::{
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::KeyCode,
};

pub struct Application {
    args: Args,
}

impl Application {
    pub fn start(&mut self) -> anyhow::Result<()> {
        let event_loop = EventLoop::new();
        let (state, cmd_rx, nav_rx) = State::new(&event_loop, &self.args.url)?;
        let state = Mutex::new(state);
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            while let Ok(url) = nav_rx.try_recv() {
                state.lock().unwrap().set_url(url);
            }
            while let Ok(cmd) = cmd_rx.try_recv() {
                let mode = state.lock().unwrap().get_key_mode();
                debug!("Command: {:#?}", cmd);
                match mode {
                    KeyMode::Normal => match cmd {
                        Command::GoBack => state.lock().unwrap().go_back(),
                        Command::GoForward => state.lock().unwrap().go_forward(),

                        Command::ScrollDown => state.lock().unwrap().scroll_down(),
                        Command::ScrollUp => state.lock().unwrap().scroll_up(),

                        Command::ModeNormal => state.lock().unwrap().set_key_mode(KeyMode::Normal),
                        Command::ModeInsert => state.lock().unwrap().set_key_mode(KeyMode::Insert),
                        Command::Exit => *control_flow = ControlFlow::Exit,
                        _ => todo!(),
                    },
                    KeyMode::Insert => {
                        if let Command::ModeNormal = cmd {
                            state.lock().unwrap().set_key_mode(KeyMode::Normal);
                        }
                    }
                    _ => unimplemented!(),
                }
            }
            match event {
                Event::NewEvents(StartCause::Init) => info!("Webview started"),
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    info!("Peyvand exiting");
                    *control_flow = ControlFlow::Exit
                }
                Event::DeviceEvent {
                    device_id,
                    event: Key(raw_key),
                    ..
                } => {
                    info!("A key is pressed: {:#?}", raw_key);
                    match raw_key.physical_key {
                        KeyCode::KeyH => {
                            info!("h is pressed");
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        });
    }
}

impl Application {
    pub fn new(args: Args) -> Self {
        Self { args }
    }
}

/* app.rs ends here */
