#[doc = "Register `tx1024bu` reader"]
pub struct R(crate::R<TX1024BU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX1024BU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX1024BU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX1024BU_SPEC>) -> Self {
        R(reader)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TX1024BU_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of 1024-MAX bytes frames transmitted, MSM\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx1024bu](index.html) module"]
pub struct TX1024BU_SPEC;
impl crate::RegisterSpec for TX1024BU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx1024bu::R](R) reader structure"]
impl crate::Readable for TX1024BU_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets tx1024bu to value 0"]
impl crate::Resettable for TX1024BU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
