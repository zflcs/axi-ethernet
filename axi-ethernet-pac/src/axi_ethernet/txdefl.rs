#[doc = "Register `txdefl` reader"]
pub type R = crate::R<TXDEFL_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXDEFL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Deferred Tx Frames, LSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdefl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXDEFL_SPEC;
impl crate::RegisterSpec for TXDEFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdefl::R`](R) reader structure"]
impl crate::Readable for TXDEFL_SPEC {}
#[doc = "`reset()` method sets txdefl to value 0"]
impl crate::Resettable for TXDEFL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
