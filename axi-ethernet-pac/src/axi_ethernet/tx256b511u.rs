#[doc = "Register `tx256b511u` reader"]
pub type R = crate::R<TX256B511U_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TX256B511U_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of 256-511 bytes frames transmitted, MSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx256b511u::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX256B511U_SPEC;
impl crate::RegisterSpec for TX256B511U_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx256b511u::R`](R) reader structure"]
impl crate::Readable for TX256B511U_SPEC {}
#[doc = "`reset()` method sets tx256b511u to value 0"]
impl crate::Resettable for TX256B511U_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
