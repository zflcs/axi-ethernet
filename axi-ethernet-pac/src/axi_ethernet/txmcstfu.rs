#[doc = "Register `txmcstfu` reader"]
pub struct R(crate::R<TXMCSTFU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXMCSTFU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXMCSTFU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXMCSTFU_SPEC>) -> Self {
        R(reader)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXMCSTFU_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of multicast frames transmitted, MSM\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txmcstfu](index.html) module"]
pub struct TXMCSTFU_SPEC;
impl crate::RegisterSpec for TXMCSTFU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txmcstfu::R](R) reader structure"]
impl crate::Readable for TXMCSTFU_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets txmcstfu to value 0"]
impl crate::Resettable for TXMCSTFU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
