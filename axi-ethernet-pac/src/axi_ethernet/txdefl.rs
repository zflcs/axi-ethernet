#[doc = "Register `txdefl` reader"]
pub struct R(crate::R<TXDEFL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXDEFL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXDEFL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXDEFL_SPEC>) -> Self {
        R(reader)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXDEFL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Deferred Tx Frames, LSM\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txdefl](index.html) module"]
pub struct TXDEFL_SPEC;
impl crate::RegisterSpec for TXDEFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txdefl::R](R) reader structure"]
impl crate::Readable for TXDEFL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets txdefl to value 0"]
impl crate::Resettable for TXDEFL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
