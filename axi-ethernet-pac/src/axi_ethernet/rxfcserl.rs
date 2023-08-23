#[doc = "Register `rxfcserl` reader"]
pub struct R(crate::R<RXFCSERL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXFCSERL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXFCSERL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXFCSERL_SPEC>) -> Self {
        R(reader)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXFCSERL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of frames received with FCS error and at least 64 bytes, LSM\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxfcserl](index.html) module"]
pub struct RXFCSERL_SPEC;
impl crate::RegisterSpec for RXFCSERL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxfcserl::R](R) reader structure"]
impl crate::Readable for RXFCSERL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets rxfcserl to value 0"]
impl crate::Resettable for RXFCSERL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
