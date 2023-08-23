#[doc = "Register `rxundrl` reader"]
pub struct R(crate::R<RXUNDRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXUNDRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXUNDRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXUNDRL_SPEC>) -> Self {
        R(reader)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXUNDRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of undersize frames received, LSM\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxundrl](index.html) module"]
pub struct RXUNDRL_SPEC;
impl crate::RegisterSpec for RXUNDRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxundrl::R](R) reader structure"]
impl crate::Readable for RXUNDRL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets rxundrl to value 0"]
impl crate::Resettable for RXUNDRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
