#[doc = "Register `txpfl` reader"]
pub type R = crate::R<TXPFL_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXPFL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of pause frames transmitted, LSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txpfl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXPFL_SPEC;
impl crate::RegisterSpec for TXPFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txpfl::R`](R) reader structure"]
impl crate::Readable for TXPFL_SPEC {}
#[doc = "`reset()` method sets txpfl to value 0"]
impl crate::Resettable for TXPFL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
