#[doc = "Register `ppst` reader"]
pub struct R(crate::R<PPST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PPST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PPST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PPST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LinkStatus` reader - "]
pub type LINK_STATUS_R = crate::BitReader<LINK_STATUS_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LINK_STATUS_A {
    #[doc = "0: `0`"]
    RESET = 0,
}
impl From<LINK_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: LINK_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
impl LINK_STATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LINK_STATUS_A> {
        match self.bits {
            false => Some(LINK_STATUS_A::RESET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == LINK_STATUS_A::RESET
    }
}
#[doc = "Field `LinkSync` reader - "]
pub type LINK_SYNC_R = crate::BitReader<LINK_SYNC_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LINK_SYNC_A {
    #[doc = "0: `0`"]
    RESET = 0,
}
impl From<LINK_SYNC_A> for bool {
    #[inline(always)]
    fn from(variant: LINK_SYNC_A) -> Self {
        variant as u8 != 0
    }
}
impl LINK_SYNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LINK_SYNC_A> {
        match self.bits {
            false => Some(LINK_SYNC_A::RESET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == LINK_SYNC_A::RESET
    }
}
#[doc = "Field `RUDI_C` reader - "]
pub type RUDI_C_R = crate::BitReader<RUDI_C_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RUDI_C_A {
    #[doc = "0: `0`"]
    RESET = 0,
}
impl From<RUDI_C_A> for bool {
    #[inline(always)]
    fn from(variant: RUDI_C_A) -> Self {
        variant as u8 != 0
    }
}
impl RUDI_C_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RUDI_C_A> {
        match self.bits {
            false => Some(RUDI_C_A::RESET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == RUDI_C_A::RESET
    }
}
#[doc = "Field `RUDI_I` reader - "]
pub type RUDI_I_R = crate::BitReader<RUDI_I_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RUDI_I_A {
    #[doc = "0: `0`"]
    RESET = 0,
}
impl From<RUDI_I_A> for bool {
    #[inline(always)]
    fn from(variant: RUDI_I_A) -> Self {
        variant as u8 != 0
    }
}
impl RUDI_I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RUDI_I_A> {
        match self.bits {
            false => Some(RUDI_I_A::RESET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == RUDI_I_A::RESET
    }
}
#[doc = "Field `RUDI_INVLD` reader - "]
pub type RUDI_INVLD_R = crate::BitReader<RUDI_INVLD_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RUDI_INVLD_A {
    #[doc = "0: `0`"]
    RESET = 0,
}
impl From<RUDI_INVLD_A> for bool {
    #[inline(always)]
    fn from(variant: RUDI_INVLD_A) -> Self {
        variant as u8 != 0
    }
}
impl RUDI_INVLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RUDI_INVLD_A> {
        match self.bits {
            false => Some(RUDI_INVLD_A::RESET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == RUDI_INVLD_A::RESET
    }
}
#[doc = "Field `RXDISPERR` reader - "]
pub type RXDISPERR_R = crate::BitReader<RXDISPERR_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXDISPERR_A {
    #[doc = "0: `0`"]
    RESET = 0,
}
impl From<RXDISPERR_A> for bool {
    #[inline(always)]
    fn from(variant: RXDISPERR_A) -> Self {
        variant as u8 != 0
    }
}
impl RXDISPERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RXDISPERR_A> {
        match self.bits {
            false => Some(RXDISPERR_A::RESET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == RXDISPERR_A::RESET
    }
}
#[doc = "Field `RXNOTINTABLE` reader - "]
pub type RXNOTINTABLE_R = crate::BitReader<RXNOTINTABLE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXNOTINTABLE_A {
    #[doc = "0: `0`"]
    RESET = 0,
}
impl From<RXNOTINTABLE_A> for bool {
    #[inline(always)]
    fn from(variant: RXNOTINTABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl RXNOTINTABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RXNOTINTABLE_A> {
        match self.bits {
            false => Some(RXNOTINTABLE_A::RESET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == RXNOTINTABLE_A::RESET
    }
}
#[doc = "Field `PhyLinkStatus` reader - "]
pub type PHY_LINK_STATUS_R = crate::BitReader<PHY_LINK_STATUS_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PHY_LINK_STATUS_A {
    #[doc = "0: `0`"]
    RESET = 0,
}
impl From<PHY_LINK_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: PHY_LINK_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
impl PHY_LINK_STATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PHY_LINK_STATUS_A> {
        match self.bits {
            false => Some(PHY_LINK_STATUS_A::RESET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == PHY_LINK_STATUS_A::RESET
    }
}
#[doc = "Field `RmtFltEnc` reader - "]
pub type RMT_FLT_ENC_R = crate::FieldReader<RMT_FLT_ENC_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RMT_FLT_ENC_A {
    #[doc = "0: `0`"]
    RESET = 0,
}
impl From<RMT_FLT_ENC_A> for u8 {
    #[inline(always)]
    fn from(variant: RMT_FLT_ENC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RMT_FLT_ENC_A {
    type Ux = u8;
}
impl RMT_FLT_ENC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RMT_FLT_ENC_A> {
        match self.bits {
            0 => Some(RMT_FLT_ENC_A::RESET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == RMT_FLT_ENC_A::RESET
    }
}
#[doc = "Field `speed` reader - "]
pub type SPEED_R = crate::FieldReader<SPEED_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPEED_A {
    #[doc = "2: `10`"]
    _1000M = 2,
    #[doc = "1: `1`"]
    _100M = 1,
    #[doc = "0: `0`"]
    _10M = 0,
}
impl From<SPEED_A> for u8 {
    #[inline(always)]
    fn from(variant: SPEED_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SPEED_A {
    type Ux = u8;
}
impl SPEED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SPEED_A> {
        match self.bits {
            2 => Some(SPEED_A::_1000M),
            1 => Some(SPEED_A::_100M),
            0 => Some(SPEED_A::_10M),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_1000M`"]
    #[inline(always)]
    pub fn is_1000m(&self) -> bool {
        *self == SPEED_A::_1000M
    }
    #[doc = "Checks if the value of the field is `_100M`"]
    #[inline(always)]
    pub fn is_100m(&self) -> bool {
        *self == SPEED_A::_100M
    }
    #[doc = "Checks if the value of the field is `_10M`"]
    #[inline(always)]
    pub fn is_10m(&self) -> bool {
        *self == SPEED_A::_10M
    }
}
#[doc = "Field `Duplex` reader - "]
pub type DUPLEX_R = crate::BitReader<DUPLEX_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DUPLEX_A {
    #[doc = "0: `0`"]
    HALF_DUPLEX = 0,
    #[doc = "1: `1`"]
    FULL_DUPLEX = 1,
}
impl From<DUPLEX_A> for bool {
    #[inline(always)]
    fn from(variant: DUPLEX_A) -> Self {
        variant as u8 != 0
    }
}
impl DUPLEX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DUPLEX_A {
        match self.bits {
            false => DUPLEX_A::HALF_DUPLEX,
            true => DUPLEX_A::FULL_DUPLEX,
        }
    }
    #[doc = "Checks if the value of the field is `HALF_DUPLEX`"]
    #[inline(always)]
    pub fn is_half_duplex(&self) -> bool {
        *self == DUPLEX_A::HALF_DUPLEX
    }
    #[doc = "Checks if the value of the field is `FULL_DUPLEX`"]
    #[inline(always)]
    pub fn is_full_duplex(&self) -> bool {
        *self == DUPLEX_A::FULL_DUPLEX
    }
}
#[doc = "Field `RmtFlt` reader - "]
pub type RMT_FLT_R = crate::BitReader<RMT_FLT_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMT_FLT_A {
    #[doc = "0: `0`"]
    RESET = 0,
}
impl From<RMT_FLT_A> for bool {
    #[inline(always)]
    fn from(variant: RMT_FLT_A) -> Self {
        variant as u8 != 0
    }
}
impl RMT_FLT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RMT_FLT_A> {
        match self.bits {
            false => Some(RMT_FLT_A::RESET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == RMT_FLT_A::RESET
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn link_status(&self) -> LINK_STATUS_R {
        LINK_STATUS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn link_sync(&self) -> LINK_SYNC_R {
        LINK_SYNC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rudi_c(&self) -> RUDI_C_R {
        RUDI_C_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rudi_i(&self) -> RUDI_I_R {
        RUDI_I_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rudi_invld(&self) -> RUDI_INVLD_R {
        RUDI_INVLD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rxdisperr(&self) -> RXDISPERR_R {
        RXDISPERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rxnotintable(&self) -> RXNOTINTABLE_R {
        RXNOTINTABLE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn phy_link_status(&self) -> PHY_LINK_STATUS_R {
        PHY_LINK_STATUS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn rmt_flt_enc(&self) -> RMT_FLT_ENC_R {
        RMT_FLT_ENC_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn duplex(&self) -> DUPLEX_R {
        DUPLEX_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn rmt_flt(&self) -> RMT_FLT_R {
        RMT_FLT_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[doc = "PCS PMA TEMAC Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppst](index.html) module"]
pub struct PPST_SPEC;
impl crate::RegisterSpec for PPST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ppst::R](R) reader structure"]
impl crate::Readable for PPST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ppst to value 0"]
impl crate::Resettable for PPST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
