#[doc = "Writer for register HSTPIPICR0_ISOPIPES"]
pub type W = crate::W<u32, super::HSTPIPICR0_ISOPIPES>;

#[doc = "Write proxy for field `RXINIC`"]
pub struct RXINIC_W<'a> { w: &'a mut W }

impl<'a> RXINIC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W { self.bit(true) }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W { self.bit(false) }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}

#[doc = "Write proxy for field `TXOUTIC`"]
pub struct TXOUTIC_W<'a> { w: &'a mut W }

impl<'a> TXOUTIC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W { self.bit(true) }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W { self.bit(false) }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}

#[doc = "Write proxy for field `UNDERFIC`"]
pub struct UNDERFIC_W<'a> { w: &'a mut W }

impl<'a> UNDERFIC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W { self.bit(true) }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W { self.bit(false) }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}

#[doc = "Write proxy for field `NAKEDIC`"]
pub struct NAKEDIC_W<'a> { w: &'a mut W }

impl<'a> NAKEDIC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W { self.bit(true) }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W { self.bit(false) }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}

#[doc = "Write proxy for field `OVERFIC`"]
pub struct OVERFIC_W<'a> { w: &'a mut W }

impl<'a> OVERFIC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W { self.bit(true) }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W { self.bit(false) }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}

#[doc = "Write proxy for field `CRCERRIC`"]
pub struct CRCERRIC_W<'a> { w: &'a mut W }

impl<'a> CRCERRIC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W { self.bit(true) }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W { self.bit(false) }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}

#[doc = "Write proxy for field `SHORTPACKETIC`"]
pub struct SHORTPACKETIC_W<'a> { w: &'a mut W }

impl<'a> SHORTPACKETIC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W { self.bit(true) }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W { self.bit(false) }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}

impl W {
    #[doc = "Bit 0 - Received IN Data Interrupt Clear"]
    #[inline(always)]
    pub fn rxinic(&mut self) -> RXINIC_W { RXINIC_W { w: self } }
    #[doc = "Bit 1 - Transmitted OUT Data Interrupt Clear"]
    #[inline(always)]
    pub fn txoutic(&mut self) -> TXOUTIC_W { TXOUTIC_W { w: self } }
    #[doc = "Bit 2 - Underflow Interrupt Clear"]
    #[inline(always)]
    pub fn underfic(&mut self) -> UNDERFIC_W { UNDERFIC_W { w: self } }
    #[doc = "Bit 4 - NAKed Interrupt Clear"]
    #[inline(always)]
    pub fn nakedic(&mut self) -> NAKEDIC_W { NAKEDIC_W { w: self } }
    #[doc = "Bit 5 - Overflow Interrupt Clear"]
    #[inline(always)]
    pub fn overfic(&mut self) -> OVERFIC_W { OVERFIC_W { w: self } }
    #[doc = "Bit 6 - CRC Error Interrupt Clear"]
    #[inline(always)]
    pub fn crcerric(&mut self) -> CRCERRIC_W { CRCERRIC_W { w: self } }
    #[doc = "Bit 7 - Short Packet Interrupt Clear"]
    #[inline(always)]
    pub fn shortpacketic(&mut self) -> SHORTPACKETIC_W { SHORTPACKETIC_W { w: self } }
}