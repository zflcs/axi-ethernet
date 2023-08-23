#[doc = "Register `txmcl` reader"]
pub struct R(crate::R<TXMCL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXMCL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXMCL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXMCL_SPEC>) -> Self {
        R(reader)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXMCL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Multiple Collision Frames Transmitted OK, LSM\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txmcl](index.html) module"]
pub struct TXMCL_SPEC;
impl crate::RegisterSpec for TXMCL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txmcl::R](R) reader structure"]
impl crate::Readable for TXMCL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets txmcl to value 0"]
impl crate::Resettable for TXMCL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
