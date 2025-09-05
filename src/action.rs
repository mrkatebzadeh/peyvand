/* action.rs

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

use strum::AsRefStr;
use strum_macros::{Display, EnumIter, EnumString};
use tao::event_loop::ControlFlow;

use crate::{key::KeyMode, state::State};

#[derive(AsRefStr, Default, Clone, Debug, EnumIter, EnumString, Display)]
#[strum(serialize_all = "kebab-case")]
pub enum Action {
    GoBack,
    GoForward,
    ScrollDown,
    ScrollUp,
    ScrollTop,
    ScrollBottom,
    ScrollHalfUp,
    ScrollHalfDown,
    ShowHelp,
    Exit,
    #[default]
    NormalMode,
    InsertMode,
    CmdMode,
    SearchMode,
    ShowURL,
    ChangeURL(String),
    HardRefreshURL,
    SoftRefreshURL,
    CopyURL,
    PasteURL,
    Search(String),
    SearchNext,
    SearchPrev,
}

impl Action {
    pub fn apply(&self, state: &mut State, control_flow: &mut ControlFlow) {
        match self {
            Action::GoBack => state.go_back(),
            Action::GoForward => state.go_forward(),
            Action::ScrollDown => state.scroll_down(),
            Action::ScrollUp => state.scroll_up(),
            Action::ScrollHalfDown => state.scroll_half_down(),
            Action::ScrollHalfUp => state.scroll_half_up(),
            Action::ScrollTop => state.scroll_top(),
            Action::ScrollBottom => state.scroll_bottom(),
            Action::NormalMode => state.set_key_mode(KeyMode::Normal),
            Action::InsertMode => state.set_key_mode(KeyMode::Insert),
            Action::CmdMode => state.set_key_mode(KeyMode::Cmd),
            Action::SearchMode => state.set_key_mode(KeyMode::Search),
            Action::ShowHelp => state.show_help(),
            Action::ShowURL => {
                state.set_key_mode(KeyMode::Insert);
                state.show_url();
            }
            Action::ChangeURL(url) => {
                state.change_url(url);
                state.set_key_mode(KeyMode::Normal);
            }
            Action::HardRefreshURL => state.refresh_url(true),
            Action::SoftRefreshURL => state.refresh_url(false),
            Action::CopyURL => state.copy_url(),
            Action::PasteURL => state.paste_url(),

            Action::Search(needle) => {
                state.search(needle);
            }
            Action::SearchNext => state.search_next(),
            Action::SearchPrev => state.search_prev(),

            Action::Exit => {
                state.exit();
                *control_flow = ControlFlow::Exit;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_display() {
        let actions = [
            (Action::GoBack, "go-back"),
            (Action::GoForward, "go-forward"),
            (Action::ScrollDown, "scroll-down"),
            (Action::ScrollUp, "scroll-up"),
            (Action::ScrollTop, "scroll-top"),
            (Action::ScrollBottom, "scroll-bottom"),
            (Action::ScrollHalfUp, "scroll-half-up"),
            (Action::ScrollHalfDown, "scroll-half-down"),
            (Action::Exit, "exit"),
            (Action::NormalMode, "normal-mode"),
            (Action::InsertMode, "insert-mode"),
            (Action::CmdMode, "cmd-mode"),
        ];

        for (action, expected) in actions.iter() {
            assert_eq!(format!("{}", action), *expected);
        }
    }
}

/* action.rs ends here */
