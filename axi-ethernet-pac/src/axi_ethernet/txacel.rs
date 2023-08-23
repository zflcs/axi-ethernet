#[doc = "Register `txacel` reader"]
pub struct R(crate::R<TXACEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXACEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXACEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXACEL_SPEC>) -> Self {
        R(reader)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXACEL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Frames aborted with excessive Collisions, LSM\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txacel](index.html) module"]
pub struct TXACEL_SPEC;
impl crate::RegisterSpec for TXACEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txacel::R](R) reader structure"]
impl crate::Readable for TXACEL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets txacel to value 0"]
impl crate::Resettable for TXACEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
