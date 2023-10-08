#[doc = "Register `rxbcstfu` reader"]
pub type R = crate::R<RXBCSTFU_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXBCSTFU_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of broadcast frames received, MSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxbcstfu::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXBCSTFU_SPEC;
impl crate::RegisterSpec for RXBCSTFU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxbcstfu::R`](R) reader structure"]
impl crate::Readable for RXBCSTFU_SPEC {}
#[doc = "`reset()` method sets rxbcstfu to value 0"]
impl crate::Resettable for RXBCSTFU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
