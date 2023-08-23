#[doc = "Register `tfcc` reader"]
pub struct R(crate::R<TFCC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TFCC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TFCC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TFCC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tfcc` writer"]
pub struct W(crate::W<TFCC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TFCC_SPEC>;
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
impl From<crate::W<TFCC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TFCC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXP0PE` reader - "]
pub type RXP0PE_R = crate::BitReader<RXP0PE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXP0PE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RXP0PE_A> for bool {
    #[inline(always)]
    fn from(variant: RXP0PE_A) -> Self {
        variant as u8 != 0
    }
}
impl RXP0PE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXP0PE_A {
        match self.bits {
            false => RXP0PE_A::DISABLE,
            true => RXP0PE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RXP0PE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RXP0PE_A::ENABLE
    }
}
#[doc = "Field `RXP0PE` writer - "]
pub type RXP0PE_W<'a, const O: u8> = crate::BitWriter<'a, TFCC_SPEC, O, RXP0PE_A>;
impl<'a, const O: u8> RXP0PE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RXP0PE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RXP0PE_A::ENABLE)
    }
}
#[doc = "Field `RXP1PE` reader - "]
pub type RXP1PE_R = crate::BitReader<RXP1PE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXP1PE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RXP1PE_A> for bool {
    #[inline(always)]
    fn from(variant: RXP1PE_A) -> Self {
        variant as u8 != 0
    }
}
impl RXP1PE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXP1PE_A {
        match self.bits {
            false => RXP1PE_A::DISABLE,
            true => RXP1PE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RXP1PE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RXP1PE_A::ENABLE
    }
}
#[doc = "Field `RXP1PE` writer - "]
pub type RXP1PE_W<'a, const O: u8> = crate::BitWriter<'a, TFCC_SPEC, O, RXP1PE_A>;
impl<'a, const O: u8> RXP1PE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RXP1PE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RXP1PE_A::ENABLE)
    }
}
#[doc = "Field `RXP2PE` reader - "]
pub type RXP2PE_R = crate::BitReader<RXP2PE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXP2PE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RXP2PE_A> for bool {
    #[inline(always)]
    fn from(variant: RXP2PE_A) -> Self {
        variant as u8 != 0
    }
}
impl RXP2PE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXP2PE_A {
        match self.bits {
            false => RXP2PE_A::DISABLE,
            true => RXP2PE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RXP2PE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RXP2PE_A::ENABLE
    }
}
#[doc = "Field `RXP2PE` writer - "]
pub type RXP2PE_W<'a, const O: u8> = crate::BitWriter<'a, TFCC_SPEC, O, RXP2PE_A>;
impl<'a, const O: u8> RXP2PE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RXP2PE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RXP2PE_A::ENABLE)
    }
}
#[doc = "Field `RXP3PE` reader - "]
pub type RXP3PE_R = crate::BitReader<RXP3PE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXP3PE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RXP3PE_A> for bool {
    #[inline(always)]
    fn from(variant: RXP3PE_A) -> Self {
        variant as u8 != 0
    }
}
impl RXP3PE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXP3PE_A {
        match self.bits {
            false => RXP3PE_A::DISABLE,
            true => RXP3PE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RXP3PE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RXP3PE_A::ENABLE
    }
}
#[doc = "Field `RXP3PE` writer - "]
pub type RXP3PE_W<'a, const O: u8> = crate::BitWriter<'a, TFCC_SPEC, O, RXP3PE_A>;
impl<'a, const O: u8> RXP3PE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RXP3PE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RXP3PE_A::ENABLE)
    }
}
#[doc = "Field `RXP4PE` reader - "]
pub type RXP4PE_R = crate::BitReader<RXP4PE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXP4PE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RXP4PE_A> for bool {
    #[inline(always)]
    fn from(variant: RXP4PE_A) -> Self {
        variant as u8 != 0
    }
}
impl RXP4PE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXP4PE_A {
        match self.bits {
            false => RXP4PE_A::DISABLE,
            true => RXP4PE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RXP4PE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RXP4PE_A::ENABLE
    }
}
#[doc = "Field `RXP4PE` writer - "]
pub type RXP4PE_W<'a, const O: u8> = crate::BitWriter<'a, TFCC_SPEC, O, RXP4PE_A>;
impl<'a, const O: u8> RXP4PE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RXP4PE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RXP4PE_A::ENABLE)
    }
}
#[doc = "Field `RXP5PE` reader - "]
pub type RXP5PE_R = crate::BitReader<RXP5PE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXP5PE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RXP5PE_A> for bool {
    #[inline(always)]
    fn from(variant: RXP5PE_A) -> Self {
        variant as u8 != 0
    }
}
impl RXP5PE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXP5PE_A {
        match self.bits {
            false => RXP5PE_A::DISABLE,
            true => RXP5PE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RXP5PE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RXP5PE_A::ENABLE
    }
}
#[doc = "Field `RXP5PE` writer - "]
pub type RXP5PE_W<'a, const O: u8> = crate::BitWriter<'a, TFCC_SPEC, O, RXP5PE_A>;
impl<'a, const O: u8> RXP5PE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RXP5PE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RXP5PE_A::ENABLE)
    }
}
#[doc = "Field `RXP6PE` reader - "]
pub type RXP6PE_R = crate::BitReader<RXP6PE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXP6PE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RXP6PE_A> for bool {
    #[inline(always)]
    fn from(variant: RXP6PE_A) -> Self {
        variant as u8 != 0
    }
}
impl RXP6PE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXP6PE_A {
        match self.bits {
            false => RXP6PE_A::DISABLE,
            true => RXP6PE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RXP6PE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RXP6PE_A::ENABLE
    }
}
#[doc = "Field `RXP6PE` writer - "]
pub type RXP6PE_W<'a, const O: u8> = crate::BitWriter<'a, TFCC_SPEC, O, RXP6PE_A>;
impl<'a, const O: u8> RXP6PE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RXP6PE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RXP6PE_A::ENABLE)
    }
}
#[doc = "Field `RXP7PE` reader - "]
pub type RXP7PE_R = crate::BitReader<RXP7PE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXP7PE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RXP7PE_A> for bool {
    #[inline(always)]
    fn from(variant: RXP7PE_A) -> Self {
        variant as u8 != 0
    }
}
impl RXP7PE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXP7PE_A {
        match self.bits {
            false => RXP7PE_A::DISABLE,
            true => RXP7PE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RXP7PE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RXP7PE_A::ENABLE
    }
}
#[doc = "Field `RXP7PE` writer - "]
pub type RXP7PE_W<'a, const O: u8> = crate::BitWriter<'a, TFCC_SPEC, O, RXP7PE_A>;
impl<'a, const O: u8> RXP7PE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RXP7PE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RXP7PE_A::ENABLE)
    }
}
#[doc = "Field `TXP0PE` reader - "]
pub type TXP0PE_R = crate::BitReader<TXP0PE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXP0PE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TXP0PE_A> for bool {
    #[inline(always)]
    fn from(variant: TXP0PE_A) -> Self {
        variant as u8 != 0
    }
}
impl TXP0PE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXP0PE_A {
        match self.bits {
            false => TXP0PE_A::DISABLE,
            true => TXP0PE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TXP0PE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TXP0PE_A::ENABLE
    }
}
#[doc = "Field `TXP0PE` writer - "]
pub type TXP0PE_W<'a, const O: u8> = crate::BitWriter<'a, TFCC_SPEC, O, TXP0PE_A>;
impl<'a, const O: u8> TXP0PE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TXP0PE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TXP0PE_A::ENABLE)
    }
}
#[doc = "Field `TXP1PE` reader - "]
pub type TXP1PE_R = crate::BitReader<TXP1PE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXP1PE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TXP1PE_A> for bool {
    #[inline(always)]
    fn from(variant: TXP1PE_A) -> Self {
        variant as u8 != 0
    }
}
impl TXP1PE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXP1PE_A {
        match self.bits {
            false => TXP1PE_A::DISABLE,
            true => TXP1PE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TXP1PE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TXP1PE_A::ENABLE
    }
}
#[doc = "Field `TXP1PE` writer - "]
pub type TXP1PE_W<'a, const O: u8> = crate::BitWriter<'a, TFCC_SPEC, O, TXP1PE_A>;
impl<'a, const O: u8> TXP1PE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TXP1PE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TXP1PE_A::ENABLE)
    }
}
#[doc = "Field `TXP2PE` reader - "]
pub type TXP2PE_R = crate::BitReader<TXP2PE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXP2PE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TXP2PE_A> for bool {
    #[inline(always)]
    fn from(variant: TXP2PE_A) -> Self {
        variant as u8 != 0
    }
}
impl TXP2PE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXP2PE_A {
        match self.bits {
            false => TXP2PE_A::DISABLE,
            true => TXP2PE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TXP2PE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TXP2PE_A::ENABLE
    }
}
#[doc = "Field `TXP2PE` writer - "]
pub type TXP2PE_W<'a, const O: u8> = crate::BitWriter<'a, TFCC_SPEC, O, TXP2PE_A>;
impl<'a, const O: u8> TXP2PE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TXP2PE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TXP2PE_A::ENABLE)
    }
}
#[doc = "Field `TXP3PE` reader - "]
pub type TXP3PE_R = crate::BitReader<TXP3PE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXP3PE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TXP3PE_A> for bool {
    #[inline(always)]
    fn from(variant: TXP3PE_A) -> Self {
        variant as u8 != 0
    }
}
impl TXP3PE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXP3PE_A {
        match self.bits {
            false => TXP3PE_A::DISABLE,
            true => TXP3PE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TXP3PE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TXP3PE_A::ENABLE
    }
}
#[doc = "Field `TXP3PE` writer - "]
pub type TXP3PE_W<'a, const O: u8> = crate::BitWriter<'a, TFCC_SPEC, O, TXP3PE_A>;
impl<'a, const O: u8> TXP3PE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TXP3PE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TXP3PE_A::ENABLE)
    }
}
#[doc = "Field `TXP4PE` reader - "]
pub type TXP4PE_R = crate::BitReader<TXP4PE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXP4PE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TXP4PE_A> for bool {
    #[inline(always)]
    fn from(variant: TXP4PE_A) -> Self {
        variant as u8 != 0
    }
}
impl TXP4PE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXP4PE_A {
        match self.bits {
            false => TXP4PE_A::DISABLE,
            true => TXP4PE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TXP4PE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TXP4PE_A::ENABLE
    }
}
#[doc = "Field `TXP4PE` writer - "]
pub type TXP4PE_W<'a, const O: u8> = crate::BitWriter<'a, TFCC_SPEC, O, TXP4PE_A>;
impl<'a, const O: u8> TXP4PE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TXP4PE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TXP4PE_A::ENABLE)
    }
}
#[doc = "Field `TXP5PE` reader - "]
pub type TXP5PE_R = crate::BitReader<TXP5PE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXP5PE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TXP5PE_A> for bool {
    #[inline(always)]
    fn from(variant: TXP5PE_A) -> Self {
        variant as u8 != 0
    }
}
impl TXP5PE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXP5PE_A {
        match self.bits {
            false => TXP5PE_A::DISABLE,
            true => TXP5PE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TXP5PE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TXP5PE_A::ENABLE
    }
}
#[doc = "Field `TXP5PE` writer - "]
pub type TXP5PE_W<'a, const O: u8> = crate::BitWriter<'a, TFCC_SPEC, O, TXP5PE_A>;
impl<'a, const O: u8> TXP5PE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TXP5PE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TXP5PE_A::ENABLE)
    }
}
#[doc = "Field `TXP6PE` reader - "]
pub type TXP6PE_R = crate::BitReader<TXP6PE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXP6PE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TXP6PE_A> for bool {
    #[inline(always)]
    fn from(variant: TXP6PE_A) -> Self {
        variant as u8 != 0
    }
}
impl TXP6PE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXP6PE_A {
        match self.bits {
            false => TXP6PE_A::DISABLE,
            true => TXP6PE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TXP6PE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TXP6PE_A::ENABLE
    }
}
#[doc = "Field `TXP6PE` writer - "]
pub type TXP6PE_W<'a, const O: u8> = crate::BitWriter<'a, TFCC_SPEC, O, TXP6PE_A>;
impl<'a, const O: u8> TXP6PE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TXP6PE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TXP6PE_A::ENABLE)
    }
}
#[doc = "Field `TXP7PE` reader - "]
pub type TXP7PE_R = crate::BitReader<TXP7PE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXP7PE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TXP7PE_A> for bool {
    #[inline(always)]
    fn from(variant: TXP7PE_A) -> Self {
        variant as u8 != 0
    }
}
impl TXP7PE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXP7PE_A {
        match self.bits {
            false => TXP7PE_A::DISABLE,
            true => TXP7PE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TXP7PE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TXP7PE_A::ENABLE
    }
}
#[doc = "Field `TXP7PE` writer - "]
pub type TXP7PE_W<'a, const O: u8> = crate::BitWriter<'a, TFCC_SPEC, O, TXP7PE_A>;
impl<'a, const O: u8> TXP7PE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TXP7PE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TXP7PE_A::ENABLE)
    }
}
#[doc = "Field `TXAXON` reader - "]
pub type TXAXON_R = crate::BitReader<TXAXON_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXAXON_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TXAXON_A> for bool {
    #[inline(always)]
    fn from(variant: TXAXON_A) -> Self {
        variant as u8 != 0
    }
}
impl TXAXON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXAXON_A {
        match self.bits {
            false => TXAXON_A::DISABLE,
            true => TXAXON_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TXAXON_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TXAXON_A::ENABLE
    }
}
#[doc = "Field `TXAXON` writer - "]
pub type TXAXON_W<'a, const O: u8> = crate::BitWriter<'a, TFCC_SPEC, O, TXAXON_A>;
impl<'a, const O: u8> TXAXON_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TXAXON_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TXAXON_A::ENABLE)
    }
}
#[doc = "Field `RXPPFCE` reader - "]
pub type RXPPFCE_R = crate::BitReader<RXPPFCE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXPPFCE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RXPPFCE_A> for bool {
    #[inline(always)]
    fn from(variant: RXPPFCE_A) -> Self {
        variant as u8 != 0
    }
}
impl RXPPFCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXPPFCE_A {
        match self.bits {
            false => RXPPFCE_A::DISABLE,
            true => RXPPFCE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RXPPFCE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RXPPFCE_A::ENABLE
    }
}
#[doc = "Field `RXPPFCE` writer - "]
pub type RXPPFCE_W<'a, const O: u8> = crate::BitWriter<'a, TFCC_SPEC, O, RXPPFCE_A>;
impl<'a, const O: u8> RXPPFCE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RXPPFCE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RXPPFCE_A::ENABLE)
    }
}
#[doc = "Field `TXPPFCE` reader - "]
pub type TXPPFCE_R = crate::BitReader<TXPPFCE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXPPFCE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TXPPFCE_A> for bool {
    #[inline(always)]
    fn from(variant: TXPPFCE_A) -> Self {
        variant as u8 != 0
    }
}
impl TXPPFCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXPPFCE_A {
        match self.bits {
            false => TXPPFCE_A::DISABLE,
            true => TXPPFCE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TXPPFCE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TXPPFCE_A::ENABLE
    }
}
#[doc = "Field `TXPPFCE` writer - "]
pub type TXPPFCE_W<'a, const O: u8> = crate::BitWriter<'a, TFCC_SPEC, O, TXPPFCE_A>;
impl<'a, const O: u8> TXPPFCE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TXPPFCE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TXPPFCE_A::ENABLE)
    }
}
#[doc = "Field `RXFCE` reader - "]
pub type RXFCE_R = crate::BitReader<RXFCE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXFCE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RXFCE_A> for bool {
    #[inline(always)]
    fn from(variant: RXFCE_A) -> Self {
        variant as u8 != 0
    }
}
impl RXFCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXFCE_A {
        match self.bits {
            false => RXFCE_A::DISABLE,
            true => RXFCE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RXFCE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RXFCE_A::ENABLE
    }
}
#[doc = "Field `RXFCE` writer - "]
pub type RXFCE_W<'a, const O: u8> = crate::BitWriter<'a, TFCC_SPEC, O, RXFCE_A>;
impl<'a, const O: u8> RXFCE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RXFCE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RXFCE_A::ENABLE)
    }
}
#[doc = "Field `TXFCE` reader - "]
pub type TXFCE_R = crate::BitReader<TXFCE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXFCE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TXFCE_A> for bool {
    #[inline(always)]
    fn from(variant: TXFCE_A) -> Self {
        variant as u8 != 0
    }
}
impl TXFCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXFCE_A {
        match self.bits {
            false => TXFCE_A::DISABLE,
            true => TXFCE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TXFCE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TXFCE_A::ENABLE
    }
}
#[doc = "Field `TXFCE` writer - "]
pub type TXFCE_W<'a, const O: u8> = crate::BitWriter<'a, TFCC_SPEC, O, TXFCE_A>;
impl<'a, const O: u8> TXFCE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TXFCE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TXFCE_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rxp0pe(&self) -> RXP0PE_R {
        RXP0PE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rxp1pe(&self) -> RXP1PE_R {
        RXP1PE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rxp2pe(&self) -> RXP2PE_R {
        RXP2PE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rxp3pe(&self) -> RXP3PE_R {
        RXP3PE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rxp4pe(&self) -> RXP4PE_R {
        RXP4PE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rxp5pe(&self) -> RXP5PE_R {
        RXP5PE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rxp6pe(&self) -> RXP6PE_R {
        RXP6PE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rxp7pe(&self) -> RXP7PE_R {
        RXP7PE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn txp0pe(&self) -> TXP0PE_R {
        TXP0PE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn txp1pe(&self) -> TXP1PE_R {
        TXP1PE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn txp2pe(&self) -> TXP2PE_R {
        TXP2PE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn txp3pe(&self) -> TXP3PE_R {
        TXP3PE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn txp4pe(&self) -> TXP4PE_R {
        TXP4PE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn txp5pe(&self) -> TXP5PE_R {
        TXP5PE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn txp6pe(&self) -> TXP6PE_R {
        TXP6PE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn txp7pe(&self) -> TXP7PE_R {
        TXP7PE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn txaxon(&self) -> TXAXON_R {
        TXAXON_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn rxppfce(&self) -> RXPPFCE_R {
        RXPPFCE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn txppfce(&self) -> TXPPFCE_R {
        TXPPFCE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rxfce(&self) -> RXFCE_R {
        RXFCE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn txfce(&self) -> TXFCE_R {
        TXFCE_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn rxp0pe(&mut self) -> RXP0PE_W<0> {
        RXP0PE_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn rxp1pe(&mut self) -> RXP1PE_W<1> {
        RXP1PE_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn rxp2pe(&mut self) -> RXP2PE_W<2> {
        RXP2PE_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn rxp3pe(&mut self) -> RXP3PE_W<3> {
        RXP3PE_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn rxp4pe(&mut self) -> RXP4PE_W<4> {
        RXP4PE_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn rxp5pe(&mut self) -> RXP5PE_W<5> {
        RXP5PE_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn rxp6pe(&mut self) -> RXP6PE_W<6> {
        RXP6PE_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn rxp7pe(&mut self) -> RXP7PE_W<7> {
        RXP7PE_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn txp0pe(&mut self) -> TXP0PE_W<8> {
        TXP0PE_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn txp1pe(&mut self) -> TXP1PE_W<9> {
        TXP1PE_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn txp2pe(&mut self) -> TXP2PE_W<10> {
        TXP2PE_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn txp3pe(&mut self) -> TXP3PE_W<11> {
        TXP3PE_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn txp4pe(&mut self) -> TXP4PE_W<12> {
        TXP4PE_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn txp5pe(&mut self) -> TXP5PE_W<13> {
        TXP5PE_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn txp6pe(&mut self) -> TXP6PE_W<14> {
        TXP6PE_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn txp7pe(&mut self) -> TXP7PE_W<15> {
        TXP7PE_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn txaxon(&mut self) -> TXAXON_W<20> {
        TXAXON_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn rxppfce(&mut self) -> RXPPFCE_W<25> {
        RXPPFCE_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn txppfce(&mut self) -> TXPPFCE_W<26> {
        TXPPFCE_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn rxfce(&mut self) -> RXFCE_W<29> {
        RXFCE_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn txfce(&mut self) -> TXFCE_W<30> {
        TXFCE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TEMAC Flow Control Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tfcc](index.html) module"]
pub struct TFCC_SPEC;
impl crate::RegisterSpec for TFCC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tfcc::R](R) reader structure"]
impl crate::Readable for TFCC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tfcc::W](W) writer structure"]
impl crate::Writable for TFCC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tfcc to value 0"]
impl crate::Resettable for TFCC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
