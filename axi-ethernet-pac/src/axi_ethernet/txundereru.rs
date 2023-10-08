#[doc = "Register `txundereru` reader"]
pub type R = crate::R<TXUNDERERU_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXUNDERERU_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of frames transmitted underrun error, MSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txundereru::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXUNDERERU_SPEC;
impl crate::RegisterSpec for TXUNDERERU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txundereru::R`](R) reader structure"]
impl crate::Readable for TXUNDERERU_SPEC {}
#[doc = "`reset()` method sets txundereru to value 0"]
impl crate::Resettable for TXUNDERERU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
