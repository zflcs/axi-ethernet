#[doc = "Register `txbcstfu` reader"]
pub type R = crate::R<TXBCSTFU_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXBCSTFU_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of broadcast frames transmitted, MSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbcstfu::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXBCSTFU_SPEC;
impl crate::RegisterSpec for TXBCSTFU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbcstfu::R`](R) reader structure"]
impl crate::Readable for TXBCSTFU_SPEC {}
#[doc = "`reset()` method sets txbcstfu to value 0"]
impl crate::Resettable for TXBCSTFU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
