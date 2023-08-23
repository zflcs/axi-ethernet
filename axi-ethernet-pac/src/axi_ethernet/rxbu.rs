#[doc = "Register `rxbu` reader"]
pub struct R(crate::R<RXBU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXBU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXBU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXBU_SPEC>) -> Self {
        R(reader)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXBU_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Received Bytes, MSW\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxbu](index.html) module"]
pub struct RXBU_SPEC;
impl crate::RegisterSpec for RXBU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxbu::R](R) reader structure"]
impl crate::Readable for RXBU_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets rxbu to value 0"]
impl crate::Resettable for RXBU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
