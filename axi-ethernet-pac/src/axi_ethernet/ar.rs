#[doc = "Register `ar` reader"]
pub struct R(crate::R<AR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `10MA` reader - "]
pub type _10MA_R = crate::BitReader<_10MA_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum _10MA_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<_10MA_A> for bool {
    #[inline(always)]
    fn from(variant: _10MA_A) -> Self {
        variant as u8 != 0
    }
}
impl _10MA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> _10MA_A {
        match self.bits {
            false => _10MA_A::DISABLE,
            true => _10MA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == _10MA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == _10MA_A::ENABLE
    }
}
#[doc = "Field `100MA` reader - "]
pub type _100MA_R = crate::BitReader<_100MA_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum _100MA_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<_100MA_A> for bool {
    #[inline(always)]
    fn from(variant: _100MA_A) -> Self {
        variant as u8 != 0
    }
}
impl _100MA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> _100MA_A {
        match self.bits {
            false => _100MA_A::DISABLE,
            true => _100MA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == _100MA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == _100MA_A::ENABLE
    }
}
#[doc = "Field `1000MA` reader - "]
pub type _1000MA_R = crate::BitReader<_1000MA_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum _1000MA_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<_1000MA_A> for bool {
    #[inline(always)]
    fn from(variant: _1000MA_A) -> Self {
        variant as u8 != 0
    }
}
impl _1000MA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> _1000MA_A {
        match self.bits {
            false => _1000MA_A::DISABLE,
            true => _1000MA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == _1000MA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == _1000MA_A::ENABLE
    }
}
#[doc = "Field `2P5A` reader - "]
pub type _2P5A_R = crate::BitReader<_2P5A_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum _2P5A_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<_2P5A_A> for bool {
    #[inline(always)]
    fn from(variant: _2P5A_A) -> Self {
        variant as u8 != 0
    }
}
impl _2P5A_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> _2P5A_A {
        match self.bits {
            false => _2P5A_A::DISABLE,
            true => _2P5A_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == _2P5A_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == _2P5A_A::ENABLE
    }
}
#[doc = "Field `SCA` reader - "]
pub type SCA_R = crate::BitReader<SCA_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCA_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<SCA_A> for bool {
    #[inline(always)]
    fn from(variant: SCA_A) -> Self {
        variant as u8 != 0
    }
}
impl SCA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCA_A {
        match self.bits {
            false => SCA_A::DISABLE,
            true => SCA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SCA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SCA_A::ENABLE
    }
}
#[doc = "Field `HDC` reader - "]
pub type HDC_R = crate::BitReader<HDC_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDC_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<HDC_A> for bool {
    #[inline(always)]
    fn from(variant: HDC_A) -> Self {
        variant as u8 != 0
    }
}
impl HDC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HDC_A {
        match self.bits {
            false => HDC_A::DISABLE,
            true => HDC_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HDC_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HDC_A::ENABLE
    }
}
#[doc = "Field `FFA` reader - "]
pub type FFA_R = crate::BitReader<FFA_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FFA_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<FFA_A> for bool {
    #[inline(always)]
    fn from(variant: FFA_A) -> Self {
        variant as u8 != 0
    }
}
impl FFA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FFA_A {
        match self.bits {
            false => FFA_A::DISABLE,
            true => FFA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FFA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FFA_A::ENABLE
    }
}
#[doc = "Field `PFCS` reader - "]
pub type PFCS_R = crate::BitReader<PFCS_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PFCS_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<PFCS_A> for bool {
    #[inline(always)]
    fn from(variant: PFCS_A) -> Self {
        variant as u8 != 0
    }
}
impl PFCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PFCS_A {
        match self.bits {
            false => PFCS_A::DISABLE,
            true => PFCS_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PFCS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PFCS_A::ENABLE
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn _10ma(&self) -> _10MA_R {
        _10MA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn _100ma(&self) -> _100MA_R {
        _100MA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn _1000ma(&self) -> _1000MA_R {
        _1000MA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn _2p5a(&self) -> _2P5A_R {
        _2P5A_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sca(&self) -> SCA_R {
        SCA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn hdc(&self) -> HDC_R {
        HDC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ffa(&self) -> FFA_R {
        FFA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pfcs(&self) -> PFCS_R {
        PFCS_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "Ability Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ar](index.html) module"]
pub struct AR_SPEC;
impl crate::RegisterSpec for AR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ar::R](R) reader structure"]
impl crate::Readable for AR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ar to value 0"]
impl crate::Resettable for AR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
