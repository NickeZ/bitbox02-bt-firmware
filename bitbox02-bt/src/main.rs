#![no_std]
#![no_main]

extern crate alloc;

#[cfg(debug_assertions)]
use panic_halt as _;

#[cfg(not(debug_assertions))]
use panic_abort as _;

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
        rwble_hl::error::HlErr::GAP_ERR_CANCELED,
    },
    platform::{arch::register_main_loop_callbacks, core_modules::crypto::aes_init},
    register_user_operation_adv,
};
use rtt_target::rtt_init_print;

use da14531_sdk::allocator::Da14531Allocator;

/// The actual application code and definition of interfaces for peripheral and BLE drivers
pub mod app;
/// Glue between SDK system and application code
pub mod app_impl;
/// BLE
pub mod ble;
/// HAL for peripherals
pub mod peripherals;

#[global_allocator]
static ALLOCATOR: Da14531Allocator = Da14531Allocator;

#[no_mangle]
fn user_on_connection(_conidx: u8, _param: *const da14531_sdk::bindings::gapc_connection_req_ind) {}

#[no_mangle]
fn user_on_disconnect(_param: *const da14531_sdk::bindings::gapc_disconnect_ind) {}
