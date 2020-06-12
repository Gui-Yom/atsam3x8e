#[doc = "Reader of register BLKR"]
pub type R = crate::R<u32, super::BLKR>;
#[doc = "Writer for register BLKR"]
pub type W = crate::W<u32, super::BLKR>;

#[doc = "Register BLKR `reset()`'s with value 0"]
impl crate::ResetValue for super::BLKR {
    #[inline(always)]
    fn reset_value() -> Self::Ux { 0 }
}

#[doc = "MMC/SDIO Block Count - SDIO Byte Count\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum BCNT_A { #[doc = "0: MMC/SDCARD Multiple BlockFrom 1 to 65635: Value 0 corresponds to an infinite block transfer."] MULTIPLE = 0, #[doc = "4: SDIO ByteFrom 1 to 512 bytes: Value 0 corresponds to a 512-byte transfer.Values from 0x200 to 0xFFFF are forbidden."] BYTE = 4, #[doc = "5: SDIO BlockFrom 1 to 511 blocks: Value 0 corresponds to an infinite block transfer.Values from 0x200 to 0xFFFF are forbidden."] BLOCK = 5 }

impl From<BCNT_A> for u16 {
    #[inline(always)]
    fn from(variant: BCNT_A) -> Self { variant as _ }
}

#[doc = "Reader of field `BCNT`"]
pub type BCNT_R = crate::R<u16, BCNT_A>;

impl BCNT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, BCNT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BCNT_A::MULTIPLE),
            4 => Val(BCNT_A::BYTE),
            5 => Val(BCNT_A::BLOCK),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MULTIPLE`"]
    #[inline(always)]
    pub fn is_multiple(&self) -> bool { *self == BCNT_A::MULTIPLE }
    #[doc = "Checks if the value of the field is `BYTE`"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool { *self == BCNT_A::BYTE }
    #[doc = "Checks if the value of the field is `BLOCK`"]
    #[inline(always)]
    pub fn is_block(&self) -> bool { *self == BCNT_A::BLOCK }
}

#[doc = "Write proxy for field `BCNT`"]
pub struct BCNT_W<'a> { w: &'a mut W }

impl<'a> BCNT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BCNT_A) -> &'a mut W { unsafe { self.bits(variant.into()) } }
    #[doc = "MMC/SDCARD Multiple BlockFrom 1 to 65635: Value 0 corresponds to an infinite block transfer."]
    #[inline(always)]
    pub fn multiple(self) -> &'a mut W { self.variant(BCNT_A::MULTIPLE) }
    #[doc = "SDIO ByteFrom 1 to 512 bytes: Value 0 corresponds to a 512-byte transfer.Values from 0x200 to 0xFFFF are forbidden."]
    #[inline(always)]
    pub fn byte(self) -> &'a mut W { self.variant(BCNT_A::BYTE) }
    #[doc = "SDIO BlockFrom 1 to 511 blocks: Value 0 corresponds to an infinite block transfer.Values from 0x200 to 0xFFFF are forbidden."]
    #[inline(always)]
    pub fn block(self) -> &'a mut W { self.variant(BCNT_A::BLOCK) }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}

#[doc = "Reader of field `BLKLEN`"]
pub type BLKLEN_R = crate::R<u16, u16>;

#[doc = "Write proxy for field `BLKLEN`"]
pub struct BLKLEN_W<'a> { w: &'a mut W }

impl<'a> BLKLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}

impl R {
    #[doc = "Bits 0:15 - MMC/SDIO Block Count - SDIO Byte Count"]
    #[inline(always)]
    pub fn bcnt(&self) -> BCNT_R { BCNT_R::new((self.bits & 0xffff) as u16) }
    #[doc = "Bits 16:31 - Data Block Length"]
    #[inline(always)]
    pub fn blklen(&self) -> BLKLEN_R { BLKLEN_R::new(((self.bits >> 16) & 0xffff) as u16) }
}

impl W {
    #[doc = "Bits 0:15 - MMC/SDIO Block Count - SDIO Byte Count"]
    #[inline(always)]
    pub fn bcnt(&mut self) -> BCNT_W { BCNT_W { w: self } }
    #[doc = "Bits 16:31 - Data Block Length"]
    #[inline(always)]
    pub fn blklen(&mut self) -> BLKLEN_W { BLKLEN_W { w: self } }
}