#[doc = "Register `txmcl` reader"]
pub type R = crate::R<TXMCL_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXMCL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Multiple Collision Frames Transmitted OK, LSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txmcl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXMCL_SPEC;
impl crate::RegisterSpec for TXMCL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txmcl::R`](R) reader structure"]
impl crate::Readable for TXMCL_SPEC {}
#[doc = "`reset()` method sets txmcl to value 0"]
impl crate::Resettable for TXMCL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
