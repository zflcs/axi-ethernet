#[doc = "Register `txdefu` reader"]
pub struct R(crate::R<TXDEFU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXDEFU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXDEFU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXDEFU_SPEC>) -> Self {
        R(reader)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXDEFU_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Deferred Tx Frames, MSM\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txdefu](index.html) module"]
pub struct TXDEFU_SPEC;
impl crate::RegisterSpec for TXDEFU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txdefu::R](R) reader structure"]
impl crate::Readable for TXDEFU_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets txdefu to value 0"]
impl crate::Resettable for TXDEFU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
