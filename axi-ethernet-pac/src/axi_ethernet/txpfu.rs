#[doc = "Register `txpfu` reader"]
pub type R = crate::R<TXPFU_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXPFU_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of pause frames transmitted, MSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txpfu::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXPFU_SPEC;
impl crate::RegisterSpec for TXPFU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txpfu::R`](R) reader structure"]
impl crate::Readable for TXPFU_SPEC {}
#[doc = "`reset()` method sets txpfu to value 0"]
impl crate::Resettable for TXPFU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
