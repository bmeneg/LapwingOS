#![allow(dead_code)]

use crate::klib::sync::SingleThreadData;

#[derive(Copy, Clone, PartialEq)]
pub enum InitPhase {
    Hardware,
    Protocol,
}

pub trait DriverDescriptor {
    fn init(&self) -> Result<(), &'static str>;
}

#[derive(Copy, Clone)]
pub struct DeviceDriver {
    pub description: &'static str,
    pub descriptor: &'static (dyn DriverDescriptor),
    pub phase: InitPhase,
}

// TODO: make number of drivers a compile time thing instead of a hardcoded
// value, preventing we forgotting to set it correctly.
// For now, we plan to have 2 drivers only:
// 1. GPIO
// 2. UART
const NUM_DRIVERS: usize = 2;

pub struct DriversManager {
    drivers: [Option<DeviceDriver>; NUM_DRIVERS],
    num_registered: usize,
}

static DRIVERS_MANAGER_ONCE: SingleThreadData<DriversManager> =
    SingleThreadData::new(DriversManager::new());

impl DriversManager {
    const fn new() -> Self {
        Self {
            drivers: [None; NUM_DRIVERS],
            num_registered: 0,
        }
    }

    pub fn instance() -> &'static mut Self {
        DRIVERS_MANAGER_ONCE.inner()
    }

    pub fn register_driver(&mut self, driver: DeviceDriver) {
        assert!(self.num_registered <= NUM_DRIVERS);
        self.drivers[self.num_registered] = Some(driver);
        self.num_registered += 1;
    }

    // This method initializes all drivers the manager has registered
    // within, thus instead of returning a boolean for each (or even a final
    // boolean for all) we panic!
    // At the same time, we allow initializing one init phase at a time.
    pub fn init_drivers(&self, init_phase: Option<InitPhase>) {
        if init_phase.is_none() {
            for driver in &self.drivers {
                // Just return in case there isn't any driver to be loaded
                if let Some(drv) = driver {
                    match drv.descriptor.init() {
                        Ok(_) => (), // we still lack print macros
                        Err(_) => panic!("failed to initialize {}", drv.description),
                    }
                }
            }
            return;
        }

        // Count the number of drivers in each initialization phase so we
        // avoid traversing the drivers list for those phases with no driver
        // at all.
        let mut phase_count = [0; core::intrinsics::variant_count::<InitPhase>()];
        for driver in &self.drivers {
            if let Some(drv) = driver {
                phase_count[drv.phase as usize] += 1;
            }
        }

        for (curr_phase, num_drivers) in phase_count.into_iter().enumerate() {
            if num_drivers > 0 {
                self.drivers
                    .iter()
                    .filter(|drv| curr_phase == drv.unwrap().phase as usize)
                    .for_each(|drv| match drv.unwrap().descriptor.init() {
                        Ok(_) => (), // we still lack print macros
                        Err(_) => panic!("failed to initialize {}", drv.unwrap().description),
                    });
            }
        }
    }
}
