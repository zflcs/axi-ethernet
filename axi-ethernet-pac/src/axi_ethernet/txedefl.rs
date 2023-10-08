#[doc = "Register `txedefl` reader"]
pub type R = crate::R<TXEDEFL_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXEDEFL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Transmit Frames with excessive Defferal, LSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txedefl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXEDEFL_SPEC;
impl crate::RegisterSpec for TXEDEFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txedefl::R`](R) reader structure"]
impl crate::Readable for TXEDEFL_SPEC {}
#[doc = "`reset()` method sets txedefl to value 0"]
impl crate::Resettable for TXEDEFL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
