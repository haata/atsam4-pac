#[doc = "Register `US_RHR` reader"]
pub struct R(crate::R<US_RHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<US_RHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<US_RHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<US_RHR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXCHR` reader - Received Character"]
pub type RXCHR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RXSYNH` reader - Received Sync"]
pub type RXSYNH_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:8 - Received Character"]
    #[inline(always)]
    pub fn rxchr(&self) -> RXCHR_R {
        RXCHR_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 15 - Received Sync"]
    #[inline(always)]
    pub fn rxsynh(&self) -> RXSYNH_R {
        RXSYNH_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Receive Holding Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_rhr](index.html) module"]
pub struct US_RHR_SPEC;
impl crate::RegisterSpec for US_RHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [us_rhr::R](R) reader structure"]
impl crate::Readable for US_RHR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets US_RHR to value 0"]
impl crate::Resettable for US_RHR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
