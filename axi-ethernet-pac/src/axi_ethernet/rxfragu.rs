#[doc = "Register `rxfragu` reader"]
pub type R = crate::R<RXFRAGU_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXFRAGU_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of undersize and bad FCS frames received, MSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxfragu::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXFRAGU_SPEC;
impl crate::RegisterSpec for RXFRAGU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxfragu::R`](R) reader structure"]
impl crate::Readable for RXFRAGU_SPEC {}
#[doc = "`reset()` method sets rxfragu to value 0"]
impl crate::Resettable for RXFRAGU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
