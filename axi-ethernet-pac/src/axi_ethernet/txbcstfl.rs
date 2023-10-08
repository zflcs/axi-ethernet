#[doc = "Register `txbcstfl` reader"]
pub type R = crate::R<TXBCSTFL_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXBCSTFL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of broadcast frames transmitted, LSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbcstfl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXBCSTFL_SPEC;
impl crate::RegisterSpec for TXBCSTFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbcstfl::R`](R) reader structure"]
impl crate::Readable for TXBCSTFL_SPEC {}
#[doc = "`reset()` method sets txbcstfl to value 0"]
impl crate::Resettable for TXBCSTFL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
