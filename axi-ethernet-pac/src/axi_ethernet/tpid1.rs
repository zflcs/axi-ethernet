#[doc = "Register `tpid1` reader"]
pub type R = crate::R<TPID1_SPEC>;
#[doc = "Register `tpid1` writer"]
pub type W = crate::W<TPID1_SPEC>;
#[doc = "Field `value2` reader - "]
pub type VALUE2_R = crate::FieldReader<VALUE2_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum VALUE2_A {
    #[doc = "0: `0`"]
    RESET = 0,
}
impl From<VALUE2_A> for u16 {
    #[inline(always)]
    fn from(variant: VALUE2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VALUE2_A {
    type Ux = u16;
}
impl VALUE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<VALUE2_A> {
        match self.bits {
            0 => Some(VALUE2_A::RESET),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == VALUE2_A::RESET
    }
}
#[doc = "Field `value2` writer - "]
pub type VALUE2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, VALUE2_A>;
impl<'a, REG, const O: u8> VALUE2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(VALUE2_A::RESET)
    }
}
#[doc = "Field `value3` reader - "]
pub type VALUE3_R = crate::FieldReader<VALUE3_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum VALUE3_A {
    #[doc = "0: `0`"]
    RESET = 0,
}
impl From<VALUE3_A> for u16 {
    #[inline(always)]
    fn from(variant: VALUE3_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VALUE3_A {
    type Ux = u16;
}
impl VALUE3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<VALUE3_A> {
        match self.bits {
            0 => Some(VALUE3_A::RESET),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == VALUE3_A::RESET
    }
}
#[doc = "Field `value3` writer - "]
pub type VALUE3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, VALUE3_A>;
impl<'a, REG, const O: u8> VALUE3_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(VALUE3_A::RESET)
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn value2(&self) -> VALUE2_R {
        VALUE2_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn value3(&self) -> VALUE3_R {
        VALUE3_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn value2(&mut self) -> VALUE2_W<TPID1_SPEC, 0> {
        VALUE2_W::new(self)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn value3(&mut self) -> VALUE3_W<TPID1_SPEC, 16> {
        VALUE3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "VLAN TPID TEMAC Word 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tpid1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tpid1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TPID1_SPEC;
impl crate::RegisterSpec for TPID1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tpid1::R`](R) reader structure"]
impl crate::Readable for TPID1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tpid1::W`](W) writer structure"]
impl crate::Writable for TPID1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tpid1 to value 0"]
impl crate::Resettable for TPID1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
