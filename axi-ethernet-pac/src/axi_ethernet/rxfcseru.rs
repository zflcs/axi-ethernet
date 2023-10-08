#[doc = "Register `rxfcseru` reader"]
pub type R = crate::R<RXFCSERU_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXFCSERU_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of frames received with FCS error and at least 64 bytes, MSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxfcseru::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXFCSERU_SPEC;
impl crate::RegisterSpec for RXFCSERU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxfcseru::R`](R) reader structure"]
impl crate::Readable for RXFCSERU_SPEC {}
#[doc = "`reset()` method sets rxfcseru to value 0"]
impl crate::Resettable for RXFCSERU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
