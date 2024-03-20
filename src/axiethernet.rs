use crate::hw::*;
use crate::phy_ksz9031rnx::*;
use axi_ethernet_pac::axi_ethernet;
use log::*;

#[derive(Debug, PartialEq)]
pub enum AxiDevType {
    FIFO,
    DMA,
    MCDMA,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TemacType {
    MBPS10_100,
    MBPS10_100_1000,
    HARD, //  Hard Temac Core for Virtex-6
}

/// This typedef contains configuration information for a Axi Ethernet device.'
pub struct AxiEthernetConfig {
    // /// DeviceId is the unique ID  of the device
    // pub device_id: usize,
    /// Temac Type can have 3 possible values. They are
    /// 0 for SoftTemac at 10/100 Mbps, 1 for SoftTemac
    /// at 10/100/1000 Mbps and 2 for Vitex6 Hard Temac
    pub temac_type: TemacType,
    /// TxCsum indicates that the device has checksum
    /// offload on the Tx channel or not.
    pub tx_csum: usize,
    /// RxCsum indicates that the device has checksum
    /// offload on the Rx channel or not.
    pub rx_csum: usize,
    /// PhyType indicates which type of PHY interface is used (MII, GMII, RGMII, etc.
    pub phy_type: PhyType,
    /// TX VLAN Translation indication
    pub tx_vlan_tran: bool,
    /// RX VLAN Translation indication
    pub rx_vlan_tran: bool,
    /// TX VLAN tagging indication
    pub tx_vlan_tag: bool,
    /// RX VLAN tagging indication
    pub rx_vlan_tag: bool,
    /// TX VLAN stripping indication
    pub tx_vlan_strp: bool,
    /// RX VLAN stripping indication
    pub rx_vlan_strp: bool,
    /// Extend multicast indication
    pub ext_mcast: bool,
    /// Statistics gathering option
    pub statics: bool,
    /// Avb option
    pub avb: bool,
    /// Enable LVDS option
    pub enable_sgmii_over_lvds: bool,
    /// Enable 1588 option
    pub enable_1588: bool,
    /// Tells whether MAC is 1G or 2p5G
    pub speed: usize,
    /// Number of table entries
    pub num_table_entries: usize,
    /// Axi Ethernet interrupt ID
    pub temac_intr: usize,
    /// AxiDevType is the type of device attached to the
    /// Axi Ethernet's AXI4-Stream interface.
    pub axi_dev_type: AxiDevType,
    /// AxiDevBaseAddress is the base address of
    /// the device attached to the Axi Ethernet's
    /// AXI4-Stream interface.
    pub axi_dev_base_address: usize,
    /// AxiFifoIntr interrupt ID (unused if DMA)
    pub axi_fifo_intr: usize,
    /// Axi DMA RX interrupt ID (unused if FIFO)
    pub axi_dma_rx_intr: usize,
    /// Axi DMA TX interrupt ID (unused if FIFO)
    pub axi_dma_tx_intr: usize,
    /// Axi MCDMA Channel Count
    pub axi_mcdma_chan_cnt: usize,
    /// Axi MCDMA Rx interrupt ID (unused if AXI DMA or FIFO)
    pub axi_mcdma_rx_intr: [usize; 16],
    /// AXI MCDMA TX interrupt ID (unused if AXIX DMA or FIFO)
    pub axi_mcdma_tx_intr: [usize; 16],
}

impl AxiEthernetConfig {
    pub fn default(dma_base_address: usize) -> Self {
        Self {
            temac_type: TemacType::MBPS10_100_1000,
            tx_csum: 0,
            rx_csum: 0,
            phy_type: PhyType::RGMII2_0,
            tx_vlan_tran: false,
            rx_vlan_tran: false,
            tx_vlan_tag: false,
            rx_vlan_tag: false,
            tx_vlan_strp: false,
            rx_vlan_strp: false,
            ext_mcast: false,
            statics: true,
            avb: false,
            enable_sgmii_over_lvds: false,
            enable_1588: false,
            speed: 1000,
            num_table_entries: 4,
            temac_intr: 2,
            axi_dev_type: AxiDevType::DMA,
            axi_dev_base_address: dma_base_address,
            axi_fifo_intr: 0,
            axi_dma_rx_intr: 5,
            axi_dma_tx_intr: 4,
            axi_mcdma_chan_cnt: 0,
            axi_mcdma_rx_intr: [0xFF; 16],
            axi_mcdma_tx_intr: [0xFF; 16],
        }
    }
}

/// struct XAxiEthernet is the type for Axi Ethernet driver instance data.
/// The calling code is required to use a unique instance of this structure
/// for every Axi Ethernet device used in the system. A reference to a structure
/// of this type is then passed to the driver API functions.
pub struct AxiEthernet {
    /// Hardware configuration
    pub config: AxiEthernetConfig,
    /// Device mmio base_address
    pub base_address: usize,
    /// Device is currently started
    pub is_started: bool,
    /// Device is initialized and ready
    pub is_ready: bool,
    /// Current options word
    pub options: usize,
    /// Internal driver flags
    pub flags: usize,
    /// phy_addr
    pub phy_addr: u32,
    pub link_status: LinkStatus,
    rx_count: usize,
    tx_count: usize,
}

/// basic field access
impl AxiEthernet {
    pub fn new(base_address: usize, dma_base_address: usize) -> Self {
        Self {
            config: AxiEthernetConfig::default(dma_base_address),
            base_address,
            is_started: false,
            is_ready: true,
            options: XAE_DEFAULT_OPTIONS,
            flags: 0,
            phy_addr: 0,
            link_status: LinkStatus::EthLinkDown,
            rx_count: 0,
            tx_count: 0,
        }
    }

    fn hardware(&self) -> &axi_ethernet::RegisterBlock {
        unsafe { &*(self.base_address as *const _) }
    }

    pub fn is_started(&self) -> bool {
        self.is_started
    }

    pub fn is_ready(&self) -> bool {
        self.is_ready
    }

    pub fn is_dma(&self) -> bool {
        self.config.axi_dev_type == AxiDevType::DMA
    }

    pub fn is_fifo(&self) -> bool {
        self.config.axi_dev_type == AxiDevType::FIFO
    }

    pub fn is_mcdma(&self) -> bool {
        self.config.axi_dev_type == AxiDevType::MCDMA
    }

    pub fn is_recv_fram_dropped(&self) -> bool {
        self.hardware().is.read().rx_rject().bit()
    }

    pub fn is_rx_partial_csum(&self) -> bool {
        self.config.rx_csum == 0x01
    }

    pub fn is_tx_partial_csum(&self) -> bool {
        self.config.tx_csum == 0x01
    }

    pub fn is_rx_full_csum(&self) -> bool {
        self.config.rx_csum == 0x02
    }

    pub fn is_tx_full_csum(&self) -> bool {
        self.config.tx_csum == 0x02
    }

    pub fn get_phy_interface(&self) -> PhyType {
        self.config.phy_type.clone()
    }

    pub fn get_intr_status(&self) -> usize {
        self.hardware().is.read().bits() as _
    }

    pub fn get_intr_pending(&self) -> usize {
        self.hardware().ip.read().bits() as _
    }

    pub fn is_tx_cmplt(&self) -> bool {
        self.hardware().ip.read().tx_cmplt().bit()
    }

    pub fn is_rx_memovr(&self) -> bool {
        self.hardware().ip.read().rx_fifoovr().bit()
    }

    pub fn is_rx_rject(&self) -> bool {
        self.hardware().ip.read().rx_rject().bit()
    }

    pub fn is_rx_cmplt(&self) -> bool {
        self.hardware().ip.read().rx_cmplt().bit()
    }

    pub fn enable_intr(&self, mask: usize) {
        let mut intrs = self.hardware().ie.read().bits();
        intrs = intrs | (mask & 0x3f) as u32;
        self.hardware().ie.write(|w| unsafe { w.bits(intrs) });
    }

    pub fn enable_tx_cmplt(&self) {
        self.hardware().ie.write(|w| w.tx_cmplt().set_bit());
    }

    pub fn enable_rx_memovr(&self) {
        self.hardware().ie.write(|w| w.rx_fifoovr().set_bit());
    }

    pub fn enable_rx_rject(&self) {
        self.hardware().ie.write(|w| w.rx_rject().set_bit());
    }

    pub fn enable_rx_cmplt(&self) {
        self.hardware().ie.write(|w| w.rx_cmplt().set_bit());
    }

    pub fn disable_tx_cmplt(&self) {
        self.hardware().ie.write(|w| w.tx_cmplt().clear_bit());
    }

    pub fn disable_rx_memovr(&self) {
        self.hardware().ie.write(|w| w.rx_fifoovr().clear_bit());
    }

    pub fn disable_rx_rject(&self) {
        self.hardware().ie.write(|w| w.rx_rject().clear_bit());
    }

    pub fn disable_rx_cmplt(&self) {
        self.hardware().ie.write(|w| w.rx_cmplt().clear_bit());
    }

    pub fn disable_intr(&self, mask: usize) {
        let mut intrs = self.hardware().ie.read().bits();
        intrs = intrs & !(mask & 0x3f) as u32;
        self.hardware().ie.write(|w| unsafe { w.bits(intrs) });
    }

    pub fn clear_intr(&self, mask: usize) {
        self.hardware()
            .is
            .write(|w| unsafe { w.bits((mask & 0x3f) as u32) });
    }

    pub fn clear_tx_cmplt(&self) {
        self.hardware().is.write(|w| w.tx_cmplt().set_bit());
    }

    pub fn clear_rx_memovr(&self) {
        self.hardware().is.write(|w| w.rx_fifoovr().set_bit());
    }

    pub fn clear_rx_rject(&self) {
        self.hardware().is.write(|w| w.rx_rject().set_bit());
    }

    pub fn clear_rx_cmplt(&self) {
        self.hardware().is.write(|w| w.rx_cmplt().set_bit());
    }

    pub fn is_ext_func_cap(&self) -> bool {
        self.hardware().raf.read().new_fnc_enbl().bit()
    }

    pub fn is_ext_mcast_enable(&self) -> bool {
        self.hardware().raf.read().emulti_fltr_enbl().bit()
    }

