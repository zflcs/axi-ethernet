#[doc = "Register `af0` reader"]
pub struct R(crate::R<AF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `af0` writer"]
pub struct W(crate::W<AF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AF0_SPEC>;
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
impl From<crate::W<AF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `address47T32` reader - "]
pub type ADDRESS47T32_R = crate::FieldReader<u16>;
#[doc = "Field `address47T32` writer - "]
pub type ADDRESS47T32_W<'a, const O: u8> = crate::FieldWriter<'a, AF0_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn address47t32(&self) -> ADDRESS47T32_R {
        ADDRESS47T32_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn address47t32(&mut self) -> ADDRESS47T32_W<0> {
        ADDRESS47T32_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Address Filter 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [af0](index.html) module"]
pub struct AF0_SPEC;
impl crate::RegisterSpec for AF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [af0::R](R) reader structure"]
impl crate::Readable for AF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [af0::W](W) writer structure"]
impl crate::Writable for AF0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets af0 to value 0"]
impl crate::Resettable for AF0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
