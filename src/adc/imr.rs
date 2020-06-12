#[doc = "Reader of register IMR"]
pub type R = crate::R<u32, super::IMR>;
#[doc = "Reader of field `EOC0`"]
pub type EOC0_R = crate::R<bool, bool>;
#[doc = "Reader of field `EOC1`"]
pub type EOC1_R = crate::R<bool, bool>;
#[doc = "Reader of field `EOC2`"]
pub type EOC2_R = crate::R<bool, bool>;
#[doc = "Reader of field `EOC3`"]
pub type EOC3_R = crate::R<bool, bool>;
#[doc = "Reader of field `EOC4`"]
pub type EOC4_R = crate::R<bool, bool>;
#[doc = "Reader of field `EOC5`"]
pub type EOC5_R = crate::R<bool, bool>;
#[doc = "Reader of field `EOC6`"]
pub type EOC6_R = crate::R<bool, bool>;
#[doc = "Reader of field `EOC7`"]
pub type EOC7_R = crate::R<bool, bool>;
#[doc = "Reader of field `EOC8`"]
pub type EOC8_R = crate::R<bool, bool>;
#[doc = "Reader of field `EOC9`"]
pub type EOC9_R = crate::R<bool, bool>;
#[doc = "Reader of field `EOC10`"]
pub type EOC10_R = crate::R<bool, bool>;
#[doc = "Reader of field `EOC11`"]
pub type EOC11_R = crate::R<bool, bool>;
#[doc = "Reader of field `EOC12`"]
pub type EOC12_R = crate::R<bool, bool>;
#[doc = "Reader of field `EOC13`"]
pub type EOC13_R = crate::R<bool, bool>;
#[doc = "Reader of field `EOC14`"]
pub type EOC14_R = crate::R<bool, bool>;
#[doc = "Reader of field `EOC15`"]
pub type EOC15_R = crate::R<bool, bool>;
#[doc = "Reader of field `DRDY`"]
pub type DRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `GOVRE`"]
pub type GOVRE_R = crate::R<bool, bool>;
#[doc = "Reader of field `COMPE`"]
pub type COMPE_R = crate::R<bool, bool>;
#[doc = "Reader of field `ENDRX`"]
pub type ENDRX_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXBUFF`"]
pub type RXBUFF_R = crate::R<bool, bool>;

impl R {
    #[doc = "Bit 0 - End of Conversion Interrupt Mask 0"]
    #[inline(always)]
    pub fn eoc0(&self) -> EOC0_R { EOC0_R::new((self.bits & 0x01) != 0) }
    #[doc = "Bit 1 - End of Conversion Interrupt Mask 1"]
    #[inline(always)]
    pub fn eoc1(&self) -> EOC1_R { EOC1_R::new(((self.bits >> 1) & 0x01) != 0) }
    #[doc = "Bit 2 - End of Conversion Interrupt Mask 2"]
    #[inline(always)]
    pub fn eoc2(&self) -> EOC2_R { EOC2_R::new(((self.bits >> 2) & 0x01) != 0) }
    #[doc = "Bit 3 - End of Conversion Interrupt Mask 3"]
    #[inline(always)]
    pub fn eoc3(&self) -> EOC3_R { EOC3_R::new(((self.bits >> 3) & 0x01) != 0) }
    #[doc = "Bit 4 - End of Conversion Interrupt Mask 4"]
    #[inline(always)]
    pub fn eoc4(&self) -> EOC4_R { EOC4_R::new(((self.bits >> 4) & 0x01) != 0) }
    #[doc = "Bit 5 - End of Conversion Interrupt Mask 5"]
    #[inline(always)]
    pub fn eoc5(&self) -> EOC5_R { EOC5_R::new(((self.bits >> 5) & 0x01) != 0) }
    #[doc = "Bit 6 - End of Conversion Interrupt Mask 6"]
    #[inline(always)]
    pub fn eoc6(&self) -> EOC6_R { EOC6_R::new(((self.bits >> 6) & 0x01) != 0) }
    #[doc = "Bit 7 - End of Conversion Interrupt Mask 7"]
    #[inline(always)]
    pub fn eoc7(&self) -> EOC7_R { EOC7_R::new(((self.bits >> 7) & 0x01) != 0) }
    #[doc = "Bit 8 - End of Conversion Interrupt Mask 8"]
    #[inline(always)]
    pub fn eoc8(&self) -> EOC8_R { EOC8_R::new(((self.bits >> 8) & 0x01) != 0) }
    #[doc = "Bit 9 - End of Conversion Interrupt Mask 9"]
    #[inline(always)]
    pub fn eoc9(&self) -> EOC9_R { EOC9_R::new(((self.bits >> 9) & 0x01) != 0) }
    #[doc = "Bit 10 - End of Conversion Interrupt Mask 10"]
    #[inline(always)]
    pub fn eoc10(&self) -> EOC10_R { EOC10_R::new(((self.bits >> 10) & 0x01) != 0) }
    #[doc = "Bit 11 - End of Conversion Interrupt Mask 11"]
    #[inline(always)]
    pub fn eoc11(&self) -> EOC11_R { EOC11_R::new(((self.bits >> 11) & 0x01) != 0) }
    #[doc = "Bit 12 - End of Conversion Interrupt Mask 12"]
    #[inline(always)]
    pub fn eoc12(&self) -> EOC12_R { EOC12_R::new(((self.bits >> 12) & 0x01) != 0) }
    #[doc = "Bit 13 - End of Conversion Interrupt Mask 13"]
    #[inline(always)]
    pub fn eoc13(&self) -> EOC13_R { EOC13_R::new(((self.bits >> 13) & 0x01) != 0) }
    #[doc = "Bit 14 - End of Conversion Interrupt Mask 14"]
    #[inline(always)]
    pub fn eoc14(&self) -> EOC14_R { EOC14_R::new(((self.bits >> 14) & 0x01) != 0) }
    #[doc = "Bit 15 - End of Conversion Interrupt Mask 15"]
    #[inline(always)]
    pub fn eoc15(&self) -> EOC15_R { EOC15_R::new(((self.bits >> 15) & 0x01) != 0) }
    #[doc = "Bit 24 - Data Ready Interrupt Mask"]
    #[inline(always)]
    pub fn drdy(&self) -> DRDY_R { DRDY_R::new(((self.bits >> 24) & 0x01) != 0) }
    #[doc = "Bit 25 - General Overrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn govre(&self) -> GOVRE_R { GOVRE_R::new(((self.bits >> 25) & 0x01) != 0) }
    #[doc = "Bit 26 - Comparison Event Interrupt Mask"]
    #[inline(always)]
    pub fn compe(&self) -> COMPE_R { COMPE_R::new(((self.bits >> 26) & 0x01) != 0) }
    #[doc = "Bit 27 - End of Receive Buffer Interrupt Mask"]
    #[inline(always)]
    pub fn endrx(&self) -> ENDRX_R { ENDRX_R::new(((self.bits >> 27) & 0x01) != 0) }
    #[doc = "Bit 28 - Receive Buffer Full Interrupt Mask"]
    #[inline(always)]
    pub fn rxbuff(&self) -> RXBUFF_R { RXBUFF_R::new(((self.bits >> 28) & 0x01) != 0) }
}