    pub fn is_ext_mcast(&self) -> bool {
        self.config.ext_mcast
    }

    pub fn is_tx_vlan_strp(&self) -> bool {
        self.config.tx_vlan_strp
    }

    pub fn is_rx_vlan_strp(&self) -> bool {
        self.config.rx_vlan_strp
    }

    pub fn is_tx_vlan_tran(&self) -> bool {
        self.config.tx_vlan_tran
    }

    pub fn is_rx_vlan_tran(&self) -> bool {
        self.config.rx_vlan_tran
    }

    pub fn is_tx_vlan_tag(&self) -> bool {
        self.config.tx_vlan_tag
    }

    pub fn is_rx_vlan_tag(&self) -> bool {
        self.config.rx_vlan_tag
    }

    pub fn get_temac_type(&self) -> TemacType {
        self.config.temac_type.clone()
    }

    pub fn is_avb_configured(&self) -> bool {
        self.config.avb
    }

    pub fn is_enable_sgmii_over_lvds(&self) -> bool {
        self.config.enable_sgmii_over_lvds
    }

    pub fn is_statics_configured(&self) -> bool {
        self.config.statics
    }

    pub fn rx_frame_count(&self) -> usize {
        let lsm = self.hardware().rxfl.read().bits() as usize;
        let msm = self.hardware().rxfu.read().bits() as usize;
        msm << 32 | lsm
    }

    pub fn tx_frame_count(&self) -> usize {
        let lsm = self.hardware().txfl.read().bits() as usize;
        let msm = self.hardware().txfu.read().bits() as usize;
        msm << 32 | lsm
    }

    // check the buffer has frame or not, if the buffer has frame, software can receive from eth
    pub fn can_receive(&mut self) -> bool {
        let res = self.rx_count < self.rx_frame_count();
        if res {
            self.rx_count += 1;
        }
        res
    }
}

/// control function
impl AxiEthernet {
    pub fn add_multicast(&self, address: &[u8; 6], entry: usize) -> isize {
        assert!(self.is_ready);
        assert!(entry < self.config.num_table_entries);
        // trace!("XAxiEthernet_MulticastAdd");
        // The device must be stopped before clearing the multicast hash table.
        if self.is_started {
            // trace!("XAxiEthernet_MulticastAdd: returning DEVICE_IS_STARTED");
            return -1;
        }
        // mac bits [31:0]
        let mut af0: u32 = address[0] as u32;
        af0 |= (address[1] as u32) << 8;
        af0 |= (address[2] as u32) << 16;
        af0 |= (address[3] as u32) << 24;
        // mac bits [47:32]
        let mut af1 = address[4] as u32;
        af1 |= (address[5] as u32) << 8;
        let mut fmi = self.hardware().fmi.read().bits() & 0xFFFFFF00;
        fmi |= entry as u32;
        self.hardware().fmi.write(|w| unsafe { w.bits(fmi) });
        // trace!("Setting MAT entry: {}", entry);
        self.hardware().af0.write(|w| unsafe { w.bits(af0) });
        self.hardware().af1.write(|w| unsafe { w.bits(af1) });
        return 0;
    }

    pub fn get_multicast(&self, address: &mut [u8; 6], entry: usize) {
        assert!(self.is_ready);
        assert!(entry < self.config.num_table_entries);
        // trace!("XAxiEthernet_MulticastGet");
        let mut fmi = self.hardware().fmi.read().bits() & 0xFFFFFF00;
        fmi |= entry as u32;
        self.hardware().fmi.write(|w| unsafe { w.bits(fmi) });
        let af0 = self.hardware().af0.read().bits();
        let af1 = self.hardware().af1.read().bits();
        address[0] = (af0 & 0xFF) as u8;
        address[1] = ((af0 >> 8) & 0xFF) as u8;
        address[2] = ((af0 >> 16) & 0xFF) as u8;
        address[3] = ((af0 >> 24) & 0xFF) as u8;
        address[4] = (af1 & 0xFF) as u8;
        address[5] = ((af1 >> 8) & 0xFF) as u8;
        // trace!("XAxiEthernet_MulticastGet: done");
    }

    pub fn clear_multicast(&self, entry: usize) -> isize {
        assert!(self.is_ready);
        assert!(entry < self.config.num_table_entries);
        // trace!("XAxiEthernet_MulticastClear");
        // The device must be stopped before clearing the multicast hash table.
        if self.is_started {
            // trace!("XAxiEthernet_MulticastClear: returning DEVICE_IS_STARTED");
            return -1;
        }
        let mut fmi = self.hardware().fmi.read().bits() & 0xFFFFFF00;
        fmi |= entry as u32;
        self.hardware().fmi.write(|w| unsafe { w.bits(fmi) });
        // Clear the entry by writing 0:0:0:0:0:0 to it
        self.hardware().af0.write(|w| unsafe { w.bits(0) });
        self.hardware().af1.write(|w| unsafe { w.bits(0) });
        // trace!("XAxiEthernet_MulticastClear: returning SUCCESS");
        return 0;
    }

    pub fn set_mac_pause_address(&self, address: &[u8; 6]) -> isize {
        assert!(self.is_ready);
        // trace!("XAxiEthernet_SetMacPauseAddress");
        // Be sure device has been stopped
        if self.is_started {
            // trace!("XAxiEthernet_SetMacPauseAddress: returning DEVICE_IS_STARTED");
            return -1;
        }
        let mut mac_addr = address[0] as u32;
        mac_addr |= (address[1] as u32) << 8;
        mac_addr |= (address[2] as u32) << 16;
        mac_addr |= (address[3] as u32) << 24;
        self.hardware().trcw0.write(|w| unsafe { w.bits(mac_addr) });
        mac_addr = self.hardware().trcw1.read().bits() & (!0xFFFF);
        mac_addr |= address[4] as u32;
        mac_addr |= (address[5] as u32) << 8;
        self.hardware().trcw1.write(|w| unsafe { w.bits(mac_addr) });
        // trace!("XAxiEthernet_SetMacPauseAddress: returning SUCCESS");
        return 0;
    }

    pub fn get_mac_pause_address(&self, address: &mut [u8; 6]) {
        assert!(self.is_ready);
        // trace!("XAxiEthernet_GetMacPauseAddress");
        // Read MAC bits [31:0] in RCW0
        let mut mac_addr = self.hardware().trcw0.read().bits();
        address[0] = (mac_addr & 0xFF) as u8;
        address[1] = ((mac_addr >> 8) & 0xFF) as u8;
        address[2] = ((mac_addr >> 16) & 0xFF) as u8;
        address[3] = ((mac_addr >> 24) & 0xFF) as u8;
        // Read MAC bits [47:32] in RCW1
        mac_addr = self.hardware().trcw1.read().bits();
        address[4] = (mac_addr & 0xFF) as u8;
        address[5] = ((mac_addr >> 8) & 0xFF) as u8;
        // trace!("XAxiEthernet_GetMacPauseAddress: done");
    }

    pub fn send_pause_packet(&self, pause_value: u16) -> isize {
        assert!(self.is_ready);
        // trace!("XAxiEthernet_SetMacPauseAddress");
        // Make sure device is ready for this operation
        if !self.is_started {
            // trace!("XAxiEthernet_SendPausePacket:returning DEVICE_IS_STOPPED");
            return -1;
        }
        self.hardware()
            .tpf
            .write(|w| unsafe { w.tpfv().bits(pause_value) });
        // trace!("XAxiEthernet_SendPausePacket: returning SUCCESS");
        return 0;
    }

    pub fn get_sgmii_status(&self, speed: &mut usize) -> isize {
        assert!(self.is_ready);
        // trace!("XAxiEthernet_GetSgmiiStatus");
        let phy_type = self.get_phy_interface();
        // Make sure PHY is SGMII
        if phy_type != PhyType::SGMII {
            // trace!("XAxiEthernet_GetSgmiiStatus: returning NO_FEATURE");
            return -1;
        }
        let egmic = self.hardware().phyc.read().sgmiilinkspeed();
        if egmic.is_10m() {
            *speed = XAE_SPEED_10_MBPS;
        } else if egmic.is_100m() {
            *speed = XAE_SPEED_100_MBPS;
        } else if egmic.is_1000m() {
            *speed = XAE_SPEED_1000_MBPS;
        } else {
            *speed = 0;
        }
        // trace!("XAxiEthernet_GetSgmiiStatus: returning SUCCESS");
        return 0;
    }

    pub fn get_rgmii_status(
        &self,
        speed: &mut usize,
        is_full_duplex: &mut bool,
        is_link: &mut bool,
    ) -> isize {
        assert!(self.is_ready);
        // trace!("XAxiEthernet_GetRgmiiStatus");
        let phy_type = self.get_phy_interface();
        // Make sure PHY is RGMII
        if (phy_type != PhyType::RGMII1_3) && (phy_type != PhyType::RGMII2_0) {
            // trace!("XAxiEthernet_GetRgmiiStatus: returning NO_FEATURE");
            return -1;
        }
        let phyc = self.hardware().phyc.read();
        let egmic_speed = phyc.rgmiilinkspeed();
        if egmic_speed.is_10m() {
            *speed = XAE_SPEED_10_MBPS;
        } else if egmic_speed.is_100m() {
            *speed = XAE_SPEED_100_MBPS;
        } else if egmic_speed.is_1000m() {
            *speed = XAE_SPEED_1000_MBPS;
        } else {
            *speed = 0;
        }
        *is_full_duplex = !phyc.rgmiihd().bit();
        *is_link = phyc.rgmiilinkmask().bit();
        // trace!("XAxiEthernet_GetRgmiiStatus: returning SUCCESS");
        return 0;
    }

