#[doc = "Register `txvlanfu` reader"]
pub struct R(crate::R<TXVLANFU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXVLANFU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXVLANFU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXVLANFU_SPEC>) -> Self {
        R(reader)
    }
}
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
#[doc = "Count of VLAN tagged frames transmitted, MSM\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txvlanfu](index.html) module"]
pub struct TXVLANFU_SPEC;
impl crate::RegisterSpec for TXVLANFU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txvlanfu::R](R) reader structure"]
impl crate::Readable for TXVLANFU_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets txvlanfu to value 0"]
impl crate::Resettable for TXVLANFU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
