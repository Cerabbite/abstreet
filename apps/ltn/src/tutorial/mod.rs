use crate::components::AppwidePanel;
use crate::App;
use widgetry::State;

pub struct Tutorial {
    appwide_panel: AppwidePanel,
}

impl Tutorial {
    pub fn new_state() -> Box<dyn State<App>> {}
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
*/
