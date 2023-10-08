#[doc = "Register `phyc` reader"]
pub type R = crate::R<PHYC_SPEC>;
#[doc = "Field `rgmiilinkmask` reader - "]
pub type RGMIILINKMASK_R = crate::BitReader;
#[doc = "Field `PL` reader - "]
pub type PL_R = crate::FieldReader;
#[doc = "Field `rgmiihd` reader - "]
pub type RGMIIHD_R = crate::BitReader;
#[doc = "Field `rgmiilinkspeed` reader - "]
pub type RGMIILINKSPEED_R = crate::FieldReader<RGMIILINKSPEED_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RGMIILINKSPEED_A {
    #[doc = "0: `0`"]
    _10M = 0,
    #[doc = "1: `1`"]
    _100M = 1,
    #[doc = "2: `10`"]
    _1000M = 2,
}
impl From<RGMIILINKSPEED_A> for u8 {
    #[inline(always)]
    fn from(variant: RGMIILINKSPEED_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RGMIILINKSPEED_A {
    type Ux = u8;
}
impl RGMIILINKSPEED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RGMIILINKSPEED_A> {
        match self.bits {
            0 => Some(RGMIILINKSPEED_A::_10M),
            1 => Some(RGMIILINKSPEED_A::_100M),
            2 => Some(RGMIILINKSPEED_A::_1000M),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_10m(&self) -> bool {
        *self == RGMIILINKSPEED_A::_10M
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_100m(&self) -> bool {
        *self == RGMIILINKSPEED_A::_100M
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_1000m(&self) -> bool {
        *self == RGMIILINKSPEED_A::_1000M
    }
}
#[doc = "Field `MIR` reader - "]
pub type MIR_R = crate::FieldReader;
#[doc = "Field `sgmiilinkspeed` reader - "]
pub type SGMIILINKSPEED_R = crate::FieldReader<SGMIILINKSPEED_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SGMIILINKSPEED_A {
    #[doc = "0: `0`"]
    _10M = 0,
    #[doc = "1: `1`"]
    _100M = 1,
    #[doc = "2: `10`"]
    _1000M = 2,
}
impl From<SGMIILINKSPEED_A> for u8 {
    #[inline(always)]
    fn from(variant: SGMIILINKSPEED_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SGMIILINKSPEED_A {
    type Ux = u8;
}
impl SGMIILINKSPEED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SGMIILINKSPEED_A> {
        match self.bits {
            0 => Some(SGMIILINKSPEED_A::_10M),
            1 => Some(SGMIILINKSPEED_A::_100M),
            2 => Some(SGMIILINKSPEED_A::_1000M),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_10m(&self) -> bool {
        *self == SGMIILINKSPEED_A::_10M
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_100m(&self) -> bool {
        *self == SGMIILINKSPEED_A::_100M
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_1000m(&self) -> bool {
        *self == SGMIILINKSPEED_A::_1000M
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rgmiilinkmask(&self) -> RGMIILINKMASK_R {
        RGMIILINKMASK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pl(&self) -> PL_R {
        PL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rgmiihd(&self) -> RGMIIHD_R {
        RGMIIHD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn rgmiilinkspeed(&self) -> RGMIILINKSPEED_R {
        RGMIILINKSPEED_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn mir(&self) -> MIR_R {
        MIR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn sgmiilinkspeed(&self) -> SGMIILINKSPEED_R {
        SGMIILINKSPEED_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[doc = "RGMII/SGMII configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phyc::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PHYC_SPEC;
impl crate::RegisterSpec for PHYC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phyc::R`](R) reader structure"]
impl crate::Readable for PHYC_SPEC {}
#[doc = "`reset()` method sets phyc to value 0"]
impl crate::Resettable for PHYC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
