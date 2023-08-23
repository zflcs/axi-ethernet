#[doc = "Register `txedefl` reader"]
pub struct R(crate::R<TXEDEFL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXEDEFL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXEDEFL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXEDEFL_SPEC>) -> Self {
        R(reader)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXEDEFL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Transmit Frames with excessive Defferal, LSM\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txedefl](index.html) module"]
pub struct TXEDEFL_SPEC;
impl crate::RegisterSpec for TXEDEFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txedefl::R](R) reader structure"]
impl crate::Readable for TXEDEFL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets txedefl to value 0"]
impl crate::Resettable for TXEDEFL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
