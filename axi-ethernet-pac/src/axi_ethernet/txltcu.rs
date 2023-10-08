#[doc = "Register `txltcu` reader"]
pub type R = crate::R<TXLTCU_SPEC>;
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
#[doc = "Frames transmitted with late Collisions, MSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txltcu::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXLTCU_SPEC;
impl crate::RegisterSpec for TXLTCU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txltcu::R`](R) reader structure"]
impl crate::Readable for TXLTCU_SPEC {}
#[doc = "`reset()` method sets txltcu to value 0"]
impl crate::Resettable for TXLTCU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
