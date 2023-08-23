#[doc = "Register `mdiomie` reader"]
pub struct R(crate::R<MDIOMIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDIOMIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDIOMIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDIOMIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `mdiomie` writer"]
pub struct W(crate::W<MDIOMIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDIOMIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<MDIOMIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDIOMIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MDIO_COMPLETE` reader - "]
pub type MDIO_COMPLETE_R = crate::BitReader<MDIO_COMPLETE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MDIO_COMPLETE_A {
    #[doc = "0: `0`"]
    DISABALE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
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
            false => MDIO_COMPLETE_A::DISABALE,
            true => MDIO_COMPLETE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABALE`"]
    #[inline(always)]
    pub fn is_disabale(&self) -> bool {
        *self == MDIO_COMPLETE_A::DISABALE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MDIO_COMPLETE_A::ENABLE
    }
}
#[doc = "Field `MDIO_COMPLETE` writer - "]
pub type MDIO_COMPLETE_W<'a, const O: u8> = crate::BitWriter<'a, MDIOMIE_SPEC, O, MDIO_COMPLETE_A>;
impl<'a, const O: u8> MDIO_COMPLETE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabale(self) -> &'a mut W {
        self.variant(MDIO_COMPLETE_A::DISABALE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MDIO_COMPLETE_A::ENABLE)
    }
}
#[doc = "Field `PTP_TX` reader - "]
pub type PTP_TX_R = crate::BitReader<PTP_TX_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTP_TX_A {
    #[doc = "0: `0`"]
    DISABALE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
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
            false => PTP_TX_A::DISABALE,
            true => PTP_TX_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABALE`"]
    #[inline(always)]
    pub fn is_disabale(&self) -> bool {
        *self == PTP_TX_A::DISABALE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PTP_TX_A::ENABLE
    }
}
#[doc = "Field `PTP_TX` writer - "]
pub type PTP_TX_W<'a, const O: u8> = crate::BitWriter<'a, MDIOMIE_SPEC, O, PTP_TX_A>;
impl<'a, const O: u8> PTP_TX_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabale(self) -> &'a mut W {
        self.variant(PTP_TX_A::DISABALE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PTP_TX_A::ENABLE)
    }
}
#[doc = "Field `PTP_RX` reader - "]
pub type PTP_RX_R = crate::BitReader<PTP_RX_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTP_RX_A {
    #[doc = "0: `0`"]
    DISABALE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
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
            false => PTP_RX_A::DISABALE,
            true => PTP_RX_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABALE`"]
    #[inline(always)]
    pub fn is_disabale(&self) -> bool {
        *self == PTP_RX_A::DISABALE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PTP_RX_A::ENABLE
    }
}
#[doc = "Field `PTP_RX` writer - "]
pub type PTP_RX_W<'a, const O: u8> = crate::BitWriter<'a, MDIOMIE_SPEC, O, PTP_RX_A>;
impl<'a, const O: u8> PTP_RX_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabale(self) -> &'a mut W {
        self.variant(PTP_RX_A::DISABALE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PTP_RX_A::ENABLE)
    }
}
#[doc = "Field `PTP_TIMER` reader - "]
pub type PTP_TIMER_R = crate::BitReader<PTP_TIMER_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTP_TIMER_A {
    #[doc = "0: `0`"]
    DISABALE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
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
            false => PTP_TIMER_A::DISABALE,
            true => PTP_TIMER_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABALE`"]
    #[inline(always)]
    pub fn is_disabale(&self) -> bool {
        *self == PTP_TIMER_A::DISABALE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PTP_TIMER_A::ENABLE
    }
}
#[doc = "Field `PTP_TIMER` writer - "]
pub type PTP_TIMER_W<'a, const O: u8> = crate::BitWriter<'a, MDIOMIE_SPEC, O, PTP_TIMER_A>;
impl<'a, const O: u8> PTP_TIMER_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabale(self) -> &'a mut W {
        self.variant(PTP_TIMER_A::DISABALE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PTP_TIMER_A::ENABLE)
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
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn mdio_complete(&mut self) -> MDIO_COMPLETE_W<0> {
        MDIO_COMPLETE_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn ptp_tx(&mut self) -> PTP_TX_W<1> {
        PTP_TX_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn ptp_rx(&mut self) -> PTP_RX_W<2> {
        PTP_RX_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn ptp_timer(&mut self) -> PTP_TIMER_W<3> {
        PTP_TIMER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MII Management Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdiomie](index.html) module"]
pub struct MDIOMIE_SPEC;
impl crate::RegisterSpec for MDIOMIE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdiomie::R](R) reader structure"]
impl crate::Readable for MDIOMIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mdiomie::W](W) writer structure"]
impl crate::Writable for MDIOMIE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets mdiomie to value 0"]
impl crate::Resettable for MDIOMIE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
