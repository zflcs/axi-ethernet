#[doc = "Register `mdiomrd` reader"]
pub struct R(crate::R<MDIOMRD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDIOMRD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDIOMRD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDIOMRD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `mdiomrd` writer"]
pub struct W(crate::W<MDIOMRD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDIOMRD_SPEC>;
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
impl From<crate::W<MDIOMRD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDIOMRD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - "]
pub type DATA_R = crate::FieldReader<u16>;
#[doc = "Field `DATA` writer - "]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, MDIOMRD_SPEC, 16, O, u16>;
#[doc = "Field `READY` reader - "]
pub type READY_R = crate::BitReader;
#[doc = "Field `READY` writer - "]
pub type READY_W<'a, const O: u8> = crate::BitWriter<'a, MDIOMRD_SPEC, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn ready(&mut self) -> READY_W<16> {
        READY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MDIO Read Data\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdiomrd](index.html) module"]
pub struct MDIOMRD_SPEC;
impl crate::RegisterSpec for MDIOMRD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdiomrd::R](R) reader structure"]
impl crate::Readable for MDIOMRD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mdiomrd::W](W) writer structure"]
impl crate::Writable for MDIOMRD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets mdiomrd to value 0"]
impl crate::Resettable for MDIOMRD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
