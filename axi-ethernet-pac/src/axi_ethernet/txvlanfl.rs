#[doc = "Register `txvlanfl` reader"]
pub struct R(crate::R<TXVLANFL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXVLANFL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXVLANFL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXVLANFL_SPEC>) -> Self {
        R(reader)
    }
}
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
#[doc = "Count of VLAN tagged frames transmitted, LSM\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txvlanfl](index.html) module"]
pub struct TXVLANFL_SPEC;
impl crate::RegisterSpec for TXVLANFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txvlanfl::R](R) reader structure"]
impl crate::Readable for TXVLANFL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets txvlanfl to value 0"]
impl crate::Resettable for TXVLANFL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