    pub fn set_tpid(&self, tpid: u16, entry: usize) -> isize {
        assert!(self.is_ready);
        assert!(entry < XAE_TPID_MAX_ENTRIES);
        // The device must be stopped before modify VLAN TPID
        if self.is_started {
            // trace!("XAxiEthernet_SetTpid: returning DEVICE_IS_STARTED");
            return -1;
        }
        // Check hw capability
        if !self.is_ext_func_cap() {
            // trace!("XAxiEthernet_SetTpid: returning DEVICE_NO_FEATURE");
            return -1;
        }
        // trace!("XAxiEthernet_SetTpid");
        // Verify TPID
        if (tpid != 0x8100) && (tpid != 0x88a8) && (tpid != 0x9100) && (tpid != 0x9200) {
            // trace!("XAxiEthernet_SetTpid: return invalid param");
            return -1;
        }
        let mut tpid_reg = 0u32;
        // Determine which register to operate on
        if entry < 2 {
            tpid_reg = self.hardware().tpid0.read().bits();
        } else {
            tpid_reg = self.hardware().tpid1.read().bits();
        }
        let mut target_value = tpid_reg;
        // Determine upper/lower 16 bits to operate on
        if entry % 2 == 1 {
            target_value = (tpid_reg & 0xFFFF0000) | ((tpid as u32) << 16);
        } else {
            target_value = (tpid_reg & 0xFFFF) | (tpid as u32);
        }
        if entry < 2 {
            self.hardware()
                .tpid0
                .write(|w| unsafe { w.bits(target_value) });
        } else {
            self.hardware()
                .tpid1
                .write(|w| unsafe { w.bits(target_value) });
        }
        // trace!("XAxiEthernet_SetTpid: returning SUCCESS");
        return 0;
    }

    pub fn clear_tpid(&self, entry: usize) -> isize {
        assert!(self.is_ready);
        assert!(entry < XAE_TPID_MAX_ENTRIES);
        // The device must be stopped before modify VLAN TPID
        if self.is_started {
            // trace!("XAxiEthernet_ClearTpid: returning DEVICE_IS_STARTED");
            return -1;
        }
        // Check hw capability
        if !self.is_ext_func_cap() {
            // trace!("XAxiEthernet_ClearTpid: returning DEVICE_NO_FEATURE");
            return -1;
        }
        // trace!("XAxiEthernet_ClearTpid");
        let mut tpid_reg = 0u32;
        // Determine which register to operate on
        if entry < 2 {
            tpid_reg = self.hardware().tpid0.read().bits();
        } else {
            tpid_reg = self.hardware().tpid1.read().bits();
        }
        let mut target_value = tpid_reg;
        // Determine upper/lower 16 bits to operate on
        if entry % 2 == 1 {
            target_value = tpid_reg & 0xFFFF0000;
        } else {
            target_value = tpid_reg & 0xFFFF;
        }
        if entry < 2 {
            self.hardware()
                .tpid0
                .write(|w| unsafe { w.bits(target_value) });
        } else {
            self.hardware()
                .tpid1
                .write(|w| unsafe { w.bits(target_value) });
        }
        // trace!("XAxiEthernet_ClearTpid: returning SUCCESS");
        return 0;
    }

    pub fn get_tpid(&self, tpid: &mut u16, entry: usize) {
        assert!(self.is_ready);
        assert!(entry < XAE_TPID_MAX_ENTRIES);
        // trace!("XAxiEthernet_GetTpid");
        let mut tpid_reg = 0u32;
        // Determine which register to operate on
        if entry < 2 {
            tpid_reg = self.hardware().tpid0.read().bits();
        } else {
            tpid_reg = self.hardware().tpid1.read().bits();
        }
        // Determine upper/lower 16 bits to operate on
        if entry % 2 == 1 {
            *tpid = (tpid_reg >> 16) as u16;
        } else {
            *tpid = (tpid_reg & 0xFFFF) as u16;
        }
        // trace!("XAxiEthernet_GetTpid: done");
    }

    pub fn set_vtag_mode(&self, mode: VtagMode, dir: Director) -> isize {
        assert!(self.is_ready);
        // The device must be stopped before modify TX VLAN Tag mode
        if self.is_started {
            // trace!("XAxiEthernet_SetVtagMode: returning DEVICE_IS_STARTED");
            return -1;
        }
        // Check hw capability
        if !self.is_ext_func_cap() {
            // trace!("XAxiEthernet_SetVtagMode: returning DEVICE_NO_FEATURE");
            return -1;
        }
        // trace!("XAxiEthernet_SetVTagMode");
        // Program HW
        match dir {
            Director::TX => {
                self.hardware()
                    .raf
                    .write(|w| unsafe { w.tx_vtag_mode().bits(mode as _) });
            }
            Director::RX => {
                self.hardware()
                    .raf
                    .write(|w| unsafe { w.rx_vtag_mode().bits(mode as _) });
            }
        }
        // trace!("XAxiEthernet_SetVTagMode: returning SUCCESS");
        return 0;
    }

    pub fn get_vtag_mode(&self, mode: &mut VtagMode, dir: Director) {
        assert!(self.is_ready);
        // trace!("XAxiEthernet_GetVTagMode");
        match dir {
            Director::TX => *mode = self.hardware().raf.read().tx_vtag_mode().bits().into(),
            Director::RX => *mode = self.hardware().raf.read().rx_vtag_mode().bits().into(),
        }
        // trace!("XAxiEthernet_GetVTagMode: done");
    }

    pub fn set_vstrip_mode(&self, mode: VtagMode, dir: Director) -> isize {
        assert!(self.is_ready);
        // The device must be stopped before modify TX VLAN Strip mode
        if self.is_started {
            // trace!("XAxiEthernet_SetVStripMode: returning DEVICE_IS_STARTED");
            return -1;
        }
        // Check hw capability
        if !self.is_ext_func_cap() {
            // trace!("XAxiEthernet_SetVStripMode: returning DEVICE_NO_FEATURE");
            return -1;
        }
        if mode == VtagMode::EXISTED {
            // trace!("XAxiEthernet_SetVStripMode: returning DEVICE_INVALID_PARAM");
            return -1;
        }
        // trace!("XAxiEthernet_SetStripMode");
        // Program HW
        match dir {
            Director::TX => {
                self.hardware()
                    .raf
                    .write(|w| unsafe { w.tx_vstrp_mode().bits(mode as _) });
            }
            Director::RX => {
                self.hardware()
                    .raf
                    .write(|w| unsafe { w.rx_vstrp_mode().bits(mode as _) });
            }
        }
        // trace!("XAxiEthernet_SetVTagMode: returning SUCCESS");
        return 0;
    }

    pub fn get_vstrip_mode(&self, mode: &mut VtagMode, dir: Director) {
        assert!(self.is_ready);
        // trace!("XAxiEthernet_GetVStripMode");
        match dir {
            Director::TX => *mode = self.hardware().raf.read().tx_vstrp_mode().bits().into(),
            Director::RX => *mode = self.hardware().raf.read().rx_vstrp_mode().bits().into(),
        }
        // trace!("XAxiEthernet_GetVStripMode: done");
    }

    pub fn set_vtag_value(&self, vtag_value: u32, dir: Director) -> isize {
        assert!(self.is_ready);
        // The device must be stopped before modify TX VLAN Tag value
        if self.is_started {
            // trace!("XAxiEthernet_SetVTagValue: returning DEVICE_IS_STARTED");
            return -1;
        }
        // Check hw capability
        if !self.is_ext_func_cap() {
            // trace!("XAxiEthernet_SetVTagValue: returning DEVICE_NO_FEATURE");
            return -1;
        }
        // Verify tpid
        let tpid = ((vtag_value >> 16) & 0xFFFF) as u16;
        if (tpid != 0x8100) && (tpid != 0x88a8) && (tpid != 0x9100) && (tpid != 0x9200) {
            // trace!("XAxiEthernet_SetVTagValue: returning DEVICE_INVALID_PARAM");
            return -1;
        }
        // trace!("XAxiEthernet_SetVTagValue");
        // Program HW
        match dir {
            Director::TX => {
                self.hardware()
                    .ttag
                    .write(|w| unsafe { w.bits(vtag_value) });
            }
            Director::RX => {
                self.hardware()
                    .rtag
                    .write(|w| unsafe { w.bits(vtag_value) });
            }
        }
        // trace!("XAxiEthernet_SetVTagValue:returning SUCCESS");
        return 0;
    }

    pub fn get_vtag_value(&self, vtag_value: &mut u32, dir: Director) {
        assert!(self.is_ready);
        // trace!("XAxiEthernet_GetVTagValue");
        match dir {
            Director::TX => *vtag_value = self.hardware().ttag.read().bits(),
            Director::RX => *vtag_value = self.hardware().rtag.read().bits(),
        }
        // trace!("XAxiEthernet_GetVTagValue: done");
    }

    pub fn set_vid_table(
        &self,
        entry: usize,
        vid: usize,
        strip: usize,
        tag: usize,
        dir: Director,
    ) -> isize {
        assert!(self.is_ready);
        assert!(entry < XAE_MAX_VLAN_TABL_ENTRY);
        assert!(vid < XAE_MAX_VLAN_TABL_ENTRY);
        assert!(strip < XAE_VLAN_TABL_STRP_FLD_LEN);
        assert!(tag < XAE_VLAN_TABL_TAG_FLD_LEN);
        // The device must be stopped before modify VLAN TPID
        if self.is_started {
            // trace!("XAxiEthernet_SetVidTable: returning DEVICE_IS_STARTED");
            return -1;
        }
        // Check hw capability
        if !self.is_ext_func_cap() {
            // trace!("XAxiEthernet_SetVidTable: returning DEVICE_NO_FEATURE");
            return -1;
        }
        // trace!("XAxiEthernet_SetVidTable");
        let value = (vid << XAE_VLAN_TABL_VID_START_OFFSET)
            | (strip << XAE_VLAN_TABL_STRP_STRT_OFFSET)
            | tag;
        match dir {
            Director::TX => {
                unsafe {
                    *(self
                        .hardware()
                        .txvlandata
                        .as_ptr()
                        .add(entry << XAE_VLAN_TABL_VID_START_OFFSET)) = value as u32
                };
            }
            Director::RX => {
                unsafe {
                    *(self
                        .hardware()
                        .rxvlandata
                        .as_ptr()
                        .add(entry << XAE_VLAN_TABL_VID_START_OFFSET)) = value as u32
                };
            }
        }
        // trace!("XAxiEthernet_SetVidTable: returning SUCCESS");
        return 0;
    }

