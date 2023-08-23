#[doc = "Register `rxuopfu` reader"]
pub struct R(crate::R<RXUOPFU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXUOPFU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXUOPFU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXUOPFU_SPEC>) -> Self {
        R(reader)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXUOPFU_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of control frames received with unsupported opcode, MSM\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxuopfu](index.html) module"]
pub struct RXUOPFU_SPEC;
impl crate::RegisterSpec for RXUOPFU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxuopfu::R](R) reader structure"]
impl crate::Readable for RXUOPFU_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets rxuopfu to value 0"]
impl crate::Resettable for RXUOPFU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
