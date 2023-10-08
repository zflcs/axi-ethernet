#[doc = "Register `mdiomc` reader"]
pub type R = crate::R<MDIOMC_SPEC>;
#[doc = "Register `mdiomc` writer"]
pub type W = crate::W<MDIOMC_SPEC>;
#[doc = "Field `CD` reader - "]
pub type CD_R = crate::FieldReader;
#[doc = "Field `CD` writer - "]
pub type CD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `MDIOE` reader - "]
pub type MDIOE_R = crate::BitReader<MDIOE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MDIOE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<MDIOE_A> for bool {
    #[inline(always)]
    fn from(variant: MDIOE_A) -> Self {
        variant as u8 != 0
    }
}
impl MDIOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MDIOE_A {
        match self.bits {
            false => MDIOE_A::DISABLE,
            true => MDIOE_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MDIOE_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MDIOE_A::ENABLE
    }
}
#[doc = "Field `MDIOE` writer - "]
pub type MDIOE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MDIOE_A>;
impl<'a, REG, const O: u8> MDIOE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(MDIOE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(MDIOE_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn cd(&self) -> CD_R {
        CD_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn mdioe(&self) -> MDIOE_R {
        MDIOE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn cd(&mut self) -> CD_W<MDIOMC_SPEC, 0> {
        CD_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn mdioe(&mut self) -> MDIOE_W<MDIOMC_SPEC, 6> {
        MDIOE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MDIO Setup\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdiomc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdiomc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDIOMC_SPEC;
impl crate::RegisterSpec for MDIOMC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdiomc::R`](R) reader structure"]
impl crate::Readable for MDIOMC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mdiomc::W`](W) writer structure"]
impl crate::Writable for MDIOMC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets mdiomc to value 0"]
impl crate::Resettable for MDIOMC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