    pub fn get_vid_table(
        &self,
        entry: usize,
        vid: &mut usize,
        strip: &mut usize,
        tag: &mut usize,
        dir: Director,
    ) {
        assert!(self.is_ready);
        assert!(entry < XAE_MAX_VLAN_TABL_ENTRY);
        // trace!("XAxiEthernet_GetVidTable");
        let reg = match dir {
            Director::TX => unsafe {
                *(self
                    .hardware()
                    .txvlandata
                    .as_ptr()
                    .add(entry << XAE_VLAN_TABL_VID_START_OFFSET))
            },
            Director::RX => unsafe {
                *(self
                    .hardware()
                    .rxvlandata
                    .as_ptr()
                    .add(entry << XAE_VLAN_TABL_VID_START_OFFSET))
            },
        };
        *vid = (reg >> XAE_VLAN_TABL_VID_START_OFFSET) as usize;
        *strip = (reg >> XAE_VLAN_TABL_STRP_STRT_OFFSET) as usize & XAE_VLAN_TABL_STRP_ENTRY_MASK;
        *tag = (reg & XAE_VLAN_TABL_TAG_ENTRY_MASK as u32) as usize;
        // trace!("XAxiEthernet_GetVidTable: done");
    }

    pub fn add_ext_mukticast_group(&self, address: &[u8; 6]) -> isize {
        assert!(self.is_ready);
        // trace!("XAxiEthernet_AddExtMulticastGroup");
        // The device must be stopped before setting the multicast table.
        if self.is_started {
            // trace!("XAxiEthernet_AddExtMulticastGroup: returning DEVICE_IS_STARTED");
            return -1;
        }
        // Check hw capability
        if !self.is_ext_func_cap() || !self.is_ext_mcast_enable() {
            // trace!("XAxiEthernet_AddExtMulticastGroup: returning DEVICE_NO_FEATURE");
            return -1;
        }
        if (address[0] != 0x01)
            || (address[1] != 0x00)
            || (address[2] != 0x5e)
            || ((address[3] & 0x80) != 0x0)
        {
            // trace!("XAxiEthernet_AddExtMulticastGroup: returning DEVICE_INVALID_PARAM");
            return -1;
        }
        let mut loc = address[3] as u32;
        loc <<= 8;
        loc |= address[4] as u32;
        loc <<= 2;
        unsafe { *self.hardware().mcasttable.as_ptr().add(loc as usize) = 0x1 };
        // trace!("XAxiEthernet_AddExtMulticastGroup: returning SUCCESS");
        return 0;
    }

    pub fn clear_ext_mukticast_group(&self, address: &[u8; 6]) -> isize {
        assert!(self.is_ready);
        // trace!("XAxiEthernet_ClearExtMulticastGroup");
        // The device must be stopped before setting the multicast table.
        if self.is_started {
            // trace!("XAxiEthernet_ClearExtMulticastGroup: returning DEVICE_IS_STARTED");
            return -1;
        }
        // Check hw capability
        if !self.is_ext_func_cap() || !self.is_ext_mcast_enable() {
            // trace!("XAxiEthernet_ClearExtMulticastGroup: returning DEVICE_NO_FEATURE");
            return -1;
        }
        if (address[0] != 0x01)
            || (address[1] != 0x00)
            || (address[2] != 0x5e)
            || ((address[3] & 0x80) != 0x0)
        {
            // trace!("XAxiEthernet_ClearExtMulticastGroup: returning DEVICE_INVALID_PARAM");
            return -1;
        }
        let mut loc = address[3] as u32;
        loc <<= 8;
        loc |= address[4] as u32;
        loc <<= 2;
        unsafe { *self.hardware().mcasttable.as_ptr().add(loc as usize) = 0x0 };
        // trace!("XAxiEthernet_ClearExtMulticastGroup: returning SUCCESS");
        return 0;
    }

    pub fn get_ext_mukticast_group(&self, address: &[u8; 6]) -> bool {
        assert!(self.is_ready);
        // trace!("XAxiEthernet_GetExtMulticastGroup");
        if (address[0] != 0x01)
            || (address[1] != 0x00)
            || (address[2] != 0x5e)
            || ((address[3] & 0x80) != 0x0)
        {
            // trace!("XAxiEthernet_GetExtMulticastGroup: returning DEVICE_INVALID_PARAM");
            return false;
        }
        let mut loc = address[3] as u32;
        loc <<= 8;
        loc |= address[4] as u32;
        loc <<= 2;
        let bit = unsafe { *self.hardware().mcasttable.as_ptr().add(loc as usize) };
        // trace!("XAxiEthernet_GetExtMulticastGroup: done");
        if (bit & 0x1) == 1 {
            return true;
        } else {
            return false;
        }
    }

    pub fn dump_ext_mukticast_group(&self) {
        let mut mac_addr: [u8; 6] = [0x01, 0x00, 0x5e, 0x0, 0x0, 0x0];
        let mut loc: u32 = 0;
        for idx in 0usize..(1 << 15) {
            mac_addr[3] = (idx << 16) as u8;
            mac_addr[4] = (idx << 8) as u8;
            mac_addr[5] = 0;
            loc = mac_addr[3] as u32;
            loc |= (mac_addr[4] as u32) << 8;
            loc <<= 2;
            let bit = unsafe { *self.hardware().mcasttable.as_ptr().add(loc as usize) };
            if (bit & 0x1) == 1 {
                // trace!("{:#x}:{:#x}:{:#x}:{:#x}:{:#x}:{:#x}", mac_addr[5], mac_addr[4], mac_addr[3], mac_addr[2], mac_addr[1], mac_addr[0])
            }
        }
    }
}

// primary functionalities of AxiEthernet
impl AxiEthernet {
    pub fn start(&mut self) {
        assert!(self.is_ready);
        // If already started, then there is nothing to do
        if self.is_started {
            return;
        }
        // trace!("XAxiEthernet_Start");
        // Enable transmitter if not already enabled
        if (self.options & XAE_TRANSMITTER_ENABLE_OPTION) != 0 {
            // // trace!("enabling transmitter");
            if self.hardware().ttc.read().tx().is_disable() {
                // // trace!("transmitter not enabled, enabling now");
                self.hardware().ttc.write(|w| w.tx().bit(true));
            }
            // trace!("transmitter enabled");
        }
        // Enable receiver
        if (self.options & XAE_RECEIVER_ENABLE_OPTION) != 0 {
            // // trace!("enabling receiver");
            if self.hardware().trcw1.read().rx().is_disable() {
                // // trace!("receiver not enabled, enabling now");
                self.hardware().trcw1.write(|w| w.rx().bit(true));
            }
            // trace!("receiver enabled");
        }
        self.is_started = true;
        // trace!("XAxiEthernet_Start: done");
    }

    pub fn stop(&mut self) {
        assert!(self.is_ready);
        // If already stopped, then there is nothing to do
        if !self.is_started {
            return;
        }
        // trace!("XAxiEthernet_Stop");
        // trace!("XAxiEthernet_Stop: disabling interrupts");
        // Disable interrupts
        self.hardware().ie.write(|w| unsafe { w.bits(0) });
        // trace!("XAxiEthernet_Stop: disabling receiver");
        // Disable the receiver
        self.hardware().trcw1.write(|w| w.rx().bit(false));
        // Stopping the receiver in mid-packet causes a dropped packet indication from HW. Clear it.
        if self.hardware().ip.read().rx_rject().bit() {
            self.hardware().is.write(|w| w.rx_rject().bit(true));
        }
        self.is_started = false;
        // trace!("XAxiEthernet_Stop: done");
    }

    pub fn reset(&mut self) {
        assert!(self.is_ready);
        // Add delay of 1 sec to give enough time to the core to come
        // out of reset. Till the time core comes out of reset none of the
        // AxiEthernet registers are accessible including the IS register.
        let mut time_out_loops = XAE_LOOPS_TO_COME_OUT_OF_RST as isize;
        while time_out_loops > 0 {
            time_out_loops -= 1;
        }
        // Check the status of the MgtRdy bit in the interrupt status
        // registers. This must be done to allow the MGT clock to become stable
        // for the Sgmii and 1000BaseX PHY interfaces. No other register reads
        // will be valid until this bit is valid.
        // The bit is always a 1 for all other PHY interfaces.
        // time_out_loops = XAE_LOOPS_TO_COME_OUT_OF_RST as isize;
        loop {
            if self.hardware().is.read().mgt_rdy().bit() {
                break;
            } else {
                time_out_loops -= 1;
                if time_out_loops < 0 {
                    break;
                }
            }
        }
        if time_out_loops == -1 {
            // trace!("XAxiEthernet_Reset: return NO_MGTPDY, TIME_OUT");
            return;
        }
        trace!("XAxiEthernet_Reset");
        self.stop();
        self.options = XAE_DEFAULT_OPTIONS;
        self.init_hw();
    }

    pub fn init_hw(&mut self) {
        // trace!("XAxiEthernet InitHw");
        // Disable the receiver
        self.hardware().trcw1.write(|w| w.rx().bit(false));
        // Stopping the receiver in mid-packet causes a dropped packet indication from HW. Clear it.
        // Get the interrupt pending register
        if self.hardware().ip.read().rx_rject().bit() {
            // Set the interrupt status register to clear the pending interrupt
            self.hardware().is.write(|w| w.rx_rject().bit(true));
        }
        // Sync default options with HW but leave receiver and transmitter
        // disabled. They get enabled with XAxiEthernet_Start() if
        // XAE_TRANSMITTER_ENABLE_OPTION and XAE_RECEIVER_ENABLE_OPTION
        // are set
        self.set_options(
            self.options & !(XAE_TRANSMITTER_ENABLE_OPTION | XAE_RECEIVER_ENABLE_OPTION),
        );
        self.clear_options(!self.options);
        // Set default MDIO divisor
        self.set_phy_mdio_divisor(XAE_MDIO_DIV_DFT as u8);
        // trace!("XAxiEthernet InitHw: done");
    }

