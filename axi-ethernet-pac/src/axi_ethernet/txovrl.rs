#[doc = "Register `txovrl` reader"]
pub type R = crate::R<TXOVRL_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXOVRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of oversize frames transmitted, LSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txovrl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXOVRL_SPEC;
impl crate::RegisterSpec for TXOVRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txovrl::R`](R) reader structure"]
impl crate::Readable for TXOVRL_SPEC {}
#[doc = "`reset()` method sets txovrl to value 0"]
impl crate::Resettable for TXOVRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
