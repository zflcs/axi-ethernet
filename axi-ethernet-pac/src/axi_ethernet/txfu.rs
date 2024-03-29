#[doc = "Register `txfu` reader"]
pub type R = crate::R<TXFU_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXFU_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of frames transmitted OK, MSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txfu::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXFU_SPEC;
impl crate::RegisterSpec for TXFU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txfu::R`](R) reader structure"]
impl crate::Readable for TXFU_SPEC {}
#[doc = "`reset()` method sets txfu to value 0"]
impl crate::Resettable for TXFU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
