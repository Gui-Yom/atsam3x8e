#[doc = "Writer for register IER_LIN_MODE"]
pub type W = crate::W<u32, super::IER_LIN_MODE>;

#[doc = "Write proxy for field `RXRDY`"]
pub struct RXRDY_W<'a> { w: &'a mut W }

impl<'a> RXRDY_W<'a> {
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

#[doc = "Write proxy for field `TXRDY`"]
pub struct TXRDY_W<'a> { w: &'a mut W }

impl<'a> TXRDY_W<'a> {
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

#[doc = "Write proxy for field `OVRE`"]
pub struct OVRE_W<'a> { w: &'a mut W }

impl<'a> OVRE_W<'a> {
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

#[doc = "Write proxy for field `FRAME`"]
pub struct FRAME_W<'a> { w: &'a mut W }

impl<'a> FRAME_W<'a> {
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

#[doc = "Write proxy for field `PARE`"]
pub struct PARE_W<'a> { w: &'a mut W }

impl<'a> PARE_W<'a> {
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

#[doc = "Write proxy for field `TIMEOUT`"]
pub struct TIMEOUT_W<'a> { w: &'a mut W }

impl<'a> TIMEOUT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W { self.bit(true) }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W { self.bit(false) }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}

#[doc = "Write proxy for field `TXEMPTY`"]
pub struct TXEMPTY_W<'a> { w: &'a mut W }

impl<'a> TXEMPTY_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W { self.bit(true) }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W { self.bit(false) }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}

#[doc = "Write proxy for field `LINBK`"]
pub struct LINBK_W<'a> { w: &'a mut W }

impl<'a> LINBK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W { self.bit(true) }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W { self.bit(false) }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}

#[doc = "Write proxy for field `LINID`"]
pub struct LINID_W<'a> { w: &'a mut W }

impl<'a> LINID_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W { self.bit(true) }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W { self.bit(false) }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}

#[doc = "Write proxy for field `LINTC`"]
pub struct LINTC_W<'a> { w: &'a mut W }

impl<'a> LINTC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W { self.bit(true) }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W { self.bit(false) }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}

#[doc = "Write proxy for field `LINBE`"]
pub struct LINBE_W<'a> { w: &'a mut W }

impl<'a> LINBE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W { self.bit(true) }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W { self.bit(false) }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}

#[doc = "Write proxy for field `LINISFE`"]
pub struct LINISFE_W<'a> { w: &'a mut W }

impl<'a> LINISFE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W { self.bit(true) }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W { self.bit(false) }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}

#[doc = "Write proxy for field `LINIPE`"]
pub struct LINIPE_W<'a> { w: &'a mut W }

impl<'a> LINIPE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W { self.bit(true) }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W { self.bit(false) }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}

#[doc = "Write proxy for field `LINCE`"]
pub struct LINCE_W<'a> { w: &'a mut W }

impl<'a> LINCE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W { self.bit(true) }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W { self.bit(false) }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}

#[doc = "Write proxy for field `LINSNRE`"]
pub struct LINSNRE_W<'a> { w: &'a mut W }

impl<'a> LINSNRE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W { self.bit(true) }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W { self.bit(false) }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}

impl W {
    #[doc = "Bit 0 - RXRDY Interrupt Enable"]
    #[inline(always)]
    pub fn rxrdy(&mut self) -> RXRDY_W { RXRDY_W { w: self } }
    #[doc = "Bit 1 - TXRDY Interrupt Enable"]
    #[inline(always)]
    pub fn txrdy(&mut self) -> TXRDY_W { TXRDY_W { w: self } }
    #[doc = "Bit 5 - Overrun Error Interrupt Enable"]
    #[inline(always)]
    pub fn ovre(&mut self) -> OVRE_W { OVRE_W { w: self } }
    #[doc = "Bit 6 - Framing Error Interrupt Enable"]
    #[inline(always)]
    pub fn frame(&mut self) -> FRAME_W { FRAME_W { w: self } }
    #[doc = "Bit 7 - Parity Error Interrupt Enable"]
    #[inline(always)]
    pub fn pare(&mut self) -> PARE_W { PARE_W { w: self } }
    #[doc = "Bit 8 - Time-out Interrupt Enable"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TIMEOUT_W { TIMEOUT_W { w: self } }
    #[doc = "Bit 9 - TXEMPTY Interrupt Enable"]
    #[inline(always)]
    pub fn txempty(&mut self) -> TXEMPTY_W { TXEMPTY_W { w: self } }
    #[doc = "Bit 13 - LIN Break Sent or LIN Break Received Interrupt Enable"]
    #[inline(always)]
    pub fn linbk(&mut self) -> LINBK_W { LINBK_W { w: self } }
    #[doc = "Bit 14 - LIN Identifier Sent or LIN Identifier Received Interrupt Enable"]
    #[inline(always)]
    pub fn linid(&mut self) -> LINID_W { LINID_W { w: self } }
    #[doc = "Bit 15 - LIN Transfer Completed Interrupt Enable"]
    #[inline(always)]
    pub fn lintc(&mut self) -> LINTC_W { LINTC_W { w: self } }
    #[doc = "Bit 25 - LIN Bus Error Interrupt Enable"]
    #[inline(always)]
    pub fn linbe(&mut self) -> LINBE_W { LINBE_W { w: self } }
    #[doc = "Bit 26 - LIN Inconsistent Synch Field Error Interrupt Enable"]
    #[inline(always)]
    pub fn linisfe(&mut self) -> LINISFE_W { LINISFE_W { w: self } }
    #[doc = "Bit 27 - LIN Identifier Parity Interrupt Enable"]
    #[inline(always)]
    pub fn linipe(&mut self) -> LINIPE_W { LINIPE_W { w: self } }
    #[doc = "Bit 28 - LIN Checksum Error Interrupt Enable"]
    #[inline(always)]
    pub fn lince(&mut self) -> LINCE_W { LINCE_W { w: self } }
    #[doc = "Bit 29 - LIN Slave Not Responding Error Interrupt Enable"]
    #[inline(always)]
    pub fn linsnre(&mut self) -> LINSNRE_W { LINSNRE_W { w: self } }
}