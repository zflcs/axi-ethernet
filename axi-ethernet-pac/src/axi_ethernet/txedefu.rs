#[doc = "Register `txedefu` reader"]
pub struct R(crate::R<TXEDEFU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXEDEFU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXEDEFU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXEDEFU_SPEC>) -> Self {
        R(reader)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXEDEFU_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Transmit Frames with excessive Defferal, MSM\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txedefu](index.html) module"]
pub struct TXEDEFU_SPEC;
impl crate::RegisterSpec for TXEDEFU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txedefu::R](R) reader structure"]
impl crate::Readable for TXEDEFU_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets txedefu to value 0"]
impl crate::Resettable for TXEDEFU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
