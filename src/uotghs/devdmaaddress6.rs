#[doc = "Reader of register DEVDMAADDRESS6"]
pub type R = crate::R<u32, super::DEVDMAADDRESS6>;
#[doc = "Writer for register DEVDMAADDRESS6"]
pub type W = crate::W<u32, super::DEVDMAADDRESS6>;

#[doc = "Register DEVDMAADDRESS6 `reset()`'s with value 0"]
impl crate::ResetValue for super::DEVDMAADDRESS6 {
    #[inline(always)]
    fn reset_value() -> Self::Ux { 0 }
}

#[doc = "Reader of field `BUFF_ADD`"]
pub type BUFF_ADD_R = crate::R<u32, u32>;

#[doc = "Write proxy for field `BUFF_ADD`"]
pub struct BUFF_ADD_W<'a> { w: &'a mut W }

impl<'a> BUFF_ADD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}

impl R {
    #[doc = "Bits 0:31 - Buffer Address"]
    #[inline(always)]
    pub fn buff_add(&self) -> BUFF_ADD_R { BUFF_ADD_R::new((self.bits & 0xffff_ffff) as u32) }
}

impl W {
    #[doc = "Bits 0:31 - Buffer Address"]
    #[inline(always)]
    pub fn buff_add(&mut self) -> BUFF_ADD_W { BUFF_ADD_W { w: self } }
}