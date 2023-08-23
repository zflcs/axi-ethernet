#[doc = "Register `mdiomc` reader"]
pub struct R(crate::R<MDIOMC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDIOMC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDIOMC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDIOMC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `mdiomc` writer"]
pub struct W(crate::W<MDIOMC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDIOMC_SPEC>;
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
impl From<crate::W<MDIOMC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDIOMC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CD` reader - "]
pub type CD_R = crate::FieldReader;
#[doc = "Field `CD` writer - "]
pub type CD_W<'a, const O: u8> = crate::FieldWriter<'a, MDIOMC_SPEC, 6, O>;
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
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MDIOE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MDIOE_A::ENABLE
    }
}
#[doc = "Field `MDIOE` writer - "]
pub type MDIOE_W<'a, const O: u8> = crate::BitWriter<'a, MDIOMC_SPEC, O, MDIOE_A>;
impl<'a, const O: u8> MDIOE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MDIOE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
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
    pub fn cd(&mut self) -> CD_W<0> {
        CD_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn mdioe(&mut self) -> MDIOE_W<6> {
        MDIOE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MDIO Setup\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdiomc](index.html) module"]
pub struct MDIOMC_SPEC;
impl crate::RegisterSpec for MDIOMC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdiomc::R](R) reader structure"]
impl crate::Readable for MDIOMC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mdiomc::W](W) writer structure"]
impl crate::Writable for MDIOMC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets mdiomc to value 0"]
impl crate::Resettable for MDIOMC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
