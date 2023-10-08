#[doc = "Register `rxvlanfl` reader"]
pub type R = crate::R<RXVLANFL_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXVLANFL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of VLAN tagged frames received, LSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxvlanfl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXVLANFL_SPEC;
impl crate::RegisterSpec for RXVLANFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxvlanfl::R`](R) reader structure"]
impl crate::Readable for RXVLANFL_SPEC {}
#[doc = "`reset()` method sets rxvlanfl to value 0"]
impl crate::Resettable for RXVLANFL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
