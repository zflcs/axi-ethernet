#[doc = "Register `fmi` reader"]
pub type R = crate::R<FMI_SPEC>;
#[doc = "Register `fmi` writer"]
pub type W = crate::W<FMI_SPEC>;
#[doc = "Field `FI` reader - "]
pub type FI_R = crate::FieldReader;
#[doc = "Field `FI` writer - "]
pub type FI_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `AVBS` reader - "]
pub type AVBS_R = crate::BitReader<AVBS_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AVBS_A {
    #[doc = "0: `0`"]
    DISBALE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<AVBS_A> for bool {
    #[inline(always)]
    fn from(variant: AVBS_A) -> Self {
        variant as u8 != 0
    }
}
impl AVBS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVBS_A {
        match self.bits {
            false => AVBS_A::DISBALE,
            true => AVBS_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disbale(&self) -> bool {
        *self == AVBS_A::DISBALE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == AVBS_A::ENABLE
    }
}
#[doc = "Field `AVBS` writer - "]
pub type AVBS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, AVBS_A>;
impl<'a, REG, const O: u8> AVBS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disbale(self) -> &'a mut crate::W<REG> {
        self.variant(AVBS_A::DISBALE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(AVBS_A::ENABLE)
    }
}
#[doc = "Field `PMODE` reader - "]
pub type PMODE_R = crate::BitReader<PMODE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMODE_A {
    #[doc = "0: `0`"]
    DISBALE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<PMODE_A> for bool {
    #[inline(always)]
    fn from(variant: PMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl PMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMODE_A {
        match self.bits {
            false => PMODE_A::DISBALE,
            true => PMODE_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disbale(&self) -> bool {
        *self == PMODE_A::DISBALE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PMODE_A::ENABLE
    }
}
#[doc = "Field `PMODE` writer - "]
pub type PMODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PMODE_A>;
impl<'a, REG, const O: u8> PMODE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disbale(self) -> &'a mut crate::W<REG> {
        self.variant(PMODE_A::DISBALE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(PMODE_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn fi(&self) -> FI_R {
        FI_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn avbs(&self) -> AVBS_R {
        AVBS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn pmode(&self) -> PMODE_R {
        PMODE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn fi(&mut self) -> FI_W<FMI_SPEC, 0> {
        FI_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn avbs(&mut self) -> AVBS_W<FMI_SPEC, 8> {
        AVBS_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn pmode(&mut self) -> PMODE_W<FMI_SPEC, 31> {
        PMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Filter Mask Index\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FMI_SPEC;
impl crate::RegisterSpec for FMI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmi::R`](R) reader structure"]
impl crate::Readable for FMI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fmi::W`](W) writer structure"]
impl crate::Writable for FMI_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets fmi to value 0"]
impl crate::Resettable for FMI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
