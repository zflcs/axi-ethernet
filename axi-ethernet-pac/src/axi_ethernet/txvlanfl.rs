#[doc = "Register `txvlanfl` reader"]
pub type R = crate::R<TXVLANFL_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXVLANFL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of VLAN tagged frames transmitted, LSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txvlanfl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXVLANFL_SPEC;
impl crate::RegisterSpec for TXVLANFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txvlanfl::R`](R) reader structure"]
impl crate::Readable for TXVLANFL_SPEC {}
#[doc = "`reset()` method sets txvlanfl to value 0"]
impl crate::Resettable for TXVLANFL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
