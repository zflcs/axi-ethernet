#[doc = "Register `ttac` reader"]
pub type R = crate::R<TTAC_SPEC>;
#[doc = "Register `ttac` writer"]
pub type W = crate::W<TTAC_SPEC>;
#[doc = "Field `TXAV` reader - "]
pub type TXAV_R = crate::FieldReader<u16>;
#[doc = "Field `TXAV` writer - "]
pub type TXAV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `TTCE` reader - "]
pub type TTCE_R = crate::BitReader<TTCE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TTCE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TTCE_A> for bool {
    #[inline(always)]
    fn from(variant: TTCE_A) -> Self {
        variant as u8 != 0
    }
}
impl TTCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TTCE_A {
        match self.bits {
            false => TTCE_A::DISABLE,
            true => TTCE_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TTCE_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TTCE_A::ENABLE
    }
}
#[doc = "Field `TTCE` writer - "]
pub type TTCE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TTCE_A>;
impl<'a, REG, const O: u8> TTCE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TTCE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TTCE_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn txav(&self) -> TXAV_R {
        TXAV_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ttce(&self) -> TTCE_R {
        TTCE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn txav(&mut self) -> TXAV_W<TTAC_SPEC, 0> {
        TXAV_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn ttce(&mut self) -> TTCE_W<TTAC_SPEC, 16> {
        TTCE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "TX Timestamp Adjust Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ttac::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ttac::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TTAC_SPEC;
impl crate::RegisterSpec for TTAC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ttac::R`](R) reader structure"]
impl crate::Readable for TTAC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ttac::W`](W) writer structure"]
impl crate::Writable for TTAC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ttac to value 0"]
impl crate::Resettable for TTAC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
