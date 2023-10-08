#[doc = "Register `rtag` reader"]
pub type R = crate::R<RTAG_SPEC>;
#[doc = "Register `rtag` writer"]
pub type W = crate::W<RTAG_SPEC>;
#[doc = "Field `vid` reader - "]
pub type VID_R = crate::FieldReader<VID_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum VID_A {
    #[doc = "0: `0`"]
    RESET = 0,
}
impl From<VID_A> for u16 {
    #[inline(always)]
    fn from(variant: VID_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VID_A {
    type Ux = u16;
}
impl VID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<VID_A> {
        match self.bits {
            0 => Some(VID_A::RESET),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == VID_A::RESET
    }
}
#[doc = "Field `vid` writer - "]
pub type VID_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, VID_A>;
impl<'a, REG, const O: u8> VID_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(VID_A::RESET)
    }
}
#[doc = "Field `cfi` reader - "]
pub type CFI_R = crate::BitReader<CFI_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFI_A {
    #[doc = "0: `0`"]
    RESET = 0,
}
impl From<CFI_A> for bool {
    #[inline(always)]
    fn from(variant: CFI_A) -> Self {
        variant as u8 != 0
    }
}
impl CFI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CFI_A> {
        match self.bits {
            false => Some(CFI_A::RESET),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == CFI_A::RESET
    }
}
#[doc = "Field `cfi` writer - "]
pub type CFI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CFI_A>;
impl<'a, REG, const O: u8> CFI_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(CFI_A::RESET)
    }
}
#[doc = "Field `priority` reader - "]
pub type PRIORITY_R = crate::FieldReader<PRIORITY_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRIORITY_A {
    #[doc = "0: `0`"]
    RESET = 0,
}
impl From<PRIORITY_A> for u8 {
    #[inline(always)]
    fn from(variant: PRIORITY_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRIORITY_A {
    type Ux = u8;
}
impl PRIORITY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRIORITY_A> {
        match self.bits {
            0 => Some(PRIORITY_A::RESET),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == PRIORITY_A::RESET
    }
}
#[doc = "Field `priority` writer - "]
pub type PRIORITY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, PRIORITY_A>;
impl<'a, REG, const O: u8> PRIORITY_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(PRIORITY_A::RESET)
    }
}
#[doc = "Field `tpid` reader - "]
pub type TPID_R = crate::FieldReader<TPID_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum TPID_A {
    #[doc = "0: `0`"]
    RESET = 0,
}
impl From<TPID_A> for u16 {
    #[inline(always)]
    fn from(variant: TPID_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TPID_A {
    type Ux = u16;
}
impl TPID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TPID_A> {
        match self.bits {
            0 => Some(TPID_A::RESET),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == TPID_A::RESET
    }
}
#[doc = "Field `tpid` writer - "]
pub type TPID_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, TPID_A>;
impl<'a, REG, const O: u8> TPID_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(TPID_A::RESET)
    }
}
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn vid(&self) -> VID_R {
        VID_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cfi(&self) -> CFI_R {
        CFI_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15"]
    #[inline(always)]
    pub fn priority(&self) -> PRIORITY_R {
        PRIORITY_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn tpid(&self) -> TPID_R {
        TPID_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    #[must_use]
    pub fn vid(&mut self) -> VID_W<RTAG_SPEC, 0> {
        VID_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn cfi(&mut self) -> CFI_W<RTAG_SPEC, 12> {
        CFI_W::new(self)
    }
    #[doc = "Bits 13:15"]
    #[inline(always)]
    #[must_use]
    pub fn priority(&mut self) -> PRIORITY_W<RTAG_SPEC, 13> {
        PRIORITY_W::new(self)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn tpid(&mut self) -> TPID_W<RTAG_SPEC, 16> {
        TPID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Receive VLAN Tag TEMAC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtag::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtag::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTAG_SPEC;
impl crate::RegisterSpec for RTAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtag::R`](R) reader structure"]
impl crate::Readable for RTAG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtag::W`](W) writer structure"]
impl crate::Writable for RTAG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rtag to value 0"]
impl crate::Resettable for RTAG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
