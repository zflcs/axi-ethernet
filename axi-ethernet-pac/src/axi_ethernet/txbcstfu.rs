#[doc = "Register `txbcstfu` reader"]
pub struct R(crate::R<TXBCSTFU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXBCSTFU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXBCSTFU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXBCSTFU_SPEC>) -> Self {
        R(reader)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXBCSTFU_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of broadcast frames transmitted, MSM\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbcstfu](index.html) module"]
pub struct TXBCSTFU_SPEC;
impl crate::RegisterSpec for TXBCSTFU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txbcstfu::R](R) reader structure"]
impl crate::Readable for TXBCSTFU_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets txbcstfu to value 0"]
impl crate::Resettable for TXBCSTFU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
