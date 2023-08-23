/// Number of loops in the driver
pub const XAE_LOOPS_TO_COME_OUT_OF_RST: usize   = 10000;	

/// XAE_PROMISC_OPTION specifies the Axi Ethernet device to accept all
/// incoming packets. This driver sets this option to disabled (cleared) by default.
pub const XAE_PROMISC_OPTION: usize =  0x00000001;

/// XAE_JUMBO_OPTION specifies the Axi Ethernet device to accept jumbo
/// frames for transmit and receive.
/// This driver sets this option to disabled (cleared) by default.
pub const XAE_JUMBO_OPTION: usize =     0x00000002;

/// XAE_VLAN_OPTION specifies the Axi Ethernet device to enable VLAN support
/// for transmit and receive.
/// This driver sets this option to disabled (cleared) by default.
pub const XAE_VLAN_OPTION: usize =			0x00000004;

/// XAE_FLOW_CONTROL_OPTION specifies the Axi Ethernet device to recognize
/// received flow control frames.
/// This driver sets this option to enabled (set) by default.
pub const XAE_FLOW_CONTROL_OPTION: usize =		0x00000008;

/// XAE_FCS_STRIP_OPTION specifies the Axi Ethernet device to strip FCS and
/// PAD from received frames. Note that PAD from VLAN frames is not stripped.
/// This driver sets this option to enabled (set) by default.
pub const XAE_FCS_STRIP_OPTION: usize =		0x00000010;

/// XAE_FCS_INSERT_OPTION specifies the Axi Ethernet device to generate the
/// FCS field and add PAD automatically for outgoing frames.
/// This driver sets this option to enabled (set) by default.
pub const XAE_FCS_INSERT_OPTION: usize =		0x00000020;

/// XAE_LENTYPE_ERR_OPTION specifies the Axi Ethernet device to enable
/// Length/Type error checking (mismatched type/length field) for received
/// frames.
/// This driver sets this option to enabled (set) by default.
pub const XAE_LENTYPE_ERR_OPTION: usize =		0x00000040;

/// XAE_TRANSMITTER_ENABLE_OPTION specifies the Axi Ethernet device
/// transmitter to be enabled.
/// This driver sets this option to enabled (set) by default.
pub const XAE_TRANSMITTER_ENABLE_OPTION: usize =	0x00000080;

/// XAE_RECEIVER_ENABLE_OPTION specifies the Axi Ethernet device receiver to
/// be enabled.
/// This driver sets this option to enabled (set) by default.
pub const XAE_RECEIVER_ENABLE_OPTION: usize =	0x00000100;

/// XAE_BROADCAST_OPTION specifies the Axi Ethernet device to receive frames
/// sent to the broadcast Ethernet address.
/// This driver sets this option to enabled (set) by default.
pub const XAE_BROADCAST_OPTION: usize =		0x00000200;

/// XAE_MULTICAST_OPTION specifies the Axi Ethernet device to receive frames
/// sent to Ethernet addresses that are programmed into the Multicast Address
/// Table (MAT).
/// This driver sets this option to disabled (cleared) by default.
pub const XAE_MULTICAST_OPTION: usize =		0x00000400;

/// XAE_EXT_MULTICAST_OPTION specifies the Axi Ethernet device to receive
/// frames sent to Ethernet addresses that are programmed into the Multicast
/// Address Table.
/// This driver sets this option to be dependent on HW configuration
/// by default.
pub const XAE_EXT_MULTICAST_OPTION: usize =	0x00000800;

/// XAE_EXT_TXVLAN_TRAN_OPTION specifies the Axi Ethernet device to enable
/// transmit VLAN translation.
/// This driver sets this option to be dependent on HW configuration
/// by default.
pub const XAE_EXT_TXVLAN_TRAN_OPTION: usize =	0x00001000;

