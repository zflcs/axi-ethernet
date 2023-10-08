#[doc = "Register `ifgp` reader"]
pub type R = crate::R<IFGP_SPEC>;
#[doc = "Register `ifgp` writer"]
pub type W = crate::W<IFGP_SPEC>;
#[doc = "Field `IFGP0` reader - "]
pub type IFGP0_R = crate::FieldReader<IFGP0_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IFGP0_A {
    #[doc = "0: `0`"]
    RESET = 0,
}
impl From<IFGP0_A> for u8 {
    #[inline(always)]
    fn from(variant: IFGP0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IFGP0_A {
    type Ux = u8;
}
impl IFGP0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IFGP0_A> {
        match self.bits {
            0 => Some(IFGP0_A::RESET),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == IFGP0_A::RESET
    }
}
#[doc = "Field `IFGP0` writer - "]
pub type IFGP0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O, IFGP0_A>;
impl<'a, REG, const O: u8> IFGP0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(IFGP0_A::RESET)
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn ifgp0(&self) -> IFGP0_R {
        IFGP0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn ifgp0(&mut self) -> IFGP0_W<IFGP_SPEC, 0> {
        IFGP0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Transmit Inter Frame Gap Adjustment TEMAC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ifgp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifgp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFGP_SPEC;
impl crate::RegisterSpec for IFGP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ifgp::R`](R) reader structure"]
impl crate::Readable for IFGP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ifgp::W`](W) writer structure"]
impl crate::Writable for IFGP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ifgp to value 0"]
impl crate::Resettable for IFGP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
