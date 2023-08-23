#[doc = "Register `raf` reader"]
pub struct R(crate::R<RAF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `raf` writer"]
pub struct W(crate::W<RAF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAF_SPEC>;
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
impl From<crate::W<RAF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `McstRej` reader - "]
pub type MCST_REJ_R = crate::BitReader<MCST_REJ_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCST_REJ_A {
    #[doc = "0: `0`"]
    ACCEPT = 0,
    #[doc = "1: `1`"]
    REJECT = 1,
}
impl From<MCST_REJ_A> for bool {
    #[inline(always)]
    fn from(variant: MCST_REJ_A) -> Self {
        variant as u8 != 0
    }
}
impl MCST_REJ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCST_REJ_A {
        match self.bits {
            false => MCST_REJ_A::ACCEPT,
            true => MCST_REJ_A::REJECT,
        }
    }
    #[doc = "Checks if the value of the field is `ACCEPT`"]
    #[inline(always)]
    pub fn is_accept(&self) -> bool {
        *self == MCST_REJ_A::ACCEPT
    }
    #[doc = "Checks if the value of the field is `REJECT`"]
    #[inline(always)]
    pub fn is_reject(&self) -> bool {
        *self == MCST_REJ_A::REJECT
    }
}
#[doc = "Field `McstRej` writer - "]
pub type MCST_REJ_W<'a, const O: u8> = crate::BitWriter<'a, RAF_SPEC, O, MCST_REJ_A>;
impl<'a, const O: u8> MCST_REJ_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn accept(self) -> &'a mut W {
        self.variant(MCST_REJ_A::ACCEPT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn reject(self) -> &'a mut W {
        self.variant(MCST_REJ_A::REJECT)
    }
}
#[doc = "Field `BcstRej` reader - "]
pub type BCST_REJ_R = crate::BitReader<BCST_REJ_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BCST_REJ_A {
    #[doc = "0: `0`"]
    ACCEPT = 0,
    #[doc = "1: `1`"]
    REJECT = 1,
}
impl From<BCST_REJ_A> for bool {
    #[inline(always)]
    fn from(variant: BCST_REJ_A) -> Self {
        variant as u8 != 0
    }
}
impl BCST_REJ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCST_REJ_A {
        match self.bits {
            false => BCST_REJ_A::ACCEPT,
            true => BCST_REJ_A::REJECT,
        }
    }
    #[doc = "Checks if the value of the field is `ACCEPT`"]
    #[inline(always)]
    pub fn is_accept(&self) -> bool {
        *self == BCST_REJ_A::ACCEPT
    }
    #[doc = "Checks if the value of the field is `REJECT`"]
    #[inline(always)]
    pub fn is_reject(&self) -> bool {
        *self == BCST_REJ_A::REJECT
    }
}
#[doc = "Field `BcstRej` writer - "]
pub type BCST_REJ_W<'a, const O: u8> = crate::BitWriter<'a, RAF_SPEC, O, BCST_REJ_A>;
impl<'a, const O: u8> BCST_REJ_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn accept(self) -> &'a mut W {
        self.variant(BCST_REJ_A::ACCEPT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn reject(self) -> &'a mut W {
        self.variant(BCST_REJ_A::REJECT)
    }
}
#[doc = "Field `TxVTagMode` reader - "]
pub type TX_VTAG_MODE_R = crate::FieldReader<TX_VTAG_MODE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TX_VTAG_MODE_A {
    #[doc = "0: `0`"]
    NO_VLAN = 0,
    #[doc = "1: `1`"]
    ONE_VLAN_FROM_ALL = 1,
    #[doc = "2: `10`"]
    ONE_VLAN_FROM_ALL_HAVE = 2,
    #[doc = "3: `11`"]
    ONE_VLAN_FROM_SELECT = 3,
}
impl From<TX_VTAG_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_VTAG_MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TX_VTAG_MODE_A {
    type Ux = u8;
}
impl TX_VTAG_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_VTAG_MODE_A {
        match self.bits {
            0 => TX_VTAG_MODE_A::NO_VLAN,
            1 => TX_VTAG_MODE_A::ONE_VLAN_FROM_ALL,
            2 => TX_VTAG_MODE_A::ONE_VLAN_FROM_ALL_HAVE,
            3 => TX_VTAG_MODE_A::ONE_VLAN_FROM_SELECT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_VLAN`"]
    #[inline(always)]
    pub fn is_no_vlan(&self) -> bool {
        *self == TX_VTAG_MODE_A::NO_VLAN
    }
    #[doc = "Checks if the value of the field is `ONE_VLAN_FROM_ALL`"]
    #[inline(always)]
    pub fn is_one_vlan_from_all(&self) -> bool {
        *self == TX_VTAG_MODE_A::ONE_VLAN_FROM_ALL
    }
    #[doc = "Checks if the value of the field is `ONE_VLAN_FROM_ALL_HAVE`"]
    #[inline(always)]
    pub fn is_one_vlan_from_all_have(&self) -> bool {
        *self == TX_VTAG_MODE_A::ONE_VLAN_FROM_ALL_HAVE
    }
    #[doc = "Checks if the value of the field is `ONE_VLAN_FROM_SELECT`"]
    #[inline(always)]
    pub fn is_one_vlan_from_select(&self) -> bool {
        *self == TX_VTAG_MODE_A::ONE_VLAN_FROM_SELECT
    }
}
#[doc = "Field `TxVTagMode` writer - "]
pub type TX_VTAG_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, RAF_SPEC, 2, O, TX_VTAG_MODE_A>;
impl<'a, const O: u8> TX_VTAG_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_vlan(self) -> &'a mut W {
        self.variant(TX_VTAG_MODE_A::NO_VLAN)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn one_vlan_from_all(self) -> &'a mut W {
        self.variant(TX_VTAG_MODE_A::ONE_VLAN_FROM_ALL)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn one_vlan_from_all_have(self) -> &'a mut W {
        self.variant(TX_VTAG_MODE_A::ONE_VLAN_FROM_ALL_HAVE)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn one_vlan_from_select(self) -> &'a mut W {
        self.variant(TX_VTAG_MODE_A::ONE_VLAN_FROM_SELECT)
    }
}
#[doc = "Field `RxVTagMode` reader - "]
pub type RX_VTAG_MODE_R = crate::FieldReader<RX_VTAG_MODE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RX_VTAG_MODE_A {
    #[doc = "0: `0`"]
    NO_VLAN = 0,
    #[doc = "1: `1`"]
    ONE_VLAN_FROM_ALL = 1,
    #[doc = "2: `10`"]
    ONE_VLAN_FROM_ALL_HAVE = 2,
    #[doc = "3: `11`"]
    ONE_VLAN_FROM_SELECT = 3,
}
impl From<RX_VTAG_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: RX_VTAG_MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RX_VTAG_MODE_A {
    type Ux = u8;
}
impl RX_VTAG_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_VTAG_MODE_A {
        match self.bits {
            0 => RX_VTAG_MODE_A::NO_VLAN,
            1 => RX_VTAG_MODE_A::ONE_VLAN_FROM_ALL,
            2 => RX_VTAG_MODE_A::ONE_VLAN_FROM_ALL_HAVE,
            3 => RX_VTAG_MODE_A::ONE_VLAN_FROM_SELECT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_VLAN`"]
    #[inline(always)]
    pub fn is_no_vlan(&self) -> bool {
        *self == RX_VTAG_MODE_A::NO_VLAN
    }
    #[doc = "Checks if the value of the field is `ONE_VLAN_FROM_ALL`"]
    #[inline(always)]
    pub fn is_one_vlan_from_all(&self) -> bool {
        *self == RX_VTAG_MODE_A::ONE_VLAN_FROM_ALL
    }
    #[doc = "Checks if the value of the field is `ONE_VLAN_FROM_ALL_HAVE`"]
    #[inline(always)]
    pub fn is_one_vlan_from_all_have(&self) -> bool {
        *self == RX_VTAG_MODE_A::ONE_VLAN_FROM_ALL_HAVE
    }
    #[doc = "Checks if the value of the field is `ONE_VLAN_FROM_SELECT`"]
    #[inline(always)]
    pub fn is_one_vlan_from_select(&self) -> bool {
        *self == RX_VTAG_MODE_A::ONE_VLAN_FROM_SELECT
    }
}
#[doc = "Field `RxVTagMode` writer - "]
pub type RX_VTAG_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, RAF_SPEC, 2, O, RX_VTAG_MODE_A>;
impl<'a, const O: u8> RX_VTAG_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_vlan(self) -> &'a mut W {
        self.variant(RX_VTAG_MODE_A::NO_VLAN)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn one_vlan_from_all(self) -> &'a mut W {
        self.variant(RX_VTAG_MODE_A::ONE_VLAN_FROM_ALL)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn one_vlan_from_all_have(self) -> &'a mut W {
        self.variant(RX_VTAG_MODE_A::ONE_VLAN_FROM_ALL_HAVE)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn one_vlan_from_select(self) -> &'a mut W {
        self.variant(RX_VTAG_MODE_A::ONE_VLAN_FROM_SELECT)
    }
}
#[doc = "Field `TxVStrpMode` reader - "]
pub type TX_VSTRP_MODE_R = crate::FieldReader<TX_VSTRP_MODE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TX_VSTRP_MODE_A {
    #[doc = "0: `0`"]
    NO_VLAN = 0,
    #[doc = "1: `1`"]
    ONE_VLAN_FROM_ALL = 1,
    #[doc = "3: `11`"]
    ONE_VLAN_FROM_SELECT = 3,
}
impl From<TX_VSTRP_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_VSTRP_MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TX_VSTRP_MODE_A {
    type Ux = u8;
}
impl TX_VSTRP_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TX_VSTRP_MODE_A> {
        match self.bits {
            0 => Some(TX_VSTRP_MODE_A::NO_VLAN),
            1 => Some(TX_VSTRP_MODE_A::ONE_VLAN_FROM_ALL),
            3 => Some(TX_VSTRP_MODE_A::ONE_VLAN_FROM_SELECT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NO_VLAN`"]
    #[inline(always)]
    pub fn is_no_vlan(&self) -> bool {
        *self == TX_VSTRP_MODE_A::NO_VLAN
    }
    #[doc = "Checks if the value of the field is `ONE_VLAN_FROM_ALL`"]
    #[inline(always)]
    pub fn is_one_vlan_from_all(&self) -> bool {
        *self == TX_VSTRP_MODE_A::ONE_VLAN_FROM_ALL
    }
    #[doc = "Checks if the value of the field is `ONE_VLAN_FROM_SELECT`"]
    #[inline(always)]
    pub fn is_one_vlan_from_select(&self) -> bool {
        *self == TX_VSTRP_MODE_A::ONE_VLAN_FROM_SELECT
    }
}
#[doc = "Field `TxVStrpMode` writer - "]
pub type TX_VSTRP_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, RAF_SPEC, 2, O, TX_VSTRP_MODE_A>;
impl<'a, const O: u8> TX_VSTRP_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_vlan(self) -> &'a mut W {
        self.variant(TX_VSTRP_MODE_A::NO_VLAN)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn one_vlan_from_all(self) -> &'a mut W {
        self.variant(TX_VSTRP_MODE_A::ONE_VLAN_FROM_ALL)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn one_vlan_from_select(self) -> &'a mut W {
        self.variant(TX_VSTRP_MODE_A::ONE_VLAN_FROM_SELECT)
    }
}
#[doc = "Field `RxVStrpMode` reader - "]
pub type RX_VSTRP_MODE_R = crate::FieldReader<RX_VSTRP_MODE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RX_VSTRP_MODE_A {
    #[doc = "0: `0`"]
    NO_VLAN = 0,
    #[doc = "1: `1`"]
    ONE_VLAN_FROM_ALL = 1,
    #[doc = "3: `11`"]
    ONE_VLAN_FROM_SELECT = 3,
}
impl From<RX_VSTRP_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: RX_VSTRP_MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RX_VSTRP_MODE_A {
    type Ux = u8;
}
impl RX_VSTRP_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RX_VSTRP_MODE_A> {
        match self.bits {
            0 => Some(RX_VSTRP_MODE_A::NO_VLAN),
            1 => Some(RX_VSTRP_MODE_A::ONE_VLAN_FROM_ALL),
            3 => Some(RX_VSTRP_MODE_A::ONE_VLAN_FROM_SELECT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NO_VLAN`"]
    #[inline(always)]
    pub fn is_no_vlan(&self) -> bool {
        *self == RX_VSTRP_MODE_A::NO_VLAN
    }
    #[doc = "Checks if the value of the field is `ONE_VLAN_FROM_ALL`"]
    #[inline(always)]
    pub fn is_one_vlan_from_all(&self) -> bool {
        *self == RX_VSTRP_MODE_A::ONE_VLAN_FROM_ALL
    }
    #[doc = "Checks if the value of the field is `ONE_VLAN_FROM_SELECT`"]
    #[inline(always)]
    pub fn is_one_vlan_from_select(&self) -> bool {
        *self == RX_VSTRP_MODE_A::ONE_VLAN_FROM_SELECT
    }
}
#[doc = "Field `RxVStrpMode` writer - "]
pub type RX_VSTRP_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, RAF_SPEC, 2, O, RX_VSTRP_MODE_A>;
impl<'a, const O: u8> RX_VSTRP_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_vlan(self) -> &'a mut W {
        self.variant(RX_VSTRP_MODE_A::NO_VLAN)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn one_vlan_from_all(self) -> &'a mut W {
        self.variant(RX_VSTRP_MODE_A::ONE_VLAN_FROM_ALL)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn one_vlan_from_select(self) -> &'a mut W {
        self.variant(RX_VSTRP_MODE_A::ONE_VLAN_FROM_SELECT)
    }
}
#[doc = "Field `NewFncEnbl` reader - "]
pub type NEW_FNC_ENBL_R = crate::BitReader<NEW_FNC_ENBL_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NEW_FNC_ENBL_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<NEW_FNC_ENBL_A> for bool {
    #[inline(always)]
    fn from(variant: NEW_FNC_ENBL_A) -> Self {
        variant as u8 != 0
    }
}
impl NEW_FNC_ENBL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NEW_FNC_ENBL_A {
        match self.bits {
            false => NEW_FNC_ENBL_A::DISABLE,
            true => NEW_FNC_ENBL_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == NEW_FNC_ENBL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == NEW_FNC_ENBL_A::ENABLE
    }
}
#[doc = "Field `NewFncEnbl` writer - "]
pub type NEW_FNC_ENBL_W<'a, const O: u8> = crate::BitWriter<'a, RAF_SPEC, O, NEW_FNC_ENBL_A>;
impl<'a, const O: u8> NEW_FNC_ENBL_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(NEW_FNC_ENBL_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(NEW_FNC_ENBL_A::ENABLE)
    }
}
#[doc = "Field `EMultiFltrEnbl` reader - "]
pub type EMULTI_FLTR_ENBL_R = crate::BitReader<EMULTI_FLTR_ENBL_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EMULTI_FLTR_ENBL_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<EMULTI_FLTR_ENBL_A> for bool {
    #[inline(always)]
    fn from(variant: EMULTI_FLTR_ENBL_A) -> Self {
        variant as u8 != 0
    }
}
impl EMULTI_FLTR_ENBL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMULTI_FLTR_ENBL_A {
        match self.bits {
            false => EMULTI_FLTR_ENBL_A::DISABLE,
            true => EMULTI_FLTR_ENBL_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EMULTI_FLTR_ENBL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EMULTI_FLTR_ENBL_A::ENABLE
    }
}
#[doc = "Field `EMultiFltrEnbl` writer - "]
pub type EMULTI_FLTR_ENBL_W<'a, const O: u8> =
    crate::BitWriter<'a, RAF_SPEC, O, EMULTI_FLTR_ENBL_A>;
