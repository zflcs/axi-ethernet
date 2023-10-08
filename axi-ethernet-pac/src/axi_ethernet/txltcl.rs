#[doc = "Register `txltcl` reader"]
pub type R = crate::R<TXLTCL_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXLTCL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Frames transmitted with late Collisions, LSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txltcl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXLTCL_SPEC;
impl crate::RegisterSpec for TXLTCL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txltcl::R`](R) reader structure"]
impl crate::Readable for TXLTCL_SPEC {}
#[doc = "`reset()` method sets txltcl to value 0"]
impl crate::Resettable for TXLTCL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
