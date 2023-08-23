#[doc = "Register `rx256b511l` reader"]
pub struct R(crate::R<RX256B511L_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX256B511L_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX256B511L_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX256B511L_SPEC>) -> Self {
        R(reader)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RX256B511L_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of 256-511 bytes frames received, LSM\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx256b511l](index.html) module"]
pub struct RX256B511L_SPEC;
impl crate::RegisterSpec for RX256B511L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx256b511l::R](R) reader structure"]
impl crate::Readable for RX256B511L_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets rx256b511l to value 0"]
impl crate::Resettable for RX256B511L_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
