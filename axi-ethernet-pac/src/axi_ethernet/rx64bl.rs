#[doc = "Register `rx64bl` reader"]
pub type R = crate::R<RX64BL_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RX64BL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of 64 bytes frames received, LSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx64bl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX64BL_SPEC;
impl crate::RegisterSpec for RX64BL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx64bl::R`](R) reader structure"]
impl crate::Readable for RX64BL_SPEC {}
#[doc = "`reset()` method sets rx64bl to value 0"]
impl crate::Resettable for RX64BL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
