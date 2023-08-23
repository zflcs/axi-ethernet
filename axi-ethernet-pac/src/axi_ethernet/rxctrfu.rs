#[doc = "Register `rxctrfu` reader"]
pub struct R(crate::R<RXCTRFU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXCTRFU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXCTRFU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXCTRFU_SPEC>) -> Self {
        R(reader)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXCTRFU_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of control frames received, MSM\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxctrfu](index.html) module"]
pub struct RXCTRFU_SPEC;
impl crate::RegisterSpec for RXCTRFU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxctrfu::R](R) reader structure"]
impl crate::Readable for RXCTRFU_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets rxctrfu to value 0"]
impl crate::Resettable for RXCTRFU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