/// XAE_EXT_RXVLAN_TRAN_OPTION specifies the Axi Ethernet device to enable
/// receive VLAN translation.
/// This driver sets this option to be dependent on HW configuration
/// by default.
pub const XAE_EXT_RXVLAN_TRAN_OPTION: usize =	0x00002000;

/// XAE_EXT_TXVLAN_TAG_OPTION specifies the Axi Ethernet device to enable
/// transmit VLAN tagging.
/// This driver sets this option to be dependent during HW build time
/// by default.
pub const XAE_EXT_TXVLAN_TAG_OPTION: usize =	0x00004000;

/// XAE_EXT_RXVLAN_TAG_OPTION specifies the Axi Ethernet device to enable
/// receive VLAN tagging.
/// This driver sets this option to be dependent during HW build time
/// by default.
pub const XAE_EXT_RXVLAN_TAG_OPTION: usize =	0x00008000;

/// XAE_EXT_TXVLAN_STRP_OPTION specifies the Axi Ethernet device to enable
/// transmit VLAN stripping.
/// This driver sets this option to be dependent during HW build time
/// by default.
pub const XAE_EXT_TXVLAN_STRP_OPTION: usize =	0x00010000;

/// XAE_EXT_RXVLAN_STRP_OPTION specifies the Axi Ethernet device to enable
/// receive VLAN stripping. This driver sets this option to be dependent during HW build time
/// by default.
pub const XAE_EXT_RXVLAN_STRP_OPTION: usize =	0x00020000;

/// XAE_DEFAULT_OPTIONS specify the options set in XAxiEthernet_Reset() and
/// XAxiEthernet_CfgInitialize()
pub const XAE_DEFAULT_OPTIONS: usize = 	(
        XAE_FLOW_CONTROL_OPTION |
        XAE_BROADCAST_OPTION |		
        XAE_FCS_INSERT_OPTION |	
        XAE_FCS_STRIP_OPTION |		
        XAE_LENTYPE_ERR_OPTION |	
        XAE_TRANSMITTER_ENABLE_OPTION | 
        XAE_RECEIVER_ENABLE_OPTION
        
);


/// Default MDIO clock divisor
pub const XAE_MDIO_DIV_DFT: usize =	29;	

// The next few constants help upper layers determine the size of memory
// pools used for Ethernet buffers and descriptor lists.

/// MAC addresses are 6 bytes 
pub const XAE_MAC_ADDR_SIZE: usize =		6;	
/// Max MTU size of an Ethernet frame 
pub const XAE_MTU: usize =				1500;	
/// Max MTU size of a jumbo Ethernet frame
pub const XAE_JUMBO_MTU: usize =			8982;	
/// Size of an Ethernet header
pub const XAE_HDR_SIZE: usize =			14;	
/// Size of an Ethernet header with VLAN 
pub const XAE_HDR_VLAN_SIZE: usize =		18;	
/// Size of an Ethernet trailer (FCS)
pub const XAE_TRL_SIZE: usize =			4;	
pub const XAE_MAX_FRAME_SIZE: usize =	 XAE_MTU + XAE_HDR_SIZE + XAE_TRL_SIZE;
pub const XAE_MAX_VLAN_FRAME_SIZE: usize =  XAE_MTU + XAE_HDR_VLAN_SIZE + XAE_TRL_SIZE;
pub const XAE_MAX_JUMBO_FRAME_SIZE: usize = XAE_JUMBO_MTU + XAE_HDR_SIZE + XAE_TRL_SIZE;

// Constant values returned by XAxiEthernet_GetPhysicalInterface(). 
// Note that these values match design parameters from the Axi Ethernet spec.
#[derive(Debug, Clone, PartialEq)]
pub enum PhyType {
    MII,
    GMII,
    RGMII1_3,
    RGMII2_0,
    SGMII,
    BASEX1000,
}
/// Number of storable TPIDs in Table
pub const XAE_TPID_MAX_ENTRIES: usize =		4; 

