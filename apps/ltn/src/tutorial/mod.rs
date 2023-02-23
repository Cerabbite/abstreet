use crate::components::AppwidePanel;
use crate::App;
use widgetry::State;

pub struct Tutorial {
    appwide_panel: AppwidePanel,
}

impl Tutorial {
    pub fn new_state() -> Box<dyn State<App>> {}
}
