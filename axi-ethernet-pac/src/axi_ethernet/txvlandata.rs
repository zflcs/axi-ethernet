#[doc = "Register `txvlandata` reader"]
pub type R = crate::R<TXVLANDATA_SPEC>;
#[doc = "Register `txvlandata` writer"]
pub type W = crate::W<TXVLANDATA_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXVLANDATA_SPEC> {
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
#[doc = "TX VLAN data table address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txvlandata::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txvlandata::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXVLANDATA_SPEC;
impl crate::RegisterSpec for TXVLANDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txvlandata::R`](R) reader structure"]
impl crate::Readable for TXVLANDATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txvlandata::W`](W) writer structure"]
impl crate::Writable for TXVLANDATA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets txvlandata to value 0"]
impl crate::Resettable for TXVLANDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
