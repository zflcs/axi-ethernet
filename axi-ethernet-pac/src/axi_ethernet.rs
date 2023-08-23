#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Reset and Address Filter TEMAC"]
    pub raf: RAF,
    #[doc = "0x04 - Transmit Pause Frame TEMAC"]
    pub tpf: TPF,
    #[doc = "0x08 - Transmit Inter Frame Gap Adjustment TEMAC"]
    pub ifgp: IFGP,
    #[doc = "0x0c - Interrupt Status"]
    pub is: IS,
    #[doc = "0x10 - Interrupt Pending"]
    pub ip: IP,
    #[doc = "0x14 - Interrupt Enable"]
    pub ie: IE,
    #[doc = "0x18 - Transmit VLAN Tag TEMAC"]
    pub ttag: TTAG,
    #[doc = "0x1c - Receive VLAN Tag TEMAC"]
    pub rtag: RTAG,
    #[doc = "0x20 - Unicast Address Word Lower TEMAC"]
    pub uawl: UAWL,
    #[doc = "0x24 - Unicast Address Word Upper TEMAC"]
    pub uawu: UAWU,
    #[doc = "0x28 - VLAN TPID TEMAC Word 0"]
    pub tpid0: TPID0,
    #[doc = "0x2c - VLAN TPID TEMAC Word 1"]
    pub tpid1: TPID1,
    #[doc = "0x30 - PCS PMA TEMAC Status"]
    pub ppst: PPST,
    _reserved13: [u8; 0x01cc],
    #[doc = "0x200 - Received Bytes, LSW"]
    pub rxbl: RXBL,
    #[doc = "0x204 - Received Bytes, MSW"]
    pub rxbu: RXBU,
    #[doc = "0x208 - Transmitted Bytes, LSW"]
    pub txbl: TXBL,
    #[doc = "0x20c - Transmitted Bytes, MSW"]
    pub txbu: TXBU,
    #[doc = "0x210 - Count of undersize frames received, LSM"]
    pub rxundrl: RXUNDRL,
    #[doc = "0x214 - Count of undersize frames received, MSM"]
    pub rxundru: RXUNDRU,
    #[doc = "0x218 - Count of undersize and bad FCS frames received, LSM"]
    pub rxfragl: RXFRAGL,
    #[doc = "0x21c - Count of undersize and bad FCS frames received, MSM"]
    pub rxfragu: RXFRAGU,
    #[doc = "0x220 - Count of 64 bytes frames received, LSM"]
    pub rx64bl: RX64BL,
    #[doc = "0x224 - Count of 64 bytes frames received, MSM"]
    pub rx64bu: RX64BU,
    #[doc = "0x228 - Count of 65-127 bytes frames received, LSM"]
    pub rx65b127l: RX65B127L,
    #[doc = "0x22c - Count of 65-127 bytes frames received, MSM"]
    pub rx65b127u: RX65B127U,
    #[doc = "0x230 - Count of 128-255 bytes frames received, LSM"]
    pub rx128b255l: RX128B255L,
    #[doc = "0x234 - Count of 128-255 bytes frames received, MSM"]
    pub rx128b255u: RX128B255U,
    #[doc = "0x238 - Count of 256-511 bytes frames received, LSM"]
    pub rx256b511l: RX256B511L,
    #[doc = "0x23c - Count of 256-511 bytes frames received, MSM"]
    pub rx256b511u: RX256B511U,
    #[doc = "0x240 - Count of 512-1023 bytes frames received, LSM"]
    pub rx512b1023l: RX512B1023L,
    #[doc = "0x244 - Count of 512-1023 bytes frames received, MSM"]
    pub rx512b1023u: RX512B1023U,
    #[doc = "0x248 - Count of 1024-MAX bytes frames received, LSM"]
    pub rx1024bl: RX1024BL,
    #[doc = "0x24c - Count of 1024-MAX bytes frames received, MSM"]
    pub rx1024bu: RX1024BU,
    #[doc = "0x250 - Count of oversize frames received, LSM"]
    pub rxovrl: RXOVRL,
    #[doc = "0x254 - Count of oversize bytes frames received, MSM"]
    pub rxovru: RXOVRU,
    #[doc = "0x258 - Count of 64 bytes frames transmitted, LSM"]
    pub tx64bl: TX64BL,
    #[doc = "0x25c - Count of 64 bytes frames transmitted, MSM"]
    pub tx64bu: TX64BU,
    #[doc = "0x260 - Count of 65-127 bytes frames transmitted, LSM"]
    pub tx65b127l: TX65B127L,
    #[doc = "0x264 - Count of 65-127 bytes frames transmitted, MSM"]
    pub tx65b127u: TX65B127U,
    #[doc = "0x268 - Count of 128-255 bytes frames transmitted, LSM"]
    pub tx128b255l: TX128B255L,
    #[doc = "0x26c - Count of 128-255 bytes frames transmitted, MSM"]
    pub tx128b255u: TX128B255U,
    #[doc = "0x270 - Count of 256-511 bytes frames transmitted, LSM"]
    pub tx256b511l: TX256B511L,
    #[doc = "0x274 - Count of 256-511 bytes frames transmitted, MSM"]
    pub tx256b511u: TX256B511U,
    #[doc = "0x278 - Count of 512-1023 bytes frames transmitted, LSM"]
    pub tx512b1023l: TX512B1023L,
    #[doc = "0x27c - Count of 512-1023 bytes frames transmitted, MSM"]
    pub tx512b1023u: TX512B1023U,
    #[doc = "0x280 - Count of 1024-MAX bytes frames transmitted, LSM"]
    pub tx1024bl: TX1024BL,
    #[doc = "0x284 - Count of 1024-MAX bytes frames transmitted, MSM"]
    pub tx1024bu: TX1024BU,
    #[doc = "0x288 - Count of oversize frames transmitted, LSM"]
    pub txovrl: TXOVRL,
    #[doc = "0x28c - Count of oversize bytes frames transmitted, MSM"]
    pub txovru: TXOVRU,
    #[doc = "0x290 - Count of frames received OK, LSM"]
    pub rxfl: RXFL,
    #[doc = "0x294 - Count of frames transmitted OK, MSM"]
    pub rxfu: RXFU,
    #[doc = "0x298 - Count of frames received with FCS error and at least 64 bytes, LSM"]
    pub rxfcserl: RXFCSERL,
    #[doc = "0x29c - Count of frames received with FCS error and at least 64 bytes, MSM"]
    pub rxfcseru: RXFCSERU,
    #[doc = "0x2a0 - Count of broadcast frames received, LSM"]
    pub rxbcstfl: RXBCSTFL,
    #[doc = "0x2a4 - Count of broadcast frames received, MSM"]
    pub rxbcstfu: RXBCSTFU,
    #[doc = "0x2a8 - Count of multicast frames received, LSM"]
    pub rxmcstfl: RXMCSTFL,
    #[doc = "0x2ac - Count of multicast frames received, MSM"]
    pub rxmcstfu: RXMCSTFU,
    #[doc = "0x2b0 - Count of control frames received, LSM"]
    pub rxctrfl: RXCTRFL,
    #[doc = "0x2b4 - Count of control frames received, MSM"]
    pub rxctrfu: RXCTRFU,
    #[doc = "0x2b8 - Count of frames received with length error, LSM"]
    pub rxlterl: RXLTERL,
    #[doc = "0x2bc - Count of frames received with length error, MSM"]
    pub rxlteru: RXLTERU,
    #[doc = "0x2c0 - Count of VLAN tagged frames received, LSM"]
    pub rxvlanfl: RXVLANFL,
    #[doc = "0x2c4 - Count of VLAN tagged frames received, MSM"]
    pub rxvlanfu: RXVLANFU,
    #[doc = "0x2c8 - Count of pause frames received, LSM"]
    pub rxpfl: RXPFL,
    #[doc = "0x2cc - Count of pause frames received, MSM"]
    pub rxpfu: RXPFU,
    #[doc = "0x2d0 - Count of control frames received with unsupported opcode, LSM"]
    pub rxuopfl: RXUOPFL,
    #[doc = "0x2d4 - Count of control frames received with unsupported opcode, MSM"]
    pub rxuopfu: RXUOPFU,
    #[doc = "0x2d8 - Count of frames transmitted OK, LSM"]
    pub txfl: TXFL,
    #[doc = "0x2dc - Count of frames transmitted OK, MSM"]
    pub txfu: TXFU,
    #[doc = "0x2e0 - Count of broadcast frames transmitted, LSM"]
    pub txbcstfl: TXBCSTFL,
    #[doc = "0x2e4 - Count of broadcast frames transmitted, MSM"]
    pub txbcstfu: TXBCSTFU,
    #[doc = "0x2e8 - Count of multicast frames transmitted, LSM"]
    pub txmcstfl: TXMCSTFL,
    #[doc = "0x2ec - Count of multicast frames transmitted, MSM"]
    pub txmcstfu: TXMCSTFU,
    #[doc = "0x2f0 - Count of frames transmitted underrun error, LSM"]
    pub txundererl: TXUNDERERL,
    #[doc = "0x2f4 - Count of frames transmitted underrun error, MSM"]
    pub txundereru: TXUNDERERU,
    #[doc = "0x2f8 - Count of control frames transmitted, LSM"]
    pub txctrfl: TXCTRFL,
    #[doc = "0x2fc - Count of control frames transmitted, MSM"]
    pub txctrfu: TXCTRFU,
    #[doc = "0x300 - Count of VLAN tagged frames transmitted, LSM"]
    pub txvlanfl: TXVLANFL,
    #[doc = "0x304 - Count of VLAN tagged frames transmitted, MSM"]
    pub txvlanfu: TXVLANFU,
    #[doc = "0x308 - Count of pause frames transmitted, LSM"]
    pub txpfl: TXPFL,
    #[doc = "0x30c - Count of pause frames transmitted, MSM"]
    pub txpfu: TXPFU,
    #[doc = "0x310 - Single Collision frames transmitted OK, LSM"]
    pub txscl: TXSCL,
    #[doc = "0x314 - Single Collision frames transmitted OK, MSM"]
    pub txscu: TXSCU,
    #[doc = "0x318 - Multiple Collision Frames Transmitted OK, LSM"]
    pub txmcl: TXMCL,
    #[doc = "0x31c - Multiple Collision frames Transmitted OK, MSM"]
    pub txmcu: TXMCU,
    #[doc = "0x320 - Deferred Tx Frames, LSM"]
    pub txdefl: TXDEFL,
    #[doc = "0x324 - Deferred Tx Frames, MSM"]
    pub txdefu: TXDEFU,
    #[doc = "0x328 - Frames transmitted with late Collisions, LSM"]
    pub txltcl: TXLTCL,
    #[doc = "0x32c - Frames transmitted with late Collisions, MSM"]
    pub txltcu: TXLTCU,
    #[doc = "0x330 - Frames aborted with excessive Collisions, LSM"]
    pub txacel: TXACEL,
    #[doc = "0x334 - Frames aborted with excessive Collisions, MSM"]
    pub txaceu: TXACEU,
    #[doc = "0x338 - Transmit Frames with excessive Defferal, LSM"]
    pub txedefl: TXEDEFL,
    #[doc = "0x33c - Transmit Frames with excessive Defferal, MSM"]
    pub txedefu: TXEDEFU,
    #[doc = "0x340 - Frames received with alignment errors, LSM"]
    pub rxaerl: RXAERL,
    _reserved94: [u8; 0x08],
    #[doc = "0x34c - Frames received with alignment errors, MSM"]
    pub rxaeru: RXAERU,
    _reserved95: [u8; 0xb0],
    #[doc = "0x400 - TEMAC Receive Configuration Word 0"]
    pub trcw0: TRCW0,
    #[doc = "0x404 - TEMAC Receive Configuration Word 1"]
    pub trcw1: TRCW1,
    #[doc = "0x408 - TEMAC Transmitter Configuration"]
    pub ttc: TTC,
    #[doc = "0x40c - TEMAC Flow Control Configuration"]
    pub tfcc: TFCC,
    #[doc = "0x410 - EMAC mode configuration"]
    pub emmc: EMMC,
    #[doc = "0x414 - RX Max Frame Configuration"]
    pub rxfc: RXFC,
    #[doc = "0x418 - TX Max Frame Configuration"]
    pub txfc: TXFC,
    #[doc = "0x41c - TX Timestamp Adjust Control"]
    pub ttac: TTAC,
    #[doc = "0x420 - RGMII/SGMII configuration"]
    pub phyc: PHYC,
    _reserved104: [u8; 0xd4],
    #[doc = "0x4f8 - Identification"]
    pub id: ID,
    #[doc = "0x4fc - Ability Register"]
    pub ar: AR,
    #[doc = "0x500 - MDIO Setup"]
    pub mdiomc: MDIOMC,
    #[doc = "0x504 - MDIO Control"]
    pub mdiomcr: MDIOMCR,
    #[doc = "0x508 - MDIO Write Data"]
    pub mdiomwd: MDIOMWD,
    #[doc = "0x50c - MDIO Read Data"]
    pub mdiomrd: MDIOMRD,
    _reserved110: [u8; 0xf0],
    #[doc = "0x600 - MII Management Interrupt Status"]
    pub mdiomis: MDIOMIS,
    _reserved111: [u8; 0x1c],
    #[doc = "0x620 - MII Management Interrupt Pending"]
    pub mdiomip: MDIOMIP,
    _reserved112: [u8; 0x1c],
    #[doc = "0x640 - MII Management Interrupt Enable"]
    pub mdiomie: MDIOMIE,
    _reserved113: [u8; 0x1c],
    #[doc = "0x660 - MII Management Interrupt Clear"]
    pub mdiomic: MDIOMIC,
    _reserved114: [u8; 0x9c],
    #[doc = "0x700 - Unicast Address Word 0 (UAW0)"]
    pub uaw0: UAW0,
    #[doc = "0x704 - Unicast Address Word 1 (UAW0)"]
    pub uaw1: UAW1,
    #[doc = "0x708 - Filter Mask Index"]
    pub fmi: FMI,
    _reserved117: [u8; 0x04],
    #[doc = "0x710 - Address Filter 0"]
    pub af0: AF0,
    #[doc = "0x714 - Address Filter 1"]
    pub af1: AF1,
    _reserved119: [u8; 0x38e8],
    #[doc = "0x4000 - TX VLAN data table address"]
    pub txvlandata: TXVLANDATA,
    _reserved120: [u8; 0x3ffc],
    #[doc = "0x8000 - RX VLAN data table address"]
    pub rxvlandata: RXVLANDATA,
    _reserved121: [u8; 0x0001_7ffc],
    #[doc = "0x20000 - Multicast table address"]
    pub mcasttable: MCASTTABLE,
}
#[doc = "raf (rw) register accessor: an alias for `Reg<RAF_SPEC>`"]
pub type RAF = crate::Reg<raf::RAF_SPEC>;
#[doc = "Reset and Address Filter TEMAC"]
pub mod raf;
#[doc = "tpf (rw) register accessor: an alias for `Reg<TPF_SPEC>`"]
pub type TPF = crate::Reg<tpf::TPF_SPEC>;
#[doc = "Transmit Pause Frame TEMAC"]
pub mod tpf;
#[doc = "ifgp (rw) register accessor: an alias for `Reg<IFGP_SPEC>`"]
pub type IFGP = crate::Reg<ifgp::IFGP_SPEC>;
#[doc = "Transmit Inter Frame Gap Adjustment TEMAC"]
pub mod ifgp;
#[doc = "is (rw) register accessor: an alias for `Reg<IS_SPEC>`"]
pub type IS = crate::Reg<is::IS_SPEC>;
#[doc = "Interrupt Status"]
pub mod is;
#[doc = "ip (r) register accessor: an alias for `Reg<IP_SPEC>`"]
pub type IP = crate::Reg<ip::IP_SPEC>;
#[doc = "Interrupt Pending"]
pub mod ip;
#[doc = "ie (rw) register accessor: an alias for `Reg<IE_SPEC>`"]
pub type IE = crate::Reg<ie::IE_SPEC>;
#[doc = "Interrupt Enable"]
pub mod ie;
#[doc = "ttag (rw) register accessor: an alias for `Reg<TTAG_SPEC>`"]
pub type TTAG = crate::Reg<ttag::TTAG_SPEC>;
#[doc = "Transmit VLAN Tag TEMAC"]
pub mod ttag;
#[doc = "rtag (rw) register accessor: an alias for `Reg<RTAG_SPEC>`"]
pub type RTAG = crate::Reg<rtag::RTAG_SPEC>;
#[doc = "Receive VLAN Tag TEMAC"]
pub mod rtag;
#[doc = "uawl (rw) register accessor: an alias for `Reg<UAWL_SPEC>`"]
pub type UAWL = crate::Reg<uawl::UAWL_SPEC>;
#[doc = "Unicast Address Word Lower TEMAC"]
pub mod uawl;
#[doc = "uawu (rw) register accessor: an alias for `Reg<UAWU_SPEC>`"]
pub type UAWU = crate::Reg<uawu::UAWU_SPEC>;
#[doc = "Unicast Address Word Upper TEMAC"]
pub mod uawu;
#[doc = "tpid0 (rw) register accessor: an alias for `Reg<TPID0_SPEC>`"]
pub type TPID0 = crate::Reg<tpid0::TPID0_SPEC>;
#[doc = "VLAN TPID TEMAC Word 0"]
pub mod tpid0;
#[doc = "tpid1 (rw) register accessor: an alias for `Reg<TPID1_SPEC>`"]
pub type TPID1 = crate::Reg<tpid1::TPID1_SPEC>;
#[doc = "VLAN TPID TEMAC Word 1"]
pub mod tpid1;
#[doc = "ppst (r) register accessor: an alias for `Reg<PPST_SPEC>`"]
pub type PPST = crate::Reg<ppst::PPST_SPEC>;
#[doc = "PCS PMA TEMAC Status"]
pub mod ppst;
#[doc = "rxbl (r) register accessor: an alias for `Reg<RXBL_SPEC>`"]
pub type RXBL = crate::Reg<rxbl::RXBL_SPEC>;
#[doc = "Received Bytes, LSW"]
pub mod rxbl;
#[doc = "rxbu (r) register accessor: an alias for `Reg<RXBU_SPEC>`"]
pub type RXBU = crate::Reg<rxbu::RXBU_SPEC>;
#[doc = "Received Bytes, MSW"]
pub mod rxbu;
#[doc = "txbl (r) register accessor: an alias for `Reg<TXBL_SPEC>`"]
pub type TXBL = crate::Reg<txbl::TXBL_SPEC>;
#[doc = "Transmitted Bytes, LSW"]
pub mod txbl;
#[doc = "txbu (r) register accessor: an alias for `Reg<TXBU_SPEC>`"]
pub type TXBU = crate::Reg<txbu::TXBU_SPEC>;
#[doc = "Transmitted Bytes, MSW"]
pub mod txbu;
#[doc = "rxundrl (r) register accessor: an alias for `Reg<RXUNDRL_SPEC>`"]
pub type RXUNDRL = crate::Reg<rxundrl::RXUNDRL_SPEC>;
#[doc = "Count of undersize frames received, LSM"]
pub mod rxundrl;
#[doc = "rxundru (r) register accessor: an alias for `Reg<RXUNDRU_SPEC>`"]
pub type RXUNDRU = crate::Reg<rxundru::RXUNDRU_SPEC>;
#[doc = "Count of undersize frames received, MSM"]
pub mod rxundru;
#[doc = "rxfragl (r) register accessor: an alias for `Reg<RXFRAGL_SPEC>`"]
pub type RXFRAGL = crate::Reg<rxfragl::RXFRAGL_SPEC>;
#[doc = "Count of undersize and bad FCS frames received, LSM"]
pub mod rxfragl;
#[doc = "rxfragu (r) register accessor: an alias for `Reg<RXFRAGU_SPEC>`"]
pub type RXFRAGU = crate::Reg<rxfragu::RXFRAGU_SPEC>;
#[doc = "Count of undersize and bad FCS frames received, MSM"]
pub mod rxfragu;
#[doc = "rx64bl (r) register accessor: an alias for `Reg<RX64BL_SPEC>`"]
pub type RX64BL = crate::Reg<rx64bl::RX64BL_SPEC>;
#[doc = "Count of 64 bytes frames received, LSM"]
pub mod rx64bl;
#[doc = "rx64bu (r) register accessor: an alias for `Reg<RX64BU_SPEC>`"]
pub type RX64BU = crate::Reg<rx64bu::RX64BU_SPEC>;
#[doc = "Count of 64 bytes frames received, MSM"]
pub mod rx64bu;
#[doc = "rx65b127l (r) register accessor: an alias for `Reg<RX65B127L_SPEC>`"]
pub type RX65B127L = crate::Reg<rx65b127l::RX65B127L_SPEC>;
#[doc = "Count of 65-127 bytes frames received, LSM"]
pub mod rx65b127l;
#[doc = "rx65b127u (r) register accessor: an alias for `Reg<RX65B127U_SPEC>`"]
pub type RX65B127U = crate::Reg<rx65b127u::RX65B127U_SPEC>;
#[doc = "Count of 65-127 bytes frames received, MSM"]
pub mod rx65b127u;
#[doc = "rx128b255l (r) register accessor: an alias for `Reg<RX128B255L_SPEC>`"]
pub type RX128B255L = crate::Reg<rx128b255l::RX128B255L_SPEC>;
#[doc = "Count of 128-255 bytes frames received, LSM"]
pub mod rx128b255l;
#[doc = "rx128b255u (r) register accessor: an alias for `Reg<RX128B255U_SPEC>`"]
pub type RX128B255U = crate::Reg<rx128b255u::RX128B255U_SPEC>;
#[doc = "Count of 128-255 bytes frames received, MSM"]
pub mod rx128b255u;
#[doc = "rx256b511l (r) register accessor: an alias for `Reg<RX256B511L_SPEC>`"]
pub type RX256B511L = crate::Reg<rx256b511l::RX256B511L_SPEC>;
#[doc = "Count of 256-511 bytes frames received, LSM"]
pub mod rx256b511l;
#[doc = "rx256b511u (r) register accessor: an alias for `Reg<RX256B511U_SPEC>`"]
pub type RX256B511U = crate::Reg<rx256b511u::RX256B511U_SPEC>;
#[doc = "Count of 256-511 bytes frames received, MSM"]
pub mod rx256b511u;
#[doc = "rx512b1023l (r) register accessor: an alias for `Reg<RX512B1023L_SPEC>`"]
pub type RX512B1023L = crate::Reg<rx512b1023l::RX512B1023L_SPEC>;
#[doc = "Count of 512-1023 bytes frames received, LSM"]
pub mod rx512b1023l;
#[doc = "rx512b1023u (r) register accessor: an alias for `Reg<RX512B1023U_SPEC>`"]
pub type RX512B1023U = crate::Reg<rx512b1023u::RX512B1023U_SPEC>;
#[doc = "Count of 512-1023 bytes frames received, MSM"]
pub mod rx512b1023u;
#[doc = "rx1024bl (r) register accessor: an alias for `Reg<RX1024BL_SPEC>`"]
pub type RX1024BL = crate::Reg<rx1024bl::RX1024BL_SPEC>;
#[doc = "Count of 1024-MAX bytes frames received, LSM"]
pub mod rx1024bl;
#[doc = "rx1024bu (r) register accessor: an alias for `Reg<RX1024BU_SPEC>`"]
pub type RX1024BU = crate::Reg<rx1024bu::RX1024BU_SPEC>;
#[doc = "Count of 1024-MAX bytes frames received, MSM"]
pub mod rx1024bu;
#[doc = "rxovrl (r) register accessor: an alias for `Reg<RXOVRL_SPEC>`"]
pub type RXOVRL = crate::Reg<rxovrl::RXOVRL_SPEC>;
#[doc = "Count of oversize frames received, LSM"]
pub mod rxovrl;
#[doc = "rxovru (r) register accessor: an alias for `Reg<RXOVRU_SPEC>`"]
pub type RXOVRU = crate::Reg<rxovru::RXOVRU_SPEC>;
#[doc = "Count of oversize bytes frames received, MSM"]
pub mod rxovru;
#[doc = "tx64bl (r) register accessor: an alias for `Reg<TX64BL_SPEC>`"]
pub type TX64BL = crate::Reg<tx64bl::TX64BL_SPEC>;
#[doc = "Count of 64 bytes frames transmitted, LSM"]
pub mod tx64bl;
#[doc = "tx64bu (r) register accessor: an alias for `Reg<TX64BU_SPEC>`"]
pub type TX64BU = crate::Reg<tx64bu::TX64BU_SPEC>;
#[doc = "Count of 64 bytes frames transmitted, MSM"]
pub mod tx64bu;
#[doc = "tx65b127l (r) register accessor: an alias for `Reg<TX65B127L_SPEC>`"]
pub type TX65B127L = crate::Reg<tx65b127l::TX65B127L_SPEC>;
#[doc = "Count of 65-127 bytes frames transmitted, LSM"]
pub mod tx65b127l;
#[doc = "tx65b127u (r) register accessor: an alias for `Reg<TX65B127U_SPEC>`"]
pub type TX65B127U = crate::Reg<tx65b127u::TX65B127U_SPEC>;
#[doc = "Count of 65-127 bytes frames transmitted, MSM"]
pub mod tx65b127u;
#[doc = "tx128b255l (r) register accessor: an alias for `Reg<TX128B255L_SPEC>`"]
pub type TX128B255L = crate::Reg<tx128b255l::TX128B255L_SPEC>;
#[doc = "Count of 128-255 bytes frames transmitted, LSM"]
pub mod tx128b255l;
#[doc = "tx128b255u (r) register accessor: an alias for `Reg<TX128B255U_SPEC>`"]
pub type TX128B255U = crate::Reg<tx128b255u::TX128B255U_SPEC>;
#[doc = "Count of 128-255 bytes frames transmitted, MSM"]
pub mod tx128b255u;
#[doc = "tx256b511l (r) register accessor: an alias for `Reg<TX256B511L_SPEC>`"]
pub type TX256B511L = crate::Reg<tx256b511l::TX256B511L_SPEC>;
#[doc = "Count of 256-511 bytes frames transmitted, LSM"]
pub mod tx256b511l;
#[doc = "tx256b511u (r) register accessor: an alias for `Reg<TX256B511U_SPEC>`"]
pub type TX256B511U = crate::Reg<tx256b511u::TX256B511U_SPEC>;
#[doc = "Count of 256-511 bytes frames transmitted, MSM"]
pub mod tx256b511u;
#[doc = "tx512b1023l (r) register accessor: an alias for `Reg<TX512B1023L_SPEC>`"]
pub type TX512B1023L = crate::Reg<tx512b1023l::TX512B1023L_SPEC>;
#[doc = "Count of 512-1023 bytes frames transmitted, LSM"]
pub mod tx512b1023l;
#[doc = "tx512b1023u (r) register accessor: an alias for `Reg<TX512B1023U_SPEC>`"]
pub type TX512B1023U = crate::Reg<tx512b1023u::TX512B1023U_SPEC>;
#[doc = "Count of 512-1023 bytes frames transmitted, MSM"]
pub mod tx512b1023u;
#[doc = "tx1024bl (r) register accessor: an alias for `Reg<TX1024BL_SPEC>`"]
pub type TX1024BL = crate::Reg<tx1024bl::TX1024BL_SPEC>;
#[doc = "Count of 1024-MAX bytes frames transmitted, LSM"]
pub mod tx1024bl;
#[doc = "tx1024bu (r) register accessor: an alias for `Reg<TX1024BU_SPEC>`"]
pub type TX1024BU = crate::Reg<tx1024bu::TX1024BU_SPEC>;
#[doc = "Count of 1024-MAX bytes frames transmitted, MSM"]
pub mod tx1024bu;
#[doc = "txovrl (r) register accessor: an alias for `Reg<TXOVRL_SPEC>`"]
pub type TXOVRL = crate::Reg<txovrl::TXOVRL_SPEC>;
#[doc = "Count of oversize frames transmitted, LSM"]
pub mod txovrl;
#[doc = "txovru (r) register accessor: an alias for `Reg<TXOVRU_SPEC>`"]
pub type TXOVRU = crate::Reg<txovru::TXOVRU_SPEC>;
#[doc = "Count of oversize bytes frames transmitted, MSM"]
pub mod txovru;
#[doc = "rxfl (r) register accessor: an alias for `Reg<RXFL_SPEC>`"]
pub type RXFL = crate::Reg<rxfl::RXFL_SPEC>;
#[doc = "Count of frames received OK, LSM"]
pub mod rxfl;
#[doc = "rxfu (r) register accessor: an alias for `Reg<RXFU_SPEC>`"]
pub type RXFU = crate::Reg<rxfu::RXFU_SPEC>;
#[doc = "Count of frames transmitted OK, MSM"]
pub mod rxfu;
#[doc = "rxfcserl (r) register accessor: an alias for `Reg<RXFCSERL_SPEC>`"]
pub type RXFCSERL = crate::Reg<rxfcserl::RXFCSERL_SPEC>;
#[doc = "Count of frames received with FCS error and at least 64 bytes, LSM"]
pub mod rxfcserl;
#[doc = "rxfcseru (r) register accessor: an alias for `Reg<RXFCSERU_SPEC>`"]
pub type RXFCSERU = crate::Reg<rxfcseru::RXFCSERU_SPEC>;
#[doc = "Count of frames received with FCS error and at least 64 bytes, MSM"]
pub mod rxfcseru;
#[doc = "rxbcstfl (r) register accessor: an alias for `Reg<RXBCSTFL_SPEC>`"]
pub type RXBCSTFL = crate::Reg<rxbcstfl::RXBCSTFL_SPEC>;
#[doc = "Count of broadcast frames received, LSM"]
pub mod rxbcstfl;
#[doc = "rxbcstfu (r) register accessor: an alias for `Reg<RXBCSTFU_SPEC>`"]
pub type RXBCSTFU = crate::Reg<rxbcstfu::RXBCSTFU_SPEC>;
#[doc = "Count of broadcast frames received, MSM"]
pub mod rxbcstfu;
#[doc = "rxmcstfl (r) register accessor: an alias for `Reg<RXMCSTFL_SPEC>`"]
pub type RXMCSTFL = crate::Reg<rxmcstfl::RXMCSTFL_SPEC>;
#[doc = "Count of multicast frames received, LSM"]
pub mod rxmcstfl;
#[doc = "rxmcstfu (r) register accessor: an alias for `Reg<RXMCSTFU_SPEC>`"]
pub type RXMCSTFU = crate::Reg<rxmcstfu::RXMCSTFU_SPEC>;
#[doc = "Count of multicast frames received, MSM"]
pub mod rxmcstfu;
#[doc = "rxctrfl (r) register accessor: an alias for `Reg<RXCTRFL_SPEC>`"]
pub type RXCTRFL = crate::Reg<rxctrfl::RXCTRFL_SPEC>;
#[doc = "Count of control frames received, LSM"]
pub mod rxctrfl;
#[doc = "rxctrfu (r) register accessor: an alias for `Reg<RXCTRFU_SPEC>`"]
pub type RXCTRFU = crate::Reg<rxctrfu::RXCTRFU_SPEC>;
#[doc = "Count of control frames received, MSM"]
pub mod rxctrfu;
#[doc = "rxlterl (r) register accessor: an alias for `Reg<RXLTERL_SPEC>`"]
pub type RXLTERL = crate::Reg<rxlterl::RXLTERL_SPEC>;
#[doc = "Count of frames received with length error, LSM"]
pub mod rxlterl;
#[doc = "rxlteru (r) register accessor: an alias for `Reg<RXLTERU_SPEC>`"]
pub type RXLTERU = crate::Reg<rxlteru::RXLTERU_SPEC>;
#[doc = "Count of frames received with length error, MSM"]
pub mod rxlteru;
#[doc = "rxvlanfl (r) register accessor: an alias for `Reg<RXVLANFL_SPEC>`"]
pub type RXVLANFL = crate::Reg<rxvlanfl::RXVLANFL_SPEC>;
#[doc = "Count of VLAN tagged frames received, LSM"]
pub mod rxvlanfl;
#[doc = "rxvlanfu (r) register accessor: an alias for `Reg<RXVLANFU_SPEC>`"]
pub type RXVLANFU = crate::Reg<rxvlanfu::RXVLANFU_SPEC>;
#[doc = "Count of VLAN tagged frames received, MSM"]
pub mod rxvlanfu;
#[doc = "rxpfl (r) register accessor: an alias for `Reg<RXPFL_SPEC>`"]
pub type RXPFL = crate::Reg<rxpfl::RXPFL_SPEC>;
#[doc = "Count of pause frames received, LSM"]
pub mod rxpfl;
#[doc = "rxpfu (r) register accessor: an alias for `Reg<RXPFU_SPEC>`"]
pub type RXPFU = crate::Reg<rxpfu::RXPFU_SPEC>;
#[doc = "Count of pause frames received, MSM"]
pub mod rxpfu;
#[doc = "rxuopfl (r) register accessor: an alias for `Reg<RXUOPFL_SPEC>`"]
pub type RXUOPFL = crate::Reg<rxuopfl::RXUOPFL_SPEC>;
#[doc = "Count of control frames received with unsupported opcode, LSM"]
pub mod rxuopfl;
#[doc = "rxuopfu (r) register accessor: an alias for `Reg<RXUOPFU_SPEC>`"]
pub type RXUOPFU = crate::Reg<rxuopfu::RXUOPFU_SPEC>;
#[doc = "Count of control frames received with unsupported opcode, MSM"]
pub mod rxuopfu;
#[doc = "txfl (r) register accessor: an alias for `Reg<TXFL_SPEC>`"]
pub type TXFL = crate::Reg<txfl::TXFL_SPEC>;
#[doc = "Count of frames transmitted OK, LSM"]
pub mod txfl;
#[doc = "txfu (r) register accessor: an alias for `Reg<TXFU_SPEC>`"]
pub type TXFU = crate::Reg<txfu::TXFU_SPEC>;
#[doc = "Count of frames transmitted OK, MSM"]
pub mod txfu;
#[doc = "txbcstfl (r) register accessor: an alias for `Reg<TXBCSTFL_SPEC>`"]
pub type TXBCSTFL = crate::Reg<txbcstfl::TXBCSTFL_SPEC>;
#[doc = "Count of broadcast frames transmitted, LSM"]
pub mod txbcstfl;
#[doc = "txbcstfu (r) register accessor: an alias for `Reg<TXBCSTFU_SPEC>`"]
pub type TXBCSTFU = crate::Reg<txbcstfu::TXBCSTFU_SPEC>;
#[doc = "Count of broadcast frames transmitted, MSM"]
pub mod txbcstfu;
#[doc = "txmcstfl (r) register accessor: an alias for `Reg<TXMCSTFL_SPEC>`"]
pub type TXMCSTFL = crate::Reg<txmcstfl::TXMCSTFL_SPEC>;
#[doc = "Count of multicast frames transmitted, LSM"]
pub mod txmcstfl;
#[doc = "txmcstfu (r) register accessor: an alias for `Reg<TXMCSTFU_SPEC>`"]
pub type TXMCSTFU = crate::Reg<txmcstfu::TXMCSTFU_SPEC>;
#[doc = "Count of multicast frames transmitted, MSM"]
pub mod txmcstfu;
#[doc = "txundererl (r) register accessor: an alias for `Reg<TXUNDERERL_SPEC>`"]
pub type TXUNDERERL = crate::Reg<txundererl::TXUNDERERL_SPEC>;
#[doc = "Count of frames transmitted underrun error, LSM"]
pub mod txundererl;
#[doc = "txundereru (r) register accessor: an alias for `Reg<TXUNDERERU_SPEC>`"]
pub type TXUNDERERU = crate::Reg<txundereru::TXUNDERERU_SPEC>;
#[doc = "Count of frames transmitted underrun error, MSM"]
pub mod txundereru;
#[doc = "txctrfl (r) register accessor: an alias for `Reg<TXCTRFL_SPEC>`"]
pub type TXCTRFL = crate::Reg<txctrfl::TXCTRFL_SPEC>;
#[doc = "Count of control frames transmitted, LSM"]
pub mod txctrfl;
#[doc = "txctrfu (r) register accessor: an alias for `Reg<TXCTRFU_SPEC>`"]
pub type TXCTRFU = crate::Reg<txctrfu::TXCTRFU_SPEC>;
#[doc = "Count of control frames transmitted, MSM"]
pub mod txctrfu;
#[doc = "txvlanfl (r) register accessor: an alias for `Reg<TXVLANFL_SPEC>`"]
pub type TXVLANFL = crate::Reg<txvlanfl::TXVLANFL_SPEC>;
#[doc = "Count of VLAN tagged frames transmitted, LSM"]
pub mod txvlanfl;
#[doc = "txvlanfu (r) register accessor: an alias for `Reg<TXVLANFU_SPEC>`"]
pub type TXVLANFU = crate::Reg<txvlanfu::TXVLANFU_SPEC>;
#[doc = "Count of VLAN tagged frames transmitted, MSM"]
pub mod txvlanfu;
#[doc = "txpfl (r) register accessor: an alias for `Reg<TXPFL_SPEC>`"]
pub type TXPFL = crate::Reg<txpfl::TXPFL_SPEC>;
#[doc = "Count of pause frames transmitted, LSM"]
pub mod txpfl;
#[doc = "txpfu (r) register accessor: an alias for `Reg<TXPFU_SPEC>`"]
pub type TXPFU = crate::Reg<txpfu::TXPFU_SPEC>;
#[doc = "Count of pause frames transmitted, MSM"]
pub mod txpfu;
#[doc = "txscl (r) register accessor: an alias for `Reg<TXSCL_SPEC>`"]
pub type TXSCL = crate::Reg<txscl::TXSCL_SPEC>;
#[doc = "Single Collision frames transmitted OK, LSM"]
pub mod txscl;
#[doc = "txscu (r) register accessor: an alias for `Reg<TXSCU_SPEC>`"]
pub type TXSCU = crate::Reg<txscu::TXSCU_SPEC>;
#[doc = "Single Collision frames transmitted OK, MSM"]
pub mod txscu;
#[doc = "txmcl (r) register accessor: an alias for `Reg<TXMCL_SPEC>`"]
pub type TXMCL = crate::Reg<txmcl::TXMCL_SPEC>;
#[doc = "Multiple Collision Frames Transmitted OK, LSM"]
pub mod txmcl;
#[doc = "txmcu (r) register accessor: an alias for `Reg<TXMCU_SPEC>`"]
pub type TXMCU = crate::Reg<txmcu::TXMCU_SPEC>;
#[doc = "Multiple Collision frames Transmitted OK, MSM"]
pub mod txmcu;
#[doc = "txdefl (r) register accessor: an alias for `Reg<TXDEFL_SPEC>`"]
pub type TXDEFL = crate::Reg<txdefl::TXDEFL_SPEC>;
#[doc = "Deferred Tx Frames, LSM"]
pub mod txdefl;
#[doc = "txdefu (r) register accessor: an alias for `Reg<TXDEFU_SPEC>`"]
pub type TXDEFU = crate::Reg<txdefu::TXDEFU_SPEC>;
#[doc = "Deferred Tx Frames, MSM"]
pub mod txdefu;
#[doc = "txltcl (r) register accessor: an alias for `Reg<TXLTCL_SPEC>`"]
pub type TXLTCL = crate::Reg<txltcl::TXLTCL_SPEC>;
#[doc = "Frames transmitted with late Collisions, LSM"]
pub mod txltcl;
#[doc = "txltcu (r) register accessor: an alias for `Reg<TXLTCU_SPEC>`"]
pub type TXLTCU = crate::Reg<txltcu::TXLTCU_SPEC>;
#[doc = "Frames transmitted with late Collisions, MSM"]
pub mod txltcu;
#[doc = "txacel (r) register accessor: an alias for `Reg<TXACEL_SPEC>`"]
pub type TXACEL = crate::Reg<txacel::TXACEL_SPEC>;
#[doc = "Frames aborted with excessive Collisions, LSM"]
pub mod txacel;
#[doc = "txaceu (r) register accessor: an alias for `Reg<TXACEU_SPEC>`"]
pub type TXACEU = crate::Reg<txaceu::TXACEU_SPEC>;
#[doc = "Frames aborted with excessive Collisions, MSM"]
pub mod txaceu;
#[doc = "txedefl (r) register accessor: an alias for `Reg<TXEDEFL_SPEC>`"]
pub type TXEDEFL = crate::Reg<txedefl::TXEDEFL_SPEC>;
#[doc = "Transmit Frames with excessive Defferal, LSM"]
pub mod txedefl;
#[doc = "txedefu (r) register accessor: an alias for `Reg<TXEDEFU_SPEC>`"]
pub type TXEDEFU = crate::Reg<txedefu::TXEDEFU_SPEC>;
#[doc = "Transmit Frames with excessive Defferal, MSM"]
pub mod txedefu;
#[doc = "rxaerl (r) register accessor: an alias for `Reg<RXAERL_SPEC>`"]
pub type RXAERL = crate::Reg<rxaerl::RXAERL_SPEC>;
#[doc = "Frames received with alignment errors, LSM"]
pub mod rxaerl;
#[doc = "rxaeru (r) register accessor: an alias for `Reg<RXAERU_SPEC>`"]
pub type RXAERU = crate::Reg<rxaeru::RXAERU_SPEC>;
#[doc = "Frames received with alignment errors, MSM"]
pub mod rxaeru;
#[doc = "trcw0 (rw) register accessor: an alias for `Reg<TRCW0_SPEC>`"]
pub type TRCW0 = crate::Reg<trcw0::TRCW0_SPEC>;
#[doc = "TEMAC Receive Configuration Word 0"]
pub mod trcw0;
#[doc = "trcw1 (rw) register accessor: an alias for `Reg<TRCW1_SPEC>`"]
pub type TRCW1 = crate::Reg<trcw1::TRCW1_SPEC>;
#[doc = "TEMAC Receive Configuration Word 1"]
pub mod trcw1;
#[doc = "ttc (rw) register accessor: an alias for `Reg<TTC_SPEC>`"]
pub type TTC = crate::Reg<ttc::TTC_SPEC>;
#[doc = "TEMAC Transmitter Configuration"]
pub mod ttc;
#[doc = "tfcc (rw) register accessor: an alias for `Reg<TFCC_SPEC>`"]
pub type TFCC = crate::Reg<tfcc::TFCC_SPEC>;
#[doc = "TEMAC Flow Control Configuration"]
pub mod tfcc;
#[doc = "emmc (rw) register accessor: an alias for `Reg<EMMC_SPEC>`"]
pub type EMMC = crate::Reg<emmc::EMMC_SPEC>;
#[doc = "EMAC mode configuration"]
pub mod emmc;
#[doc = "rxfc (rw) register accessor: an alias for `Reg<RXFC_SPEC>`"]
pub type RXFC = crate::Reg<rxfc::RXFC_SPEC>;
#[doc = "RX Max Frame Configuration"]
pub mod rxfc;
#[doc = "txfc (rw) register accessor: an alias for `Reg<TXFC_SPEC>`"]
pub type TXFC = crate::Reg<txfc::TXFC_SPEC>;
#[doc = "TX Max Frame Configuration"]
pub mod txfc;
#[doc = "ttac (rw) register accessor: an alias for `Reg<TTAC_SPEC>`"]
pub type TTAC = crate::Reg<ttac::TTAC_SPEC>;
#[doc = "TX Timestamp Adjust Control"]
pub mod ttac;
#[doc = "phyc (r) register accessor: an alias for `Reg<PHYC_SPEC>`"]
pub type PHYC = crate::Reg<phyc::PHYC_SPEC>;
#[doc = "RGMII/SGMII configuration"]
pub mod phyc;
#[doc = "id (r) register accessor: an alias for `Reg<ID_SPEC>`"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = "Identification"]
pub mod id;
#[doc = "ar (r) register accessor: an alias for `Reg<AR_SPEC>`"]
pub type AR = crate::Reg<ar::AR_SPEC>;
#[doc = "Ability Register"]
pub mod ar;
#[doc = "mdiomc (rw) register accessor: an alias for `Reg<MDIOMC_SPEC>`"]
pub type MDIOMC = crate::Reg<mdiomc::MDIOMC_SPEC>;
#[doc = "MDIO Setup"]
pub mod mdiomc;
#[doc = "mdiomcr (rw) register accessor: an alias for `Reg<MDIOMCR_SPEC>`"]
pub type MDIOMCR = crate::Reg<mdiomcr::MDIOMCR_SPEC>;
#[doc = "MDIO Control"]
pub mod mdiomcr;
#[doc = "mdiomwd (rw) register accessor: an alias for `Reg<MDIOMWD_SPEC>`"]
pub type MDIOMWD = crate::Reg<mdiomwd::MDIOMWD_SPEC>;
#[doc = "MDIO Write Data"]
pub mod mdiomwd;
#[doc = "mdiomrd (rw) register accessor: an alias for `Reg<MDIOMRD_SPEC>`"]
pub type MDIOMRD = crate::Reg<mdiomrd::MDIOMRD_SPEC>;
#[doc = "MDIO Read Data"]
pub mod mdiomrd;
#[doc = "mdiomis (rw) register accessor: an alias for `Reg<MDIOMIS_SPEC>`"]
pub type MDIOMIS = crate::Reg<mdiomis::MDIOMIS_SPEC>;
#[doc = "MII Management Interrupt Status"]
pub mod mdiomis;
#[doc = "mdiomip (r) register accessor: an alias for `Reg<MDIOMIP_SPEC>`"]
pub type MDIOMIP = crate::Reg<mdiomip::MDIOMIP_SPEC>;
#[doc = "MII Management Interrupt Pending"]
pub mod mdiomip;
#[doc = "mdiomie (rw) register accessor: an alias for `Reg<MDIOMIE_SPEC>`"]
pub type MDIOMIE = crate::Reg<mdiomie::MDIOMIE_SPEC>;
#[doc = "MII Management Interrupt Enable"]
pub mod mdiomie;
#[doc = "mdiomic (rw) register accessor: an alias for `Reg<MDIOMIC_SPEC>`"]
pub type MDIOMIC = crate::Reg<mdiomic::MDIOMIC_SPEC>;
#[doc = "MII Management Interrupt Clear"]
pub mod mdiomic;
#[doc = "uaw0 (rw) register accessor: an alias for `Reg<UAW0_SPEC>`"]
pub type UAW0 = crate::Reg<uaw0::UAW0_SPEC>;
#[doc = "Unicast Address Word 0 (UAW0)"]
pub mod uaw0;
#[doc = "uaw1 (rw) register accessor: an alias for `Reg<UAW1_SPEC>`"]
pub type UAW1 = crate::Reg<uaw1::UAW1_SPEC>;
#[doc = "Unicast Address Word 1 (UAW0)"]
pub mod uaw1;
#[doc = "fmi (rw) register accessor: an alias for `Reg<FMI_SPEC>`"]
pub type FMI = crate::Reg<fmi::FMI_SPEC>;
#[doc = "Filter Mask Index"]
pub mod fmi;
#[doc = "af0 (rw) register accessor: an alias for `Reg<AF0_SPEC>`"]
pub type AF0 = crate::Reg<af0::AF0_SPEC>;
#[doc = "Address Filter 0"]
pub mod af0;
#[doc = "af1 (rw) register accessor: an alias for `Reg<AF1_SPEC>`"]
pub type AF1 = crate::Reg<af1::AF1_SPEC>;
#[doc = "Address Filter 1"]
pub mod af1;
#[doc = "txvlandata (rw) register accessor: an alias for `Reg<TXVLANDATA_SPEC>`"]
pub type TXVLANDATA = crate::Reg<txvlandata::TXVLANDATA_SPEC>;
#[doc = "TX VLAN data table address"]
pub mod txvlandata;
#[doc = "rxvlandata (rw) register accessor: an alias for `Reg<RXVLANDATA_SPEC>`"]
pub type RXVLANDATA = crate::Reg<rxvlandata::RXVLANDATA_SPEC>;
#[doc = "RX VLAN data table address"]
pub mod rxvlandata;
#[doc = "mcasttable (rw) register accessor: an alias for `Reg<MCASTTABLE_SPEC>`"]
pub type MCASTTABLE = crate::Reg<mcasttable::MCASTTABLE_SPEC>;
#[doc = "Multicast table address"]
pub mod mcasttable;
