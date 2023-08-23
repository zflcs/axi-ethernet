#[doc = "Register `rxpfl` reader"]
pub struct R(crate::R<RXPFL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXPFL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXPFL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXPFL_SPEC>) -> Self {
        R(reader)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXPFL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of pause frames received, LSM\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxpfl](index.html) module"]
pub struct RXPFL_SPEC;
impl crate::RegisterSpec for RXPFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxpfl::R](R) reader structure"]
impl crate::Readable for RXPFL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets rxpfl to value 0"]
impl crate::Resettable for RXPFL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
