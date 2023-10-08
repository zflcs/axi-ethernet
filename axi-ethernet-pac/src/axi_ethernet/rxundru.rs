#[doc = "Register `rxundru` reader"]
pub type R = crate::R<RXUNDRU_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXUNDRU_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of undersize frames received, MSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxundru::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXUNDRU_SPEC;
impl crate::RegisterSpec for RXUNDRU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxundru::R`](R) reader structure"]
impl crate::Readable for RXUNDRU_SPEC {}
#[doc = "`reset()` method sets rxundru to value 0"]
impl crate::Resettable for RXUNDRU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
