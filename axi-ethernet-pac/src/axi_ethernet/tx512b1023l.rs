#[doc = "Register `tx512b1023l` reader"]
pub struct R(crate::R<TX512B1023L_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX512B1023L_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX512B1023L_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX512B1023L_SPEC>) -> Self {
        R(reader)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TX512B1023L_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of 512-1023 bytes frames transmitted, LSM\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx512b1023l](index.html) module"]
pub struct TX512B1023L_SPEC;
impl crate::RegisterSpec for TX512B1023L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx512b1023l::R](R) reader structure"]
impl crate::Readable for TX512B1023L_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets tx512b1023l to value 0"]
impl crate::Resettable for TX512B1023L_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
