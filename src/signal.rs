/* signal.rs

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

use libc::{self, WNOHANG};
use signal_hook::{consts::signal::*, iterator::Signals};
use std::thread;

pub fn setup_signal_handlers() -> anyhow::Result<()> {
    let mut signals = Signals::new([SIGHUP, SIGCHLD])?;
    thread::spawn(move || {
        for signal in signals.forever() {
            match signal {
                SIGHUP => handle_sighup(),
                SIGCHLD => handle_sigchld(),
                _ => unreachable!(),
            }
        }
    });
    Ok(())
}

fn handle_sigchld() {
    let mut status = 0;
    println!("Received SIGCHLD");
    unsafe { while libc::waitpid(-1, &mut status, WNOHANG) > 0 {} }
}

fn handle_sighup() {
    // Implement the logic to handle SIGHUP
    println!("Received SIGHUP");
}

/* signal.rs ends here */
