#[doc = "Register `mdiomcr` reader"]
pub type R = crate::R<MDIOMCR_SPEC>;
#[doc = "Register `mdiomcr` writer"]
pub type W = crate::W<MDIOMCR_SPEC>;
#[doc = "Field `READY` reader - "]
pub type READY_R = crate::BitReader<READY_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum READY_A {
    #[doc = "0: `0`"]
    NOR_READY = 0,
    #[doc = "1: `1`"]
    READY = 1,
}
impl From<READY_A> for bool {
    #[inline(always)]
    fn from(variant: READY_A) -> Self {
        variant as u8 != 0
    }
}
impl READY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READY_A {
        match self.bits {
            false => READY_A::NOR_READY,
            true => READY_A::READY,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_nor_ready(&self) -> bool {
        *self == READY_A::NOR_READY
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == READY_A::READY
    }
}
#[doc = "Field `READY` writer - "]
pub type READY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, READY_A>;
impl<'a, REG, const O: u8> READY_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn nor_ready(self) -> &'a mut crate::W<REG> {
        self.variant(READY_A::NOR_READY)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut crate::W<REG> {
        self.variant(READY_A::READY)
    }
}
#[doc = "Field `INIT` reader - "]
pub type INIT_R = crate::BitReader<INIT_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INIT_A {
    #[doc = "1: `1`"]
    START = 1,
}
impl From<INIT_A> for bool {
    #[inline(always)]
    fn from(variant: INIT_A) -> Self {
        variant as u8 != 0
    }
}
impl INIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INIT_A> {
        match self.bits {
            true => Some(INIT_A::START),
            _ => None,
        }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == INIT_A::START
    }
}
#[doc = "Field `INIT` writer - "]
pub type INIT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, INIT_A>;
impl<'a, REG, const O: u8> INIT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`1`"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(INIT_A::START)
    }
}
#[doc = "Field `TX_OP` reader - "]
pub type TX_OP_R = crate::FieldReader<TX_OP_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TX_OP_A {
    #[doc = "1: `1`"]
    WA = 1,
    #[doc = "2: `10`"]
    RA = 2,
}
impl From<TX_OP_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_OP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TX_OP_A {
    type Ux = u8;
}
impl TX_OP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TX_OP_A> {
        match self.bits {
            1 => Some(TX_OP_A::WA),
            2 => Some(TX_OP_A::RA),
            _ => None,
        }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_wa(&self) -> bool {
        *self == TX_OP_A::WA
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_ra(&self) -> bool {
        *self == TX_OP_A::RA
    }
}
#[doc = "Field `TX_OP` writer - "]
pub type TX_OP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, TX_OP_A>;
impl<'a, REG, const O: u8> TX_OP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`1`"]
    #[inline(always)]
    pub fn wa(self) -> &'a mut crate::W<REG> {
        self.variant(TX_OP_A::WA)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn ra(self) -> &'a mut crate::W<REG> {
        self.variant(TX_OP_A::RA)
    }
}
#[doc = "Field `TX_REGAD` reader - "]
pub type TX_REGAD_R = crate::FieldReader;
#[doc = "Field `TX_REGAD` writer - "]
pub type TX_REGAD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `TX_PHYAD` reader - "]
pub type TX_PHYAD_R = crate::FieldReader;
#[doc = "Field `TX_PHYAD` writer - "]
pub type TX_PHYAD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn tx_op(&self) -> TX_OP_R {
        TX_OP_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn tx_regad(&self) -> TX_REGAD_R {
        TX_REGAD_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28"]
    #[inline(always)]
    pub fn tx_phyad(&self) -> TX_PHYAD_R {
        TX_PHYAD_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn ready(&mut self) -> READY_W<MDIOMCR_SPEC, 7> {
        READY_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn init(&mut self) -> INIT_W<MDIOMCR_SPEC, 11> {
        INIT_W::new(self)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    #[must_use]
    pub fn tx_op(&mut self) -> TX_OP_W<MDIOMCR_SPEC, 14> {
        TX_OP_W::new(self)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    #[must_use]
    pub fn tx_regad(&mut self) -> TX_REGAD_W<MDIOMCR_SPEC, 16> {
        TX_REGAD_W::new(self)
    }
    #[doc = "Bits 24:28"]
    #[inline(always)]
    #[must_use]
    pub fn tx_phyad(&mut self) -> TX_PHYAD_W<MDIOMCR_SPEC, 24> {
        TX_PHYAD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MDIO Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdiomcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdiomcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDIOMCR_SPEC;
impl crate::RegisterSpec for MDIOMCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdiomcr::R`](R) reader structure"]
impl crate::Readable for MDIOMCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mdiomcr::W`](W) writer structure"]
impl crate::Writable for MDIOMCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets mdiomcr to value 0"]
impl crate::Resettable for MDIOMCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
