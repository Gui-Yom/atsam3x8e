#[doc = "Reader of register IMR"]
pub type R = crate::R<u32, super::IMR>;
#[doc = "Reader of field `RDRF`"]
pub type RDRF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TDRE`"]
pub type TDRE_R = crate::R<bool, bool>;
#[doc = "Reader of field `MODF`"]
pub type MODF_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVRES`"]
pub type OVRES_R = crate::R<bool, bool>;
#[doc = "Reader of field `NSSR`"]
pub type NSSR_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXEMPTY`"]
pub type TXEMPTY_R = crate::R<bool, bool>;
#[doc = "Reader of field `UNDES`"]
pub type UNDES_R = crate::R<bool, bool>;

impl R {
    #[doc = "Bit 0 - Receive Data Register Full Interrupt Mask"]
    #[inline(always)]
    pub fn rdrf(&self) -> RDRF_R { RDRF_R::new((self.bits & 0x01) != 0) }
    #[doc = "Bit 1 - SPI Transmit Data Register Empty Interrupt Mask"]
    #[inline(always)]
    pub fn tdre(&self) -> TDRE_R { TDRE_R::new(((self.bits >> 1) & 0x01) != 0) }
    #[doc = "Bit 2 - Mode Fault Error Interrupt Mask"]
    #[inline(always)]
    pub fn modf(&self) -> MODF_R { MODF_R::new(((self.bits >> 2) & 0x01) != 0) }
    #[doc = "Bit 3 - Overrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn ovres(&self) -> OVRES_R { OVRES_R::new(((self.bits >> 3) & 0x01) != 0) }
    #[doc = "Bit 8 - NSS Rising Interrupt Mask"]
    #[inline(always)]
    pub fn nssr(&self) -> NSSR_R { NSSR_R::new(((self.bits >> 8) & 0x01) != 0) }
    #[doc = "Bit 9 - Transmission Registers Empty Mask"]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R { TXEMPTY_R::new(((self.bits >> 9) & 0x01) != 0) }
    #[doc = "Bit 10 - Underrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn undes(&self) -> UNDES_R { UNDES_R::new(((self.bits >> 10) & 0x01) != 0) }
}