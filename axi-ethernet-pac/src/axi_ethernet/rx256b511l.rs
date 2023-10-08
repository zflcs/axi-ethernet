#[doc = "Register `rx256b511l` reader"]
pub type R = crate::R<RX256B511L_SPEC>;
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
#[doc = "Count of 256-511 bytes frames received, LSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx256b511l::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX256B511L_SPEC;
impl crate::RegisterSpec for RX256B511L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx256b511l::R`](R) reader structure"]
impl crate::Readable for RX256B511L_SPEC {}
#[doc = "`reset()` method sets rx256b511l to value 0"]
impl crate::Resettable for RX256B511L_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
