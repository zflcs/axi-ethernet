#[doc = "Register `rxmcstfu` reader"]
pub struct R(crate::R<RXMCSTFU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXMCSTFU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXMCSTFU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXMCSTFU_SPEC>) -> Self {
        R(reader)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXMCSTFU_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of multicast frames received, MSM\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxmcstfu](index.html) module"]
pub struct RXMCSTFU_SPEC;
impl crate::RegisterSpec for RXMCSTFU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxmcstfu::R](R) reader structure"]
impl crate::Readable for RXMCSTFU_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets rxmcstfu to value 0"]
impl crate::Resettable for RXMCSTFU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
