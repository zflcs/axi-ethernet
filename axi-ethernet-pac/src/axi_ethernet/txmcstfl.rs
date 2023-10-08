#[doc = "Register `txmcstfl` reader"]
pub type R = crate::R<TXMCSTFL_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXMCSTFL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of multicast frames transmitted, LSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txmcstfl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXMCSTFL_SPEC;
impl crate::RegisterSpec for TXMCSTFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txmcstfl::R`](R) reader structure"]
impl crate::Readable for TXMCSTFL_SPEC {}
#[doc = "`reset()` method sets txmcstfl to value 0"]
impl crate::Resettable for TXMCSTFL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
