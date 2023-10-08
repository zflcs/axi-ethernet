#[doc = "Register `uaw0` reader"]
pub type R = crate::R<UAW0_SPEC>;
#[doc = "Register `uaw0` writer"]
pub type W = crate::W<UAW0_SPEC>;
#[doc = "Field `address` reader - "]
pub type ADDRESS_R = crate::FieldReader<u32>;
#[doc = "Field `address` writer - "]
pub type ADDRESS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn address(&self) -> ADDRESS_R {
        ADDRESS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn address(&mut self) -> ADDRESS_W<UAW0_SPEC, 0> {
        ADDRESS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Unicast Address Word 0 (UAW0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uaw0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uaw0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UAW0_SPEC;
impl crate::RegisterSpec for UAW0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uaw0::R`](R) reader structure"]
impl crate::Readable for UAW0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uaw0::W`](W) writer structure"]
impl crate::Writable for UAW0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets uaw0 to value 0"]
impl crate::Resettable for UAW0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
