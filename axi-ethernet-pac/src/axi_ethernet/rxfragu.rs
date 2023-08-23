#[doc = "Register `rxfragu` reader"]
pub struct R(crate::R<RXFRAGU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXFRAGU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXFRAGU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXFRAGU_SPEC>) -> Self {
        R(reader)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXFRAGU_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of undersize and bad FCS frames received, MSM\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxfragu](index.html) module"]
pub struct RXFRAGU_SPEC;
impl crate::RegisterSpec for RXFRAGU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxfragu::R](R) reader structure"]
impl crate::Readable for RXFRAGU_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets rxfragu to value 0"]
impl crate::Resettable for RXFRAGU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
