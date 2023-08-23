#[doc = "Register `rxovrl` reader"]
pub struct R(crate::R<RXOVRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXOVRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXOVRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXOVRL_SPEC>) -> Self {
        R(reader)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXOVRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of oversize frames received, LSM\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxovrl](index.html) module"]
pub struct RXOVRL_SPEC;
impl crate::RegisterSpec for RXOVRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxovrl::R](R) reader structure"]
impl crate::Readable for RXOVRL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets rxovrl to value 0"]
impl crate::Resettable for RXOVRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
