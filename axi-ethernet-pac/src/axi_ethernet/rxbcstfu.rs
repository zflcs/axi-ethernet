#[doc = "Register `rxbcstfu` reader"]
pub struct R(crate::R<RXBCSTFU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXBCSTFU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXBCSTFU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXBCSTFU_SPEC>) -> Self {
        R(reader)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXBCSTFU_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of broadcast frames received, MSM\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxbcstfu](index.html) module"]
pub struct RXBCSTFU_SPEC;
impl crate::RegisterSpec for RXBCSTFU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxbcstfu::R](R) reader structure"]
impl crate::Readable for RXBCSTFU_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets rxbcstfu to value 0"]
impl crate::Resettable for RXBCSTFU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