impl<'a, const O: u8> EMULTI_FLTR_ENBL_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EMULTI_FLTR_ENBL_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EMULTI_FLTR_ENBL_A::ENABLE)
    }
}
#[doc = "Field `statsrst` reader - "]
pub type STATSRST_R = crate::BitReader<STATSRST_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STATSRST_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<STATSRST_A> for bool {
    #[inline(always)]
    fn from(variant: STATSRST_A) -> Self {
        variant as u8 != 0
    }
}
impl STATSRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STATSRST_A {
        match self.bits {
            false => STATSRST_A::DISABLE,
            true => STATSRST_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STATSRST_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STATSRST_A::ENABLE
    }
}
#[doc = "Field `statsrst` writer - "]
pub type STATSRST_W<'a, const O: u8> = crate::BitWriter<'a, RAF_SPEC, O, STATSRST_A>;
impl<'a, const O: u8> STATSRST_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(STATSRST_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(STATSRST_A::ENABLE)
    }
}
#[doc = "Field `RxBadFrmEn` reader - "]
pub type RX_BAD_FRM_EN_R = crate::BitReader<RX_BAD_FRM_EN_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_BAD_FRM_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RX_BAD_FRM_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RX_BAD_FRM_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_BAD_FRM_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_BAD_FRM_EN_A {
        match self.bits {
            false => RX_BAD_FRM_EN_A::DISABLE,
            true => RX_BAD_FRM_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RX_BAD_FRM_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RX_BAD_FRM_EN_A::ENABLE
    }
}
#[doc = "Field `RxBadFrmEn` writer - "]
pub type RX_BAD_FRM_EN_W<'a, const O: u8> = crate::BitWriter<'a, RAF_SPEC, O, RX_BAD_FRM_EN_A>;
impl<'a, const O: u8> RX_BAD_FRM_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RX_BAD_FRM_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RX_BAD_FRM_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn mcst_rej(&self) -> MCST_REJ_R {
        MCST_REJ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn bcst_rej(&self) -> BCST_REJ_R {
        BCST_REJ_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn tx_vtag_mode(&self) -> TX_VTAG_MODE_R {
        TX_VTAG_MODE_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn rx_vtag_mode(&self) -> RX_VTAG_MODE_R {
        RX_VTAG_MODE_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 7:8"]
    #[inline(always)]
    pub fn tx_vstrp_mode(&self) -> TX_VSTRP_MODE_R {
        TX_VSTRP_MODE_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn rx_vstrp_mode(&self) -> RX_VSTRP_MODE_R {
        RX_VSTRP_MODE_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn new_fnc_enbl(&self) -> NEW_FNC_ENBL_R {
        NEW_FNC_ENBL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn emulti_fltr_enbl(&self) -> EMULTI_FLTR_ENBL_R {
        EMULTI_FLTR_ENBL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn statsrst(&self) -> STATSRST_R {
        STATSRST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn rx_bad_frm_en(&self) -> RX_BAD_FRM_EN_R {
        RX_BAD_FRM_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn mcst_rej(&mut self) -> MCST_REJ_W<1> {
        MCST_REJ_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn bcst_rej(&mut self) -> BCST_REJ_W<2> {
        BCST_REJ_W::new(self)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    #[must_use]
    pub fn tx_vtag_mode(&mut self) -> TX_VTAG_MODE_W<3> {
        TX_VTAG_MODE_W::new(self)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    #[must_use]
    pub fn rx_vtag_mode(&mut self) -> RX_VTAG_MODE_W<5> {
        RX_VTAG_MODE_W::new(self)
    }
    #[doc = "Bits 7:8"]
    #[inline(always)]
    #[must_use]
    pub fn tx_vstrp_mode(&mut self) -> TX_VSTRP_MODE_W<7> {
        TX_VSTRP_MODE_W::new(self)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    #[must_use]
    pub fn rx_vstrp_mode(&mut self) -> RX_VSTRP_MODE_W<9> {
        RX_VSTRP_MODE_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn new_fnc_enbl(&mut self) -> NEW_FNC_ENBL_W<11> {
        NEW_FNC_ENBL_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn emulti_fltr_enbl(&mut self) -> EMULTI_FLTR_ENBL_W<12> {
        EMULTI_FLTR_ENBL_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn statsrst(&mut self) -> STATSRST_W<13> {
        STATSRST_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn rx_bad_frm_en(&mut self) -> RX_BAD_FRM_EN_W<14> {
        RX_BAD_FRM_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset and Address Filter TEMAC\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [raf](index.html) module"]
pub struct RAF_SPEC;
impl crate::RegisterSpec for RAF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [raf::R](R) reader structure"]
impl crate::Readable for RAF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [raf::W](W) writer structure"]
impl crate::Writable for RAF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets raf to value 0"]
impl crate::Resettable for RAF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
