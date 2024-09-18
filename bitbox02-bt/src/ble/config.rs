use da14531_sdk::{
    app_modules::app_custs::{custs1::app_custs1_create_db, CustPrfFuncCallbacks},
    app_modules::configure_custom_server1_service,
    app_modules::{
        default_handlers_configuration, ms_to_timer_units, DEF_ADV_WITH_TIMEOUT, DEF_SEC_REQ_NEVER,
    },
    platform::core_modules::rwip::TASK_ID_CUSTS1,
};

// Setup service database
configure_custom_server1_service![abc: {
    uuid: 0x0001,
    characteristics: {},
    //etype: service,
    //uuid16: 0xFD6B // Rapitag 16bit UUID
},];

// Set the advertisement period
const ADV_PERIOD: i32 = ms_to_timer_units(4000) as i32;

// Configure default handlers
default_handlers_configuration! {
    adv_scenario: DEF_ADV_WITH_TIMEOUT,
    advertise_period: ADV_PERIOD,
    security_request_scenario: DEF_SEC_REQ_NEVER
}
