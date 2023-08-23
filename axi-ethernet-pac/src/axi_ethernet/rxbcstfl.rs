#[doc = "Register `rxbcstfl` reader"]
pub struct R(crate::R<RXBCSTFL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXBCSTFL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXBCSTFL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXBCSTFL_SPEC>) -> Self {
        R(reader)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXBCSTFL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of broadcast frames received, LSM\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxbcstfl](index.html) module"]
pub struct RXBCSTFL_SPEC;
impl crate::RegisterSpec for RXBCSTFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxbcstfl::R](R) reader structure"]
impl crate::Readable for RXBCSTFL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets rxbcstfl to value 0"]
impl crate::Resettable for RXBCSTFL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
