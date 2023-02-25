use crate::components::AppwidePanel;
use widgetry::{tools::PopupMsg, EventCtx, Line, State, Text};

pub struct Tutorial {
    appwide_panel: AppwidePanel,
}

impl Tutorial {
    pub fn new_state<A>(ctx: &mut EventCtx) -> Box<dyn State<A>> {}
}

/*
TUTORIAL P1:
Screen 1: Centered -- BTN's
Screen 2: Custom Pos -- Arrow Box Left -- Popup on side needs to be open -- Everything needs to be grayed out except for 1 part
Screen 3: Change bottom panel -- Arrow Box Down -- Everything needs to be grayed out except for 1 part

To achieve the grayed out except for 1 part I can render that part of the popup/panel again only ontop of the grayed part. This way I prevent having to work with masks.
    Check how other tutorial for game achieved this effect.
        It looks like it does not do that.

Event checking. AKA make sure the user has done what it is supposed to do before continueing the tutorial (or next button has to be pressed)


I could just implement my own popups which also take in a position and then just mask out
whatever the user is supposed to focus on.

I currently have no idea on how to achieve the Arrow Boxes
*/
