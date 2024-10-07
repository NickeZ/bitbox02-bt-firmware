use da14531_sdk::{
    app_modules::app_env_get_conidx,
    bindings::KE_API_ID_TASK_ID_CUSTS1,
    ble_stack::{
        profiles::{
            custom::custs::custs1::task::{
                Custs1ValWriteInd, Custs1ValueReqInd, KeMsgDynCusts1ValueReqRsp,
            },
            prf::prf_get_task_from_id,
        },
        rwble_hl::error::HlErr::GAP_ERR_NO_ERROR as ATT_ERR_NO_ERROR,
    },
    platform::core_modules::{ke::task::KeTaskId, rwip::TASK_APP},
};
use rtt_target::rprintln;

static mut BUF: [u8; 512] = [0; 512];
static mut BUF_LEN: usize = 0;

pub fn data_in_write_handler(param: &Custs1ValWriteInd) {
    if param.length > 512 {
        /* 512 is max length according to spec */
        rprintln!("write, got more bytes than I can handle");
    }
    let input = unsafe { param.value.as_slice(param.length as usize) };
    rprintln!("write got {:?} bytes", input.len());
    let buf = unsafe { &mut BUF[..] };
    unsafe { BUF_LEN = input.len() };
    buf[0..input.len()].copy_from_slice(input)
}

pub fn data_out_read_handler(param: &Custs1ValueReqInd) {
    let buf = unsafe { &BUF[..] };
    let buf_len = unsafe { BUF_LEN };
    rprintln!("read {}", buf_len);
    let mut response = KeMsgDynCusts1ValueReqRsp::<255>::new(
        TASK_APP as u16,
        prf_get_task_from_id(KE_API_ID_TASK_ID_CUSTS1 as KeTaskId),
    );

    let conidx = app_env_get_conidx(param.conidx);

    // Provide the connection index.
    response.fields().conidx = conidx;

    // Provide the attribute index.
    response.fields().att_idx = param.att_idx;

    // Provide length of the payload (bool = 1)
    response.fields().length = buf_len as u16;

    // Provide the ATT error code.
    response.fields().status = ATT_ERR_NO_ERROR as u8;

    // Copy value
    let value = unsafe { response.fields().value.as_mut_slice(255) };

    value[0..buf_len].copy_from_slice(&buf[0..buf_len]);

    response.send();
}
