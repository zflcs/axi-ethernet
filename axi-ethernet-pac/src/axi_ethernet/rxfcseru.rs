#[doc = "Register `rxfcseru` reader"]
pub struct R(crate::R<RXFCSERU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXFCSERU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXFCSERU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXFCSERU_SPEC>) -> Self {
        R(reader)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXFCSERU_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of frames received with FCS error and at least 64 bytes, MSM\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxfcseru](index.html) module"]
pub struct RXFCSERU_SPEC;
impl crate::RegisterSpec for RXFCSERU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxfcseru::R](R) reader structure"]
impl crate::Readable for RXFCSERU_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets rxfcseru to value 0"]
impl crate::Resettable for RXFCSERU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
