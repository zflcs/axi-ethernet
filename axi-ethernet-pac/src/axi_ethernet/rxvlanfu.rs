#[doc = "Register `rxvlanfu` reader"]
pub type R = crate::R<RXVLANFU_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXVLANFU_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of VLAN tagged frames received, MSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxvlanfu::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXVLANFU_SPEC;
impl crate::RegisterSpec for RXVLANFU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxvlanfu::R`](R) reader structure"]
impl crate::Readable for RXVLANFU_SPEC {}
#[doc = "`reset()` method sets rxvlanfu to value 0"]
impl crate::Resettable for RXVLANFU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
