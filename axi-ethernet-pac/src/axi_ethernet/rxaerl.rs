#[doc = "Register `rxaerl` reader"]
pub struct R(crate::R<RXAERL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXAERL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXAERL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXAERL_SPEC>) -> Self {
        R(reader)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXAERL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Frames received with alignment errors, LSM\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxaerl](index.html) module"]
pub struct RXAERL_SPEC;
impl crate::RegisterSpec for RXAERL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxaerl::R](R) reader structure"]
impl crate::Readable for RXAERL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets rxaerl to value 0"]
impl crate::Resettable for RXAERL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
