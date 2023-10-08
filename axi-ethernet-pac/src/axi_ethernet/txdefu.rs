#[doc = "Register `txdefu` reader"]
pub type R = crate::R<TXDEFU_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXDEFU_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Deferred Tx Frames, MSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdefu::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXDEFU_SPEC;
impl crate::RegisterSpec for TXDEFU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdefu::R`](R) reader structure"]
impl crate::Readable for TXDEFU_SPEC {}
#[doc = "`reset()` method sets txdefu to value 0"]
impl crate::Resettable for TXDEFU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
