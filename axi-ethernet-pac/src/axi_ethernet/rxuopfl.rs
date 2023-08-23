#[doc = "Register `rxuopfl` reader"]
pub struct R(crate::R<RXUOPFL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXUOPFL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXUOPFL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXUOPFL_SPEC>) -> Self {
        R(reader)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXUOPFL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of control frames received with unsupported opcode, LSM\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxuopfl](index.html) module"]
pub struct RXUOPFL_SPEC;
impl crate::RegisterSpec for RXUOPFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxuopfl::R](R) reader structure"]
impl crate::Readable for RXUOPFL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets rxuopfl to value 0"]
impl crate::Resettable for RXUOPFL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
