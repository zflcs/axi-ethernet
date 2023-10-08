#[doc = "Register `mdiomic` reader"]
pub type R = crate::R<MDIOMIC_SPEC>;
#[doc = "Register `mdiomic` writer"]
pub type W = crate::W<MDIOMIC_SPEC>;
#[doc = "Field `MDIO_COMPLETE` reader - "]
pub type MDIO_COMPLETE_R = crate::BitReader<MDIO_COMPLETE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MDIO_COMPLETE_A {
    #[doc = "1: `1`"]
    CLEAR = 1,
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
    pub fn variant(&self) -> Option<MDIO_COMPLETE_A> {
        match self.bits {
            true => Some(MDIO_COMPLETE_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == MDIO_COMPLETE_A::CLEAR
    }
}
#[doc = "Field `MDIO_COMPLETE` writer - "]
pub type MDIO_COMPLETE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MDIO_COMPLETE_A>;
impl<'a, REG, const O: u8> MDIO_COMPLETE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(MDIO_COMPLETE_A::CLEAR)
    }
}
#[doc = "Field `PTP_TX` reader - "]
pub type PTP_TX_R = crate::BitReader<PTP_TX_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTP_TX_A {
    #[doc = "1: `1`"]
    CLEAR = 1,
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
    pub fn variant(&self) -> Option<PTP_TX_A> {
        match self.bits {
            true => Some(PTP_TX_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == PTP_TX_A::CLEAR
    }
}
#[doc = "Field `PTP_TX` writer - "]
pub type PTP_TX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PTP_TX_A>;
impl<'a, REG, const O: u8> PTP_TX_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PTP_TX_A::CLEAR)
    }
}
#[doc = "Field `PTP_RX` reader - "]
pub type PTP_RX_R = crate::BitReader<PTP_RX_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTP_RX_A {
    #[doc = "1: `1`"]
    CLEAR = 1,
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
    pub fn variant(&self) -> Option<PTP_RX_A> {
        match self.bits {
            true => Some(PTP_RX_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == PTP_RX_A::CLEAR
    }
}
#[doc = "Field `PTP_RX` writer - "]
pub type PTP_RX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PTP_RX_A>;
impl<'a, REG, const O: u8> PTP_RX_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PTP_RX_A::CLEAR)
    }
}
#[doc = "Field `PTP_TIMER` reader - "]
pub type PTP_TIMER_R = crate::BitReader<PTP_TIMER_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTP_TIMER_A {
    #[doc = "1: `1`"]
    CLEAR = 1,
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
    pub fn variant(&self) -> Option<PTP_TIMER_A> {
        match self.bits {
            true => Some(PTP_TIMER_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == PTP_TIMER_A::CLEAR
    }
}
#[doc = "Field `PTP_TIMER` writer - "]
pub type PTP_TIMER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PTP_TIMER_A>;
impl<'a, REG, const O: u8> PTP_TIMER_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PTP_TIMER_A::CLEAR)
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
    pub fn mdio_complete(&mut self) -> MDIO_COMPLETE_W<MDIOMIC_SPEC, 0> {
        MDIO_COMPLETE_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn ptp_tx(&mut self) -> PTP_TX_W<MDIOMIC_SPEC, 1> {
        PTP_TX_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn ptp_rx(&mut self) -> PTP_RX_W<MDIOMIC_SPEC, 2> {
        PTP_RX_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn ptp_timer(&mut self) -> PTP_TIMER_W<MDIOMIC_SPEC, 3> {
        PTP_TIMER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MII Management Interrupt Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdiomic::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdiomic::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDIOMIC_SPEC;
impl crate::RegisterSpec for MDIOMIC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdiomic::R`](R) reader structure"]
impl crate::Readable for MDIOMIC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mdiomic::W`](W) writer structure"]
impl crate::Writable for MDIOMIC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets mdiomic to value 0"]
impl crate::Resettable for MDIOMIC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
