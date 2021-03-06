#[doc = "Reader of register IMR_LIN_MODE"]
pub type R = crate::R<u32, super::IMR_LIN_MODE>;
#[doc = "Reader of field `RXRDY`"]
pub type RXRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXRDY`"]
pub type TXRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVRE`"]
pub type OVRE_R = crate::R<bool, bool>;
#[doc = "Reader of field `FRAME`"]
pub type FRAME_R = crate::R<bool, bool>;
#[doc = "Reader of field `PARE`"]
pub type PARE_R = crate::R<bool, bool>;
#[doc = "Reader of field `TIMEOUT`"]
pub type TIMEOUT_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXEMPTY`"]
pub type TXEMPTY_R = crate::R<bool, bool>;
#[doc = "Reader of field `LINBK`"]
pub type LINBK_R = crate::R<bool, bool>;
#[doc = "Reader of field `LINID`"]
pub type LINID_R = crate::R<bool, bool>;
#[doc = "Reader of field `LINTC`"]
pub type LINTC_R = crate::R<bool, bool>;
#[doc = "Reader of field `LINBE`"]
pub type LINBE_R = crate::R<bool, bool>;
#[doc = "Reader of field `LINISFE`"]
pub type LINISFE_R = crate::R<bool, bool>;
#[doc = "Reader of field `LINIPE`"]
pub type LINIPE_R = crate::R<bool, bool>;
#[doc = "Reader of field `LINCE`"]
pub type LINCE_R = crate::R<bool, bool>;
#[doc = "Reader of field `LINSNRE`"]
pub type LINSNRE_R = crate::R<bool, bool>;

impl R {
    #[doc = "Bit 0 - RXRDY Interrupt Mask"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RXRDY_R { RXRDY_R::new((self.bits & 0x01) != 0) }
    #[doc = "Bit 1 - TXRDY Interrupt Mask"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R { TXRDY_R::new(((self.bits >> 1) & 0x01) != 0) }
    #[doc = "Bit 5 - Overrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn ovre(&self) -> OVRE_R { OVRE_R::new(((self.bits >> 5) & 0x01) != 0) }
    #[doc = "Bit 6 - Framing Error Interrupt Mask"]
    #[inline(always)]
    pub fn frame(&self) -> FRAME_R { FRAME_R::new(((self.bits >> 6) & 0x01) != 0) }
    #[doc = "Bit 7 - Parity Error Interrupt Mask"]
    #[inline(always)]
    pub fn pare(&self) -> PARE_R { PARE_R::new(((self.bits >> 7) & 0x01) != 0) }
    #[doc = "Bit 8 - Time-out Interrupt Mask"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R { TIMEOUT_R::new(((self.bits >> 8) & 0x01) != 0) }
    #[doc = "Bit 9 - TXEMPTY Interrupt Mask"]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R { TXEMPTY_R::new(((self.bits >> 9) & 0x01) != 0) }
    #[doc = "Bit 13 - LIN Break Sent or LIN Break Received Interrupt Mask"]
    #[inline(always)]
    pub fn linbk(&self) -> LINBK_R { LINBK_R::new(((self.bits >> 13) & 0x01) != 0) }
    #[doc = "Bit 14 - LIN Identifier Sent or LIN Identifier Received Interrupt Mask"]
    #[inline(always)]
    pub fn linid(&self) -> LINID_R { LINID_R::new(((self.bits >> 14) & 0x01) != 0) }
    #[doc = "Bit 15 - LIN Transfer Completed Interrupt Mask"]
    #[inline(always)]
    pub fn lintc(&self) -> LINTC_R { LINTC_R::new(((self.bits >> 15) & 0x01) != 0) }
    #[doc = "Bit 25 - LIN Bus Error Interrupt Mask"]
    #[inline(always)]
    pub fn linbe(&self) -> LINBE_R { LINBE_R::new(((self.bits >> 25) & 0x01) != 0) }
    #[doc = "Bit 26 - LIN Inconsistent Synch Field Error Interrupt Mask"]
    #[inline(always)]
    pub fn linisfe(&self) -> LINISFE_R { LINISFE_R::new(((self.bits >> 26) & 0x01) != 0) }
    #[doc = "Bit 27 - LIN Identifier Parity Interrupt Mask"]
    #[inline(always)]
    pub fn linipe(&self) -> LINIPE_R { LINIPE_R::new(((self.bits >> 27) & 0x01) != 0) }
    #[doc = "Bit 28 - LIN Checksum Error Interrupt Mask"]
    #[inline(always)]
    pub fn lince(&self) -> LINCE_R { LINCE_R::new(((self.bits >> 28) & 0x01) != 0) }
    #[doc = "Bit 29 - LIN Slave Not Responding Error Interrupt Mask"]
    #[inline(always)]
    pub fn linsnre(&self) -> LINSNRE_R { LINSNRE_R::new(((self.bits >> 29) & 0x01) != 0) }
}