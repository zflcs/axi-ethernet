#[doc = "Register `tpf` reader"]
pub struct R(crate::R<TPF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TPF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TPF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TPF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tpf` writer"]
pub struct W(crate::W<TPF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TPF_SPEC>;
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
impl From<crate::W<TPF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TPF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TPFV` reader - "]
pub type TPFV_R = crate::FieldReader<TPFV_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum TPFV_A {
    #[doc = "0: `0`"]
    RESET = 0,
}
impl From<TPFV_A> for u16 {
    #[inline(always)]
    fn from(variant: TPFV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TPFV_A {
    type Ux = u16;
}
impl TPFV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TPFV_A> {
        match self.bits {
            0 => Some(TPFV_A::RESET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == TPFV_A::RESET
    }
}
#[doc = "Field `TPFV` writer - "]
pub type TPFV_W<'a, const O: u8> = crate::FieldWriter<'a, TPF_SPEC, 16, O, TPFV_A>;
impl<'a, const O: u8> TPFV_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TPFV_A::RESET)
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn tpfv(&self) -> TPFV_R {
        TPFV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn tpfv(&mut self) -> TPFV_W<0> {
        TPFV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Pause Frame TEMAC\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tpf](index.html) module"]
pub struct TPF_SPEC;
impl crate::RegisterSpec for TPF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tpf::R](R) reader structure"]
impl crate::Readable for TPF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tpf::W](W) writer structure"]
impl crate::Writable for TPF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tpf to value 0"]
impl crate::Resettable for TPF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
