#[doc = "Register `rxfcserl` reader"]
pub type R = crate::R<RXFCSERL_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXFCSERL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of frames received with FCS error and at least 64 bytes, LSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxfcserl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXFCSERL_SPEC;
impl crate::RegisterSpec for RXFCSERL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxfcserl::R`](R) reader structure"]
impl crate::Readable for RXFCSERL_SPEC {}
#[doc = "`reset()` method sets rxfcserl to value 0"]
impl crate::Resettable for RXFCSERL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
