#[doc = "Reader of register ECC_SR2"]
pub type R = crate::R<u32, super::ECC_SR2>;
#[doc = "Reader of field `RECERR8`"]
pub type RECERR8_R = crate::R<bool, bool>;
#[doc = "Reader of field `ECCERR8`"]
pub type ECCERR8_R = crate::R<u8, u8>;
#[doc = "Reader of field `RECERR9`"]
pub type RECERR9_R = crate::R<bool, bool>;
#[doc = "Reader of field `ECCERR9`"]
pub type ECCERR9_R = crate::R<bool, bool>;
#[doc = "Reader of field `MULERR9`"]
pub type MULERR9_R = crate::R<bool, bool>;
#[doc = "Reader of field `RECERR10`"]
pub type RECERR10_R = crate::R<bool, bool>;
#[doc = "Reader of field `ECCERR10`"]
pub type ECCERR10_R = crate::R<bool, bool>;
#[doc = "Reader of field `MULERR10`"]
pub type MULERR10_R = crate::R<bool, bool>;
#[doc = "Reader of field `RECERR11`"]
pub type RECERR11_R = crate::R<bool, bool>;
#[doc = "Reader of field `ECCERR11`"]
pub type ECCERR11_R = crate::R<bool, bool>;
#[doc = "Reader of field `MULERR11`"]
pub type MULERR11_R = crate::R<bool, bool>;
#[doc = "Reader of field `RECERR12`"]
pub type RECERR12_R = crate::R<bool, bool>;
#[doc = "Reader of field `ECCERR12`"]
pub type ECCERR12_R = crate::R<u8, u8>;
#[doc = "Reader of field `RECERR13`"]
pub type RECERR13_R = crate::R<bool, bool>;
#[doc = "Reader of field `ECCERR13`"]
pub type ECCERR13_R = crate::R<u8, u8>;
#[doc = "Reader of field `RECERR14`"]
pub type RECERR14_R = crate::R<bool, bool>;
#[doc = "Reader of field `ECCERR14`"]
pub type ECCERR14_R = crate::R<u8, u8>;
#[doc = "Reader of field `RECERR15`"]
pub type RECERR15_R = crate::R<bool, bool>;
#[doc = "Reader of field `ECCERR15`"]
pub type ECCERR15_R = crate::R<u8, u8>;

impl R {
    #[doc = "Bit 0 - Recoverable Error in the page between the 2048th and the 2303rd bytes"]
    #[inline(always)]
    pub fn recerr8(&self) -> RECERR8_R { RECERR8_R::new((self.bits & 0x01) != 0) }
    #[doc = "Bits 1:2 - ECC Error in the page between the 2048th and the 2303rd bytes"]
    #[inline(always)]
    pub fn eccerr8(&self) -> ECCERR8_R { ECCERR8_R::new(((self.bits >> 1) & 0x03) as u8) }
    #[doc = "Bit 4 - Recoverable Error in the page between the 2304th and the 2559th bytes"]
    #[inline(always)]
    pub fn recerr9(&self) -> RECERR9_R { RECERR9_R::new(((self.bits >> 4) & 0x01) != 0) }
    #[doc = "Bit 5 - ECC Error in the page between the 2304th and the 2559th bytes"]
    #[inline(always)]
    pub fn eccerr9(&self) -> ECCERR9_R { ECCERR9_R::new(((self.bits >> 5) & 0x01) != 0) }
    #[doc = "Bit 6 - Multiple Error in the page between the 2304th and the 2559th bytes"]
    #[inline(always)]
    pub fn mulerr9(&self) -> MULERR9_R { MULERR9_R::new(((self.bits >> 6) & 0x01) != 0) }
    #[doc = "Bit 8 - Recoverable Error in the page between the 2560th and the 2815th bytes"]
    #[inline(always)]
    pub fn recerr10(&self) -> RECERR10_R { RECERR10_R::new(((self.bits >> 8) & 0x01) != 0) }
    #[doc = "Bit 9 - ECC Error in the page between the 2560th and the 2815th bytes"]
    #[inline(always)]
    pub fn eccerr10(&self) -> ECCERR10_R { ECCERR10_R::new(((self.bits >> 9) & 0x01) != 0) }
    #[doc = "Bit 10 - Multiple Error in the page between the 2560th and the 2815th bytes"]
    #[inline(always)]
    pub fn mulerr10(&self) -> MULERR10_R { MULERR10_R::new(((self.bits >> 10) & 0x01) != 0) }
    #[doc = "Bit 12 - Recoverable Error in the page between the 2816th and the 3071st bytes"]
    #[inline(always)]
    pub fn recerr11(&self) -> RECERR11_R { RECERR11_R::new(((self.bits >> 12) & 0x01) != 0) }
    #[doc = "Bit 13 - ECC Error in the page between the 2816th and the 3071st bytes"]
    #[inline(always)]
    pub fn eccerr11(&self) -> ECCERR11_R { ECCERR11_R::new(((self.bits >> 13) & 0x01) != 0) }
    #[doc = "Bit 14 - Multiple Error in the page between the 2816th and the 3071st bytes"]
    #[inline(always)]
    pub fn mulerr11(&self) -> MULERR11_R { MULERR11_R::new(((self.bits >> 14) & 0x01) != 0) }
    #[doc = "Bit 16 - Recoverable Error in the page between the 3072nd and the 3327th bytes"]
    #[inline(always)]
    pub fn recerr12(&self) -> RECERR12_R { RECERR12_R::new(((self.bits >> 16) & 0x01) != 0) }
    #[doc = "Bits 17:18 - ECC Error in the page between the 3072nd and the 3327th bytes"]
    #[inline(always)]
    pub fn eccerr12(&self) -> ECCERR12_R { ECCERR12_R::new(((self.bits >> 17) & 0x03) as u8) }
    #[doc = "Bit 20 - Recoverable Error in the page between the 3328th and the 3583rd bytes"]
    #[inline(always)]
    pub fn recerr13(&self) -> RECERR13_R { RECERR13_R::new(((self.bits >> 20) & 0x01) != 0) }
    #[doc = "Bits 21:22 - ECC Error in the page between the 3328th and the 3583rd bytes"]
    #[inline(always)]
    pub fn eccerr13(&self) -> ECCERR13_R { ECCERR13_R::new(((self.bits >> 21) & 0x03) as u8) }
    #[doc = "Bit 24 - Recoverable Error in the page between the 3584th and the 3839th bytes"]
    #[inline(always)]
    pub fn recerr14(&self) -> RECERR14_R { RECERR14_R::new(((self.bits >> 24) & 0x01) != 0) }
    #[doc = "Bits 25:26 - ECC Error in the page between the 3584th and the 3839th bytes"]
    #[inline(always)]
    pub fn eccerr14(&self) -> ECCERR14_R { ECCERR14_R::new(((self.bits >> 25) & 0x03) as u8) }
    #[doc = "Bit 28 - Recoverable Error in the page between the 3840th and the 4095th bytes"]
    #[inline(always)]
    pub fn recerr15(&self) -> RECERR15_R { RECERR15_R::new(((self.bits >> 28) & 0x01) != 0) }
    #[doc = "Bits 29:30 - ECC Error in the page between the 3840th and the 4095th bytes"]
    #[inline(always)]
    pub fn eccerr15(&self) -> ECCERR15_R { ECCERR15_R::new(((self.bits >> 29) & 0x03) as u8) }
}