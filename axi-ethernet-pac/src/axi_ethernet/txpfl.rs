#[doc = "Register `txpfl` reader"]
pub struct R(crate::R<TXPFL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXPFL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXPFL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXPFL_SPEC>) -> Self {
        R(reader)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXPFL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of pause frames transmitted, LSM\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txpfl](index.html) module"]
pub struct TXPFL_SPEC;
impl crate::RegisterSpec for TXPFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txpfl::R](R) reader structure"]
impl crate::Readable for TXPFL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets txpfl to value 0"]
impl crate::Resettable for TXPFL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
