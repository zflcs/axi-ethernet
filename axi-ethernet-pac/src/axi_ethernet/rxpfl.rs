#[doc = "Register `rxpfl` reader"]
pub type R = crate::R<RXPFL_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXPFL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of pause frames received, LSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxpfl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXPFL_SPEC;
impl crate::RegisterSpec for RXPFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxpfl::R`](R) reader structure"]
impl crate::Readable for RXPFL_SPEC {}
#[doc = "`reset()` method sets rxpfl to value 0"]
impl crate::Resettable for RXPFL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
