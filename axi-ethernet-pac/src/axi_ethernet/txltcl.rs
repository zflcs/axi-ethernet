#[doc = "Register `txltcl` reader"]
pub struct R(crate::R<TXLTCL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXLTCL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXLTCL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXLTCL_SPEC>) -> Self {
        R(reader)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXLTCL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Frames transmitted with late Collisions, LSM\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txltcl](index.html) module"]
pub struct TXLTCL_SPEC;
impl crate::RegisterSpec for TXLTCL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txltcl::R](R) reader structure"]
impl crate::Readable for TXLTCL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets txltcl to value 0"]
impl crate::Resettable for TXLTCL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
