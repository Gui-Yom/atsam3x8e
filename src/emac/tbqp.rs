#[doc = "Reader of register TBQP"]
pub type R = crate::R<u32, super::TBQP>;
#[doc = "Writer for register TBQP"]
pub type W = crate::W<u32, super::TBQP>;

#[doc = "Register TBQP `reset()`'s with value 0"]
impl crate::ResetValue for super::TBQP {
    #[inline(always)]
    fn reset_value() -> Self::Ux { 0 }
}

#[doc = "Reader of field `ADDR`"]
pub type ADDR_R = crate::R<u32, u32>;

#[doc = "Write proxy for field `ADDR`"]
pub struct ADDR_W<'a> { w: &'a mut W }

impl<'a> ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | (((value as u32) & 0x3fff_ffff) << 2);
        self.w
    }
}

impl R {
    #[doc = "Bits 2:31 - Transmit buffer queue pointer address"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R { ADDR_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32) }
}

impl W {
    #[doc = "Bits 2:31 - Transmit buffer queue pointer address"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W { ADDR_W { w: self } }
}