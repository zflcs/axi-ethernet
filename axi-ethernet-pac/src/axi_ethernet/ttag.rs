#[doc = "Register `ttag` reader"]
pub struct R(crate::R<TTAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TTAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TTAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TTAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ttag` writer"]
pub struct W(crate::W<TTAG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TTAG_SPEC>;
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
impl From<crate::W<TTAG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TTAG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `vid` reader - "]
pub type VID_R = crate::FieldReader<VID_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum VID_A {
    #[doc = "0: `0`"]
    RESET = 0,
}
impl From<VID_A> for u16 {
    #[inline(always)]
    fn from(variant: VID_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VID_A {
    type Ux = u16;
}
impl VID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<VID_A> {
        match self.bits {
            0 => Some(VID_A::RESET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == VID_A::RESET
    }
}
#[doc = "Field `vid` writer - "]
pub type VID_W<'a, const O: u8> = crate::FieldWriter<'a, TTAG_SPEC, 12, O, VID_A>;
impl<'a, const O: u8> VID_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(VID_A::RESET)
    }
}
#[doc = "Field `cfi` reader - "]
pub type CFI_R = crate::BitReader<CFI_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFI_A {
    #[doc = "0: `0`"]
    RESET = 0,
}
impl From<CFI_A> for bool {
    #[inline(always)]
    fn from(variant: CFI_A) -> Self {
        variant as u8 != 0
    }
}
impl CFI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CFI_A> {
        match self.bits {
            false => Some(CFI_A::RESET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == CFI_A::RESET
    }
}
#[doc = "Field `cfi` writer - "]
pub type CFI_W<'a, const O: u8> = crate::BitWriter<'a, TTAG_SPEC, O, CFI_A>;
impl<'a, const O: u8> CFI_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CFI_A::RESET)
    }
}
#[doc = "Field `priority` reader - "]
pub type PRIORITY_R = crate::FieldReader<PRIORITY_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRIORITY_A {
    #[doc = "0: `0`"]
    RESET = 0,
}
impl From<PRIORITY_A> for u8 {
    #[inline(always)]
    fn from(variant: PRIORITY_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRIORITY_A {
    type Ux = u8;
}
impl PRIORITY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRIORITY_A> {
        match self.bits {
            0 => Some(PRIORITY_A::RESET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == PRIORITY_A::RESET
    }
}
#[doc = "Field `priority` writer - "]
pub type PRIORITY_W<'a, const O: u8> = crate::FieldWriter<'a, TTAG_SPEC, 3, O, PRIORITY_A>;
impl<'a, const O: u8> PRIORITY_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(PRIORITY_A::RESET)
    }
}
#[doc = "Field `tpid` reader - "]
pub type TPID_R = crate::FieldReader<TPID_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum TPID_A {
    #[doc = "0: `0`"]
    RESET = 0,
}
impl From<TPID_A> for u16 {
    #[inline(always)]
    fn from(variant: TPID_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TPID_A {
    type Ux = u16;
}
impl TPID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TPID_A> {
        match self.bits {
            0 => Some(TPID_A::RESET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == TPID_A::RESET
    }
}
#[doc = "Field `tpid` writer - "]
pub type TPID_W<'a, const O: u8> = crate::FieldWriter<'a, TTAG_SPEC, 16, O, TPID_A>;
impl<'a, const O: u8> TPID_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TPID_A::RESET)
    }
}
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn vid(&self) -> VID_R {
        VID_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cfi(&self) -> CFI_R {
        CFI_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15"]
    #[inline(always)]
    pub fn priority(&self) -> PRIORITY_R {
        PRIORITY_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn tpid(&self) -> TPID_R {
        TPID_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    #[must_use]
    pub fn vid(&mut self) -> VID_W<0> {
        VID_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn cfi(&mut self) -> CFI_W<12> {
        CFI_W::new(self)
    }
    #[doc = "Bits 13:15"]
    #[inline(always)]
    #[must_use]
    pub fn priority(&mut self) -> PRIORITY_W<13> {
        PRIORITY_W::new(self)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn tpid(&mut self) -> TPID_W<16> {
        TPID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit VLAN Tag TEMAC\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ttag](index.html) module"]
pub struct TTAG_SPEC;
impl crate::RegisterSpec for TTAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ttag::R](R) reader structure"]
impl crate::Readable for TTAG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ttag::W](W) writer structure"]
impl crate::Writable for TTAG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ttag to value 0"]
impl crate::Resettable for TTAG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
