#[doc = "Reader of register CWR"]
pub type R = crate::R<u32, super::CWR>;
#[doc = "Writer for register CWR"]
pub type W = crate::W<u32, super::CWR>;

#[doc = "Register CWR `reset()`'s with value 0"]
impl crate::ResetValue for super::CWR {
    #[inline(always)]
    fn reset_value() -> Self::Ux { 0 }
}

#[doc = "Reader of field `LOWTHRES`"]
pub type LOWTHRES_R = crate::R<u16, u16>;

#[doc = "Write proxy for field `LOWTHRES`"]
pub struct LOWTHRES_W<'a> { w: &'a mut W }

impl<'a> LOWTHRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}

#[doc = "Reader of field `HIGHTHRES`"]
pub type HIGHTHRES_R = crate::R<u16, u16>;

#[doc = "Write proxy for field `HIGHTHRES`"]
pub struct HIGHTHRES_W<'a> { w: &'a mut W }

impl<'a> HIGHTHRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}

impl R {
    #[doc = "Bits 0:11 - Low Threshold"]
    #[inline(always)]
    pub fn lowthres(&self) -> LOWTHRES_R { LOWTHRES_R::new((self.bits & 0x0fff) as u16) }
    #[doc = "Bits 16:27 - High Threshold"]
    #[inline(always)]
    pub fn highthres(&self) -> HIGHTHRES_R { HIGHTHRES_R::new(((self.bits >> 16) & 0x0fff) as u16) }
}

impl W {
    #[doc = "Bits 0:11 - Low Threshold"]
    #[inline(always)]
    pub fn lowthres(&mut self) -> LOWTHRES_W { LOWTHRES_W { w: self } }
    #[doc = "Bits 16:27 - High Threshold"]
    #[inline(always)]
    pub fn highthres(&mut self) -> HIGHTHRES_W { HIGHTHRES_W { w: self } }
}