    pub fn set_mac_address(&self, address: &[u8; 6]) -> isize {
        assert!(self.is_ready);
        // Be sure device has been stopped
        if self.is_started {
            // trace!("XAxiEthernet_SetMacAddress: return DEVICE_IS_STARTED");
            return -1;
        }
        // trace!("XAxiEthernet_SetMacAddress: set mac address to: {:#x}:{:#x}:{:#x}:{:#x}:{:#x}:{:#x}", address[0], address[1], address[2], address[3], address[4], address[5]);
        let mut mac_address = address[0] as u32;
        mac_address |= (address[1] as u32) << 8;
        mac_address |= (address[2] as u32) << 16;
        mac_address |= (address[3] as u32) << 24;
        // Check to see if it is in extended/new mode.
        if !self.is_ext_func_cap() {
            // Set the MAC bits [31:0] in UAW0. Having Aptr be unsigned type prevents the following
            // operations from sign extending.
            self.hardware()
                .uaw0
                .write(|w| unsafe { w.bits(mac_address) });
            let mut addr47t32 = address[4] as u16;
            addr47t32 |= (address[5] as u16) << 8;
            // Set MAC bits [47:32] in UAW1
            self.hardware()
                .uaw1
                .write(|w| unsafe { w.address47t32().bits(addr47t32) });
            // trace!("XAxiEthernet_SetMacAddress: SUCCESS");
            return 0;
        } else {
            // Set the MAC bits [31:0] in UAWL register. Having Aptr be unsigned type prevents the following
            // operations from sign extending.
            self.hardware()
                .uawl
                .write(|w| unsafe { w.bits(mac_address) });
            mac_address = address[4] as u32;
            mac_address |= (address[5] as u32) << 8;
            // Set MAC bits [47:32] in UAWU register.
            self.hardware()
                .uawu
                .write(|w| unsafe { w.bits(mac_address) });
            // trace!("XAxiEthernet_SetMacAddress: SUCCESS");
            return 0;
        }
    }

    pub fn get_mac_address(&self, address: &mut [u8; 6]) {
        assert!(self.is_ready);
        if !self.is_ext_func_cap() {
            let mut mac_address = self.hardware().uaw0.read().bits();
            address[0] = (mac_address & 0xFF) as u8;
            address[1] = ((mac_address >> 8) & 0xFF) as u8;
            address[2] = ((mac_address >> 16) & 0xFF) as u8;
            address[3] = ((mac_address >> 24) & 0xFF) as u8;
            mac_address = self.hardware().uaw1.read().bits();
            address[4] = (mac_address & 0xFF) as u8;
            address[5] = ((mac_address >> 8) & 0xFF) as u8;
        } else {
            let mut mac_address = self.hardware().uawl.read().bits();
            address[0] = (mac_address & 0xFF) as u8;
            address[1] = ((mac_address >> 8) & 0xFF) as u8;
            address[2] = ((mac_address >> 16) & 0xFF) as u8;
            address[3] = ((mac_address >> 24) & 0xFF) as u8;
            mac_address = self.hardware().uawu.read().bits();
            address[4] = (mac_address & 0xFF) as u8;
            address[5] = ((mac_address >> 8) & 0xFF) as u8;
        }
    }

    pub fn update_dep_options(&self) -> usize {
        let mut dep_options = self.options;
        // Enable extended multicast option
        if (dep_options & XAE_EXT_MULTICAST_OPTION) != 0 {
            if self.is_ext_mcast() {
                dep_options |= XAE_PROMISC_OPTION;
                // trace!("CheckDepOptions: enabling ext multicast");
            } else {
                // trace!("EXT MULTICAST is not built in hardware");
            }
        }
        // Enable extended transmit VLAN translation option
        if (dep_options & XAE_EXT_TXVLAN_TRAN_OPTION) != 0 {
            if self.is_tx_vlan_tran() {
                dep_options |= XAE_FCS_INSERT_OPTION;
                dep_options &= !XAE_VLAN_OPTION;
                // trace!("CheckDepOptions: enabling ext Tx VLAN trans");
            } else {
                // trace!("TX VLAN TRANSLATION is not built in hardware");
            }
        }
        // Enable extended receive VLAN translation option
        if (dep_options & XAE_EXT_RXVLAN_TRAN_OPTION) != 0 {
            if self.is_rx_vlan_tran() {
                dep_options |= XAE_FCS_STRIP_OPTION;
                dep_options &= !XAE_VLAN_OPTION;
                // trace!("CheckDepOptions: enabling ext Rx VLAN trans");
            } else {
                // trace!("RX VLAN TRANSLATION is not built in hardware");
            }
        }
        // Enable extended transmit VLAN tag option
        if (dep_options & XAE_EXT_TXVLAN_TAG_OPTION) != 0 {
            if self.is_tx_vlan_tag() {
                dep_options |= XAE_FCS_INSERT_OPTION;
                dep_options &= !XAE_VLAN_OPTION;
                dep_options |= XAE_JUMBO_OPTION;
                // trace!("CheckDepOptions: enabling ext Tx VLAN tag");
            } else {
                // trace!("TX VLAN TAG is not built in hardware");
            }
        }
        // Enable extended receive VLAN tag option
        if (dep_options & XAE_EXT_RXVLAN_TAG_OPTION) != 0 {
            if self.is_rx_vlan_tag() {
                dep_options |= XAE_FCS_STRIP_OPTION;
                dep_options &= !XAE_VLAN_OPTION;
                dep_options |= XAE_JUMBO_OPTION;
                // trace!("CheckDepOptions: enabling ext Rx VLAN tag");
            } else {
                // trace!("RX VLAN TAG is not built in hardware");
            }
        }
        // Enable extended transmit VLAN strip option
        if (dep_options & XAE_EXT_TXVLAN_STRP_OPTION) != 0 {
            if self.is_tx_vlan_strp() {
                dep_options |= XAE_FCS_INSERT_OPTION;
                dep_options &= !XAE_VLAN_OPTION;
                dep_options |= XAE_JUMBO_OPTION;
                // trace!("CheckDepOptions: enabling ext Tx VLAN strip");
            } else {
                // trace!("TX VLAN strip is not built in hardware");
            }
        }
        // Enable extended receive VLAN strip option
        if (dep_options & XAE_EXT_TXVLAN_STRP_OPTION) != 0 {
            if self.is_rx_vlan_strp() {
                dep_options |= XAE_FCS_STRIP_OPTION;
                dep_options &= !XAE_VLAN_OPTION;
                dep_options |= XAE_JUMBO_OPTION;
                // trace!("CheckDepOptions: enabling ext Rx VLAN strip");
            } else {
                // trace!("RX VLAN strip is not built in hardware");
            }
        }
        return dep_options;
    }

    pub fn set_options(&mut self, options: usize) -> isize {
        assert!(self.is_ready);
        // Be sure device has been stopped
        if self.is_started {
            // trace!("XAxiEthernet_SetOptions: return DEVICE_IS_STARTED");
            return -1;
        }
        // trace!("XAxiEthernet_SetOptions");
        self.options |= options;
        let dep_options = self.update_dep_options();
        // New/extended function bit should be on if any new/extended features
        // are on and hardware is built with them.
        if (dep_options
            & (XAE_EXT_MULTICAST_OPTION
                | XAE_EXT_TXVLAN_TRAN_OPTION
                | XAE_EXT_RXVLAN_TRAN_OPTION
                | XAE_EXT_TXVLAN_TAG_OPTION
                | XAE_EXT_RXVLAN_TAG_OPTION
                | XAE_EXT_TXVLAN_STRP_OPTION
                | XAE_EXT_RXVLAN_STRP_OPTION))
            != 0
        {
            self.hardware().raf.write(|w| w.new_fnc_enbl().bit(true));
        }
        // Many of these options will change the RCW1 or TC registers.
        // To reduce the amount of IO to the device, group these options here
        // and change them all at once.
        let mut trcw1 = self.hardware().trcw1.read().bits();
        let mut ttc = self.hardware().ttc.read().bits();
        let mut new_trcw1 = trcw1;
        let mut new_ttc = ttc;
        // // trace!("current control regs: RCW1: {:#x}; TC: {:#x}", trcw1, ttc);
        // // trace!("Options: {:#x}; default options: {:#x}", options, XAE_DEFAULT_OPTIONS);
        // Turn on jumbo packet support for both Rx and Tx
        if (dep_options & XAE_JUMBO_OPTION) != 0 {
            new_ttc |= XAE_TC_JUM_MASK as u32;
            new_trcw1 |= XAE_RCW1_JUM_MASK as u32;
        }
        // Turn on VLAN packet support for both Rx and Tx
        if (dep_options & XAE_VLAN_OPTION) != 0 {
            new_ttc |= XAE_TC_VLAN_MASK as u32;
            new_trcw1 |= XAE_RCW1_VLAN_MASK as u32;
        }
        // Turn on FCS stripping on receive packets
        if (dep_options & XAE_FCS_STRIP_OPTION) != 0 {
            // // trace!("setOptions: enabling fcs stripping");
            new_trcw1 &= !(XAE_RCW1_FCS_MASK as u32);
        }
        // Turn on FCS insertion on transmit packets
        if (dep_options & XAE_FCS_INSERT_OPTION) != 0 {
            // // trace!("setOptions: enabling fcs insertion");
            new_ttc &= !(XAE_TC_FCS_MASK as u32);
        }
        // Turn on length/type field checking on receive packets
        if (dep_options & XAE_LENTYPE_ERR_OPTION) != 0 {
            new_trcw1 &= !(XAE_RCW1_LT_DIS_MASK as u32);
        }
        // Enable transmitter
        if (dep_options & XAE_TRANSMITTER_ENABLE_OPTION) != 0 {
            new_ttc |= XAE_TC_TX_MASK as u32;
        }
        // Enable receiver
        if (dep_options & XAE_RECEIVER_ENABLE_OPTION) != 0 {
            new_trcw1 |= XAE_RCW1_RX_MASK as u32;
        }
        // Change the TC or RCW1 registers if they need to be modified
        if ttc != new_ttc {
            // // trace!("setOptions: writing tc: {:#x}", new_ttc);
            self.hardware().ttc.write(|w| unsafe { w.bits(new_ttc) });
        }
        if trcw1 != new_trcw1 {
            // // trace!("setOptions: writing rcw1: {:#x}", new_trcw1);
            self.hardware()
                .trcw1
                .write(|w| unsafe { w.bits(new_trcw1) });
        }
        // Rest of options twiddle bits of other registers.
        // Handle them one at a time

        // Turn on flow control
        if (dep_options & XAE_FLOW_CONTROL_OPTION) != 0 {
            // // trace!("setOptions: enabling flow control");
            self.hardware().tfcc.write(|w| w.rxfce().bit(true));
        }
        // // trace!("setOptions: rcw1 is now (fcc):{:#x}",
        //     self.hardware().trcw1.read().bits());
        // Turn on promiscuous frame filtering (all frames are received )
        if (dep_options & XAE_PROMISC_OPTION) != 0 {
            // // trace!("setOptions: enabling promiscuous mode");
            self.hardware().fmi.write(|w| w.pmode().bit(true));
        }
        // // trace!("setOptions: rcw1 is now (fcc):{:#x}",
        //     self.hardware().trcw1.read().bits());
        // Allow broadcast address filtering
        if (dep_options & XAE_BROADCAST_OPTION) != 0 {
            self.hardware().raf.write(|w| w.bcst_rej().bit(false));
        }
        // // trace!("setOptions: rcw1 is now (fcc):{:#x}",
        //     self.hardware().trcw1.read().bits());
        // Allow multicast address filtering
        if (dep_options & (XAE_MULTICAST_OPTION | XAE_EXT_MULTICAST_OPTION)) != 0 {
            self.hardware().raf.write(|w| w.mcst_rej().bit(false));
        }
        // // trace!("setOptions: rcw1 is now (fcc):{:#x}",
        //     self.hardware().trcw1.read().bits());
        // The remaining options not handled here are managed elsewhere in the
        // driver. No register modifications are needed at this time.
        // Reflecting the option in InstancePtr->Options is good enough for
        // now.

        // Enable extended multicast option
        if (dep_options & XAE_EXT_MULTICAST_OPTION) != 0 {
            self.hardware()
                .raf
                .write(|w| w.emulti_fltr_enbl().bit(true));
        }
        // New/extended [TX|RX] VLAN translation option does not have specific
        // bits to on/off.

        // Enable extended transmit VLAN tag option
        if (dep_options & XAE_EXT_TXVLAN_TAG_OPTION) != 0 {
            self.hardware()
                .raf
                .write(|w| w.tx_vtag_mode().bits(XAE_DEFAULT_TXVTAG_MODE as u8));
        }
        // Enable extended receive VLAN tag option
        if (dep_options & XAE_EXT_RXVLAN_TAG_OPTION) != 0 {
            self.hardware()
                .raf
                .write(|w| w.rx_vtag_mode().bits(XAE_DEFAULT_RXVTAG_MODE as u8));
        }
        // Enable extended transmit VLAN strip option
        if (dep_options & XAE_EXT_TXVLAN_STRP_OPTION) != 0 {
            self.hardware()
                .raf
                .write(|w| unsafe { w.tx_vstrp_mode().bits(XAE_DEFAULT_TXVSTRP_MODE as u8) });
        }
        // Enable extended receive VLAN strip option
        if (dep_options & XAE_EXT_TXVLAN_STRP_OPTION) != 0 {
            self.hardware()
                .raf
                .write(|w| unsafe { w.rx_vstrp_mode().bits(XAE_DEFAULT_RXVSTRP_MODE as u8) });
        }
        // trace!("setOptions: returning SUCCESS");
        return 0;
    }

