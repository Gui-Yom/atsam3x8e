#[doc = "Reader of register SA2T"]
pub type R = crate::R<u32, super::SA2T>;
#[doc = "Writer for register SA2T"]
pub type W = crate::W<u32, super::SA2T>;

#[doc = "Register SA2T `reset()`'s with value 0"]
impl crate::ResetValue for super::SA2T {
    #[inline(always)]
    fn reset_value() -> Self::Ux { 0 }
}

#[doc = "Reader of field `ADDR`"]
pub type ADDR_R = crate::R<u16, u16>;

#[doc = "Write proxy for field `ADDR`"]
pub struct ADDR_W<'a> { w: &'a mut W }

impl<'a> ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}

impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R { ADDR_R::new((self.bits & 0xffff) as u16) }
}

impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W { ADDR_W { w: self } }
}