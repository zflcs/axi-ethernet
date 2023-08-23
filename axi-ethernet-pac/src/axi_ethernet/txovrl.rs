#[doc = "Register `txovrl` reader"]
pub struct R(crate::R<TXOVRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXOVRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXOVRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXOVRL_SPEC>) -> Self {
        R(reader)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXOVRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of oversize frames transmitted, LSM\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txovrl](index.html) module"]
pub struct TXOVRL_SPEC;
impl crate::RegisterSpec for TXOVRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txovrl::R](R) reader structure"]
impl crate::Readable for TXOVRL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets txovrl to value 0"]
impl crate::Resettable for TXOVRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
