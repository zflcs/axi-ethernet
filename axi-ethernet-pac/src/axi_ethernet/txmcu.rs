#[doc = "Register `txmcu` reader"]
pub type R = crate::R<TXMCU_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXMCU_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Multiple Collision frames Transmitted OK, MSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txmcu::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXMCU_SPEC;
impl crate::RegisterSpec for TXMCU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txmcu::R`](R) reader structure"]
impl crate::Readable for TXMCU_SPEC {}
#[doc = "`reset()` method sets txmcu to value 0"]
impl crate::Resettable for TXMCU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
