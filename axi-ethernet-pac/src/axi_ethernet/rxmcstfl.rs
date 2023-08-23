#[doc = "Register `rxmcstfl` reader"]
pub struct R(crate::R<RXMCSTFL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXMCSTFL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXMCSTFL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXMCSTFL_SPEC>) -> Self {
        R(reader)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXMCSTFL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of multicast frames received, LSM\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxmcstfl](index.html) module"]
pub struct RXMCSTFL_SPEC;
impl crate::RegisterSpec for RXMCSTFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxmcstfl::R](R) reader structure"]
impl crate::Readable for RXMCSTFL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets rxmcstfl to value 0"]
impl crate::Resettable for RXMCSTFL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
