#[doc = "Register `txovru` reader"]
pub type R = crate::R<TXOVRU_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXOVRU_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of oversize bytes frames transmitted, MSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txovru::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXOVRU_SPEC;
impl crate::RegisterSpec for TXOVRU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txovru::R`](R) reader structure"]
impl crate::Readable for TXOVRU_SPEC {}
#[doc = "`reset()` method sets txovru to value 0"]
impl crate::Resettable for TXOVRU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
