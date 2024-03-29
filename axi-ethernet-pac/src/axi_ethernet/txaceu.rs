#[doc = "Register `txaceu` reader"]
pub type R = crate::R<TXACEU_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXACEU_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Frames aborted with excessive Collisions, MSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txaceu::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXACEU_SPEC;
impl crate::RegisterSpec for TXACEU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txaceu::R`](R) reader structure"]
impl crate::Readable for TXACEU_SPEC {}
#[doc = "`reset()` method sets txaceu to value 0"]
impl crate::Resettable for TXACEU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
