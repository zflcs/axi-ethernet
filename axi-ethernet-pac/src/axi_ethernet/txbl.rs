#[doc = "Register `txbl` reader"]
pub type R = crate::R<TXBL_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXBL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Transmitted Bytes, LSW\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXBL_SPEC;
impl crate::RegisterSpec for TXBL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbl::R`](R) reader structure"]
impl crate::Readable for TXBL_SPEC {}
#[doc = "`reset()` method sets txbl to value 0"]
impl crate::Resettable for TXBL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
