#[doc = "Register `rxuopfl` reader"]
pub type R = crate::R<RXUOPFL_SPEC>;
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
#[doc = "Count of control frames received with unsupported opcode, LSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxuopfl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXUOPFL_SPEC;
impl crate::RegisterSpec for RXUOPFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxuopfl::R`](R) reader structure"]
impl crate::Readable for RXUOPFL_SPEC {}
#[doc = "`reset()` method sets rxuopfl to value 0"]
impl crate::Resettable for RXUOPFL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
