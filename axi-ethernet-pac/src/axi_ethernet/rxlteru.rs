#[doc = "Register `rxlteru` reader"]
pub type R = crate::R<RXLTERU_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXLTERU_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of frames received with length error, MSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxlteru::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXLTERU_SPEC;
impl crate::RegisterSpec for RXLTERU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxlteru::R`](R) reader structure"]
impl crate::Readable for RXLTERU_SPEC {}
#[doc = "`reset()` method sets rxlteru to value 0"]
impl crate::Resettable for RXLTERU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
