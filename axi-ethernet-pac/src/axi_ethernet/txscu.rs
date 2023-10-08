#[doc = "Register `txscu` reader"]
pub type R = crate::R<TXSCU_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXSCU_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Single Collision frames transmitted OK, MSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txscu::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXSCU_SPEC;
impl crate::RegisterSpec for TXSCU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txscu::R`](R) reader structure"]
impl crate::Readable for TXSCU_SPEC {}
#[doc = "`reset()` method sets txscu to value 0"]
impl crate::Resettable for TXSCU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
