#[doc = "Reader of register MFID6"]
pub type R = crate::R<u32, super::MFID6>;
#[doc = "Reader of field `MFID`"]
pub type MFID_R = crate::R<u32, u32>;

impl R {
    #[doc = "Bits 0:28 - Family ID"]
    #[inline(always)]
    pub fn mfid(&self) -> MFID_R { MFID_R::new((self.bits & 0x1fff_ffff) as u32) }
}