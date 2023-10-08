#[doc = "Register `uawl` reader"]
pub type R = crate::R<UAWL_SPEC>;
#[doc = "Register `uawl` writer"]
pub type W = crate::W<UAWL_SPEC>;
#[doc = "Field `UnicastAddr` reader - "]
pub type UNICAST_ADDR_R = crate::FieldReader<UNICAST_ADDR_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum UNICAST_ADDR_A {
    #[doc = "0: `0`"]
    RESET = 0,
}
impl From<UNICAST_ADDR_A> for u32 {
    #[inline(always)]
    fn from(variant: UNICAST_ADDR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UNICAST_ADDR_A {
    type Ux = u32;
}
impl UNICAST_ADDR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<UNICAST_ADDR_A> {
        match self.bits {
            0 => Some(UNICAST_ADDR_A::RESET),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == UNICAST_ADDR_A::RESET
    }
}
#[doc = "Field `UnicastAddr` writer - "]
pub type UNICAST_ADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, UNICAST_ADDR_A>;
impl<'a, REG, const O: u8> UNICAST_ADDR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(UNICAST_ADDR_A::RESET)
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn unicast_addr(&self) -> UNICAST_ADDR_R {
        UNICAST_ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn unicast_addr(&mut self) -> UNICAST_ADDR_W<UAWL_SPEC, 0> {
        UNICAST_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Unicast Address Word Lower TEMAC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uawl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uawl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UAWL_SPEC;
impl crate::RegisterSpec for UAWL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uawl::R`](R) reader structure"]
impl crate::Readable for UAWL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uawl::W`](W) writer structure"]
impl crate::Writable for UAWL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets uawl to value 0"]
impl crate::Resettable for UAWL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
