use crate::option::LocomotiveOption;

use super::Core;

impl Core {
    pub fn check_smooth_scroll(options: &LocomotiveOption) -> bool {
        if (options.smooth && !options.is_mobile)  || 
        (options.tablet.as_ref().unwrap().smooth && options.is_tablet) ||
        (options.smartphone.as_ref().unwrap().smooth && options.is_mobile && !options.is_tablet) {
            true
        } else {
            false
        }
    }
}