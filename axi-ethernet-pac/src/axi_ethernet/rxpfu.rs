#[doc = "Register `rxpfu` reader"]
pub type R = crate::R<RXPFU_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXPFU_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of pause frames received, MSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxpfu::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXPFU_SPEC;
impl crate::RegisterSpec for RXPFU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxpfu::R`](R) reader structure"]
impl crate::Readable for RXPFU_SPEC {}
#[doc = "`reset()` method sets rxpfu to value 0"]
impl crate::Resettable for RXPFU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
