#[doc = "Register `rxmcstfu` reader"]
pub type R = crate::R<RXMCSTFU_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXMCSTFU_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of multicast frames received, MSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxmcstfu::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXMCSTFU_SPEC;
impl crate::RegisterSpec for RXMCSTFU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxmcstfu::R`](R) reader structure"]
impl crate::Readable for RXMCSTFU_SPEC {}
#[doc = "`reset()` method sets rxmcstfu to value 0"]
impl crate::Resettable for RXMCSTFU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
