#[doc = "Register `tpid0` reader"]
pub type R = crate::R<TPID0_SPEC>;
#[doc = "Register `tpid0` writer"]
pub type W = crate::W<TPID0_SPEC>;
#[doc = "Field `value0` reader - "]
pub type VALUE0_R = crate::FieldReader<VALUE0_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum VALUE0_A {
    #[doc = "0: `0`"]
    RESET = 0,
}
impl From<VALUE0_A> for u16 {
    #[inline(always)]
    fn from(variant: VALUE0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VALUE0_A {
    type Ux = u16;
}
impl VALUE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<VALUE0_A> {
        match self.bits {
            0 => Some(VALUE0_A::RESET),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == VALUE0_A::RESET
    }
}
#[doc = "Field `value0` writer - "]
pub type VALUE0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, VALUE0_A>;
impl<'a, REG, const O: u8> VALUE0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(VALUE0_A::RESET)
    }
}
#[doc = "Field `value1` reader - "]
pub type VALUE1_R = crate::FieldReader<VALUE1_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum VALUE1_A {
    #[doc = "0: `0`"]
    RESET = 0,
}
impl From<VALUE1_A> for u16 {
    #[inline(always)]
    fn from(variant: VALUE1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VALUE1_A {
    type Ux = u16;
}
impl VALUE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<VALUE1_A> {
        match self.bits {
            0 => Some(VALUE1_A::RESET),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == VALUE1_A::RESET
    }
}
#[doc = "Field `value1` writer - "]
pub type VALUE1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, VALUE1_A>;
impl<'a, REG, const O: u8> VALUE1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(VALUE1_A::RESET)
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn value0(&self) -> VALUE0_R {
        VALUE0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn value1(&self) -> VALUE1_R {
        VALUE1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn value0(&mut self) -> VALUE0_W<TPID0_SPEC, 0> {
        VALUE0_W::new(self)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn value1(&mut self) -> VALUE1_W<TPID0_SPEC, 16> {
        VALUE1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "VLAN TPID TEMAC Word 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tpid0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tpid0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TPID0_SPEC;
impl crate::RegisterSpec for TPID0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tpid0::R`](R) reader structure"]
impl crate::Readable for TPID0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tpid0::W`](W) writer structure"]
impl crate::Writable for TPID0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tpid0 to value 0"]
impl crate::Resettable for TPID0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
