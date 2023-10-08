#[doc = "Register `tpf` reader"]
pub type R = crate::R<TPF_SPEC>;
#[doc = "Register `tpf` writer"]
pub type W = crate::W<TPF_SPEC>;
#[doc = "Field `TPFV` reader - "]
pub type TPFV_R = crate::FieldReader<TPFV_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum TPFV_A {
    #[doc = "0: `0`"]
    RESET = 0,
}
impl From<TPFV_A> for u16 {
    #[inline(always)]
    fn from(variant: TPFV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TPFV_A {
    type Ux = u16;
}
impl TPFV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TPFV_A> {
        match self.bits {
            0 => Some(TPFV_A::RESET),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == TPFV_A::RESET
    }
}
#[doc = "Field `TPFV` writer - "]
pub type TPFV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, TPFV_A>;
impl<'a, REG, const O: u8> TPFV_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(TPFV_A::RESET)
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn tpfv(&self) -> TPFV_R {
        TPFV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn tpfv(&mut self) -> TPFV_W<TPF_SPEC, 0> {
        TPFV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Transmit Pause Frame TEMAC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tpf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tpf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TPF_SPEC;
impl crate::RegisterSpec for TPF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tpf::R`](R) reader structure"]
impl crate::Readable for TPF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tpf::W`](W) writer structure"]
impl crate::Writable for TPF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tpf to value 0"]
impl crate::Resettable for TPF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
