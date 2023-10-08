#[doc = "Register `rxctrfl` reader"]
pub type R = crate::R<RXCTRFL_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXCTRFL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of control frames received, LSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxctrfl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXCTRFL_SPEC;
impl crate::RegisterSpec for RXCTRFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxctrfl::R`](R) reader structure"]
impl crate::Readable for RXCTRFL_SPEC {}
#[doc = "`reset()` method sets rxctrfl to value 0"]
impl crate::Resettable for RXCTRFL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
