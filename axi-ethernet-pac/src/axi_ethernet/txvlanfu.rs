#[doc = "Register `txvlanfu` reader"]
pub type R = crate::R<TXVLANFU_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXVLANFU_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of VLAN tagged frames transmitted, MSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txvlanfu::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXVLANFU_SPEC;
impl crate::RegisterSpec for TXVLANFU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txvlanfu::R`](R) reader structure"]
impl crate::Readable for TXVLANFU_SPEC {}
#[doc = "`reset()` method sets txvlanfu to value 0"]
impl crate::Resettable for TXVLANFU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
