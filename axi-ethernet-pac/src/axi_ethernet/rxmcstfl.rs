#[doc = "Register `rxmcstfl` reader"]
pub type R = crate::R<RXMCSTFL_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXMCSTFL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of multicast frames received, LSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxmcstfl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXMCSTFL_SPEC;
impl crate::RegisterSpec for RXMCSTFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxmcstfl::R`](R) reader structure"]
impl crate::Readable for RXMCSTFL_SPEC {}
#[doc = "`reset()` method sets rxmcstfl to value 0"]
impl crate::Resettable for RXMCSTFL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
