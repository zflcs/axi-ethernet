#[doc = "Register `txbcstfl` reader"]
pub struct R(crate::R<TXBCSTFL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXBCSTFL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXBCSTFL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXBCSTFL_SPEC>) -> Self {
        R(reader)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXBCSTFL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of broadcast frames transmitted, LSM\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbcstfl](index.html) module"]
pub struct TXBCSTFL_SPEC;
impl crate::RegisterSpec for TXBCSTFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txbcstfl::R](R) reader structure"]
impl crate::Readable for TXBCSTFL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets txbcstfl to value 0"]
impl crate::Resettable for TXBCSTFL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
