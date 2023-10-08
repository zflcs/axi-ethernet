#[doc = "Register `id` reader"]
pub type R = crate::R<ID_SPEC>;
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
#[doc = "Identification\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ID_SPEC;
impl crate::RegisterSpec for ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`id::R`](R) reader structure"]
impl crate::Readable for ID_SPEC {}
#[doc = "`reset()` method sets id to value 0"]
impl crate::Resettable for ID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
