mod gpio;

use crate::klib::device;

pub fn devices_build() {
    let manager = device::manager();
    manager.register_driver(gpio::build());
}
