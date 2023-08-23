#[doc = "Register `rxfc` reader"]
pub struct R(crate::R<RXFC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXFC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXFC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXFC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rxfc` writer"]
pub struct W(crate::W<RXFC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXFC_SPEC>;
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
impl From<crate::W<RXFC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXFC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RMFL` reader - "]
pub type RMFL_R = crate::FieldReader<u16>;
#[doc = "Field `RMFL` writer - "]
pub type RMFL_W<'a, const O: u8> = crate::FieldWriter<'a, RXFC_SPEC, 15, O, u16>;
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
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RMFE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RMFE_A::ENABLE
    }
}
#[doc = "Field `RMFE` writer - "]
pub type RMFE_W<'a, const O: u8> = crate::BitWriter<'a, RXFC_SPEC, O, RMFE_A>;
impl<'a, const O: u8> RMFE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RMFE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
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
    pub fn rmfl(&mut self) -> RMFL_W<0> {
        RMFL_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn rmfe(&mut self) -> RMFE_W<16> {
        RMFE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RX Max Frame Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxfc](index.html) module"]
pub struct RXFC_SPEC;
impl crate::RegisterSpec for RXFC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxfc::R](R) reader structure"]
impl crate::Readable for RXFC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxfc::W](W) writer structure"]
impl crate::Writable for RXFC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rxfc to value 0"]
impl crate::Resettable for RXFC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
