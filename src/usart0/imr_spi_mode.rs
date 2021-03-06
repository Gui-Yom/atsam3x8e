#[doc = "Reader of register IMR_SPI_MODE"]
pub type R = crate::R<u32, super::IMR_SPI_MODE>;
#[doc = "Reader of field `RXRDY`"]
pub type RXRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXRDY`"]
pub type TXRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVRE`"]
pub type OVRE_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXEMPTY`"]
pub type TXEMPTY_R = crate::R<bool, bool>;
#[doc = "Reader of field `UNRE`"]
pub type UNRE_R = crate::R<bool, bool>;

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
    #[doc = "Bit 9 - TXEMPTY Interrupt Mask"]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R { TXEMPTY_R::new(((self.bits >> 9) & 0x01) != 0) }
    #[doc = "Bit 10 - SPI Underrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn unre(&self) -> UNRE_R { UNRE_R::new(((self.bits >> 10) & 0x01) != 0) }
}