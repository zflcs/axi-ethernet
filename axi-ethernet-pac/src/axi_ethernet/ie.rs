#[doc = "Register `ie` reader"]
pub struct R(crate::R<IE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ie` writer"]
pub struct W(crate::W<IE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IE_SPEC>;
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
impl From<crate::W<IE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HardAcsCmplt` reader - "]
pub type HARD_ACS_CMPLT_R = crate::BitReader<HARD_ACS_CMPLT_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HARD_ACS_CMPLT_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<HARD_ACS_CMPLT_A> for bool {
    #[inline(always)]
    fn from(variant: HARD_ACS_CMPLT_A) -> Self {
        variant as u8 != 0
    }
}
impl HARD_ACS_CMPLT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HARD_ACS_CMPLT_A {
        match self.bits {
            false => HARD_ACS_CMPLT_A::DISABLE,
            true => HARD_ACS_CMPLT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HARD_ACS_CMPLT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HARD_ACS_CMPLT_A::ENABLE
    }
}
#[doc = "Field `HardAcsCmplt` writer - "]
pub type HARD_ACS_CMPLT_W<'a, const O: u8> = crate::BitWriter<'a, IE_SPEC, O, HARD_ACS_CMPLT_A>;
impl<'a, const O: u8> HARD_ACS_CMPLT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(HARD_ACS_CMPLT_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(HARD_ACS_CMPLT_A::ENABLE)
    }
}
#[doc = "Field `All` reader - "]
pub type ALL_R = crate::FieldReader;
#[doc = "Field `All` writer - "]
pub type ALL_W<'a, const O: u8> = crate::FieldWriter<'a, IE_SPEC, 6, O>;
#[doc = "Field `AutoNeg` reader - "]
pub type AUTO_NEG_R = crate::BitReader<AUTO_NEG_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUTO_NEG_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<AUTO_NEG_A> for bool {
    #[inline(always)]
    fn from(variant: AUTO_NEG_A) -> Self {
        variant as u8 != 0
    }
}
impl AUTO_NEG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUTO_NEG_A {
        match self.bits {
            false => AUTO_NEG_A::DISABLE,
            true => AUTO_NEG_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == AUTO_NEG_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == AUTO_NEG_A::ENABLE
    }
}
#[doc = "Field `AutoNeg` writer - "]
pub type AUTO_NEG_W<'a, const O: u8> = crate::BitWriter<'a, IE_SPEC, O, AUTO_NEG_A>;
impl<'a, const O: u8> AUTO_NEG_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(AUTO_NEG_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(AUTO_NEG_A::ENABLE)
    }
}
#[doc = "Field `RxCmplt` reader - "]
pub type RX_CMPLT_R = crate::BitReader<RX_CMPLT_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_CMPLT_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RX_CMPLT_A> for bool {
    #[inline(always)]
    fn from(variant: RX_CMPLT_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_CMPLT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_CMPLT_A {
        match self.bits {
            false => RX_CMPLT_A::DISABLE,
            true => RX_CMPLT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RX_CMPLT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RX_CMPLT_A::ENABLE
    }
}
#[doc = "Field `RxCmplt` writer - "]
pub type RX_CMPLT_W<'a, const O: u8> = crate::BitWriter<'a, IE_SPEC, O, RX_CMPLT_A>;
impl<'a, const O: u8> RX_CMPLT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RX_CMPLT_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RX_CMPLT_A::ENABLE)
    }
}
#[doc = "Field `RxRject` reader - "]
pub type RX_RJECT_R = crate::BitReader<RX_RJECT_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_RJECT_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RX_RJECT_A> for bool {
    #[inline(always)]
    fn from(variant: RX_RJECT_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_RJECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_RJECT_A {
        match self.bits {
            false => RX_RJECT_A::DISABLE,
            true => RX_RJECT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RX_RJECT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RX_RJECT_A::ENABLE
    }
}
#[doc = "Field `RxRject` writer - "]
pub type RX_RJECT_W<'a, const O: u8> = crate::BitWriter<'a, IE_SPEC, O, RX_RJECT_A>;
impl<'a, const O: u8> RX_RJECT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RX_RJECT_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RX_RJECT_A::ENABLE)
    }
}
#[doc = "Field `RxFIFOOvr` reader - "]
pub type RX_FIFOOVR_R = crate::BitReader<RX_FIFOOVR_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_FIFOOVR_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RX_FIFOOVR_A> for bool {
    #[inline(always)]
    fn from(variant: RX_FIFOOVR_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_FIFOOVR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_FIFOOVR_A {
        match self.bits {
            false => RX_FIFOOVR_A::DISABLE,
            true => RX_FIFOOVR_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RX_FIFOOVR_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RX_FIFOOVR_A::ENABLE
    }
}
#[doc = "Field `RxFIFOOvr` writer - "]
pub type RX_FIFOOVR_W<'a, const O: u8> = crate::BitWriter<'a, IE_SPEC, O, RX_FIFOOVR_A>;
impl<'a, const O: u8> RX_FIFOOVR_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RX_FIFOOVR_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RX_FIFOOVR_A::ENABLE)
    }
}
#[doc = "Field `TxCmplt` reader - "]
pub type TX_CMPLT_R = crate::BitReader<TX_CMPLT_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_CMPLT_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TX_CMPLT_A> for bool {
    #[inline(always)]
    fn from(variant: TX_CMPLT_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_CMPLT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_CMPLT_A {
        match self.bits {
            false => TX_CMPLT_A::DISABLE,
            true => TX_CMPLT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TX_CMPLT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TX_CMPLT_A::ENABLE
    }
}
#[doc = "Field `TxCmplt` writer - "]
pub type TX_CMPLT_W<'a, const O: u8> = crate::BitWriter<'a, IE_SPEC, O, TX_CMPLT_A>;
impl<'a, const O: u8> TX_CMPLT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TX_CMPLT_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TX_CMPLT_A::ENABLE)
    }
}
#[doc = "Field `RxDcmLock` reader - "]
pub type RX_DCM_LOCK_R = crate::BitReader<RX_DCM_LOCK_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_DCM_LOCK_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RX_DCM_LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: RX_DCM_LOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_DCM_LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_DCM_LOCK_A {
        match self.bits {
            false => RX_DCM_LOCK_A::DISABLE,
            true => RX_DCM_LOCK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RX_DCM_LOCK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RX_DCM_LOCK_A::ENABLE
    }
}
#[doc = "Field `RxDcmLock` writer - "]
pub type RX_DCM_LOCK_W<'a, const O: u8> = crate::BitWriter<'a, IE_SPEC, O, RX_DCM_LOCK_A>;
impl<'a, const O: u8> RX_DCM_LOCK_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RX_DCM_LOCK_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RX_DCM_LOCK_A::ENABLE)
    }
}
#[doc = "Field `MgtRdy` reader - "]
pub type MGT_RDY_R = crate::BitReader<MGT_RDY_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MGT_RDY_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<MGT_RDY_A> for bool {
    #[inline(always)]
    fn from(variant: MGT_RDY_A) -> Self {
        variant as u8 != 0
    }
}
impl MGT_RDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MGT_RDY_A {
        match self.bits {
            false => MGT_RDY_A::DISABLE,
            true => MGT_RDY_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MGT_RDY_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MGT_RDY_A::ENABLE
    }
}
#[doc = "Field `MgtRdy` writer - "]
pub type MGT_RDY_W<'a, const O: u8> = crate::BitWriter<'a, IE_SPEC, O, MGT_RDY_A>;
impl<'a, const O: u8> MGT_RDY_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MGT_RDY_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MGT_RDY_A::ENABLE)
    }
}
#[doc = "Field `PhyRstCmplt` reader - "]
pub type PHY_RST_CMPLT_R = crate::BitReader<PHY_RST_CMPLT_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PHY_RST_CMPLT_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<PHY_RST_CMPLT_A> for bool {
    #[inline(always)]
    fn from(variant: PHY_RST_CMPLT_A) -> Self {
        variant as u8 != 0
    }
}
impl PHY_RST_CMPLT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PHY_RST_CMPLT_A {
        match self.bits {
            false => PHY_RST_CMPLT_A::DISABLE,
            true => PHY_RST_CMPLT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PHY_RST_CMPLT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PHY_RST_CMPLT_A::ENABLE
    }
}
#[doc = "Field `PhyRstCmplt` writer - "]
pub type PHY_RST_CMPLT_W<'a, const O: u8> = crate::BitWriter<'a, IE_SPEC, O, PHY_RST_CMPLT_A>;
impl<'a, const O: u8> PHY_RST_CMPLT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PHY_RST_CMPLT_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PHY_RST_CMPLT_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn hard_acs_cmplt(&self) -> HARD_ACS_CMPLT_R {
        HARD_ACS_CMPLT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn all(&self) -> ALL_R {
        ALL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn auto_neg(&self) -> AUTO_NEG_R {
        AUTO_NEG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rx_cmplt(&self) -> RX_CMPLT_R {
        RX_CMPLT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rx_rject(&self) -> RX_RJECT_R {
        RX_RJECT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rx_fifoovr(&self) -> RX_FIFOOVR_R {
        RX_FIFOOVR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tx_cmplt(&self) -> TX_CMPLT_R {
        TX_CMPLT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rx_dcm_lock(&self) -> RX_DCM_LOCK_R {
        RX_DCM_LOCK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn mgt_rdy(&self) -> MGT_RDY_R {
        MGT_RDY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn phy_rst_cmplt(&self) -> PHY_RST_CMPLT_R {
        PHY_RST_CMPLT_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn hard_acs_cmplt(&mut self) -> HARD_ACS_CMPLT_W<0> {
        HARD_ACS_CMPLT_W::new(self)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn all(&mut self) -> ALL_W<0> {
        ALL_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn auto_neg(&mut self) -> AUTO_NEG_W<1> {
        AUTO_NEG_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn rx_cmplt(&mut self) -> RX_CMPLT_W<2> {
        RX_CMPLT_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn rx_rject(&mut self) -> RX_RJECT_W<3> {
        RX_RJECT_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifoovr(&mut self) -> RX_FIFOOVR_W<4> {
        RX_FIFOOVR_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn tx_cmplt(&mut self) -> TX_CMPLT_W<5> {
        TX_CMPLT_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn rx_dcm_lock(&mut self) -> RX_DCM_LOCK_W<6> {
        RX_DCM_LOCK_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn mgt_rdy(&mut self) -> MGT_RDY_W<7> {
        MGT_RDY_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn phy_rst_cmplt(&mut self) -> PHY_RST_CMPLT_W<8> {
        PHY_RST_CMPLT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ie](index.html) module"]
pub struct IE_SPEC;
impl crate::RegisterSpec for IE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ie::R](R) reader structure"]
impl crate::Readable for IE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ie::W](W) writer structure"]
impl crate::Writable for IE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ie to value 0"]
impl crate::Resettable for IE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
