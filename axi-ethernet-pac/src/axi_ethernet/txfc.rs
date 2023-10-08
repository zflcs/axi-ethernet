#[doc = "Register `txfc` reader"]
pub type R = crate::R<TXFC_SPEC>;
#[doc = "Register `txfc` writer"]
pub type W = crate::W<TXFC_SPEC>;
#[doc = "Field `TMFL` reader - "]
pub type TMFL_R = crate::FieldReader<u16>;
#[doc = "Field `TMFL` writer - "]
pub type TMFL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 15, O, u16>;
#[doc = "Field `TMFE` reader - "]
pub type TMFE_R = crate::BitReader<TMFE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TMFE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TMFE_A> for bool {
    #[inline(always)]
    fn from(variant: TMFE_A) -> Self {
        variant as u8 != 0
    }
}
impl TMFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMFE_A {
        match self.bits {
            false => TMFE_A::DISABLE,
            true => TMFE_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TMFE_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TMFE_A::ENABLE
    }
}
#[doc = "Field `TMFE` writer - "]
pub type TMFE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TMFE_A>;
impl<'a, REG, const O: u8> TMFE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TMFE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TMFE_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:14"]
    #[inline(always)]
    pub fn tmfl(&self) -> TMFL_R {
        TMFL_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tmfe(&self) -> TMFE_R {
        TMFE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:14"]
    #[inline(always)]
    #[must_use]
    pub fn tmfl(&mut self) -> TMFL_W<TXFC_SPEC, 0> {
        TMFL_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn tmfe(&mut self) -> TMFE_W<TXFC_SPEC, 16> {
        TMFE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "TX Max Frame Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txfc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txfc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXFC_SPEC;
impl crate::RegisterSpec for TXFC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txfc::R`](R) reader structure"]
impl crate::Readable for TXFC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txfc::W`](W) writer structure"]
impl crate::Writable for TXFC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets txfc to value 0"]
impl crate::Resettable for TXFC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
