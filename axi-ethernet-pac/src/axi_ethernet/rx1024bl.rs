#[doc = "Register `rx1024bl` reader"]
pub type R = crate::R<RX1024BL_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RX1024BL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of 1024-MAX bytes frames received, LSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx1024bl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX1024BL_SPEC;
impl crate::RegisterSpec for RX1024BL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx1024bl::R`](R) reader structure"]
impl crate::Readable for RX1024BL_SPEC {}
#[doc = "`reset()` method sets rx1024bl to value 0"]
impl crate::Resettable for RX1024BL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
