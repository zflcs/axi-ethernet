#[doc = "Register `rxovru` reader"]
pub type R = crate::R<RXOVRU_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXOVRU_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of oversize bytes frames received, MSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxovru::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXOVRU_SPEC;
impl crate::RegisterSpec for RXOVRU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxovru::R`](R) reader structure"]
impl crate::Readable for RXOVRU_SPEC {}
#[doc = "`reset()` method sets rxovru to value 0"]
impl crate::Resettable for RXOVRU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
