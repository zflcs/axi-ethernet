#[doc = "Register `rxovrl` reader"]
pub type R = crate::R<RXOVRL_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXOVRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of oversize frames received, LSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxovrl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXOVRL_SPEC;
impl crate::RegisterSpec for RXOVRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxovrl::R`](R) reader structure"]
impl crate::Readable for RXOVRL_SPEC {}
#[doc = "`reset()` method sets rxovrl to value 0"]
impl crate::Resettable for RXOVRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
