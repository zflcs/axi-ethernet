#[doc = "Register `mdiomip` reader"]
pub struct R(crate::R<MDIOMIP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDIOMIP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDIOMIP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDIOMIP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MDIO_COMPLETE` reader - "]
pub type MDIO_COMPLETE_R = crate::BitReader<MDIO_COMPLETE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MDIO_COMPLETE_A {
    #[doc = "0: `0`"]
    NO_INTR = 0,
    #[doc = "1: `1`"]
    HAS_INTR = 1,
}
impl From<MDIO_COMPLETE_A> for bool {
    #[inline(always)]
    fn from(variant: MDIO_COMPLETE_A) -> Self {
        variant as u8 != 0
    }
}
impl MDIO_COMPLETE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MDIO_COMPLETE_A {
        match self.bits {
            false => MDIO_COMPLETE_A::NO_INTR,
            true => MDIO_COMPLETE_A::HAS_INTR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTR`"]
    #[inline(always)]
    pub fn is_no_intr(&self) -> bool {
        *self == MDIO_COMPLETE_A::NO_INTR
    }
    #[doc = "Checks if the value of the field is `HAS_INTR`"]
    #[inline(always)]
    pub fn is_has_intr(&self) -> bool {
        *self == MDIO_COMPLETE_A::HAS_INTR
    }
}
#[doc = "Field `PTP_TX` reader - "]
pub type PTP_TX_R = crate::BitReader<PTP_TX_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTP_TX_A {
    #[doc = "0: `0`"]
    NO_INTR = 0,
    #[doc = "1: `1`"]
    HAS_INTR = 1,
}
impl From<PTP_TX_A> for bool {
    #[inline(always)]
    fn from(variant: PTP_TX_A) -> Self {
        variant as u8 != 0
    }
}
impl PTP_TX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTP_TX_A {
        match self.bits {
            false => PTP_TX_A::NO_INTR,
            true => PTP_TX_A::HAS_INTR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTR`"]
    #[inline(always)]
    pub fn is_no_intr(&self) -> bool {
        *self == PTP_TX_A::NO_INTR
    }
    #[doc = "Checks if the value of the field is `HAS_INTR`"]
    #[inline(always)]
    pub fn is_has_intr(&self) -> bool {
        *self == PTP_TX_A::HAS_INTR
    }
}
#[doc = "Field `PTP_RX` reader - "]
pub type PTP_RX_R = crate::BitReader<PTP_RX_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTP_RX_A {
    #[doc = "0: `0`"]
    NO_INTR = 0,
    #[doc = "1: `1`"]
    HAS_INTR = 1,
}
impl From<PTP_RX_A> for bool {
    #[inline(always)]
    fn from(variant: PTP_RX_A) -> Self {
        variant as u8 != 0
    }
}
impl PTP_RX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTP_RX_A {
        match self.bits {
            false => PTP_RX_A::NO_INTR,
            true => PTP_RX_A::HAS_INTR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTR`"]
    #[inline(always)]
    pub fn is_no_intr(&self) -> bool {
        *self == PTP_RX_A::NO_INTR
    }
    #[doc = "Checks if the value of the field is `HAS_INTR`"]
    #[inline(always)]
    pub fn is_has_intr(&self) -> bool {
        *self == PTP_RX_A::HAS_INTR
    }
}
#[doc = "Field `PTP_TIMER` reader - "]
pub type PTP_TIMER_R = crate::BitReader<PTP_TIMER_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTP_TIMER_A {
    #[doc = "0: `0`"]
    NO_INTR = 0,
    #[doc = "1: `1`"]
    HAS_INTR = 1,
}
impl From<PTP_TIMER_A> for bool {
    #[inline(always)]
    fn from(variant: PTP_TIMER_A) -> Self {
        variant as u8 != 0
    }
}
impl PTP_TIMER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTP_TIMER_A {
        match self.bits {
            false => PTP_TIMER_A::NO_INTR,
            true => PTP_TIMER_A::HAS_INTR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTR`"]
    #[inline(always)]
    pub fn is_no_intr(&self) -> bool {
        *self == PTP_TIMER_A::NO_INTR
    }
    #[doc = "Checks if the value of the field is `HAS_INTR`"]
    #[inline(always)]
    pub fn is_has_intr(&self) -> bool {
        *self == PTP_TIMER_A::HAS_INTR
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn mdio_complete(&self) -> MDIO_COMPLETE_R {
        MDIO_COMPLETE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ptp_tx(&self) -> PTP_TX_R {
        PTP_TX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ptp_rx(&self) -> PTP_RX_R {
        PTP_RX_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ptp_timer(&self) -> PTP_TIMER_R {
        PTP_TIMER_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "MII Management Interrupt Pending\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdiomip](index.html) module"]
pub struct MDIOMIP_SPEC;
impl crate::RegisterSpec for MDIOMIP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdiomip::R](R) reader structure"]
impl crate::Readable for MDIOMIP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets mdiomip to value 0"]
impl crate::Resettable for MDIOMIP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
