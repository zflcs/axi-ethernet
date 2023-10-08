#[doc = "Register `txctrfu` reader"]
pub type R = crate::R<TXCTRFU_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXCTRFU_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of control frames transmitted, MSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txctrfu::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXCTRFU_SPEC;
impl crate::RegisterSpec for TXCTRFU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txctrfu::R`](R) reader structure"]
impl crate::Readable for TXCTRFU_SPEC {}
#[doc = "`reset()` method sets txctrfu to value 0"]
impl crate::Resettable for TXCTRFU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
