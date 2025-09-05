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

use crate::action::Action;
use crate::key::KeyMode;
use crate::{args::Args, state::State};
use spdlog::{debug, info};
use std::sync::Mutex;
use tao::{
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

pub struct Application {
    args: Args,
}

impl Application {
    pub fn start(&mut self) -> anyhow::Result<()> {
        let event_loop = EventLoop::new();
        let (state, act_rx, nav_rx) = State::new(&self.args, &event_loop, &self.args.url)?;
        let state = Mutex::new(state);
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            while let Ok(url) = nav_rx.try_recv() {
                state.lock().unwrap().set_url(url);
            }
            while let Ok(act) = act_rx.try_recv() {
                dispatch_act(&state, control_flow, act);
            }
            handle_event(event, control_flow);
        });
    }
}

fn handle_event(event: Event<'_, ()>, control_flow: &mut ControlFlow) {
    match event {
        Event::NewEvents(StartCause::Init) => info!("Webview started"),
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => {
            info!("Peyvand exiting");
            *control_flow = ControlFlow::Exit
        }
        _ => {}
    }
}

fn dispatch_act(state: &Mutex<State>, control_flow: &mut ControlFlow, act: Action) {
    let mode = state.lock().unwrap().get_key_mode();
    debug!("Action in {mode} mode: {:#?}", act);
    act.apply(&mut state.lock().unwrap(), control_flow)
}

impl Application {
    pub fn new(args: Args) -> Self {
        Self { args }
    }
}

/* app.rs ends here */
