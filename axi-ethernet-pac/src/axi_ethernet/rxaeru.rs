#[doc = "Register `rxaeru` reader"]
pub type R = crate::R<RXAERU_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXAERU_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Frames received with alignment errors, MSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxaeru::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXAERU_SPEC;
impl crate::RegisterSpec for RXAERU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxaeru::R`](R) reader structure"]
impl crate::Readable for RXAERU_SPEC {}
#[doc = "`reset()` method sets rxaeru to value 0"]
impl crate::Resettable for RXAERU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
