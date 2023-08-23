#[doc = "Register `tpid1` reader"]
pub struct R(crate::R<TPID1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TPID1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TPID1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TPID1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tpid1` writer"]
pub struct W(crate::W<TPID1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TPID1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<TPID1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TPID1_SPEC>) -> Self {
        W(writer)
    }
}
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
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == VALUE2_A::RESET
    }
}
#[doc = "Field `value2` writer - "]
pub type VALUE2_W<'a, const O: u8> = crate::FieldWriter<'a, TPID1_SPEC, 16, O, VALUE2_A>;
impl<'a, const O: u8> VALUE2_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
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
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == VALUE3_A::RESET
    }
}
#[doc = "Field `value3` writer - "]
pub type VALUE3_W<'a, const O: u8> = crate::FieldWriter<'a, TPID1_SPEC, 16, O, VALUE3_A>;
impl<'a, const O: u8> VALUE3_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
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
    pub fn value2(&mut self) -> VALUE2_W<0> {
        VALUE2_W::new(self)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn value3(&mut self) -> VALUE3_W<16> {
        VALUE3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VLAN TPID TEMAC Word 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tpid1](index.html) module"]
pub struct TPID1_SPEC;
impl crate::RegisterSpec for TPID1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tpid1::R](R) reader structure"]
impl crate::Readable for TPID1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tpid1::W](W) writer structure"]
impl crate::Writable for TPID1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tpid1 to value 0"]
impl crate::Resettable for TPID1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
