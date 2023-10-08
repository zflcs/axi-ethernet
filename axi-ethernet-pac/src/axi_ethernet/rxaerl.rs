#[doc = "Register `rxaerl` reader"]
pub type R = crate::R<RXAERL_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXAERL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Frames received with alignment errors, LSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxaerl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXAERL_SPEC;
impl crate::RegisterSpec for RXAERL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxaerl::R`](R) reader structure"]
impl crate::Readable for RXAERL_SPEC {}
#[doc = "`reset()` method sets rxaerl to value 0"]
impl crate::Resettable for RXAERL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
