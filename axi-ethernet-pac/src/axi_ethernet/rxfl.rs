#[doc = "Register `rxfl` reader"]
pub type R = crate::R<RXFL_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXFL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of frames received OK, LSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxfl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXFL_SPEC;
impl crate::RegisterSpec for RXFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxfl::R`](R) reader structure"]
impl crate::Readable for RXFL_SPEC {}
#[doc = "`reset()` method sets rxfl to value 0"]
impl crate::Resettable for RXFL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
