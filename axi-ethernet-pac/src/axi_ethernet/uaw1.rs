#[doc = "Register `uaw1` reader"]
pub type R = crate::R<UAW1_SPEC>;
#[doc = "Register `uaw1` writer"]
pub type W = crate::W<UAW1_SPEC>;
#[doc = "Field `address47T32` reader - "]
pub type ADDRESS47T32_R = crate::FieldReader<u16>;
#[doc = "Field `address47T32` writer - "]
pub type ADDRESS47T32_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn address47t32(&self) -> ADDRESS47T32_R {
        ADDRESS47T32_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn address47t32(&mut self) -> ADDRESS47T32_W<UAW1_SPEC, 0> {
        ADDRESS47T32_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Unicast Address Word 1 (UAW0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uaw1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uaw1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UAW1_SPEC;
impl crate::RegisterSpec for UAW1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uaw1::R`](R) reader structure"]
impl crate::Readable for UAW1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uaw1::W`](W) writer structure"]
impl crate::Writable for UAW1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets uaw1 to value 0"]
impl crate::Resettable for UAW1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
