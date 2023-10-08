#[doc = "Register `txbu` reader"]
pub type R = crate::R<TXBU_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXBU_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Transmitted Bytes, MSW\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbu::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXBU_SPEC;
impl crate::RegisterSpec for TXBU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbu::R`](R) reader structure"]
impl crate::Readable for TXBU_SPEC {}
#[doc = "`reset()` method sets txbu to value 0"]
impl crate::Resettable for TXBU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
