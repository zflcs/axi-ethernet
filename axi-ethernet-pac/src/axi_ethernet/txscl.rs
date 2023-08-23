#[doc = "Register `txscl` reader"]
pub struct R(crate::R<TXSCL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXSCL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXSCL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXSCL_SPEC>) -> Self {
        R(reader)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXSCL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Single Collision frames transmitted OK, LSM\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txscl](index.html) module"]
pub struct TXSCL_SPEC;
impl crate::RegisterSpec for TXSCL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txscl::R](R) reader structure"]
impl crate::Readable for TXSCL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets txscl to value 0"]
impl crate::Resettable for TXSCL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