// Constant values pass into XAxiEthernet_SetV[tag|Strp]Mode() and returned by XAxiEthernet_GetV[tag|Strp]Mode().

/// No tagging
#[derive(Debug, Clone, PartialEq)]
pub enum VtagMode {
    NONE,
    ALL,
    EXISTED,
    SELECT,
    INVALID
}

impl From<u8> for VtagMode {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::NONE, 
            1 => Self::ALL,
            2 => Self::EXISTED,
            3 => Self::SELECT,
            _ => Self::NONE,
        }
    }
}
pub const XAE_DEFAULT_TXVTAG_MODE:	VtagMode = VtagMode::ALL;
pub const XAE_DEFAULT_RXVTAG_MODE:	VtagMode = VtagMode::ALL;


/// No stripping
pub const XAE_VSTRP_NONE: VtagMode =			VtagMode::NONE;	
/// Strip one tag from all frames
pub const XAE_VSTRP_ALL: VtagMode =			VtagMode::ALL;	
/// Strip one tag from selected frames 
pub const XAE_VSTRP_SELECT: VtagMode =		VtagMode::SELECT;	
pub const XAE_DEFAULT_TXVSTRP_MODE: VtagMode =	XAE_VSTRP_ALL;
pub const XAE_DEFAULT_RXVSTRP_MODE: VtagMode =	XAE_VSTRP_ALL;
/// Receive direction
#[derive(Debug, Clone, PartialEq)]
pub enum Director {
    RX = 1,
    TX = 2,
}

pub const XAE_SOFT_TEMAC_10_100_MBPS: usize =	0;
pub const XAE_SOFT_TEMAC_10_100_1000_MBPS: usize =	1;
pub const XAE_HARD_TEMC: usize =			2;

// Other Constant definitions used in the driver

/// Speed of 10 Mbps
pub const XAE_SPEED_10_MBPS: usize =		10;	
/// Speed of 100 Mbps 
pub const XAE_SPEED_100_MBPS: usize =		100;	
/// Speed of 1000 Mbps 
pub const XAE_SPEED_1000_MBPS: usize =		1000;	
/// Speed of 2500 Mbps 
pub const XAE_SPEED_2500_MBPS: usize =		2500;	
/// For soft cores with 10/100 Mbps speed.
pub const XAE_SOFT_TEMAC_LOW_SPEED: usize =	0;	
/// For soft cores with 10/100/1000 Mbps speed.
pub const XAE_SOFT_TEMAC_HIGH_SPEED: usize =	1;	
/// For hard TEMAC cores used virtex-6.
pub const XAE_HARD_TEMAC_TYPE: usize =		2;	
/// Max limit while accessing and searching for available PHYs.
pub const XAE_PHY_ADDR_LIMIT: usize =		31;	
/// Max register limit in PHY as mandated by the spec.
pub const XAE_PHY_REG_NUM_LIMIT: usize =		31;	
/// Timeout in us used while checking 
/// if the core had come out of reset or for the driver API to wait for 
/// before returning a failure case.
pub const XAE_RST_DEFAULT_TIMEOUT_VAL: usize = 1000000; 
/// Strip field length in vlan table used for extended vlan features.
pub const XAE_VLAN_TABL_STRP_FLD_LEN: usize =	1;	
/// Tag field length in vlan table used for extended vlan features.
pub const XAE_VLAN_TABL_TAG_FLD_LEN: usize =	1;	
/// Max possible number of entries in vlan table used for extended vlan features.
pub const XAE_MAX_VLAN_TABL_ENTRY: usize =		0xFFF;	
/// VID field start offset in each entry in the VLAN table.
pub const XAE_VLAN_TABL_VID_START_OFFSET: usize =	2;	
/// Strip field start offset in each entry in the VLAN table.
pub const XAE_VLAN_TABL_STRP_STRT_OFFSET: usize =	1;	
/// Mask used to extract the the strip field from an entry in VLAN table.
pub const XAE_VLAN_TABL_STRP_ENTRY_MASK: usize =	0x01;	
/// Mask used to extract the the tag field from an entry in VLAN table.
pub const XAE_VLAN_TABL_TAG_ENTRY_MASK: usize =	0x01;	


