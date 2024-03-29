#[doc = "Register `txctrfl` reader"]
pub type R = crate::R<TXCTRFL_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXCTRFL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of control frames transmitted, LSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txctrfl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXCTRFL_SPEC;
impl crate::RegisterSpec for TXCTRFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txctrfl::R`](R) reader structure"]
impl crate::Readable for TXCTRFL_SPEC {}
#[doc = "`reset()` method sets txctrfl to value 0"]
impl crate::Resettable for TXCTRFL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
