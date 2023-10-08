#[doc = "Register `rxctrfu` reader"]
pub type R = crate::R<RXCTRFU_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXCTRFU_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of control frames received, MSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxctrfu::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXCTRFU_SPEC;
impl crate::RegisterSpec for RXCTRFU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxctrfu::R`](R) reader structure"]
impl crate::Readable for RXCTRFU_SPEC {}
#[doc = "`reset()` method sets rxctrfu to value 0"]
impl crate::Resettable for RXCTRFU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