//Receive Configuration Word 1 (RCW1) Register bit definitions
/// Reset
pub const XAE_RCW1_RST_MASK: usize = 	0x80000000; 
/** Jumbo frame enable */
pub const XAE_RCW1_JUM_MASK: usize = 	0x40000000; 
/** In-Band FCS enable (FCS not stripped) */
pub const XAE_RCW1_FCS_MASK: usize = 	0x20000000; 
/** Receiver enable */
pub const XAE_RCW1_RX_MASK: usize = 	0x10000000; 
/** VLAN frame enable */
pub const XAE_RCW1_VLAN_MASK: usize = 	0x08000000; 
/** Length/type field valid check disable */
pub const XAE_RCW1_LT_DIS_MASK: usize = 0x02000000; 
/** Control frame Length check disable */
pub const XAE_RCW1_CL_DIS_MASK: usize = 0x01000000; 
/** Inband 1588 time stamp enable */
pub const XAE_RCW1_1588_TIMESTAMP_EN_MASK: usize = 	0x00400000; 
/** Pause frame source address bits [47:32].Bits 
    [31:0] are stored in register RCW0 */
pub const XAE_RCW1_PAUSEADDR_MASK: usize =  0x0000FFFF; 


//Transmitter Configuration (TC) Register bit definitions
/** Reset */
pub const XAE_TC_RST_MASK: usize = 		0x80000000; 
/** Jumbo frame enable */
pub const XAE_TC_JUM_MASK: usize = 		0x40000000; 
/** In-Band FCS enable (FCS not generated) */
pub const XAE_TC_FCS_MASK: usize = 		0x20000000; 
/** Transmitter enable */
pub const XAE_TC_TX_MASK: usize = 		0x10000000; 
/** VLAN frame enable */
pub const XAE_TC_VLAN_MASK: usize = 	0x08000000; 
/** Inter-frame gap adjustment enable */
pub const XAE_TC_IFG_MASK: usize = 		0x02000000;
/** 1588 Cmd field enable */
pub const XAE_TC_1588_CMD_EN_MASK: usize = 0x00400000; 
/** MII management enable*/
pub const XAE_MDIO_MC_MDIOEN_MASK: usize = 0x00000040;
/** Maximum MDIO divisor */
pub const XAE_MDIO_MC_CLOCK_DIVIDE_MAX: usize =	0x3F;	   

/// Ethernet link status
#[derive(Debug)]
pub enum LinkStatus {
    EthLinkUndefined,
    EthLinkUp,
    EthLinkDown,
    EthLinkNegotiating,
}


/** Interrupt Status/Enable/Mask Registers bit definitions
  The bit definition of these three interrupt registers are the same.
  These bits are associated with the XAE_IS_OFFSET, XAE_IP_OFFSET, and
  XAE_IE_OFFSET registers. 
 */
/** Hard register access complete */
pub const XAE_INT_HARDACSCMPLT_MASK: usize  = 0x00000001; 
/** Auto negotiation complete  */
pub const XAE_INT_AUTONEG_MASK: usize       = 0x00000002; 
/** Rx complete */
pub const XAE_INT_RXCMPIT_MASK: usize       = 0x00000004; 
/** Rx frame rejected */
pub const XAE_INT_RXRJECT_MASK: usize       = 0x00000008; 
/** Rx fifo overrun */
pub const XAE_INT_RXFIFOOVR_MASK: usize     = 0x00000010; 
/** Tx complete */
pub const XAE_INT_TXCMPIT_MASK: usize       = 0x00000020; 
/** Rx Dcm Lock */
pub const XAE_INT_RXDCMLOCK_MASK: usize     = 0x00000040; 
/** MGT clock Lock */
pub const XAE_INT_MGTRDY_MASK: usize        = 0x00000080; 
 /** Phy Reset complete */
 pub const XAE_INT_PHYRSTCMPLT_MASK: usize   = 0x00000100;
