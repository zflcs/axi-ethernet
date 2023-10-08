#[doc = "Register `rxlterl` reader"]
pub type R = crate::R<RXLTERL_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXLTERL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of frames received with length error, LSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxlterl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXLTERL_SPEC;
impl crate::RegisterSpec for RXLTERL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxlterl::R`](R) reader structure"]
impl crate::Readable for RXLTERL_SPEC {}
#[doc = "`reset()` method sets rxlterl to value 0"]
impl crate::Resettable for RXLTERL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
