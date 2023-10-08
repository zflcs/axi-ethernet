#[doc = "Register `rx65b127u` reader"]
pub type R = crate::R<RX65B127U_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RX65B127U_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of 65-127 bytes frames received, MSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx65b127u::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX65B127U_SPEC;
impl crate::RegisterSpec for RX65B127U_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx65b127u::R`](R) reader structure"]
impl crate::Readable for RX65B127U_SPEC {}
#[doc = "`reset()` method sets rx65b127u to value 0"]
impl crate::Resettable for RX65B127U_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
