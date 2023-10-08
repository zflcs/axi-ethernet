#[doc = "Register `rxundrl` reader"]
pub type R = crate::R<RXUNDRL_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXUNDRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of undersize frames received, LSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxundrl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXUNDRL_SPEC;
impl crate::RegisterSpec for RXUNDRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxundrl::R`](R) reader structure"]
impl crate::Readable for RXUNDRL_SPEC {}
#[doc = "`reset()` method sets rxundrl to value 0"]
impl crate::Resettable for RXUNDRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
