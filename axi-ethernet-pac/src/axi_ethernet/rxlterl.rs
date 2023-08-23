#[doc = "Register `rxlterl` reader"]
pub struct R(crate::R<RXLTERL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXLTERL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXLTERL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXLTERL_SPEC>) -> Self {
        R(reader)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXLTERL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of frames received with length error, LSM\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxlterl](index.html) module"]
pub struct RXLTERL_SPEC;
impl crate::RegisterSpec for RXLTERL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxlterl::R](R) reader structure"]
impl crate::Readable for RXLTERL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets rxlterl to value 0"]
impl crate::Resettable for RXLTERL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
