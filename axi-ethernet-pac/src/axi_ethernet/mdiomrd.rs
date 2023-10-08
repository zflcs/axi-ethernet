#[doc = "Register `mdiomrd` reader"]
pub type R = crate::R<MDIOMRD_SPEC>;
#[doc = "Register `mdiomrd` writer"]
pub type W = crate::W<MDIOMRD_SPEC>;
#[doc = "Field `DATA` reader - "]
pub type DATA_R = crate::FieldReader<u16>;
#[doc = "Field `DATA` writer - "]
pub type DATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `READY` reader - "]
pub type READY_R = crate::BitReader;
#[doc = "Field `READY` writer - "]
pub type READY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<MDIOMRD_SPEC, 0> {
        DATA_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn ready(&mut self) -> READY_W<MDIOMRD_SPEC, 16> {
        READY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MDIO Read Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdiomrd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdiomrd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDIOMRD_SPEC;
impl crate::RegisterSpec for MDIOMRD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdiomrd::R`](R) reader structure"]
impl crate::Readable for MDIOMRD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mdiomrd::W`](W) writer structure"]
impl crate::Writable for MDIOMRD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets mdiomrd to value 0"]
impl crate::Resettable for MDIOMRD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
