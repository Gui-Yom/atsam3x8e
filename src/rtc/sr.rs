#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Reader of field `ACKUPD`"]
pub type ACKUPD_R = crate::R<bool, bool>;
#[doc = "Reader of field `ALARM`"]
pub type ALARM_R = crate::R<bool, bool>;
#[doc = "Reader of field `SEC`"]
pub type SEC_R = crate::R<bool, bool>;
#[doc = "Reader of field `TIMEV`"]
pub type TIMEV_R = crate::R<bool, bool>;
#[doc = "Reader of field `CALEV`"]
pub type CALEV_R = crate::R<bool, bool>;

impl R {
    #[doc = "Bit 0 - Acknowledge for Update"]
    #[inline(always)]
    pub fn ackupd(&self) -> ACKUPD_R { ACKUPD_R::new((self.bits & 0x01) != 0) }
    #[doc = "Bit 1 - Alarm Flag"]
    #[inline(always)]
    pub fn alarm(&self) -> ALARM_R { ALARM_R::new(((self.bits >> 1) & 0x01) != 0) }
    #[doc = "Bit 2 - Second Event"]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R { SEC_R::new(((self.bits >> 2) & 0x01) != 0) }
    #[doc = "Bit 3 - Time Event"]
    #[inline(always)]
    pub fn timev(&self) -> TIMEV_R { TIMEV_R::new(((self.bits >> 3) & 0x01) != 0) }
    #[doc = "Bit 4 - Calendar Event"]
    #[inline(always)]
    pub fn calev(&self) -> CALEV_R { CALEV_R::new(((self.bits >> 4) & 0x01) != 0) }
}