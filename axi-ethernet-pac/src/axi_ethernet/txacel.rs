#[doc = "Register `txacel` reader"]
pub type R = crate::R<TXACEL_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXACEL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Frames aborted with excessive Collisions, LSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txacel::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXACEL_SPEC;
impl crate::RegisterSpec for TXACEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txacel::R`](R) reader structure"]
impl crate::Readable for TXACEL_SPEC {}
#[doc = "`reset()` method sets txacel to value 0"]
impl crate::Resettable for TXACEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
