use crate::{
    layouts::{common::Layout, master},
    PorcignoWM,
};

impl PorcignoWM {
    pub fn on_window_created(&self) {
        match self.current_layout {
            Layout::Master => master::master(&self.windows, &self.monitor_size),
        }
    }
}
