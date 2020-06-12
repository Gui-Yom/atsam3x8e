#[doc = "Reader of register TTGR"]
pub type R = crate::R<u32, super::TTGR>;
#[doc = "Writer for register TTGR"]
pub type W = crate::W<u32, super::TTGR>;

#[doc = "Register TTGR `reset()`'s with value 0"]
impl crate::ResetValue for super::TTGR {
    #[inline(always)]
    fn reset_value() -> Self::Ux { 0 }
}

#[doc = "Reader of field `TG`"]
pub type TG_R = crate::R<u8, u8>;

#[doc = "Write proxy for field `TG`"]
pub struct TG_W<'a> { w: &'a mut W }

impl<'a> TG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}

impl R {
    #[doc = "Bits 0:7 - Timeguard Value"]
    #[inline(always)]
    pub fn tg(&self) -> TG_R { TG_R::new((self.bits & 0xff) as u8) }
}

impl W {
    #[doc = "Bits 0:7 - Timeguard Value"]
    #[inline(always)]
    pub fn tg(&mut self) -> TG_W { TG_W { w: self } }
}