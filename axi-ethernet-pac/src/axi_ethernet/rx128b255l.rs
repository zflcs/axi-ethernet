#[doc = "Register `rx128b255l` reader"]
pub struct R(crate::R<RX128B255L_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX128B255L_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX128B255L_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX128B255L_SPEC>) -> Self {
        R(reader)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RX128B255L_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of 128-255 bytes frames received, LSM\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx128b255l](index.html) module"]
pub struct RX128B255L_SPEC;
impl crate::RegisterSpec for RX128B255L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx128b255l::R](R) reader structure"]
impl crate::Readable for RX128B255L_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets rx128b255l to value 0"]
impl crate::Resettable for RX128B255L_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
