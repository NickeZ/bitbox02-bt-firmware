use core::marker::PhantomData;

use rtt_target::{rprint, rprintln};

use da14531_sdk::app_modules::configure_device_information_service;

/// Stop advertising and go to hibernation after X secs
const APP_ADVERTISTE_STOP_TIMER: u32 = 60;

/// Defines an interface to access the peripherals
pub trait PeripheralsDriver {
    fn new() -> Self;
    fn feed_watchdog(&mut self);
}

/// Defines an interface to control the BLE stack
pub trait BleDriver {
    fn start_adverstising();
    fn stop_adverstising();
    fn disconnect(connection_handle: u32);
}

/// Holds the state of the application
pub struct App<P, BLE>
where
    Self: 'static,
    P: 'static + PeripheralsDriver,
    BLE: 'static + BleDriver,
{
    ///
    peripherals: Option<P>,
    connection_handle: Option<u32>,
    _ble: PhantomData<BLE>,
}

/// Business logic of the application
impl<P, BLE> App<P, BLE>
where
    P: PeripheralsDriver,
    BLE: BleDriver,
{
    /// Create new instance of App
    pub const fn new() -> Self {
        Self {
            peripherals: None,
            _ble: PhantomData,
            connection_handle: None,
        }
    }

    /// Initialize peripherals
    pub fn init_peripherals(&mut self) {
        rprint!("Initializing peripherals...");

        self.peripherals = Some(P::new());

        rprintln!("done!");
    }

    /// Start advertising handler
    pub fn on_start_advertising(&mut self) {
        rprintln!("App::on_start_advertising()");

        BLE::start_adverstising();
    }

    /// Connect event handler
    pub fn on_connect(&mut self, connection_handle: Option<u32>) {
        rprintln!("on_connect");
        self.connection_handle = connection_handle;
    }

    /// Disonnect event handler
    pub fn on_disconnect(&mut self) {
        rprintln!("on_disconnect");
        self.connection_handle = None;
    }

    pub fn feed_watchdog(&mut self) {
        self.peripherals().feed_watchdog();
    }

    pub fn peripherals(&mut self) -> &mut P {
        self.peripherals.as_mut().unwrap()
    }
}

configure_device_information_service! {
    manufacturer_name: "BitBox Swiss"
}

#[no_mangle]
pub static USER_DEVICE_NAME: &str = "bitbox";
