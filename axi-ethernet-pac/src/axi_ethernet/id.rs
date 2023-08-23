#[doc = "Register `id` reader"]
pub struct R(crate::R<ID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PL` reader - "]
pub type PL_R = crate::FieldReader;
#[doc = "Field `MIR` reader - "]
pub type MIR_R = crate::FieldReader;
#[doc = "Field `MAR` reader - "]
pub type MAR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pl(&self) -> PL_R {
        PL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn mir(&self) -> MIR_R {
        MIR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn mar(&self) -> MAR_R {
        MAR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Identification\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id](index.html) module"]
pub struct ID_SPEC;
impl crate::RegisterSpec for ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [id::R](R) reader structure"]
impl crate::Readable for ID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets id to value 0"]
impl crate::Resettable for ID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
