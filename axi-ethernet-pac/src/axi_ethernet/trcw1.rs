#[doc = "Register `trcw1` reader"]
pub struct R(crate::R<TRCW1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRCW1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRCW1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRCW1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `trcw1` writer"]
pub struct W(crate::W<TRCW1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRCW1_SPEC>;
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
impl From<crate::W<TRCW1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRCW1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PauseAddr` reader - "]
pub type PAUSE_ADDR_R = crate::FieldReader<PAUSE_ADDR_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum PAUSE_ADDR_A {
    #[doc = "65535: `1111111111111111`"]
    RESET = 65535,
}
impl From<PAUSE_ADDR_A> for u16 {
    #[inline(always)]
    fn from(variant: PAUSE_ADDR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PAUSE_ADDR_A {
    type Ux = u16;
}
impl PAUSE_ADDR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAUSE_ADDR_A> {
        match self.bits {
            65535 => Some(PAUSE_ADDR_A::RESET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == PAUSE_ADDR_A::RESET
    }
}
#[doc = "Field `PauseAddr` writer - "]
pub type PAUSE_ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, TRCW1_SPEC, 16, O, PAUSE_ADDR_A>;
impl<'a, const O: u8> PAUSE_ADDR_W<'a, O> {
    #[doc = "`1111111111111111`"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(PAUSE_ADDR_A::RESET)
    }
}
#[doc = "Field `TsEn` reader - "]
pub type TS_EN_R = crate::BitReader<TS_EN_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TS_EN_A {
    #[doc = "0: `0`"]
    RESET = 0,
}
impl From<TS_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TS_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TS_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TS_EN_A> {
        match self.bits {
            false => Some(TS_EN_A::RESET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == TS_EN_A::RESET
    }
}
#[doc = "Field `TsEn` writer - "]
pub type TS_EN_W<'a, const O: u8> = crate::BitWriter<'a, TRCW1_SPEC, O, TS_EN_A>;
impl<'a, const O: u8> TS_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TS_EN_A::RESET)
    }
}
#[doc = "Field `CL_DIS` reader - "]
pub type CL_DIS_R = crate::BitReader<CL_DIS_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CL_DIS_A {
    #[doc = "0: `0`"]
    RESET = 0,
}
impl From<CL_DIS_A> for bool {
    #[inline(always)]
    fn from(variant: CL_DIS_A) -> Self {
        variant as u8 != 0
    }
}
impl CL_DIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CL_DIS_A> {
        match self.bits {
            false => Some(CL_DIS_A::RESET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == CL_DIS_A::RESET
    }
}
#[doc = "Field `CL_DIS` writer - "]
pub type CL_DIS_W<'a, const O: u8> = crate::BitWriter<'a, TRCW1_SPEC, O, CL_DIS_A>;
impl<'a, const O: u8> CL_DIS_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CL_DIS_A::RESET)
    }
}
#[doc = "Field `LT_DIS` reader - "]
pub type LT_DIS_R = crate::BitReader<LT_DIS_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LT_DIS_A {
    #[doc = "0: `0`"]
    CHECK = 0,
    #[doc = "1: `1`"]
    NOT_CHECK = 1,
}
impl From<LT_DIS_A> for bool {
    #[inline(always)]
    fn from(variant: LT_DIS_A) -> Self {
        variant as u8 != 0
    }
}
impl LT_DIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LT_DIS_A {
        match self.bits {
            false => LT_DIS_A::CHECK,
            true => LT_DIS_A::NOT_CHECK,
        }
    }
    #[doc = "Checks if the value of the field is `CHECK`"]
    #[inline(always)]
    pub fn is_check(&self) -> bool {
        *self == LT_DIS_A::CHECK
    }
    #[doc = "Checks if the value of the field is `NOT_CHECK`"]
    #[inline(always)]
    pub fn is_not_check(&self) -> bool {
        *self == LT_DIS_A::NOT_CHECK
    }
}
#[doc = "Field `LT_DIS` writer - "]
pub type LT_DIS_W<'a, const O: u8> = crate::BitWriter<'a, TRCW1_SPEC, O, LT_DIS_A>;
impl<'a, const O: u8> LT_DIS_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn check(self) -> &'a mut W {
        self.variant(LT_DIS_A::CHECK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn not_check(self) -> &'a mut W {
        self.variant(LT_DIS_A::NOT_CHECK)
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
pub type HD_W<'a, const O: u8> = crate::BitWriter<'a, TRCW1_SPEC, O, HD_A>;
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
pub type VLAN_W<'a, const O: u8> = crate::BitWriter<'a, TRCW1_SPEC, O, VLAN_A>;
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
#[doc = "Field `RX` reader - "]
pub type RX_R = crate::BitReader<RX_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RX_A> for bool {
    #[inline(always)]
    fn from(variant: RX_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_A {
        match self.bits {
            false => RX_A::DISABLE,
            true => RX_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RX_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RX_A::ENABLE
    }
}
#[doc = "Field `RX` writer - "]
pub type RX_W<'a, const O: u8> = crate::BitWriter<'a, TRCW1_SPEC, O, RX_A>;
impl<'a, const O: u8> RX_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RX_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RX_A::ENABLE)
    }
}
#[doc = "Field `FCS` reader - "]
pub type FCS_R = crate::BitReader<FCS_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FCS_A {
    #[doc = "0: `0`"]
    STRIP = 0,
    #[doc = "1: `1`"]
    NO_STRIP = 1,
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
            false => FCS_A::STRIP,
            true => FCS_A::NO_STRIP,
        }
    }
    #[doc = "Checks if the value of the field is `STRIP`"]
    #[inline(always)]
    pub fn is_strip(&self) -> bool {
        *self == FCS_A::STRIP
    }
    #[doc = "Checks if the value of the field is `NO_STRIP`"]
    #[inline(always)]
    pub fn is_no_strip(&self) -> bool {
        *self == FCS_A::NO_STRIP
    }
}
#[doc = "Field `FCS` writer - "]
pub type FCS_W<'a, const O: u8> = crate::BitWriter<'a, TRCW1_SPEC, O, FCS_A>;
impl<'a, const O: u8> FCS_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn strip(self) -> &'a mut W {
        self.variant(FCS_A::STRIP)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn no_strip(self) -> &'a mut W {
        self.variant(FCS_A::NO_STRIP)
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
pub type JUM_W<'a, const O: u8> = crate::BitWriter<'a, TRCW1_SPEC, O, JUM_A>;
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
pub type RST_W<'a, const O: u8> = crate::BitWriter<'a, TRCW1_SPEC, O, RST_A>;
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
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pause_addr(&self) -> PAUSE_ADDR_R {
        PAUSE_ADDR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ts_en(&self) -> TS_EN_R {
        TS_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cl_dis(&self) -> CL_DIS_R {
        CL_DIS_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn lt_dis(&self) -> LT_DIS_R {
        LT_DIS_R::new(((self.bits >> 25) & 1) != 0)
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
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 28) & 1) != 0)
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
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn pause_addr(&mut self) -> PAUSE_ADDR_W<0> {
        PAUSE_ADDR_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn ts_en(&mut self) -> TS_EN_W<22> {
        TS_EN_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn cl_dis(&mut self) -> CL_DIS_W<24> {
        CL_DIS_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn lt_dis(&mut self) -> LT_DIS_W<25> {
        LT_DIS_W::new(self)
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
    pub fn rx(&mut self) -> RX_W<28> {
        RX_W::new(self)
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
#[doc = "TEMAC Receive Configuration Word 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trcw1](index.html) module"]
pub struct TRCW1_SPEC;
impl crate::RegisterSpec for TRCW1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trcw1::R](R) reader structure"]
impl crate::Readable for TRCW1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trcw1::W](W) writer structure"]
impl crate::Writable for TRCW1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets trcw1 to value 0"]
impl crate::Resettable for TRCW1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