    pub fn clear_options(&mut self, options: usize) -> isize {
        assert!(self.is_ready);
        // trace!("XAxiEthernet_ClearOptions: {:#x}", options);
        // Be sure device has been stopped
        if self.is_started {
            // trace!("XAxiEthernet_ClearOptions: return DEVICE_IS_STARTED");
            return -1;
        }
        self.options &= !options;
        let dep_options = !(self.update_dep_options());
        // New/extended function bit should be off if none of new/extended
        // features is on after hardware is validated and gone through
        // _UpdateDepOptions().
        if (!(!dep_options
            & (XAE_EXT_MULTICAST_OPTION
                | XAE_EXT_TXVLAN_TRAN_OPTION
                | XAE_EXT_RXVLAN_TRAN_OPTION
                | XAE_EXT_TXVLAN_TAG_OPTION
                | XAE_EXT_RXVLAN_TAG_OPTION
                | XAE_EXT_TXVLAN_STRP_OPTION
                | XAE_EXT_RXVLAN_STRP_OPTION)))
            != 0
        {
            self.hardware().raf.write(|w| w.new_fnc_enbl().bit(false));
        }
        // Many of these options will change the RCW1 or TC registers.
        // Group these options here and change them all at once. What we are
        // trying to accomplish is to reduce the amount of IO to the device

        // Get the current register contents
        let mut trcw1 = self.hardware().trcw1.read().bits();
        let mut ttc = self.hardware().ttc.read().bits();
        let mut new_trcw1 = trcw1;
        let mut new_ttc = ttc;
        // Turn off jumbo packet support for both Rx and Tx
        if (dep_options & XAE_JUMBO_OPTION) != 0 {
            // // trace!("XAxiEthernet_ClearOptions: disabling jumbo");
            new_ttc &= !XAE_TC_JUM_MASK as u32;
            new_trcw1 &= !XAE_RCW1_JUM_MASK as u32;
        }
        // Turn off VLAN packet support for both Rx and Tx
        if (dep_options & XAE_VLAN_OPTION) != 0 {
            // // trace!("XAxiEthernet_ClearOptions: disabling vlan");
            new_ttc &= !XAE_TC_VLAN_MASK as u32;
            new_trcw1 &= !XAE_RCW1_VLAN_MASK as u32;
        }
        // Turn off FCS stripping on receive packets
        if (dep_options & XAE_FCS_STRIP_OPTION) != 0 {
            // // trace!("XAxiEthernet_ClearOptions: disabling fcs strip");
            new_trcw1 |= XAE_RCW1_FCS_MASK as u32;
        }
        // Turn off FCS insertion on transmit packets
        if (dep_options & XAE_FCS_INSERT_OPTION) != 0 {
            // // trace!("XAxiEthernet_ClearOptions: disabling fcs insert");
            new_ttc |= XAE_TC_FCS_MASK as u32;
        }
        // Turn off length/type field checking on receive packets
        if (dep_options & XAE_LENTYPE_ERR_OPTION) != 0 {
            // // trace!("XAxiEthernet_ClearOptions: disabling lentype err");
            new_trcw1 |= XAE_RCW1_LT_DIS_MASK as u32;
        }
        // Disable transmitter
        if (dep_options & XAE_TRANSMITTER_ENABLE_OPTION) != 0 {
            // // trace!("XAxiEthernet_ClearOptions: disabling transmitter");
            new_ttc &= !XAE_TC_TX_MASK as u32;
        }
        // Disable receiver
        if (dep_options & XAE_RECEIVER_ENABLE_OPTION) != 0 {
            // // trace!("XAxiEthernet_ClearOptions: disabling receiver");
            new_trcw1 &= !XAE_RCW1_RX_MASK as u32;
        }
        // Change the TC and RCW1 registers if they need to be modified
        if ttc != new_ttc {
            // // trace!("XAxiEthernet_ClearOptions: setting TC: {:#x}", new_ttc);
            self.hardware().ttc.write(|w| unsafe { w.bits(new_ttc) });
        }
        if trcw1 != new_trcw1 {
            // // trace!("XAxiEthernet_ClearOptions: setting RCW1: {:#x}", new_trcw1);
            self.hardware()
                .trcw1
                .write(|w| unsafe { w.bits(new_trcw1) });
        }
        // Rest of options twiddle bits of other registers. Handle them one at a time

        // Turn off flow control
        if (dep_options & XAE_FLOW_CONTROL_OPTION) != 0 {
            // // trace!("XAxiEthernet_ClearOptions: disabling flow control");
            self.hardware().tfcc.write(|w| w.rxfce().bit(false));
        }
        // Turn off promiscuous frame filtering
        if (dep_options & XAE_PROMISC_OPTION) != 0 {
            // // trace!("XAxiEthernet_ClearOptions: disabling promiscuous mode");
            self.hardware().fmi.write(|w| w.pmode().bit(false));
        }
        // Disable broadcast address filtering
        if (dep_options & XAE_BROADCAST_OPTION) != 0 {
            // trace!("XAxiEthernet_ClearOptions: disabling broadcast mode");
            self.hardware().raf.write(|w| w.bcst_rej().bit(true));
        }
        // Disable multicast address filtering
        if (dep_options & XAE_MULTICAST_OPTION) != 0
            && (dep_options & XAE_EXT_MULTICAST_OPTION) != 0
        {
            // // trace!("XAxiEthernet_ClearOptions: disabling multicast mode");
            self.hardware().raf.write(|w| w.mcst_rej().bit(true));
        }
        // Disable extended multicast option
        if (dep_options & XAE_EXT_MULTICAST_OPTION) != 0 {
            // // trace!("XAxiEthernet_ClearOptions:disabling extended multicast mode");
            self.hardware()
                .raf
                .write(|w| w.emulti_fltr_enbl().bit(false));
        }
        // Disable extended transmit VLAN tag option
        if (dep_options & XAE_EXT_TXVLAN_TAG_OPTION) != 0 {
            // // trace!("XAxiEthernet_ClearOptions:disabling extended TX VLAN tag mode");
            self.hardware()
                .raf
                .write(|w| w.tx_vtag_mode().bits(VtagMode::NONE as u8));
        }
        // Disable extended receive VLAN tag option
        if (dep_options & XAE_EXT_RXVLAN_TAG_OPTION) != 0 {
            // // trace!("XAxiEthernet_ClearOptions:disabling extended RX VLAN tag mode");
            self.hardware()
                .raf
                .write(|w| w.rx_vtag_mode().bits(VtagMode::NONE as u8));
        }
        // Disable extended transmit VLAN strip option
        if (dep_options & XAE_EXT_TXVLAN_STRP_OPTION) != 0 {
            // // trace!("XAxiEthernet_ClearOptions:disabling extended TX VLAN strip mode");
            self.hardware()
                .raf
                .write(|w| unsafe { w.tx_vstrp_mode().bits(VtagMode::NONE as u8) });
        }
        // Disable extended receive VLAN strip option
        if (dep_options & XAE_EXT_TXVLAN_STRP_OPTION) != 0 {
            // // trace!("XAxiEthernet_ClearOptions:disabling extended RX VLAN strip mode");
            self.hardware()
                .raf
                .write(|w| unsafe { w.rx_vstrp_mode().bits(VtagMode::NONE as u8) });
        }
        // trace!("ClearOptions: returning SUCCESS");
        return 0;
    }

