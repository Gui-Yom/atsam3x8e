#[doc = "Reader of register CGR"]
pub type R = crate::R<u32, super::CGR>;
#[doc = "Writer for register CGR"]
pub type W = crate::W<u32, super::CGR>;

#[doc = "Register CGR `reset()`'s with value 0"]
impl crate::ResetValue for super::CGR {
    #[inline(always)]
    fn reset_value() -> Self::Ux { 0 }
}

#[doc = "Reader of field `GAIN0`"]
pub type GAIN0_R = crate::R<u8, u8>;

#[doc = "Write proxy for field `GAIN0`"]
pub struct GAIN0_W<'a> { w: &'a mut W }

impl<'a> GAIN0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}

#[doc = "Reader of field `GAIN1`"]
pub type GAIN1_R = crate::R<u8, u8>;

#[doc = "Write proxy for field `GAIN1`"]
pub struct GAIN1_W<'a> { w: &'a mut W }

impl<'a> GAIN1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}

#[doc = "Reader of field `GAIN2`"]
pub type GAIN2_R = crate::R<u8, u8>;

#[doc = "Write proxy for field `GAIN2`"]
pub struct GAIN2_W<'a> { w: &'a mut W }

impl<'a> GAIN2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}

#[doc = "Reader of field `GAIN3`"]
pub type GAIN3_R = crate::R<u8, u8>;

#[doc = "Write proxy for field `GAIN3`"]
pub struct GAIN3_W<'a> { w: &'a mut W }

impl<'a> GAIN3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}

#[doc = "Reader of field `GAIN4`"]
pub type GAIN4_R = crate::R<u8, u8>;

#[doc = "Write proxy for field `GAIN4`"]
pub struct GAIN4_W<'a> { w: &'a mut W }

impl<'a> GAIN4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}

#[doc = "Reader of field `GAIN5`"]
pub type GAIN5_R = crate::R<u8, u8>;

#[doc = "Write proxy for field `GAIN5`"]
pub struct GAIN5_W<'a> { w: &'a mut W }

impl<'a> GAIN5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}

#[doc = "Reader of field `GAIN6`"]
pub type GAIN6_R = crate::R<u8, u8>;

#[doc = "Write proxy for field `GAIN6`"]
pub struct GAIN6_W<'a> { w: &'a mut W }

impl<'a> GAIN6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}

#[doc = "Reader of field `GAIN7`"]
pub type GAIN7_R = crate::R<u8, u8>;

#[doc = "Write proxy for field `GAIN7`"]
pub struct GAIN7_W<'a> { w: &'a mut W }

impl<'a> GAIN7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}

#[doc = "Reader of field `GAIN8`"]
pub type GAIN8_R = crate::R<u8, u8>;

#[doc = "Write proxy for field `GAIN8`"]
pub struct GAIN8_W<'a> { w: &'a mut W }

impl<'a> GAIN8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}

#[doc = "Reader of field `GAIN9`"]
pub type GAIN9_R = crate::R<u8, u8>;

#[doc = "Write proxy for field `GAIN9`"]
pub struct GAIN9_W<'a> { w: &'a mut W }

impl<'a> GAIN9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}

#[doc = "Reader of field `GAIN10`"]
pub type GAIN10_R = crate::R<u8, u8>;

#[doc = "Write proxy for field `GAIN10`"]
pub struct GAIN10_W<'a> { w: &'a mut W }

impl<'a> GAIN10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}

#[doc = "Reader of field `GAIN11`"]
pub type GAIN11_R = crate::R<u8, u8>;

#[doc = "Write proxy for field `GAIN11`"]
pub struct GAIN11_W<'a> { w: &'a mut W }

impl<'a> GAIN11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}

#[doc = "Reader of field `GAIN12`"]
pub type GAIN12_R = crate::R<u8, u8>;

#[doc = "Write proxy for field `GAIN12`"]
pub struct GAIN12_W<'a> { w: &'a mut W }

impl<'a> GAIN12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}

#[doc = "Reader of field `GAIN13`"]
pub type GAIN13_R = crate::R<u8, u8>;

#[doc = "Write proxy for field `GAIN13`"]
pub struct GAIN13_W<'a> { w: &'a mut W }

impl<'a> GAIN13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}

#[doc = "Reader of field `GAIN14`"]
pub type GAIN14_R = crate::R<u8, u8>;

#[doc = "Write proxy for field `GAIN14`"]
pub struct GAIN14_W<'a> { w: &'a mut W }

impl<'a> GAIN14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}

#[doc = "Reader of field `GAIN15`"]
pub type GAIN15_R = crate::R<u8, u8>;

#[doc = "Write proxy for field `GAIN15`"]
pub struct GAIN15_W<'a> { w: &'a mut W }

impl<'a> GAIN15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}

