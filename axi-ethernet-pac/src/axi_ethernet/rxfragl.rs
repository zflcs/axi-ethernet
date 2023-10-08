#[doc = "Register `rxfragl` reader"]
pub type R = crate::R<RXFRAGL_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXFRAGL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of undersize and bad FCS frames received, LSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxfragl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXFRAGL_SPEC;
impl crate::RegisterSpec for RXFRAGL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxfragl::R`](R) reader structure"]
impl crate::Readable for RXFRAGL_SPEC {}
#[doc = "`reset()` method sets rxfragl to value 0"]
impl crate::Resettable for RXFRAGL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
