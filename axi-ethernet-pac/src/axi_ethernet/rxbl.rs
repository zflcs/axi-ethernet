#[doc = "Register `rxbl` reader"]
pub type R = crate::R<RXBL_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXBL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Received Bytes, LSW\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxbl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXBL_SPEC;
impl crate::RegisterSpec for RXBL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxbl::R`](R) reader structure"]
impl crate::Readable for RXBL_SPEC {}
#[doc = "`reset()` method sets rxbl to value 0"]
impl crate::Resettable for RXBL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
