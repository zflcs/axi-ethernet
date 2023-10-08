#[doc = "Register `mdiomwd` reader"]
pub type R = crate::R<MDIOMWD_SPEC>;
#[doc = "Register `mdiomwd` writer"]
pub type W = crate::W<MDIOMWD_SPEC>;
#[doc = "Field `DATA` reader - "]
pub type DATA_R = crate::FieldReader<u16>;
#[doc = "Field `DATA` writer - "]
pub type DATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<MDIOMWD_SPEC, 0> {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MDIO Write Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdiomwd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdiomwd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDIOMWD_SPEC;
impl crate::RegisterSpec for MDIOMWD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdiomwd::R`](R) reader structure"]
impl crate::Readable for MDIOMWD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mdiomwd::W`](W) writer structure"]
impl crate::Writable for MDIOMWD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets mdiomwd to value 0"]
impl crate::Resettable for MDIOMWD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
