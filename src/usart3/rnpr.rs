#[doc = "Reader of register RNPR"]
pub type R = crate::R<u32, super::RNPR>;
#[doc = "Writer for register RNPR"]
pub type W = crate::W<u32, super::RNPR>;

#[doc = "Register RNPR `reset()`'s with value 0"]
impl crate::ResetValue for super::RNPR {
    #[inline(always)]
    fn reset_value() -> Self::Ux { 0 }
}

#[doc = "Reader of field `RXNPTR`"]
pub type RXNPTR_R = crate::R<u32, u32>;

#[doc = "Write proxy for field `RXNPTR`"]
pub struct RXNPTR_W<'a> { w: &'a mut W }

impl<'a> RXNPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}

impl R {
    #[doc = "Bits 0:31 - Receive Next Pointer"]
    #[inline(always)]
    pub fn rxnptr(&self) -> RXNPTR_R { RXNPTR_R::new((self.bits & 0xffff_ffff) as u32) }
}

impl W {
    #[doc = "Bits 0:31 - Receive Next Pointer"]
    #[inline(always)]
    pub fn rxnptr(&mut self) -> RXNPTR_W { RXNPTR_W { w: self } }
}