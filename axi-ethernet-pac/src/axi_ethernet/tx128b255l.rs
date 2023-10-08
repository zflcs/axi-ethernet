#[doc = "Register `tx128b255l` reader"]
pub type R = crate::R<TX128B255L_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TX128B255L_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of 128-255 bytes frames transmitted, LSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx128b255l::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX128B255L_SPEC;
impl crate::RegisterSpec for TX128B255L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx128b255l::R`](R) reader structure"]
impl crate::Readable for TX128B255L_SPEC {}
#[doc = "`reset()` method sets tx128b255l to value 0"]
impl crate::Resettable for TX128B255L_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
