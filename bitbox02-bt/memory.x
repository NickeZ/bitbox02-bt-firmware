/*
 * ==============================================================================================
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

// -- from da1458x_scatter_config.h
// #define __SCT_OLD_BLE_BASE  (0x07FC9C00) // 1Kbyte aligned
// #define __SCT_BLE_BASE      ((__SCT_OLD_BLE_BASE + __SCT_OFFSET_DIFF) & (0xFFFFC00))
// #define __SCT_ALIGN4_UP(len)        ((((len) + 3) / 4) * 4)
// // Align up to a 4 byte boundary + RWIP_HEAP_HEADER
// #define __SCT_CALC_HEAP_LEN(len)    (__SCT_ALIGN4_UP(len) + 12)
// // Not retained heap
// #ifndef NON_RET_HEAP_SZ
//     #define __SCT_HEAP_NON_RET_SIZE      __SCT_CALC_HEAP_LEN( __SCT_AUTO_HEAP_NON_RET_SIZE )// in bytes
// #else
//     #define __SCT_HEAP_NON_RET_SIZE      __SCT_CALC_HEAP_LEN( NON_RET_HEAP_SZ )             // in bytes
// #endif
// 
// // Env Heap size
// #ifndef ENV_HEAP_SZ
//     #define __SCT_HEAP_ENV_SIZE         __SCT_CALC_HEAP_LEN( __SCT_AUTO_HEAP_ENV_SIZE ) // in bytes
// #else
//     #define __SCT_HEAP_ENV_SIZE         __SCT_CALC_HEAP_LEN( ENV_HEAP_SZ )              // in bytes
// #endif
// 
// // DB Heap size
// #ifndef DB_HEAP_SZ
//     #define __SCT_HEAP_DB_SIZE          __SCT_CALC_HEAP_LEN( __SCT_AUTO_HEAP_DB_SIZE )     // in bytes
// #else
//     #define __SCT_HEAP_DB_SIZE          __SCT_CALC_HEAP_LEN( DB_HEAP_SZ )                  // in bytes
// #endif
// 
// // Msg Heap size
// #ifndef MSG_HEAP_SZ
//     #define __SCT_HEAP_MSG_SIZE         __SCT_CALC_HEAP_LEN( __SCT_AUTO_HEAP_MSG_SIZE )  // in bytes
// #else
//     #define __SCT_HEAP_MSG_SIZE         __SCT_CALC_HEAP_LEN( MSG_HEAP_SZ )               // in bytes
// #endif
// #define RET_HEAP_SIZE           ( __SCT_HEAP_ENV_SIZE + __SCT_HEAP_DB_SIZE + __SCT_HEAP_MSG_SIZE )


// -- from mem_DA14531.lds
// #define SRAM_BASE_ADDR      0x07fc0000
// #define BOOT_VECTOR_AREA_SZ 0xC0
// #define PATCH_TABLE_AREA_SZ 0x50

// #define CODE_AREA_BASE          (SRAM_BASE_ADDR + BOOT_VECTOR_AREA_SZ + PATCH_TABLE_AREA_SZ)

// #define RET_MEM_SIZE        (CFG_RET_DATA_UNINIT_SIZE + CFG_RET_DATA_SIZE + RET_HEAP_SIZE)
// #define RET_MEM_BASE_ADDR    ALIGN4_HI(__SCT_BLE_BASE - RET_MEM_SIZE)

// #define CODE_AREA_MAX_SIZE      (RET_MEM_BASE_ADDR - CODE_AREA_BASE)
// #define CODE_AREA_SIZE          (CODE_AREA_MAX_SIZE)


MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  LR_IROM1          (rwx) : ORIGIN = 0x07fc0000,                                                  LENGTH = 0xC0
  LR_IROM2          (rwx) : ORIGIN = 0x07fc00C0,                                                  LENGTH = 0x50
  LR_IROM3          (rwx) : ORIGIN = 0x07fc0110,                                                  LENGTH = CODE_AREA_SIZE
  LR_RETAINED_RAM0  (rw)  : ORIGIN = RET_MEM_BASE_ADDR,                                           LENGTH = RET_MEM_SIZE
  /* After this there's only BLE Exchange Memory, externally defined by the __SCT_BLE_BASE address and with custom zeroing code in arch_rom.c */

  /* Free area to be used by the application (change attribute to rw if used) */
  LR_FREE           (r)          : ORIGIN = FREE_AREA_BASE_ADDR,                                  LENGTH = FREE_AREA_SIZE
  /* Fixed area used by TRNG */
  LR_RETAINED_TRNG_STATE   (rw)  : ORIGIN = TRNG_STATE_BASE_ADDR,                                 LENGTH = RET_DATA_UNINIT_TRNG_STATE_SIZE
  /* Fixed area used by CHACHA20 */
  LR_RETAINED_CHACHA_STATE (rw)  : ORIGIN = CHACHA_STATE_BASE_ADDR,                               LENGTH = RET_DATA_UNINIT_CHACHA_STATE_SIZE
                                              
}
//
//MEMORY
//{
//  /* NOTE 1 K = 1 KiBi = 1024 bytes */
//  LR_IROM1          (rwx) : ORIGIN = SRAM_BASE_ADDR,                                              LENGTH = BOOT_VECTOR_AREA_SZ
//  LR_IROM2          (rwx) : ORIGIN = SRAM_BASE_ADDR + BOOT_VECTOR_AREA_SZ,                        LENGTH = PATCH_TABLE_AREA_SZ
//  LR_IROM3          (rwx) : ORIGIN = CODE_AREA_BASE,                                              LENGTH = CODE_AREA_SIZE
//  LR_RETAINED_RAM0  (rw)  : ORIGIN = RET_MEM_BASE_ADDR,                                           LENGTH = RET_MEM_SIZE
//  /* After this there's only BLE Exchange Memory, externally defined by the __SCT_BLE_BASE address and with custom zeroing code in arch_rom.c */
//
//  /* Free area to be used by the application (change attribute to rw if used) */
//  LR_FREE           (r)          : ORIGIN = FREE_AREA_BASE_ADDR,                                  LENGTH = FREE_AREA_SIZE
//  /* Fixed area used by TRNG */
//  LR_RETAINED_TRNG_STATE   (rw)  : ORIGIN = TRNG_STATE_BASE_ADDR,                                 LENGTH = RET_DATA_UNINIT_TRNG_STATE_SIZE
//  /* Fixed area used by CHACHA20 */
//  LR_RETAINED_CHACHA_STATE (rw)  : ORIGIN = CHACHA_STATE_BASE_ADDR,                               LENGTH = RET_DATA_UNINIT_CHACHA_STATE_SIZE
//                                              
//}
