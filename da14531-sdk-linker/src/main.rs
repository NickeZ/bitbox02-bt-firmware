fn main() {
    // Defines typically given on the command line:
    const CFG_PRODUCTION_TEST: bool = false;
    // In the template, the following is included on the command line:
    // `-include da1458x_config_basic.h -include da1458x_config_advanced.h -include user_config.h`
    //
    // -- da14531_config_basic.h
    const CFG_MAX_CONNECTIONS: i32 = 1;

    // -- da14531_config_advanced.h
    const CFG_RET_DATA_SIZE: i32 = 2048;
    const CFG_RET_DATA_UNINIT_SIZE: i32 = 0;
    const CFG_MAX_TX_PACKET_LENGTH: i32 = 251;
    const CFG_MAX_RX_PACKET_LENGTH: i32 = 251;
    // By default the following 4 constants are commented out.
    // #define DB_HEAP_SZ              1024
    const DB_HEAP_SZ: Option<i32> = None;
    // #define ENV_HEAP_SZ             4928
    const ENV_HEAP_SZ: Option<i32> = None;
    // #define MSG_HEAP_SZ             6880
    const MSG_HEAP_SZ: Option<i32> = None;
    // #define NON_RET_HEAP_SZ         2048
    // XXX(nd): Not used in SDK
    // const NON_RET_HEAP_SZ: Option<i32> = None;
    const CFG_CODE_LOCATION_EXT: i32 = 1;
    const CFG_CODE_LOCATION_OTP: i32 = 0;

    // -- da1458x_stack_config.h
    const CFG_CON: i32 = 3;

    // -- da1458x_scatter_config.h
    // Convert an even integer number to the next odd one
    // #define EVEN_TO_NEXT_ODD_NUM(x)      ((x) | 0x01)
    const fn even_to_next_odd_num(val: i32) -> i32 {
        val | 0x1
    }

    // Align val on the multiple of 4 equal or nearest higher
    // #define ALIGN4_HI(val)               (((val)+3) & (~3))
    const fn align4_hi(val: i32) -> i32 {
        (val + 3) & !3
    }

    const BLE_CONNECTION_MAX_USER: i32 = {
        if CFG_MAX_CONNECTIONS > CFG_CON {
            panic!("Config error: the number of supported connections is exceeded")
        }
        CFG_MAX_CONNECTIONS
    };

    const MAX_PAYLOAD_LENGTH_BY_SPEC: i32 = if CFG_PRODUCTION_TEST { 255 } else { 251 };
    const MIN_PAYLOAD_LENGTH_BY_SPEC: i32 = if CFG_PRODUCTION_TEST { 37 } else { 27 };

    // #if !((CFG_MAX_TX_PACKET_LENGTH >= MIN_PAYLOAD_LENGTH_BY_SPEC) && (CFG_MAX_TX_PACKET_LENGTH <= MAX_PAYLOAD_LENGTH_BY_SPEC))
    if !(CFG_MAX_TX_PACKET_LENGTH >= MIN_PAYLOAD_LENGTH_BY_SPEC
        && CFG_MAX_TX_PACKET_LENGTH <= MAX_PAYLOAD_LENGTH_BY_SPEC)
    {
        panic!("Config error: invalid value for CFG_MAX_TX_PACKET_LENGTH.");
    }
    // #if !((CFG_MAX_RX_PACKET_LENGTH >= MIN_PAYLOAD_LENGTH_BY_SPEC) && (CFG_MAX_RX_PACKET_LENGTH <= MAX_PAYLOAD_LENGTH_BY_SPEC))
    if !(CFG_MAX_RX_PACKET_LENGTH >= MIN_PAYLOAD_LENGTH_BY_SPEC
        && CFG_MAX_RX_PACKET_LENGTH <= MAX_PAYLOAD_LENGTH_BY_SPEC)
    {
        panic!("Config error: invalid value for CFG_MAX_RX_PACKET_LENGTH.");
    }

    const __SCT_CFG_MAX_TX_PACKET_LENGTH: i32 = even_to_next_odd_num(CFG_MAX_TX_PACKET_LENGTH);
    const __SCT_CFG_MAX_RX_PACKET_LENGTH: i32 = even_to_next_odd_num(CFG_MAX_RX_PACKET_LENGTH);

    #[allow(non_upper_case_globals)]
    const __SCT__EM_BLE_TX_BUFFER_COUNT: i32 = CFG_CON * 3 + CFG_CON + 3;
    #[allow(non_upper_case_globals)]
    const __SCT__REG_BLE_EM_TX_BUFFER_SIZE: i32 = 262;

    const __SCT_BLE_TX_DESC_DATA: i32 = if BLE_CONNECTION_MAX_USER == 1 {
        5
    } else {
        BLE_CONNECTION_MAX_USER * 3
    };

    const __SCT_BLE_TX_DESC_CNTL: i32 = BLE_CONNECTION_MAX_USER;
    const __SCT_BLE_TX_DESC_ADV: i32 = 3;
    const __SCT_EM_BLE_TX_BUFFER_COUNT: i32 =
        __SCT_BLE_TX_DESC_DATA + __SCT_BLE_TX_DESC_CNTL + __SCT_BLE_TX_DESC_ADV;
    const __SCT_REG_BLE_EM_TX_BUFFER_SIZE: i32 = __SCT_CFG_MAX_TX_PACKET_LENGTH + 4 + 7;

    const __SCT_EM_BLE_RX_BUFFER_COUNT: i32 = 8;
    #[allow(non_upper_case_globals)]
    const __SCT__REG_BLE_EM_RX_BUFFER_SIZE: i32 = 262;
    const __SCT_REG_BLE_EM_RX_BUFFER_SIZE: i32 = __SCT_CFG_MAX_RX_PACKET_LENGTH + 4 + 7;

    // Only valid for 14535 and 14531
    const __SCT_EM_BLE_TX_BUFFER_OFFSET: i32 = 0x0000033E;

    #[allow(non_upper_case_globals)]
    const __SCT__EM_BLE_RX_BUFFER_OFFSET: i32 = __SCT_EM_BLE_TX_BUFFER_OFFSET
        + (__SCT__EM_BLE_TX_BUFFER_COUNT * __SCT__REG_BLE_EM_TX_BUFFER_SIZE);
    const __SCT_EM_BLE_RX_BUFFER_OFFSET: i32 = __SCT_EM_BLE_TX_BUFFER_OFFSET
        + (__SCT_EM_BLE_TX_BUFFER_COUNT * __SCT_REG_BLE_EM_TX_BUFFER_SIZE);

    #[allow(non_upper_case_globals)]
    const __SCT__EM_BLE_CNXADD_OFFSET: i32 = __SCT__EM_BLE_RX_BUFFER_OFFSET
        + (__SCT_EM_BLE_RX_BUFFER_COUNT * __SCT__REG_BLE_EM_RX_BUFFER_SIZE);
    const __SCT_EM_BLE_CNXADD_OFFSET: i32 = __SCT_EM_BLE_RX_BUFFER_OFFSET
        + (__SCT_EM_BLE_RX_BUFFER_COUNT * __SCT_REG_BLE_EM_RX_BUFFER_SIZE);

    const __SCT_BD_ADDR_LEN: i32 = 6;
    #[allow(non_upper_case_globals)]
    const __SCT__EM_BLE_END: i32 = __SCT__EM_BLE_CNXADD_OFFSET + __SCT_BD_ADDR_LEN;
    const __SCT_EM_BLE_END: i32 = __SCT_EM_BLE_CNXADD_OFFSET + __SCT_BD_ADDR_LEN;

    const __SCT_OFFSET_DIFF: i32 = __SCT__EM_BLE_END - __SCT_EM_BLE_END;

    // Only valid for 14531
    const __SCT_OLD_BLE_BASE: i32 = 0x07FC9C00; // 1Kbyte aligned

    const __SCT_BLE_BASE: i32 = (__SCT_OLD_BLE_BASE + __SCT_OFFSET_DIFF) & 0xFFFFC00;

    const EXCH_MEM_TX_BUFFER_BASE_ADDR: i32 = __SCT_BLE_BASE + __SCT_EM_BLE_TX_BUFFER_OFFSET;
    const EXCH_MEM_TX_CNTL_BUFFER_BASE_ADDR: i32 =
        EXCH_MEM_TX_BUFFER_BASE_ADDR + (__SCT_BLE_TX_DESC_DATA * __SCT_REG_BLE_EM_TX_BUFFER_SIZE);
    const EXCH_MEM_TX_ADV_1_BUFFER_BASE_ADDR: i32 = EXCH_MEM_TX_CNTL_BUFFER_BASE_ADDR
        + (__SCT_BLE_TX_DESC_CNTL * __SCT_REG_BLE_EM_TX_BUFFER_SIZE);
    const EXCH_MEM_TX_ADV_2_BUFFER_BASE_ADDR: i32 =
        EXCH_MEM_TX_ADV_1_BUFFER_BASE_ADDR + __SCT_REG_BLE_EM_TX_BUFFER_SIZE;
    const EXCH_MEM_TX_ADV_3_BUFFER_BASE_ADDR: i32 =
        EXCH_MEM_TX_ADV_2_BUFFER_BASE_ADDR + __SCT_REG_BLE_EM_TX_BUFFER_SIZE;
    const EXCH_MEM_RX_BUFFER_BASE_ADDR: i32 = __SCT_BLE_BASE + __SCT_EM_BLE_RX_BUFFER_OFFSET;
    // XXX(nd): Not used in SDK
    // const EXCH_MEM_CNXADD_BASE_ADDR: i32 = __SCT_BLE_BASE + __SCT_EM_BLE_CNXADD_OFFSET;

    // /*
    //  * The base addresses of the 4 free RAM areas inside the Exchange Memory.
    //  * Consider that, no matter the Tx data packet size configuration is (even if
    //  * DLE is used), the usable occupied length of the Tx CNTL/Tx ADV buffers will
    //  * always be 38 bytes. When max Tx data packet size is used (251 bytes) we can
    //  * save (4 * (262 - 38)) = 896 bytes of RAM space.
    //  */
    const FREE_AREA_AT_TX_CNTL_BUFFER_BASE_ADDR: i32 =
        align4_hi(EXCH_MEM_TX_CNTL_BUFFER_BASE_ADDR + 38);
    const FREE_AREA_AT_TX_ADV_1_BUFFER_BASE_ADDR: i32 =
        align4_hi(EXCH_MEM_TX_ADV_1_BUFFER_BASE_ADDR + 38);
    const FREE_AREA_AT_TX_ADV_2_BUFFER_BASE_ADDR: i32 =
        align4_hi(EXCH_MEM_TX_ADV_2_BUFFER_BASE_ADDR + 38);
    const FREE_AREA_AT_TX_ADV_3_BUFFER_BASE_ADDR: i32 =
        align4_hi(EXCH_MEM_TX_ADV_3_BUFFER_BASE_ADDR + 38);

    // /*
    //  * The sizes of the 4 free RAM areas inside the Exchange Memory.
    //  */
    const FREE_AREA_AT_TX_CNTL_BUFFER_SIZE: i32 =
        EXCH_MEM_TX_ADV_1_BUFFER_BASE_ADDR - FREE_AREA_AT_TX_CNTL_BUFFER_BASE_ADDR; // leftover space in Tx Control buffer
    const FREE_AREA_AT_TX_ADV_1_BUFFER_SIZE: i32 =
        EXCH_MEM_TX_ADV_2_BUFFER_BASE_ADDR - FREE_AREA_AT_TX_ADV_1_BUFFER_BASE_ADDR; // leftover space in Tx ADV 1 buffer
    const FREE_AREA_AT_TX_ADV_2_BUFFER_SIZE: i32 =
        EXCH_MEM_TX_ADV_3_BUFFER_BASE_ADDR - FREE_AREA_AT_TX_ADV_2_BUFFER_BASE_ADDR; // leftover space in Tx ADV 2 buffer
    const FREE_AREA_AT_TX_ADV_3_BUFFER_SIZE: i32 =
        EXCH_MEM_RX_BUFFER_BASE_ADDR - FREE_AREA_AT_TX_ADV_3_BUFFER_BASE_ADDR; // leftover space in Tx ADV 3 buffer

    // /*
    //  * Calculation of the default heap sizes
    //  ****************************************************************************************
    //  */
    //
    // Automatically calculated non retained heap size
    // XXX(nd): Not used in SDK
    // const __SCT_AUTO_HEAP_NON_RET_SIZE: i32 = if BLE_CONNECTION_MAX_USER == 1 {
    //     1024
    // } else {
    //     2048
    // };

    // Automatically calculated ENV heap size (macros used by scatter file)
    // Calculation formula is: ENV heap size = (sizeof(struct llc_env_tag)   + KE_HEAP_MEM_RESERVED +
    //                                          sizeof(struct gapc_env_tag)  + KE_HEAP_MEM_RESERVED +
    //                                          sizeof(struct gattc_env_tag) + KE_HEAP_MEM_RESERVED +
    //                                          sizeof(struct l2cc_env_tag)  + KE_HEAP_MEM_RESERVED) *
    //                                          BLE_CONNECTION_MAX_USER
    // Only valid for 14531
    const __SCT_AUTO_HEAP_ENV_SIZE: i32 =
        ((244 + 4) + (248 + 4) + (148 + 4) + (28 + 4)) * BLE_CONNECTION_MAX_USER;

    // Automatically calculated DB heap size
    const __SCT_AUTO_HEAP_DB_SIZE: i32 = 1024;

    // Automatically calculated MSG heap size
    const __SCT_AUTO_HEAP_MSG_SIZE: i32 = 256 * (BLE_CONNECTION_MAX_USER + 1)
        + 80 * BLE_CONNECTION_MAX_USER
        + 96 * (2 * BLE_CONNECTION_MAX_USER + 1)
        + 256
        + 256 * BLE_CONNECTION_MAX_USER;

    // Align up to a 4 byte boundary + RWIP_HEAP_HEADER
    // #define __SCT_CALC_HEAP_LEN(len)    (__SCT_ALIGN4_UP(len) + 12)
    const fn __sct_calc_heap_len(len: i32) -> i32 {
        align4_hi(len) + 12
    }

    // Not retained heap
    // XXX(nd): Not used in SDK
    // const __SCT_HEAP_NON_RET_SIZE: i32 = if let Some(sz) = NON_RET_HEAP_SZ {
    //     __sct_calc_heap_len(sz)
    // } else {
    //     __sct_calc_heap_len(__SCT_AUTO_HEAP_NON_RET_SIZE)
    // };

    // Env Heap size
    const __SCT_HEAP_ENV_SIZE: i32 = if let Some(sz) = ENV_HEAP_SZ {
        __sct_calc_heap_len(sz)
    } else {
        __sct_calc_heap_len(__SCT_AUTO_HEAP_ENV_SIZE)
    };

    // DB Heap size
    const __SCT_HEAP_DB_SIZE: i32 = if let Some(sz) = DB_HEAP_SZ {
        __sct_calc_heap_len(sz)
    } else {
        __sct_calc_heap_len(__SCT_AUTO_HEAP_DB_SIZE)
    };

    // Msg Heap size
    const __SCT_HEAP_MSG_SIZE: i32 = if let Some(sz) = MSG_HEAP_SZ {
        __sct_calc_heap_len(sz)
    } else {
        __sct_calc_heap_len(__SCT_AUTO_HEAP_MSG_SIZE)
    };

    // The total size of the retained heap memory
    const RET_HEAP_SIZE: i32 = __SCT_HEAP_ENV_SIZE + __SCT_HEAP_DB_SIZE + __SCT_HEAP_MSG_SIZE;

    // sizeof(trng_state)
    const RET_DATA_UNINIT_TRNG_STATE_SIZE: i32 = 4;

    // chacha20.c - sizeof(struct chacha20_state)
    //
    // Note: The variable must be placed in retained RAM area.
    //       The variable must belong to uninitialized RAM area due to
    //       the fact that it must hold its previous value when the system
    //       re-runs the Reset handler and the TRNG mechanism is not
    //       re-triggered.
    const RET_DATA_UNINIT_CHACHA_STATE_SIZE: i32 = 8 + (4 * 4) + (16 * 4) + (1 + 7);

    // Only valid for da14531
    const ROM_DATA_BASE_ADDR: i32 = 0x07FCB900;

    /* ==============================================================================================
     * |                                          System RAM                                        |
     * ----------------------------------------------------------------------------------------------
     * |+ 1st RAM block (16KB)        + 2rd RAM block (12KB)         + 3th RAM block (20KB)         |
     * ----------------------------------------------------------------------------------------------
     * |                              ^                 ^            ^                   ^          |
     * |                              |                 |            |                   |          |
     * |                              |        RET_MEM_BASE_ADDR     |                   |          |
     * |                              |                       RAM_3_BASE_ADDR            |          |
     * |                       RAM_2_BASE_ADDR                                      __SCT_BLE_BASE  |
     * ==============================================================================================
     */

    // /********************************************************************************************
    //  * Memory area where retained data will be stored.
    //  ********************************************************************************************/
    const RET_MEM_SIZE: i32 = CFG_RET_DATA_UNINIT_SIZE + CFG_RET_DATA_SIZE + RET_HEAP_SIZE;

    /* Retained data base address */
    const RET_MEM_BASE_ADDR: i32 = align4_hi(__SCT_BLE_BASE - RET_MEM_SIZE);

    /* chacha20_state base address */
    const CHACHA_STATE_BASE_ADDR: i32 =
        align4_hi(ROM_DATA_BASE_ADDR - RET_DATA_UNINIT_CHACHA_STATE_SIZE);

    /* trng_state base address */
    const TRNG_STATE_BASE_ADDR: i32 = align4_hi(
        ROM_DATA_BASE_ADDR - RET_DATA_UNINIT_CHACHA_STATE_SIZE - RET_DATA_UNINIT_TRNG_STATE_SIZE,
    );

    /********************************************************************************************
     * Free area resides between the Exchange memory and ROM data.
     ********************************************************************************************/
    /* Free area base address */
    const FREE_AREA_BASE_ADDR: i32 = align4_hi(__SCT_BLE_BASE + __SCT_EM_BLE_END);

    /* Free area size */
    const FREE_AREA_SIZE: i32 = (ROM_DATA_BASE_ADDR - FREE_AREA_BASE_ADDR)
        - (RET_DATA_UNINIT_CHACHA_STATE_SIZE + RET_DATA_UNINIT_TRNG_STATE_SIZE);

    // #if defined(CFG_CODE_LOCATION_OTP) && defined(CFG_CODE_LOCATION_EXT)
    //     #error "Only one of CFG_CODE_LOCATION_OTP and CFG_CODE_LOCATION_EXT must be defined!"
    const CODE_LOCATION_OTP: bool = CFG_CODE_LOCATION_OTP == 1;
    const CODE_LOCATION_EXT: bool = CFG_CODE_LOCATION_EXT == 1;
    if CODE_LOCATION_OTP == CODE_LOCATION_EXT {
        panic!("Only one of CFG_CODE_LOCATION_OTP and CFG_CODE_LOCATION_EXT must be defined!");
    }

    /* These defines are specific to DA14531, do not alter. */
    const SRAM_BASE_ADDR: i32 = 0x07fc0000;

    const BOOT_VECTOR_AREA_SZ: i32 = 0xC0;
    const PATCH_TABLE_AREA_SZ: i32 = 0x50;

    /* OTP memory size = 32K*/
    const OTP_MEM_SIZE: i32 = 32 * 1024;

    /* OTP header section size = 64 bytes*/
    const OTP_HEADER_SIZE: i32 = 64;

    /* OTP CS section size = 240 bytes*/
    const OTP_CS_SIZE: i32 = 240;

    /* Useful OTP memory size*/
    const OTP_MEM_USEFUL_SIZE: i32 = OTP_MEM_SIZE - OTP_HEADER_SIZE - OTP_CS_SIZE;

    /* Base address of code (RAM base address + interrupt vector table size + patch table size)*/
    const CODE_AREA_BASE: i32 = SRAM_BASE_ADDR + BOOT_VECTOR_AREA_SZ + PATCH_TABLE_AREA_SZ;

    /* Max needs in RAM per application - excluding the retained data, the interrupt vector table and the patch table*/
    const CODE_AREA_MAX_SIZE: i32 = RET_MEM_BASE_ADDR - CODE_AREA_BASE;

    const CODE_AREA_SIZE: i32 = if CODE_LOCATION_OTP {
        OTP_MEM_USEFUL_SIZE - (BOOT_VECTOR_AREA_SZ + PATCH_TABLE_AREA_SZ)
    } else {
        // CODE_LOCATION_EXT = true
        CODE_AREA_MAX_SIZE
    };

    println!("MEMORY");
    println!("{{");
    println!("    LR_IROM1          (rwx) : ORIGIN = {SRAM_BASE_ADDR:#X},                                              LENGTH = {BOOT_VECTOR_AREA_SZ:#X}");
    println!("    LR_IROM2          (rwx) : ORIGIN = {SRAM_BASE_ADDR:#X} + {BOOT_VECTOR_AREA_SZ:#X},                        LENGTH = {PATCH_TABLE_AREA_SZ:#X}");
    println!("    LR_IROM3          (rwx) : ORIGIN = {CODE_AREA_BASE:#X},                                              LENGTH = {CODE_AREA_SIZE:#X}");
    println!("    LR_RETAINED_RAM0  (rw)  : ORIGIN = {RET_MEM_BASE_ADDR:#X},                                           LENGTH = {RET_MEM_SIZE:#X}");
    println!("    /* After this there's only BLE Exchange Memory, externally defined by the __SCT_BLE_BASE address and with custom zeroing code in arch_rom.c */");
    println!();
    println!("    /* Free area to be used by the application (free areas are zero initialized after reset) */");
    if CFG_MAX_TX_PACKET_LENGTH > 27 {
        println!("    LR_FREE_AREA_AT_TX_CNTL_BUFFER      (rwx)  : ORIGIN = {FREE_AREA_AT_TX_CNTL_BUFFER_BASE_ADDR:#X},    LENGTH = {FREE_AREA_AT_TX_CNTL_BUFFER_SIZE:#X}");
        println!("    LR_FREE_AREA_AT_TX_ADV_1_BUFFER     (rwx)  : ORIGIN = {FREE_AREA_AT_TX_ADV_1_BUFFER_BASE_ADDR:#X},   LENGTH = {FREE_AREA_AT_TX_ADV_1_BUFFER_SIZE:#X}");
        println!("    LR_FREE_AREA_AT_TX_ADV_2_BUFFER     (rwx)  : ORIGIN = {FREE_AREA_AT_TX_ADV_2_BUFFER_BASE_ADDR:#X},   LENGTH = {FREE_AREA_AT_TX_ADV_2_BUFFER_SIZE:#X}");
        println!("    LR_FREE_AREA_AT_TX_ADV_3_BUFFER     (rwx)  : ORIGIN = {FREE_AREA_AT_TX_ADV_3_BUFFER_BASE_ADDR:#X},   LENGTH = {FREE_AREA_AT_TX_ADV_3_BUFFER_SIZE:#X}");
    }
    println!("    LR_FREE_AREA                        (rwx)  : ORIGIN = {FREE_AREA_BASE_ADDR:#X},                      LENGTH = {FREE_AREA_SIZE:#X}");
    println!("    /* Fixed area used by TRNG */");
    println!("    LR_RETAINED_TRNG_STATE   (rw)  : ORIGIN = {TRNG_STATE_BASE_ADDR:#X},                                 LENGTH = {RET_DATA_UNINIT_TRNG_STATE_SIZE:#X}");
    println!("    /* Fixed area used by CHACHA20 */");
    println!("    LR_RETAINED_CHACHA_STATE (rw)  : ORIGIN = {CHACHA_STATE_BASE_ADDR:#X},                               LENGTH = {RET_DATA_UNINIT_CHACHA_STATE_SIZE:#X}");
    println!("}}");
}
