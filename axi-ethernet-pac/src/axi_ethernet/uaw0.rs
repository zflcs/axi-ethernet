#[doc = "Register `uaw0` reader"]
pub struct R(crate::R<UAW0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UAW0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UAW0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UAW0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `uaw0` writer"]
pub struct W(crate::W<UAW0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UAW0_SPEC>;
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
impl From<crate::W<UAW0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UAW0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `address` reader - "]
pub type ADDRESS_R = crate::FieldReader<u32>;
#[doc = "Field `address` writer - "]
pub type ADDRESS_W<'a, const O: u8> = crate::FieldWriter<'a, UAW0_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn address(&self) -> ADDRESS_R {
        ADDRESS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn address(&mut self) -> ADDRESS_W<0> {
        ADDRESS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Unicast Address Word 0 (UAW0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uaw0](index.html) module"]
pub struct UAW0_SPEC;
impl crate::RegisterSpec for UAW0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uaw0::R](R) reader structure"]
impl crate::Readable for UAW0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uaw0::W](W) writer structure"]
impl crate::Writable for UAW0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets uaw0 to value 0"]
impl crate::Resettable for UAW0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
