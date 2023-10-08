#[doc = "Register `txundererl` reader"]
pub type R = crate::R<TXUNDERERL_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXUNDERERL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of frames transmitted underrun error, LSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txundererl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXUNDERERL_SPEC;
impl crate::RegisterSpec for TXUNDERERL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txundererl::R`](R) reader structure"]
impl crate::Readable for TXUNDERERL_SPEC {}
#[doc = "`reset()` method sets txundererl to value 0"]
impl crate::Resettable for TXUNDERERL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
