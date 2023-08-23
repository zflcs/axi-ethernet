#[doc = "Register `ttc` reader"]
pub struct R(crate::R<TTC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TTC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TTC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TTC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ttc` writer"]
pub struct W(crate::W<TTC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TTC_SPEC>;
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
impl From<crate::W<TTC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TTC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICFE` reader - "]
pub type ICFE_R = crate::BitReader<ICFE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICFE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<ICFE_A> for bool {
    #[inline(always)]
    fn from(variant: ICFE_A) -> Self {
        variant as u8 != 0
    }
}
impl ICFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICFE_A {
        match self.bits {
            false => ICFE_A::DISABLE,
            true => ICFE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ICFE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ICFE_A::ENABLE
    }
}
#[doc = "Field `ICFE` writer - "]
pub type ICFE_W<'a, const O: u8> = crate::BitWriter<'a, TTC_SPEC, O, ICFE_A>;
impl<'a, const O: u8> ICFE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ICFE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ICFE_A::ENABLE)
    }
}
#[doc = "Field `IFG` reader - "]
pub type IFG_R = crate::BitReader<IFG_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IFG_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<IFG_A> for bool {
    #[inline(always)]
    fn from(variant: IFG_A) -> Self {
        variant as u8 != 0
    }
}
impl IFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IFG_A {
        match self.bits {
            false => IFG_A::DISABLE,
            true => IFG_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IFG_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IFG_A::ENABLE
    }
}
#[doc = "Field `IFG` writer - "]
pub type IFG_W<'a, const O: u8> = crate::BitWriter<'a, TTC_SPEC, O, IFG_A>;
impl<'a, const O: u8> IFG_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IFG_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IFG_A::ENABLE)
    }
}
#[doc = "Field `HD` reader - "]
pub type HD_R = crate::BitReader<HD_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HD_A {
    #[doc = "0: `0`"]
    FULL_DUPLEX = 0,
    #[doc = "1: `1`"]
    HALF_DUPLEX = 1,
}
impl From<HD_A> for bool {
    #[inline(always)]
    fn from(variant: HD_A) -> Self {
        variant as u8 != 0
    }
}
impl HD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HD_A {
        match self.bits {
            false => HD_A::FULL_DUPLEX,
            true => HD_A::HALF_DUPLEX,
        }
    }
    #[doc = "Checks if the value of the field is `FULL_DUPLEX`"]
    #[inline(always)]
    pub fn is_full_duplex(&self) -> bool {
        *self == HD_A::FULL_DUPLEX
    }
    #[doc = "Checks if the value of the field is `HALF_DUPLEX`"]
    #[inline(always)]
    pub fn is_half_duplex(&self) -> bool {
        *self == HD_A::HALF_DUPLEX
    }
}
#[doc = "Field `HD` writer - "]
pub type HD_W<'a, const O: u8> = crate::BitWriter<'a, TTC_SPEC, O, HD_A>;
impl<'a, const O: u8> HD_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn full_duplex(self) -> &'a mut W {
        self.variant(HD_A::FULL_DUPLEX)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn half_duplex(self) -> &'a mut W {
        self.variant(HD_A::HALF_DUPLEX)
    }
}
#[doc = "Field `VLAN` reader - "]
pub type VLAN_R = crate::BitReader<VLAN_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VLAN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<VLAN_A> for bool {
    #[inline(always)]
    fn from(variant: VLAN_A) -> Self {
        variant as u8 != 0
    }
}
impl VLAN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VLAN_A {
        match self.bits {
            false => VLAN_A::DISABLE,
            true => VLAN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == VLAN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == VLAN_A::ENABLE
    }
}
#[doc = "Field `VLAN` writer - "]
pub type VLAN_W<'a, const O: u8> = crate::BitWriter<'a, TTC_SPEC, O, VLAN_A>;
impl<'a, const O: u8> VLAN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(VLAN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(VLAN_A::ENABLE)
    }
}
#[doc = "Field `TX` reader - "]
pub type TX_R = crate::BitReader<TX_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TX_A> for bool {
    #[inline(always)]
    fn from(variant: TX_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_A {
        match self.bits {
            false => TX_A::DISABLE,
            true => TX_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TX_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TX_A::ENABLE
    }
}
#[doc = "Field `TX` writer - "]
pub type TX_W<'a, const O: u8> = crate::BitWriter<'a, TTC_SPEC, O, TX_A>;
impl<'a, const O: u8> TX_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TX_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TX_A::ENABLE)
    }
}
#[doc = "Field `FCS` reader - "]
pub type FCS_R = crate::BitReader<FCS_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FCS_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<FCS_A> for bool {
    #[inline(always)]
    fn from(variant: FCS_A) -> Self {
        variant as u8 != 0
    }
}
impl FCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCS_A {
        match self.bits {
            false => FCS_A::DISABLE,
            true => FCS_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FCS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FCS_A::ENABLE
    }
}
#[doc = "Field `FCS` writer - "]
pub type FCS_W<'a, const O: u8> = crate::BitWriter<'a, TTC_SPEC, O, FCS_A>;
impl<'a, const O: u8> FCS_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FCS_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FCS_A::ENABLE)
    }
}
#[doc = "Field `JUM` reader - "]
pub type JUM_R = crate::BitReader<JUM_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JUM_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<JUM_A> for bool {
    #[inline(always)]
    fn from(variant: JUM_A) -> Self {
        variant as u8 != 0
    }
}
impl JUM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JUM_A {
        match self.bits {
            false => JUM_A::DISABLE,
            true => JUM_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == JUM_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == JUM_A::ENABLE
    }
}
#[doc = "Field `JUM` writer - "]
pub type JUM_W<'a, const O: u8> = crate::BitWriter<'a, TTC_SPEC, O, JUM_A>;
impl<'a, const O: u8> JUM_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(JUM_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(JUM_A::ENABLE)
    }
}
#[doc = "Field `RST` reader - "]
pub type RST_R = crate::BitReader<RST_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RST_A {
    #[doc = "0: `0`"]
    NO_RESET = 0,
    #[doc = "1: `1`"]
    INIT_RESET = 1,
}
impl From<RST_A> for bool {
    #[inline(always)]
    fn from(variant: RST_A) -> Self {
        variant as u8 != 0
    }
}
impl RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RST_A {
        match self.bits {
            false => RST_A::NO_RESET,
            true => RST_A::INIT_RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NO_RESET`"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == RST_A::NO_RESET
    }
    #[doc = "Checks if the value of the field is `INIT_RESET`"]
    #[inline(always)]
    pub fn is_init_reset(&self) -> bool {
        *self == RST_A::INIT_RESET
    }
}
#[doc = "Field `RST` writer - "]
pub type RST_W<'a, const O: u8> = crate::BitWriter<'a, TTC_SPEC, O, RST_A>;
impl<'a, const O: u8> RST_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut W {
        self.variant(RST_A::NO_RESET)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn init_reset(self) -> &'a mut W {
        self.variant(RST_A::INIT_RESET)
    }
}
impl R {
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn icfe(&self) -> ICFE_R {
        ICFE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn ifg(&self) -> IFG_R {
        IFG_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn hd(&self) -> HD_R {
        HD_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn vlan(&self) -> VLAN_R {
        VLAN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn fcs(&self) -> FCS_R {
        FCS_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn jum(&self) -> JUM_R {
        JUM_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn icfe(&mut self) -> ICFE_W<22> {
        ICFE_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn ifg(&mut self) -> IFG_W<25> {
        IFG_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn hd(&mut self) -> HD_W<26> {
        HD_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn vlan(&mut self) -> VLAN_W<27> {
        VLAN_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn tx(&mut self) -> TX_W<28> {
        TX_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn fcs(&mut self) -> FCS_W<29> {
        FCS_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn jum(&mut self) -> JUM_W<30> {
        JUM_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn rst(&mut self) -> RST_W<31> {
        RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TEMAC Transmitter Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ttc](index.html) module"]
pub struct TTC_SPEC;
impl crate::RegisterSpec for TTC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ttc::R](R) reader structure"]
impl crate::Readable for TTC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ttc::W](W) writer structure"]
impl crate::Writable for TTC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ttc to value 0"]
impl crate::Resettable for TTC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
