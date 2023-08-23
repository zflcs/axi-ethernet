use crate::AxiEthernet;
use log::*;

/// Micro phy device identifier
pub const MICREL_PHY_IDENTIFIER: usize  =       0x22;
/// ksz9031
pub const MICREL_PHY_KSZ9031_MODEL: usize =     0x220;

pub const PHY_R0_ISOLATE: usize  						= 0x0400;
pub const PHY_DETECT_REG: usize  						= 1;
pub const PHY_IDENTIFIER_1_REG: usize					= 2;
pub const PHY_IDENTIFIER_2_REG: usize					= 3;
pub const PHY_DETECT_MASK: usize 						= 0x1808;
pub const PHY_MARVELL_IDENTIFIER: usize					= 0x0141;
pub const PHY_TI_IDENTIFIER: usize					    = 0x2000;

pub const PHY_XILINX_PCS_PMA_ID1: usize                 = 0x0174;
pub const PHY_XILINX_PCS_PMA_ID2: usize                 = 0x0C00;


pub const IEEE_CONTROL_REG_OFFSET: usize                = 0;
pub const IEEE_STATUS_REG_OFFSET: usize                 = 1;
pub const IEEE_AUTONEGO_ADVERTISE_REG: usize            = 4;
pub const IEEE_PARTNER_ABILITIES_1_REG_OFFSET: usize    = 5;
pub const IEEE_PARTNER_ABILITIES_2_REG_OFFSET:usize     = 8;
pub const IEEE_PARTNER_ABILITIES_3_REG_OFFSET: usize    = 10;
pub const IEEE_1000_ADVERTISE_REG_OFFSET: usize         = 9;
pub const IEEE_MMD_ACCESS_CONTROL_REG: usize            = 13;
pub const IEEE_MMD_ACCESS_ADDRESS_DATA_REG: usize       = 14;
pub const IEEE_COPPER_SPECIFIC_CONTROL_REG: usize       = 16;
pub const IEEE_SPECIFIC_STATUS_REG: usize               = 17;
pub const IEEE_COPPER_SPECIFIC_STATUS_REG_2: usize      = 19;
pub const IEEE_EXT_PHY_SPECIFIC_CONTROL_REG: usize      = 20;
pub const IEEE_CONTROL_REG_MAC: usize                   = 21;
pub const IEEE_PAGE_ADDRESS_REGISTER: usize             = 22;
pub const IEEE_CTRL_1GBPS_LINKSPEED_MASK: usize         = 0x2040;
pub const IEEE_CTRL_LINKSPEED_MASK: usize               = 0x0040;
pub const IEEE_CTRL_LINKSPEED_1000M: usize              = 0x0040;
pub const IEEE_CTRL_LINKSPEED_100M: usize               = 0x2000;
pub const IEEE_CTRL_LINKSPEED_10M: usize                = 0x0000;
pub const IEEE_CTRL_RESET_MASK: usize                   = 0x8000;
pub const IEEE_CTRL_AUTONEGOTIATE_ENABLE: usize         = 0x1000;
pub const IEEE_STAT_AUTONEGOTIATE_CAPABLE: usize        = 0x0008;
pub const IEEE_STAT_AUTONEGOTIATE_COMPLETE: usize       = 0x0020;
pub const IEEE_STAT_AUTONEGOTIATE_RESTART: usize        = 0x0200;
pub const IEEE_STAT_1GBPS_EXTENSIONS: usize             = 0x0100;
pub const IEEE_AN1_ABILITY_MASK: usize                  = 0x1FE0;
pub const IEEE_AN3_ABILITY_MASK_1GBPS: usize            = 0x0C00;
pub const IEEE_AN1_ABILITY_MASK_100MBPS: usize          = 0x0380;
pub const IEEE_AN1_ABILITY_MASK_10MBPS: usize           = 0x0060;

pub const IEEE_ASYMMETRIC_PAUSE_MASK: usize             = 0x0800;
pub const IEEE_PAUSE_MASK: usize                        = 0x0400;
pub const IEEE_AUTONEG_ERROR_MASK: usize                = 0x8000;

/* Advertisement control register. */
pub const ADVERTISE_10HALF: usize                       = 0x0020;  /* Try for 10mbps half-duplex  */
pub const ADVERTISE_1000XFULL: usize                    = 0x0020;  /* Try for 1000BASE-X full-duplex */
pub const ADVERTISE_10FULL: usize                       = 0x0040;  /* Try for 10mbps full-duplex  */
pub const ADVERTISE_1000XHALF: usize                    = 0x0040;  /* Try for 1000BASE-X half-duplex */
pub const ADVERTISE_100HALF: usize                      = 0x0080;  /* Try for 100mbps half-duplex */
pub const ADVERTISE_1000XPAUSE: usize                   = 0x0080;  /* Try for 1000BASE-X pause    */
pub const ADVERTISE_100FULL: usize                      = 0x0100; /* Try for 100mbps full-duplex */
pub const ADVERTISE_1000XPSE_ASYM: usize                = 0x0100;  /* Try for 1000BASE-X asym pause */
pub const ADVERTISE_100BASE4: usize                     = 0x0200;  /* Try for 100mbps 4k packets  */


pub const ADVERTISE_100_AND_10: usize   = ADVERTISE_10FULL | ADVERTISE_100FULL | ADVERTISE_10HALF | ADVERTISE_100HALF;
pub const ADVERTISE_100: usize          = ADVERTISE_100FULL | ADVERTISE_100HALF;
pub const ADVERTISE_10: usize           = ADVERTISE_10FULL | ADVERTISE_10HALF;

pub const ADVERTISE_1000: usize                         = 0x0300;



