#[doc = "Reader of register PCSR0"]
pub type R = crate::R<u32, super::PCSR0>;
#[doc = "Reader of field `PID7`"]
pub type PID7_R = crate::R<bool, bool>;
#[doc = "Reader of field `PID8`"]
pub type PID8_R = crate::R<bool, bool>;
#[doc = "Reader of field `PID10`"]
pub type PID10_R = crate::R<bool, bool>;
#[doc = "Reader of field `PID11`"]
pub type PID11_R = crate::R<bool, bool>;
#[doc = "Reader of field `PID13`"]
pub type PID13_R = crate::R<bool, bool>;
#[doc = "Reader of field `PID14`"]
pub type PID14_R = crate::R<bool, bool>;
#[doc = "Reader of field `PID16`"]
pub type PID16_R = crate::R<bool, bool>;
#[doc = "Reader of field `PID19`"]
pub type PID19_R = crate::R<bool, bool>;
#[doc = "Reader of field `PID20`"]
pub type PID20_R = crate::R<bool, bool>;
#[doc = "Reader of field `PID22`"]
pub type PID22_R = crate::R<bool, bool>;
#[doc = "Reader of field `PID23`"]
pub type PID23_R = crate::R<bool, bool>;
#[doc = "Reader of field `PID24`"]
pub type PID24_R = crate::R<bool, bool>;
#[doc = "Reader of field `PID25`"]
pub type PID25_R = crate::R<bool, bool>;
#[doc = "Reader of field `PID26`"]
pub type PID26_R = crate::R<bool, bool>;
#[doc = "Reader of field `PID27`"]
pub type PID27_R = crate::R<bool, bool>;
#[doc = "Reader of field `PID28`"]
pub type PID28_R = crate::R<bool, bool>;
#[doc = "Reader of field `PID29`"]
pub type PID29_R = crate::R<bool, bool>;
#[doc = "Reader of field `PID30`"]
pub type PID30_R = crate::R<bool, bool>;
#[doc = "Reader of field `PID31`"]
pub type PID31_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 7 - Peripheral Clock 7 Status"]
    #[inline(always)]
    pub fn pid7(&self) -> PID7_R {
        PID7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Peripheral Clock 8 Status"]
    #[inline(always)]
    pub fn pid8(&self) -> PID8_R {
        PID8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Peripheral Clock 10 Status"]
    #[inline(always)]
    pub fn pid10(&self) -> PID10_R {
        PID10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Peripheral Clock 11 Status"]
    #[inline(always)]
    pub fn pid11(&self) -> PID11_R {
        PID11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Peripheral Clock 13 Status"]
    #[inline(always)]
    pub fn pid13(&self) -> PID13_R {
        PID13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Peripheral Clock 14 Status"]
    #[inline(always)]
    pub fn pid14(&self) -> PID14_R {
        PID14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Peripheral Clock 16 Status"]
    #[inline(always)]
    pub fn pid16(&self) -> PID16_R {
        PID16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Peripheral Clock 19 Status"]
    #[inline(always)]
    pub fn pid19(&self) -> PID19_R {
        PID19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Peripheral Clock 20 Status"]
    #[inline(always)]
    pub fn pid20(&self) -> PID20_R {
        PID20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Peripheral Clock 22 Status"]
    #[inline(always)]
    pub fn pid22(&self) -> PID22_R {
        PID22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Peripheral Clock 23 Status"]
    #[inline(always)]
    pub fn pid23(&self) -> PID23_R {
        PID23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Peripheral Clock 24 Status"]
    #[inline(always)]
    pub fn pid24(&self) -> PID24_R {
        PID24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Peripheral Clock 25 Status"]
    #[inline(always)]
    pub fn pid25(&self) -> PID25_R {
        PID25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Peripheral Clock 26 Status"]
    #[inline(always)]
    pub fn pid26(&self) -> PID26_R {
        PID26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Peripheral Clock 27 Status"]
    #[inline(always)]
    pub fn pid27(&self) -> PID27_R {
        PID27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Peripheral Clock 28 Status"]
    #[inline(always)]
    pub fn pid28(&self) -> PID28_R {
        PID28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Peripheral Clock 29 Status"]
    #[inline(always)]
    pub fn pid29(&self) -> PID29_R {
        PID29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Peripheral Clock 30 Status"]
    #[inline(always)]
    pub fn pid30(&self) -> PID30_R {
        PID30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Peripheral Clock 31 Status"]
    #[inline(always)]
    pub fn pid31(&self) -> PID31_R {
        PID31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
