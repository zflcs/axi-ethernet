#[doc = "Register `txscl` reader"]
pub type R = crate::R<TXSCL_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXSCL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Single Collision frames transmitted OK, LSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txscl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXSCL_SPEC;
impl crate::RegisterSpec for TXSCL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txscl::R`](R) reader structure"]
impl crate::Readable for TXSCL_SPEC {}
#[doc = "`reset()` method sets txscl to value 0"]
impl crate::Resettable for TXSCL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
