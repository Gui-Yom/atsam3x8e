#[doc = "Reader of register CDTY5"]
pub type R = crate::R<u32, super::CDTY5>;
#[doc = "Writer for register CDTY5"]
pub type W = crate::W<u32, super::CDTY5>;

#[doc = "Register CDTY5 `reset()`'s with value 0"]
impl crate::ResetValue for super::CDTY5 {
    #[inline(always)]
    fn reset_value() -> Self::Ux { 0 }
}

#[doc = "Reader of field `CDTY`"]
pub type CDTY_R = crate::R<u32, u32>;

#[doc = "Write proxy for field `CDTY`"]
pub struct CDTY_W<'a> { w: &'a mut W }

impl<'a> CDTY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}

impl R {
    #[doc = "Bits 0:23 - Channel Duty-Cycle"]
    #[inline(always)]
    pub fn cdty(&self) -> CDTY_R { CDTY_R::new((self.bits & 0x00ff_ffff) as u32) }
}

impl W {
    #[doc = "Bits 0:23 - Channel Duty-Cycle"]
    #[inline(always)]
    pub fn cdty(&mut self) -> CDTY_W { CDTY_W { w: self } }
}