#[doc = "Register `txltcu` reader"]
pub struct R(crate::R<TXLTCU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXLTCU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXLTCU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXLTCU_SPEC>) -> Self {
        R(reader)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXLTCU_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Frames transmitted with late Collisions, MSM\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txltcu](index.html) module"]
pub struct TXLTCU_SPEC;
impl crate::RegisterSpec for TXLTCU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txltcu::R](R) reader structure"]
impl crate::Readable for TXLTCU_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets txltcu to value 0"]
impl crate::Resettable for TXLTCU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