impl R {
    #[doc = "Bits 0:1 - Gain for channel 0"]
    #[inline(always)]
    pub fn gain0(&self) -> GAIN0_R { GAIN0_R::new((self.bits & 0x03) as u8) }
    #[doc = "Bits 2:3 - Gain for channel 1"]
    #[inline(always)]
    pub fn gain1(&self) -> GAIN1_R { GAIN1_R::new(((self.bits >> 2) & 0x03) as u8) }
    #[doc = "Bits 4:5 - Gain for channel 2"]
    #[inline(always)]
    pub fn gain2(&self) -> GAIN2_R { GAIN2_R::new(((self.bits >> 4) & 0x03) as u8) }
    #[doc = "Bits 6:7 - Gain for channel 3"]
    #[inline(always)]
    pub fn gain3(&self) -> GAIN3_R { GAIN3_R::new(((self.bits >> 6) & 0x03) as u8) }
    #[doc = "Bits 8:9 - Gain for channel 4"]
    #[inline(always)]
    pub fn gain4(&self) -> GAIN4_R { GAIN4_R::new(((self.bits >> 8) & 0x03) as u8) }
    #[doc = "Bits 10:11 - Gain for channel 5"]
    #[inline(always)]
    pub fn gain5(&self) -> GAIN5_R { GAIN5_R::new(((self.bits >> 10) & 0x03) as u8) }
    #[doc = "Bits 12:13 - Gain for channel 6"]
    #[inline(always)]
    pub fn gain6(&self) -> GAIN6_R { GAIN6_R::new(((self.bits >> 12) & 0x03) as u8) }
    #[doc = "Bits 14:15 - Gain for channel 7"]
    #[inline(always)]
    pub fn gain7(&self) -> GAIN7_R { GAIN7_R::new(((self.bits >> 14) & 0x03) as u8) }
    #[doc = "Bits 16:17 - Gain for channel 8"]
    #[inline(always)]
    pub fn gain8(&self) -> GAIN8_R { GAIN8_R::new(((self.bits >> 16) & 0x03) as u8) }
    #[doc = "Bits 18:19 - Gain for channel 9"]
    #[inline(always)]
    pub fn gain9(&self) -> GAIN9_R { GAIN9_R::new(((self.bits >> 18) & 0x03) as u8) }
    #[doc = "Bits 20:21 - Gain for channel 10"]
    #[inline(always)]
    pub fn gain10(&self) -> GAIN10_R { GAIN10_R::new(((self.bits >> 20) & 0x03) as u8) }
    #[doc = "Bits 22:23 - Gain for channel 11"]
    #[inline(always)]
    pub fn gain11(&self) -> GAIN11_R { GAIN11_R::new(((self.bits >> 22) & 0x03) as u8) }
    #[doc = "Bits 24:25 - Gain for channel 12"]
    #[inline(always)]
    pub fn gain12(&self) -> GAIN12_R { GAIN12_R::new(((self.bits >> 24) & 0x03) as u8) }
    #[doc = "Bits 26:27 - Gain for channel 13"]
    #[inline(always)]
    pub fn gain13(&self) -> GAIN13_R { GAIN13_R::new(((self.bits >> 26) & 0x03) as u8) }
    #[doc = "Bits 28:29 - Gain for channel 14"]
    #[inline(always)]
    pub fn gain14(&self) -> GAIN14_R { GAIN14_R::new(((self.bits >> 28) & 0x03) as u8) }
    #[doc = "Bits 30:31 - Gain for channel 15"]
    #[inline(always)]
    pub fn gain15(&self) -> GAIN15_R { GAIN15_R::new(((self.bits >> 30) & 0x03) as u8) }
}

impl W {
    #[doc = "Bits 0:1 - Gain for channel 0"]
    #[inline(always)]
    pub fn gain0(&mut self) -> GAIN0_W { GAIN0_W { w: self } }
    #[doc = "Bits 2:3 - Gain for channel 1"]
    #[inline(always)]
    pub fn gain1(&mut self) -> GAIN1_W { GAIN1_W { w: self } }
    #[doc = "Bits 4:5 - Gain for channel 2"]
    #[inline(always)]
    pub fn gain2(&mut self) -> GAIN2_W { GAIN2_W { w: self } }
    #[doc = "Bits 6:7 - Gain for channel 3"]
    #[inline(always)]
    pub fn gain3(&mut self) -> GAIN3_W { GAIN3_W { w: self } }
    #[doc = "Bits 8:9 - Gain for channel 4"]
    #[inline(always)]
    pub fn gain4(&mut self) -> GAIN4_W { GAIN4_W { w: self } }
    #[doc = "Bits 10:11 - Gain for channel 5"]
    #[inline(always)]
    pub fn gain5(&mut self) -> GAIN5_W { GAIN5_W { w: self } }
    #[doc = "Bits 12:13 - Gain for channel 6"]
    #[inline(always)]
    pub fn gain6(&mut self) -> GAIN6_W { GAIN6_W { w: self } }
    #[doc = "Bits 14:15 - Gain for channel 7"]
    #[inline(always)]
    pub fn gain7(&mut self) -> GAIN7_W { GAIN7_W { w: self } }
    #[doc = "Bits 16:17 - Gain for channel 8"]
    #[inline(always)]
    pub fn gain8(&mut self) -> GAIN8_W { GAIN8_W { w: self } }
    #[doc = "Bits 18:19 - Gain for channel 9"]
    #[inline(always)]
    pub fn gain9(&mut self) -> GAIN9_W { GAIN9_W { w: self } }
    #[doc = "Bits 20:21 - Gain for channel 10"]
    #[inline(always)]
    pub fn gain10(&mut self) -> GAIN10_W { GAIN10_W { w: self } }
    #[doc = "Bits 22:23 - Gain for channel 11"]
    #[inline(always)]
    pub fn gain11(&mut self) -> GAIN11_W { GAIN11_W { w: self } }
    #[doc = "Bits 24:25 - Gain for channel 12"]
    #[inline(always)]
    pub fn gain12(&mut self) -> GAIN12_W { GAIN12_W { w: self } }
    #[doc = "Bits 26:27 - Gain for channel 13"]
    #[inline(always)]
    pub fn gain13(&mut self) -> GAIN13_W { GAIN13_W { w: self } }
    #[doc = "Bits 28:29 - Gain for channel 14"]
    #[inline(always)]
    pub fn gain14(&mut self) -> GAIN14_W { GAIN14_W { w: self } }
    #[doc = "Bits 30:31 - Gain for channel 15"]
    #[inline(always)]
    pub fn gain15(&mut self) -> GAIN15_W { GAIN15_W { w: self } }
}