    pub fn get_options(&self) -> usize {
        assert!(self.is_ready);
        self.options
    }

    pub fn get_operating_speed(&self) -> usize {
        assert!(self.is_ready);
        let speed_reg = self.hardware().emmc.read().msc();
        if speed_reg.is_1000m() {
            // trace!("XAxiEthernet_GetOperatingSpeed: returning 1000");
            XAE_SPEED_1000_MBPS
        } else if speed_reg.is_100m() {
            // trace!("XAxiEthernet_GetOperatingSpeed: returning 100");
            XAE_SPEED_100_MBPS
        } else if speed_reg.is_10m() {
            // trace!("XAxiEthernet_GetOperatingSpeed: returning 10");
            XAE_SPEED_10_MBPS
        } else {
            0
        }
    }

    pub fn set_operating_speed(&self, speed: u16) -> isize {
        assert!(self.is_ready);
        // trace!("XAxiEthernet_SetOperatingSpeed");
        // trace!("XAxiEthernet_SetOperatingSpeed: setting speed to: {} {:#x}", speed, speed);
        let temac_type = self.get_temac_type();
        // trace!("temac_type: {:?}", temac_type);
        let phy_type = self.get_phy_interface();
        // trace!("phy_type: {:?}", phy_type);
        // The following code checks for all allowable speed conditions before
        // writing to the register. Please refer to the hardware specs for
        // more information on it.
        // For PHY type 1000Base-x, 10 and 100 Mbps are not supported.
        // For soft/hard Axi Ethernet core, 1000 Mbps is supported in all PHY
        // types except MII.
        let mut can_set_speed = true;
        if (speed == XAE_SPEED_10_MBPS as u16) || (speed == XAE_SPEED_100_MBPS as u16) {
            if phy_type == PhyType::BASEX1000 {
                can_set_speed = false;
            }
        } else {
            if (speed == XAE_SPEED_1000_MBPS as u16) && (phy_type == PhyType::MII) {
                can_set_speed = false;
            }
        }
        if can_set_speed {
            // Get the current contents of the EMAC config register and
            // zero out speed bits
            let speed_reg = self.hardware().emmc.read().msc();
            if speed_reg.is_1000m() {
                // trace!("XAxiEthernet_SetOperatingSpeed: current speed: 1000");
            } else if speed_reg.is_100m() {
                // trace!("XAxiEthernet_SetOperatingSpeed: current speed: 100");
            } else if speed_reg.is_10m() {
                // trace!("XAxiEthernet_SetOperatingSpeed: current speed: 10");
            } else {
                // trace!("XAxiEthernet_SetOperatingSpeed: current speed: 0");
            }
            match speed as usize {
                XAE_SPEED_10_MBPS => {}
                XAE_SPEED_100_MBPS => self.hardware().emmc.write(|w| w.msc()._100m()),
                XAE_SPEED_1000_MBPS => self.hardware().emmc.write(|w| w.msc()._1000m()),
                XAE_SPEED_2500_MBPS => self.hardware().emmc.write(|w| w.msc()._1000m()),
                _ => {
                    // trace!("XAxiEthernet_SetOperatingSpeed: invalid speed");
                    return -1;
                }
            };
            // trace!("XAxiEthernet_SetOperatingSpeed: done");
            return 0;
        } else {
            // trace!("Speed not compatible with the Axi Ethernet Phy type");
            return -1;
        }
    }

    pub fn set_bad_frm_rcv_option(&self) {
        self.hardware().raf.write(|w| w.rx_bad_frm_en().bit(true));
    }

    pub fn clear_bad_frm_rcv_option(&self) {
        self.hardware().raf.write(|w| w.rx_bad_frm_en().bit(false));
    }

    pub fn disable_ctrl_frm_len_check(&self) {
        self.hardware().trcw1.write(|w| w.cl_dis().bit(true));
    }

    pub fn enable_ctrl_frm_len_check(&self) {
        self.hardware().trcw1.write(|w| w.cl_dis().bit(false));
    }

    pub fn set_phy_mdio_divisor(&self, divisor: u8) {
        assert!(self.is_ready);
        assert!(divisor < XAE_MDIO_MC_CLOCK_DIVIDE_MAX as u8);
        self.hardware().trcw1.write(|w| w.cl_dis().bit(true));
        // trace!("XAxiEthernet_PhySetMdioDivisor");

        self.hardware()
            .mdiomc
            .write(|w| unsafe { w.bits((divisor as u32) | XAE_MDIO_MC_MDIOEN_MASK as u32) });
    }

    pub fn phy_read(&self, phy_address: u32, register_num: u32, phy_data: &mut u16) {
        assert!(self.is_ready);
        assert!(phy_address <= XAE_PHY_ADDR_LIMIT as u32);
        assert!(register_num <= XAE_PHY_REG_NUM_LIMIT as u32);
        // // trace!("XAxiEthernet_PhyRead: BaseAddress: {:#x}", self.base_address);
        // Wait till MDIO interface is ready to accept a new transaction.
        let mut time_out_loops = XAE_LOOPS_TO_COME_OUT_OF_RST as isize;
        loop {
            if self.hardware().mdiomcr.read().ready().bit() {
                break;
            } else {
                time_out_loops -= 1;
                if time_out_loops < 0 {
                    break;
                }
            }
        }
        if time_out_loops == -1 {
            // trace!("XAxiEthernet_PhyRead: return MDIO is not ready, TIME_OUT");
            return;
        }
        self.hardware().mdiomcr.write(|w| unsafe {
            w.tx_phyad()
                .bits(phy_address as u8)
                .tx_regad()
                .bits(register_num as u8)
                .init()
                .bit(true)
                .tx_op()
                .ra()
        });
        // Wait till MDIO transaction is completed.
        time_out_loops = XAE_LOOPS_TO_COME_OUT_OF_RST as isize;
        loop {
            if self.hardware().mdiomcr.read().ready().bit() {
                break;
            } else {
                time_out_loops -= 1;
                if time_out_loops < 0 {
                    break;
                }
            }
        }
        if time_out_loops == -1 {
            // trace!("XAxiEthernet_PhyRead: MDIO is not complete, return TIME_OUT");
            return;
        }
        *phy_data = self.hardware().mdiomrd.read().bits() as u16;
        // // trace!("XAxiEthernet_PhyRead: Value retrieved: {:3x}", *phy_data);
    }

    pub fn phy_write(&self, phy_address: u32, register_num: u32, phy_data: u16) {
        assert!(self.is_ready);
        assert!(phy_address <= XAE_PHY_ADDR_LIMIT as u32);
        assert!(register_num <= XAE_PHY_REG_NUM_LIMIT as u32);
        // // trace!("XAxiEthernet_PhyWrite");
        // Wait till MDIO interface is ready to accept a new transaction.
        let mut time_out_loops = XAE_LOOPS_TO_COME_OUT_OF_RST as isize;
        loop {
            if self.hardware().mdiomcr.read().ready().bit() {
                break;
            } else {
                time_out_loops -= 1;
                if time_out_loops < 0 {
                    break;
                }
            }
        }
        if time_out_loops == -1 {
            // trace!("XAxiEthernet_PhyWrite: return MDIO is not ready, TIME_OUT");
            return;
        }
        self.hardware()
            .mdiomwd
            .write(|w| unsafe { w.bits(phy_data as u32) });
        self.hardware().mdiomcr.write(|w| unsafe {
            w.tx_phyad()
                .bits(phy_address as u8)
                .tx_regad()
                .bits(register_num as u8)
                .init()
                .bit(true)
                .tx_op()
                .wa()
        });
        // Wait till MDIO transaction is completed.
        time_out_loops = XAE_LOOPS_TO_COME_OUT_OF_RST as isize;
        loop {
            if self.hardware().mdiomcr.read().ready().bit() {
                break;
            } else {
                time_out_loops -= 1;
                if time_out_loops < 0 {
                    break;
                }
            }
        }
        if time_out_loops == -1 {
            // trace!("XAxiEthernet_PhyWrite: MDIO is not complete, return TIME_OUT");
            return;
        }
    }
}

// phy detect and get_speed
impl AxiEthernet {
    // this function will detect phy device
    pub fn detect_phy(&mut self) {
        let mut phy_reg = 0u16;
        let mut phy_id = 0u16;
        for phy_addr in 0..32 {
            self.phy_read(phy_addr, 1, &mut phy_reg);
            if (phy_reg != 0xFFFF) && ((phy_reg & PHY_DETECT_MASK as u16) == PHY_DETECT_MASK as u16)
            {
                // Found a valid PHY address
                // trace!("XAxiEthernet detect_phy: PHY detected at address {}.", phy_addr);
                self.phy_read(phy_addr, PHY_IDENTIFIER_1_REG as u32, &mut phy_reg);
                if (phy_reg != PHY_MARVELL_IDENTIFIER as u16)
                    && (phy_reg != PHY_TI_IDENTIFIER as u16)
                {
                    // trace!("WARNING: Not a Marvell or TI Ethernet PHY. Please verify the initialization sequence");
                }
                self.phy_addr = phy_addr;
                return;
            }
            self.phy_read(phy_addr, PHY_IDENTIFIER_1_REG as u32, &mut phy_id);
            if phy_id == PHY_XILINX_PCS_PMA_ID1 as u16 {
                self.phy_read(phy_addr, PHY_IDENTIFIER_2_REG as u32, &mut phy_id);
                if phy_id == PHY_XILINX_PCS_PMA_ID2 as u16 {
                    // trace!("XAxiEthernet detect_phy: PHY detected at address {}.", phy_addr);
                    self.phy_addr = phy_addr;
                    return;
                }
            }
        }
    }