/** All the ints */
pub const XAE_INT_ALL_MASK: usize           = 0x0000003F; 
/** indicate receive errors */
pub const XAE_INT_RECV_ERROR_MASK: usize	= XAE_INT_RXRJECT_MASK | XAE_INT_RXFIFOOVR_MASK; 

/* IEEE PHY Specific definitions */
pub const PHY_R0_CTRL_REG: usize        = 0;
pub const PHY_R3_PHY_IDENT_REG: usize   = 3;
pub const PHY_R0_RESET: usize           = 0x8000;
pub const PHY_R0_LOOPBACK: usize        = 0x4000;
pub const PHY_R0_ANEG_ENABLE: usize     = 0x1000;
pub const PHY_R0_DFT_SPD_MASK: usize    = 0x2040;
pub const PHY_R0_DFT_SPD_10: usize      = 0x0000;
pub const PHY_R0_DFT_SPD_100: usize     = 0x2000;
pub const PHY_R0_DFT_SPD_1000: usize    = 0x0040;
pub const PHY_R0_DFT_SPD_2500: usize    = 0x0040;
pub const PHY_R0_ISOLATE: usize         = 0x0400;
pub const PHY_MODEL_NUM_MASK: usize     = 0x3F0;
pub const PHY_R20_EXTND_CTRL_REG: usize = 20;
pub const PHY_R27_EXTND_STS_REG: usize  = 27;
pub const PHY_R20_DFT_SPD_10: usize     = 0x20;
pub const PHY_R20_DFT_SPD_100: usize    = 0x50;
pub const PHY_R20_DFT_SPD_1000: usize   = 0x60;
pub const PHY_R20_RX_DLY: usize         = 0x80;
pub const PHY_R27_MAC_CONFIG_GMII: usize = 0x000F;
pub const PHY_R27_MAC_CONFIG_MII: usize  = 0x000F;
pub const PHY_R27_MAC_CONFIG_RGMII: usize  = 0x000B;
pub const PHY_R27_MAC_CONFIG_SGMII: usize  = 0x0004;

/* Marvel PHY 88E1116R Specific definitions */
pub const PHY_R22_PAGE_ADDR_REG: usize  = 22;
pub const PHY_PG2_R21_CTRL_REG: usize  = 21;

pub const PHY_REG21_10: usize  = 0x0030;
pub const PHY_REG21_100: usize  = 0x2030;
pub const PHY_REG21_1000: usize  = 0x0070;
pub const MARVEL_PHY_88E1111_MODEL: usize = 0xC0;
pub const MARVEL_PHY_88E1116R_MODEL: usize = 0x240;
pub const AXIETHERNET_PHY_DELAY_SEC: usize = 4;
pub const TI_PHY_IDENTIFIER: usize = 0x2000;
pub const TI_PHY_MODEL: usize = 0x230;
pub const TI_PHY_CR: usize = 0xD;
pub const TI_PHY_PHYCTRL: usize = 0x10;
pub const TI_PHY_CR_SGMII_EN: usize = 0x0800;
pub const TI_PHY_ADDDR: usize = 0xE;
pub const TI_PHY_CFGR2: usize = 0x14;
pub const TI_PHY_SGMIITYPE: usize = 0xD3;
pub const TI_PHY_CFGR2_SGMII_AUTONEG_EN: usize = 0x0080;
pub const TI_PHY_SGMIICLK_EN: usize = 0x4000;
pub const TI_PHY_CR_DEVAD_EN: usize = 0x001F;
pub const TI_PHY_CR_DEVAD_DATAEN: usize = 0x4000;