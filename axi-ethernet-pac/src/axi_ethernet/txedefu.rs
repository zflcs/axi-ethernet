#[doc = "Register `txedefu` reader"]
pub type R = crate::R<TXEDEFU_SPEC>;
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
#[doc = "Transmit Frames with excessive Defferal, MSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txedefu::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXEDEFU_SPEC;
impl crate::RegisterSpec for TXEDEFU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txedefu::R`](R) reader structure"]
impl crate::Readable for TXEDEFU_SPEC {}
#[doc = "`reset()` method sets txedefu to value 0"]
impl crate::Resettable for TXEDEFU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