    pub fn get_phy_speed_ksz9031(&self) -> usize {
        let mut control = 0u16;
        let mut status = 0u16;
        let mut partner_capabilities = 0u16;
        // trace!("Start PHY autonegotiation");
        self.phy_write(self.phy_addr, IEEE_PAGE_ADDRESS_REGISTER as _, 2);
        self.phy_read(self.phy_addr, IEEE_CONTROL_REG_MAC as _, &mut control);
        control &= !0x10;
        self.phy_write(self.phy_addr, IEEE_CONTROL_REG_MAC as u32, control);
        self.phy_write(self.phy_addr, IEEE_PAGE_ADDRESS_REGISTER as _, 0);
        self.phy_read(
            self.phy_addr,
            IEEE_AUTONEGO_ADVERTISE_REG as _,
            &mut control,
        );
        control |= IEEE_ASYMMETRIC_PAUSE_MASK as u16;
        control |= IEEE_PAUSE_MASK as u16;
        control |= ADVERTISE_100 as u16;
        control |= ADVERTISE_10 as u16;
        self.phy_write(self.phy_addr, IEEE_AUTONEGO_ADVERTISE_REG as _, control);
        self.phy_read(
            self.phy_addr,
            IEEE_1000_ADVERTISE_REG_OFFSET as _,
            &mut control,
        );
        control |= ADVERTISE_1000 as u16;
        self.phy_write(self.phy_addr, IEEE_1000_ADVERTISE_REG_OFFSET as _, control);
        self.phy_write(self.phy_addr, IEEE_PAGE_ADDRESS_REGISTER as _, control);
        self.phy_read(
            self.phy_addr,
            IEEE_COPPER_SPECIFIC_CONTROL_REG as _,
            &mut control,
        );
        control |= 7 << 12; /* max number of gigabit attempts */
        control |= 1 << 11; /* enable downshift */
        self.phy_write(
            self.phy_addr,
            IEEE_COPPER_SPECIFIC_CONTROL_REG as _,
            control,
        );
        self.phy_read(self.phy_addr, IEEE_CONTROL_REG_OFFSET as _, &mut control);
        control |= IEEE_CTRL_AUTONEGOTIATE_ENABLE as u16;
        control |= IEEE_STAT_AUTONEGOTIATE_RESTART as u16;
        self.phy_write(self.phy_addr, IEEE_CONTROL_REG_OFFSET as _, control);
        self.phy_read(self.phy_addr, IEEE_CONTROL_REG_OFFSET as _, &mut control);
        control |= IEEE_CTRL_RESET_MASK as u16;
        self.phy_write(self.phy_addr, IEEE_CONTROL_REG_OFFSET as _, control);
        loop {
            self.phy_read(self.phy_addr, IEEE_CONTROL_REG_OFFSET as _, &mut control);
            if (control & IEEE_CTRL_RESET_MASK as u16) == 0 {
                break;
            }
        }
        // trace!("Waiting for PHY to complete autonegotiation.");
        self.phy_read(self.phy_addr, IEEE_STATUS_REG_OFFSET as _, &mut status);
        while (status & IEEE_STAT_AUTONEGOTIATE_COMPLETE as u16) == 0 {
            self.phy_read(self.phy_addr, IEEE_STATUS_REG_OFFSET as _, &mut status);
        }
        // trace!("autonegotiation complete");
        self.phy_read(self.phy_addr, 0x1f, &mut partner_capabilities);
        if (partner_capabilities & 0x40) == 0x40 {
            /* 1000Mbps */
            return 1000;
        } else if (partner_capabilities & 0x20) == 0x20 {
            /* 100Mbps */
            return 100;
        } else if (partner_capabilities & 0x10) == 0x10 {
            /* 10Mbps */
            return 10;
        } else {
            return 0;
        }
    }
}

/// local_loopback
impl AxiEthernet {
    pub fn enter_local_loopback(&self) -> isize {
        // trace!("enter local loopback");
        let speed = self.get_operating_speed();
        let phy_type = self.get_phy_interface();
        let mut phy_model = 0u16;
        self.phy_read(self.phy_addr, PHY_R3_PHY_IDENT_REG as _, &mut phy_model);
        phy_model &= PHY_MODEL_NUM_MASK as u16;
        let mut phy_reg0 = 0u16;
        let mut phy_reg20 = 0u16;
        let mut phy_reg21 = 0u16;
        match speed {
            XAE_SPEED_10_MBPS => {
                phy_reg0 |= PHY_R0_DFT_SPD_10 as u16;
                phy_reg20 |= PHY_R20_DFT_SPD_10 as u16;
                phy_reg21 |= PHY_REG21_10 as u16;
            }
            XAE_SPEED_100_MBPS => {
                phy_reg0 |= PHY_R0_DFT_SPD_100 as u16;
                phy_reg20 |= PHY_R20_DFT_SPD_100 as u16;
                phy_reg21 |= PHY_REG21_100 as u16;
            }
            XAE_SPEED_1000_MBPS => {
                phy_reg0 |= PHY_R0_DFT_SPD_1000 as u16;
                phy_reg20 |= PHY_R20_DFT_SPD_1000 as u16;
                phy_reg21 |= PHY_REG21_1000 as u16;
            }
            XAE_SPEED_2500_MBPS => {
                phy_reg0 |= PHY_R0_DFT_SPD_2500 as u16;
                phy_reg20 |= PHY_R20_DFT_SPD_1000 as u16;
                phy_reg21 |= PHY_REG21_1000 as u16;
            }
            _ => {
                error!("Intg_LinkSpeed not 10, 100, or 1000 mbps");
                return -1;
            }
        };
        // RGMII mode Phy specific registers initialization
        if phy_type == PhyType::RGMII2_0 || phy_type == PhyType::RGMII1_3 {
            if phy_model == MARVEL_PHY_88E1111_MODEL as u16 {
                phy_reg20 |= PHY_R20_RX_DLY as u16;
                // Adding Rx delay. Configuring loopback speed.
                self.phy_write(self.phy_addr, PHY_R20_EXTND_CTRL_REG as u32, phy_reg20);
            } else if phy_model == MARVEL_PHY_88E1116R_MODEL as u16 {
                // Switching to PAGE2
                self.phy_write(self.phy_addr, PHY_R22_PAGE_ADDR_REG as u32, 2);
                // Adding Tx and Rx delay. Configuring loopback speed.
                self.phy_write(self.phy_addr, PHY_PG2_R21_CTRL_REG as u32, phy_reg21);
                // Switching to PAGE0
                self.phy_write(self.phy_addr, PHY_R22_PAGE_ADDR_REG as u32, 0);
            }
            phy_reg0 &= !(PHY_R0_ANEG_ENABLE as u16);
        }
        // Configure interface modes
        if phy_model == MARVEL_PHY_88E1111_MODEL as u16 {
            if phy_type == PhyType::RGMII2_0 || phy_type == PhyType::RGMII1_3 {
                self.phy_write(
                    self.phy_addr,
                    PHY_R27_EXTND_STS_REG as u32,
                    PHY_R27_MAC_CONFIG_RGMII as u16,
                );
            } else if phy_type == PhyType::SGMII {
                self.phy_write(
                    self.phy_addr,
                    PHY_R27_EXTND_STS_REG as u32,
                    PHY_R27_MAC_CONFIG_SGMII as u16,
                );
            } else if phy_type == PhyType::GMII || phy_type == PhyType::MII {
                self.phy_write(
                    self.phy_addr,
                    PHY_R27_EXTND_STS_REG as u32,
                    PHY_R27_MAC_CONFIG_GMII as u16,
                );
            }
        }
        // Set the speed and put the PHY in reset, then put the PHY in loopback
        self.phy_write(
            self.phy_addr,
            PHY_R0_CTRL_REG as u32,
            phy_reg0 | (PHY_R0_RESET as u16),
        );
        let mut time = AXIETHERNET_PHY_DELAY_SEC * 1000;
        while time > 0 {
            time -= 1;
        }
        self.phy_read(self.phy_addr, PHY_R0_CTRL_REG as u32, &mut phy_reg0);
        // enable loopback
        self.phy_write(
            self.phy_addr,
            PHY_R0_CTRL_REG as u32,
            phy_reg0 | (PHY_R0_LOOPBACK as u16),
        );
        if phy_model == TI_PHY_MODEL as u16 && phy_type == PhyType::SGMII {
            self.phy_read(self.phy_addr, PHY_R0_CTRL_REG as u32, &mut phy_reg0);
            phy_reg0 &= !(PHY_R0_ANEG_ENABLE as u16);
            self.phy_write(self.phy_addr, PHY_R0_CTRL_REG as u32, phy_reg0);
            self.config_ti_phy();
        }
        time = 1 * 1000;
        while time > 0 {
            time -= 1;
        }
        return 0;
    }

    pub fn config_ti_phy(&self) -> isize {
        let mut phy_reg14 = 0u16;
        // Enable SGMII Clock
        self.phy_write(self.phy_addr, TI_PHY_CR as u32, TI_PHY_CR_DEVAD_EN as u16);
        self.phy_write(self.phy_addr, TI_PHY_ADDDR as u32, TI_PHY_SGMIITYPE as u16);
        self.phy_write(
            self.phy_addr,
            TI_PHY_CR as u32,
            (TI_PHY_CR_DEVAD_EN | TI_PHY_CR_DEVAD_DATAEN) as u16,
        );
        self.phy_write(
            self.phy_addr,
            TI_PHY_ADDDR as u32,
            TI_PHY_SGMIICLK_EN as u16,
        );
        // Enable SGMII
        self.phy_write(
            self.phy_addr,
            TI_PHY_PHYCTRL as u32,
            TI_PHY_CR_SGMII_EN as u16,
        );
        self.phy_read(self.phy_addr, TI_PHY_CFGR2 as u32, &mut phy_reg14);
        self.phy_write(
            self.phy_addr,
            TI_PHY_CFGR2 as u32,
            phy_reg14 & (!(TI_PHY_CFGR2_SGMII_AUTONEG_EN as u16)),
        );
        self.phy_read(self.phy_addr, TI_PHY_CFGR2 as u32, &mut phy_reg14);
        return 0;
    }
}
