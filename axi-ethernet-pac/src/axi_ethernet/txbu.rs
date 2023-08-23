#[doc = "Register `txbu` reader"]
pub struct R(crate::R<TXBU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXBU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXBU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXBU_SPEC>) -> Self {
        R(reader)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXBU_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Transmitted Bytes, MSW\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbu](index.html) module"]
pub struct TXBU_SPEC;
impl crate::RegisterSpec for TXBU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txbu::R](R) reader structure"]
impl crate::Readable for TXBU_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets txbu to value 0"]
impl crate::Resettable for TXBU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
