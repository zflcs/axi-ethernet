#[doc = "Register `rxfc` reader"]
pub type R = crate::R<RXFC_SPEC>;
#[doc = "Register `rxfc` writer"]
pub type W = crate::W<RXFC_SPEC>;
#[doc = "Field `RMFL` reader - "]
pub type RMFL_R = crate::FieldReader<u16>;
#[doc = "Field `RMFL` writer - "]
pub type RMFL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 15, O, u16>;
#[doc = "Field `RMFE` reader - "]
pub type RMFE_R = crate::BitReader<RMFE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMFE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RMFE_A> for bool {
    #[inline(always)]
    fn from(variant: RMFE_A) -> Self {
        variant as u8 != 0
    }
}
impl RMFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RMFE_A {
        match self.bits {
            false => RMFE_A::DISABLE,
            true => RMFE_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RMFE_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RMFE_A::ENABLE
    }
}
#[doc = "Field `RMFE` writer - "]
pub type RMFE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RMFE_A>;
impl<'a, REG, const O: u8> RMFE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RMFE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RMFE_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:14"]
    #[inline(always)]
    pub fn rmfl(&self) -> RMFL_R {
        RMFL_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rmfe(&self) -> RMFE_R {
        RMFE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:14"]
    #[inline(always)]
    #[must_use]
    pub fn rmfl(&mut self) -> RMFL_W<RXFC_SPEC, 0> {
        RMFL_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn rmfe(&mut self) -> RMFE_W<RXFC_SPEC, 16> {
        RMFE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RX Max Frame Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxfc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxfc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXFC_SPEC;
impl crate::RegisterSpec for RXFC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxfc::R`](R) reader structure"]
impl crate::Readable for RXFC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxfc::W`](W) writer structure"]
impl crate::Writable for RXFC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rxfc to value 0"]
impl crate::Resettable for RXFC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
