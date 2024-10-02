//use core::ptr::addr_of_mut;
use da14531_sdk::{
    app_modules::{
        app_common::app::app_prf_enable, app_env_get_conidx, default_app_on_init,
        register_app_callbacks,
    },
    bindings::default_app_on_disconnect,
    ble_stack::{
        host::gap::{
            gapc::task::{GapcConnectionReqInd, GapcDisconnectInd},
            GAP_INVALID_CONIDX,
        },
        //rwble_hl::error::HlErr::GAP_ERR_CANCELED,
    },
    platform::{arch::register_main_loop_callbacks, core_modules::crypto::aes_init},
    register_user_operation_adv,
};
use rtt_target::{rprintln, rtt_init_print, ChannelMode::NoBlockSkip};

use crate::{app::App, ble::Da14531Ble, peripherals::Da14531Peripherals};

/// Defines the `Da14531App` for convenience
type Da14531App = App<Da14531Peripherals, Da14531Ble>;

/// The actual instance of the app struct
static mut APP: Da14531App = Da14531App::new();

/// Get a mutable reference to the app
pub fn app() -> &'static mut Da14531App {
    unsafe { &mut APP }
}

#[allow(non_upper_case_globals)]

/// Initialize peripherals
#[no_mangle]
pub extern "C" fn periph_init() {
    rtt_init_print!(NoBlockSkip, 640);
    //let uart_cfg = da14531_sdk::bindings::uart_cfg_t {
    //    _bitfield_align_1: [],
    //    _bitfield_1: da14531_sdk::bindings::uart_cfg_t::new_bitfield_1(
    //        // Set Baud Rate
    //        da14531_sdk::bindings::UART_BAUDRATE_UART_BAUDRATE_115200,
    //        // Set data bits
    //        da14531_sdk::bindings::UART_DATABITS_UART_DATABITS_8,
    //        // Set parity
    //        da14531_sdk::bindings::UART_PARITY_UART_PARITY_NONE,
    //        // Set stop bits
    //        da14531_sdk::bindings::UART_STOPBITS_UART_STOPBITS_1,
    //        // Set flow control
    //        da14531_sdk::bindings::UART_AFCE_CFG_UART_AFCE_DIS,
    //        // Set FIFO enable
    //        da14531_sdk::bindings::UART_FIFO_CFG_UART_FIFO_EN,
    //        // Set Tx FIFO trigger level
    //        da14531_sdk::bindings::UART_TX_FIFO_LEVEL_UART_TX_FIFO_LEVEL_0,
    //        // Set Rx FIFO trigger level
    //        da14531_sdk::bindings::UART_RX_FIFO_LEVEL_UART_RX_FIFO_LEVEL_0,
    //    ),
    //    // Set interrupt priority
    //    intr_priority: 2,
    //    uart_err_cb: None,
    //    uart_tx_cb: None,
    //    uart_rx_cb: None,
    //};
    //let UART1: *mut da14531_sdk::bindings::uart_t =
    //    da14531_sdk::bindings::UART_RBR_THR_DLL_REG as *mut da14531_sdk::bindings::uart_t;

    //unsafe {
    //    da14531_sdk::bindings::uart_initialize(UART1, &uart_cfg);
    //    da14531_sdk::bindings::GPIO_ConfigurePin(
    //        da14531_sdk::bindings::GPIO_PORT_GPIO_PORT_0,
    //        da14531_sdk::bindings::GPIO_PIN_GPIO_PIN_5,
    //        da14531_sdk::bindings::GPIO_PUPD_OUTPUT,
    //        da14531_sdk::bindings::GPIO_FUNCTION_PID_UART1_TX,
    //        false,
    //    );
    //    da14531_sdk::bindings::GPIO_ConfigurePin(
    //        da14531_sdk::bindings::GPIO_PORT_GPIO_PORT_0,
    //        da14531_sdk::bindings::GPIO_PIN_GPIO_PIN_0,
    //        da14531_sdk::bindings::GPIO_PUPD_INPUT,
    //        da14531_sdk::bindings::GPIO_FUNCTION_PID_UART1_RX,
    //        false,
    //    );
    //    da14531_sdk::bindings::GPIO_set_pad_latch_en(true);
    //    da14531_sdk::bindings::printf_string(UART1, b"test\n\r\0".as_ptr() as *mut u8);
    //}

    app().init_peripherals();
}

// Register handler for `default_operation_adv` as default app operation
register_user_operation_adv!(app_advertising_start_callback);

/// Trigger advertising in app
#[inline]
fn app_advertising_start_callback() {
    app().on_start_advertising();
}
// Register the app_on_init handler
register_main_loop_callbacks! {
    app_on_init: app_on_init_callback,
}

/// Initialize AES and run `default_app_on_init` from SDK
#[inline]
pub fn app_on_init_callback() {
    aes_init(false);
    app().init();
    //unsafe {
    //    rprintln!("testing ke_malloc...");
    //    if da14531_sdk::bindings::ke_check_malloc(4, 3) {
    //        let ptr = da14531_sdk::bindings::ke_malloc(4, 3) as *mut u32;
    //        *ptr = 1;
    //        rprintln!("{}", *ptr);
    //    }
    //    rprintln!("testing ke_malloc... done");
    //}

    default_app_on_init();
}

// Register app callback handlers
register_app_callbacks! {
    app_on_connection: user_app_connection,
    app_on_adv_undirect_complete: user_app_adv_undirect_complete,
    app_on_disconnect: user_app_disconnect,
    // app_on_addr_resolve_failed: todo!(),
    // app_on_addr_solved_ind: todo!(),
    // app_on_csrk_exch: todo!(),
}

#[inline]
pub fn user_app_adv_undirect_complete(_status: u8) {
    //if status == GAP_ERR_CANCELED as u8 {
    //    app().on_start_hibernation();
    //}
}

#[inline]
pub fn user_app_connection(conidx: u8, _param: &GapcConnectionReqInd) {
    if app_env_get_conidx(conidx) != GAP_INVALID_CONIDX as u8 {
        app_prf_enable(conidx);

        app().on_connect(Some(conidx as u32));
    } else {
        app().on_connect(None);
    }
}

#[inline]
pub fn user_app_disconnect(_param: &GapcDisconnectInd) {
    unsafe { default_app_on_disconnect(core::ptr::null()) };

    app().on_disconnect();
}
