#[doc = "Register `rxaeru` reader"]
pub struct R(crate::R<RXAERU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXAERU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXAERU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXAERU_SPEC>) -> Self {
        R(reader)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXAERU_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Frames received with alignment errors, MSM\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxaeru](index.html) module"]
pub struct RXAERU_SPEC;
impl crate::RegisterSpec for RXAERU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxaeru::R](R) reader structure"]
impl crate::Readable for RXAERU_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets rxaeru to value 0"]
impl crate::Resettable for RXAERU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
