#[doc = "Register `rxfu` reader"]
pub type R = crate::R<RXFU_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXFU_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of frames received OK, MSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxfu::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXFU_SPEC;
impl crate::RegisterSpec for RXFU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxfu::R`](R) reader structure"]
impl crate::Readable for RXFU_SPEC {}
#[doc = "`reset()` method sets rxfu to value 0"]
impl crate::Resettable for RXFU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
