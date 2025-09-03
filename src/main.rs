/* main.rs

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

mod agent;
mod app;
mod args;
mod command;
mod history;
mod key;
mod signal;
mod state;

use app::Application;
use signal::setup_signal_handlers;
use spdlog::{Level, LevelFilter, Logger, debug, info};
use std::sync::Arc;

fn main() -> anyhow::Result<()> {
    let default_logger: Arc<Logger> = spdlog::default_logger();

    let args = args::parse();

    let level = match args.verbose {
        0 => Level::Error,
        1 => Level::Warn,
        2 => Level::Info,
        3 => Level::Debug,
        _ => Level::Trace,
    };

    default_logger.set_level_filter(LevelFilter::MoreSevereEqual(level));

    let pid = std::process::id();
    info!("Started: Peyvand, PID: {}", pid);
    debug!("{:#?}", args);

    setup_signal_handlers()?;
    debug!("Finished: setup signals");

    let mut app = Application::new(args);
    app.start()
}

/* main.rs ends here */
