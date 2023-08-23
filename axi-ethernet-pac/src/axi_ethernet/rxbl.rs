#[doc = "Register `rxbl` reader"]
pub struct R(crate::R<RXBL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXBL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXBL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXBL_SPEC>) -> Self {
        R(reader)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXBL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Received Bytes, LSW\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxbl](index.html) module"]
pub struct RXBL_SPEC;
impl crate::RegisterSpec for RXBL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxbl::R](R) reader structure"]
impl crate::Readable for RXBL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets rxbl to value 0"]
impl crate::Resettable for RXBL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
