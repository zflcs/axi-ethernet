#[doc = "Register `txundereru` reader"]
pub struct R(crate::R<TXUNDERERU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXUNDERERU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXUNDERERU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXUNDERERU_SPEC>) -> Self {
        R(reader)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXUNDERERU_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of frames transmitted underrun error, MSM\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txundereru](index.html) module"]
pub struct TXUNDERERU_SPEC;
impl crate::RegisterSpec for TXUNDERERU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txundereru::R](R) reader structure"]
impl crate::Readable for TXUNDERERU_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets txundereru to value 0"]
impl crate::Resettable for TXUNDERERU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
