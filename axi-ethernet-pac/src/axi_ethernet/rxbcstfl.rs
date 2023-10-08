#[doc = "Register `rxbcstfl` reader"]
pub type R = crate::R<RXBCSTFL_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXBCSTFL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of broadcast frames received, LSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxbcstfl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXBCSTFL_SPEC;
impl crate::RegisterSpec for RXBCSTFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxbcstfl::R`](R) reader structure"]
impl crate::Readable for RXBCSTFL_SPEC {}
#[doc = "`reset()` method sets rxbcstfl to value 0"]
impl crate::Resettable for RXBCSTFL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
