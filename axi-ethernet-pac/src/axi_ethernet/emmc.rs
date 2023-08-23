#[doc = "Register `emmc` reader"]
pub struct R(crate::R<EMMC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMMC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMMC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMMC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `emmc` writer"]
pub struct W(crate::W<EMMC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMMC_SPEC>;
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
impl From<crate::W<EMMC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMMC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rx16bit` reader - "]
pub type RX16BIT_R = crate::BitReader<RX16BIT_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX16BIT_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RX16BIT_A> for bool {
    #[inline(always)]
    fn from(variant: RX16BIT_A) -> Self {
        variant as u8 != 0
    }
}
impl RX16BIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX16BIT_A {
        match self.bits {
            false => RX16BIT_A::DISABLE,
            true => RX16BIT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RX16BIT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RX16BIT_A::ENABLE
    }
}
#[doc = "Field `rx16bit` writer - "]
pub type RX16BIT_W<'a, const O: u8> = crate::BitWriter<'a, EMMC_SPEC, O, RX16BIT_A>;
impl<'a, const O: u8> RX16BIT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RX16BIT_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RX16BIT_A::ENABLE)
    }
}
#[doc = "Field `tx16bit` reader - "]
pub type TX16BIT_R = crate::BitReader<TX16BIT_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX16BIT_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TX16BIT_A> for bool {
    #[inline(always)]
    fn from(variant: TX16BIT_A) -> Self {
        variant as u8 != 0
    }
}
impl TX16BIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX16BIT_A {
        match self.bits {
            false => TX16BIT_A::DISABLE,
            true => TX16BIT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TX16BIT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TX16BIT_A::ENABLE
    }
}
#[doc = "Field `tx16bit` writer - "]
pub type TX16BIT_W<'a, const O: u8> = crate::BitWriter<'a, EMMC_SPEC, O, TX16BIT_A>;
impl<'a, const O: u8> TX16BIT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TX16BIT_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TX16BIT_A::ENABLE)
    }
}
#[doc = "Field `host` reader - "]
pub type HOST_R = crate::BitReader<HOST_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HOST_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<HOST_A> for bool {
    #[inline(always)]
    fn from(variant: HOST_A) -> Self {
        variant as u8 != 0
    }
}
impl HOST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HOST_A {
        match self.bits {
            false => HOST_A::DISABLE,
            true => HOST_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HOST_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HOST_A::ENABLE
    }
}
#[doc = "Field `host` writer - "]
pub type HOST_W<'a, const O: u8> = crate::BitWriter<'a, EMMC_SPEC, O, HOST_A>;
impl<'a, const O: u8> HOST_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(HOST_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(HOST_A::ENABLE)
    }
}
#[doc = "Field `gpcs` reader - "]
pub type GPCS_R = crate::BitReader<GPCS_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPCS_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<GPCS_A> for bool {
    #[inline(always)]
    fn from(variant: GPCS_A) -> Self {
        variant as u8 != 0
    }
}
impl GPCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPCS_A {
        match self.bits {
            false => GPCS_A::DISABLE,
            true => GPCS_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GPCS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GPCS_A::ENABLE
    }
}
#[doc = "Field `gpcs` writer - "]
pub type GPCS_W<'a, const O: u8> = crate::BitWriter<'a, EMMC_SPEC, O, GPCS_A>;
impl<'a, const O: u8> GPCS_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPCS_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPCS_A::ENABLE)
    }
}
#[doc = "Field `sgmii` reader - "]
pub type SGMII_R = crate::BitReader<SGMII_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SGMII_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<SGMII_A> for bool {
    #[inline(always)]
    fn from(variant: SGMII_A) -> Self {
        variant as u8 != 0
    }
}
impl SGMII_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SGMII_A {
        match self.bits {
            false => SGMII_A::DISABLE,
            true => SGMII_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SGMII_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SGMII_A::ENABLE
    }
}
#[doc = "Field `sgmii` writer - "]
pub type SGMII_W<'a, const O: u8> = crate::BitWriter<'a, EMMC_SPEC, O, SGMII_A>;
impl<'a, const O: u8> SGMII_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SGMII_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SGMII_A::ENABLE)
    }
}
#[doc = "Field `rgmii` reader - "]
pub type RGMII_R = crate::BitReader<RGMII_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RGMII_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RGMII_A> for bool {
    #[inline(always)]
    fn from(variant: RGMII_A) -> Self {
        variant as u8 != 0
    }
}
impl RGMII_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RGMII_A {
        match self.bits {
            false => RGMII_A::DISABLE,
            true => RGMII_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RGMII_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RGMII_A::ENABLE
    }
}
#[doc = "Field `rgmii` writer - "]
pub type RGMII_W<'a, const O: u8> = crate::BitWriter<'a, EMMC_SPEC, O, RGMII_A>;
impl<'a, const O: u8> RGMII_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RGMII_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RGMII_A::ENABLE)
    }
}
#[doc = "Field `MSC` reader - "]
pub type MSC_R = crate::FieldReader<MSC_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MSC_A {
    #[doc = "0: `0`"]
    _10M = 0,
    #[doc = "1: `1`"]
    _100M = 1,
    #[doc = "2: `10`"]
    _1000M = 2,
}
impl From<MSC_A> for u8 {
    #[inline(always)]
    fn from(variant: MSC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MSC_A {
    type Ux = u8;
}
impl MSC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MSC_A> {
        match self.bits {
            0 => Some(MSC_A::_10M),
            1 => Some(MSC_A::_100M),
            2 => Some(MSC_A::_1000M),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_10M`"]
    #[inline(always)]
    pub fn is_10m(&self) -> bool {
        *self == MSC_A::_10M
    }
    #[doc = "Checks if the value of the field is `_100M`"]
    #[inline(always)]
    pub fn is_100m(&self) -> bool {
        *self == MSC_A::_100M
    }
    #[doc = "Checks if the value of the field is `_1000M`"]
    #[inline(always)]
    pub fn is_1000m(&self) -> bool {
        *self == MSC_A::_1000M
    }
}
#[doc = "Field `MSC` writer - "]
pub type MSC_W<'a, const O: u8> = crate::FieldWriter<'a, EMMC_SPEC, 2, O, MSC_A>;
impl<'a, const O: u8> MSC_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn _10m(self) -> &'a mut W {
        self.variant(MSC_A::_10M)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn _100m(self) -> &'a mut W {
        self.variant(MSC_A::_100M)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn _1000m(self) -> &'a mut W {
        self.variant(MSC_A::_1000M)
    }
}
impl R {
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn rx16bit(&self) -> RX16BIT_R {
        RX16BIT_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn tx16bit(&self) -> TX16BIT_R {
        TX16BIT_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn host(&self) -> HOST_R {
        HOST_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn gpcs(&self) -> GPCS_R {
        GPCS_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn sgmii(&self) -> SGMII_R {
        SGMII_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rgmii(&self) -> RGMII_R {
        RGMII_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn msc(&self) -> MSC_R {
        MSC_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn rx16bit(&mut self) -> RX16BIT_W<24> {
        RX16BIT_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn tx16bit(&mut self) -> TX16BIT_W<25> {
        TX16BIT_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn host(&mut self) -> HOST_W<26> {
        HOST_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn gpcs(&mut self) -> GPCS_W<27> {
        GPCS_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn sgmii(&mut self) -> SGMII_W<28> {
        SGMII_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn rgmii(&mut self) -> RGMII_W<29> {
        RGMII_W::new(self)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    #[must_use]
    pub fn msc(&mut self) -> MSC_W<30> {
        MSC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EMAC mode configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emmc](index.html) module"]
pub struct EMMC_SPEC;
impl crate::RegisterSpec for EMMC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emmc::R](R) reader structure"]
impl crate::Readable for EMMC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emmc::W](W) writer structure"]
impl crate::Writable for EMMC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets emmc to value 0"]
impl crate::Resettable for EMMC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
