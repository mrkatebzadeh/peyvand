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

use crate::{args::Args, state::State};
use winit::event_loop::EventLoop;

pub struct Application {
    args: Args,
    state: State,
}

impl Application {
    pub fn start(&mut self) -> anyhow::Result<()> {
        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd",
        ))]
        {
            use gtk::prelude::DisplayExtManual;

            gtk::init().unwrap();
            if gtk::gdk::Display::default().unwrap().backend().is_wayland() {
                panic!("This example doesn't support wayland!");
            }

            winit::platform::x11::register_xlib_error_hook(Box::new(|_display, error| {
                let error = error as *mut x11_dl::xlib::XErrorEvent;
                (unsafe { (*error).error_code }) == 170
            }));
        }

        let event_loop = EventLoop::new().unwrap();
        let mut state = State::default();
        state.set_url(&self.args.url);
        event_loop.run_app(&mut state).unwrap();
        Ok(())
    }
}

impl Application {
    pub fn new(args: Args) -> Self {
        Self {
            args,
            state: State::default(),
        }
    }
}

/* app.rs ends here */
