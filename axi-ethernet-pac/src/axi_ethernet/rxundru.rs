#[doc = "Register `rxundru` reader"]
pub struct R(crate::R<RXUNDRU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXUNDRU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXUNDRU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXUNDRU_SPEC>) -> Self {
        R(reader)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXUNDRU_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of undersize frames received, MSM\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxundru](index.html) module"]
pub struct RXUNDRU_SPEC;
impl crate::RegisterSpec for RXUNDRU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxundru::R](R) reader structure"]
impl crate::Readable for RXUNDRU_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets rxundru to value 0"]
impl crate::Resettable for RXUNDRU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
