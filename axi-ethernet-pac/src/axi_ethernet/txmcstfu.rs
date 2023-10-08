#[doc = "Register `txmcstfu` reader"]
pub type R = crate::R<TXMCSTFU_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXMCSTFU_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of multicast frames transmitted, MSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txmcstfu::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXMCSTFU_SPEC;
impl crate::RegisterSpec for TXMCSTFU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txmcstfu::R`](R) reader structure"]
impl crate::Readable for TXMCSTFU_SPEC {}
#[doc = "`reset()` method sets txmcstfu to value 0"]
impl crate::Resettable for TXMCSTFU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
