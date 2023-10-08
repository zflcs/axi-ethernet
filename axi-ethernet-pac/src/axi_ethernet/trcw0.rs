#[doc = "Register `trcw0` reader"]
pub type R = crate::R<TRCW0_SPEC>;
#[doc = "Register `trcw0` writer"]
pub type W = crate::W<TRCW0_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TRCW0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "TEMAC Receive Configuration Word 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trcw0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trcw0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCW0_SPEC;
impl crate::RegisterSpec for TRCW0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcw0::R`](R) reader structure"]
impl crate::Readable for TRCW0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trcw0::W`](W) writer structure"]
impl crate::Writable for TRCW0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets trcw0 to value 0"]
impl crate::Resettable for TRCW0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
