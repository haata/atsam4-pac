#[doc = "Register `TXFQS` reader"]
pub struct R(crate::R<TXFQS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXFQS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXFQS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXFQS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TFFL` reader - Tx FIFO Free Level"]
pub type TFFL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TFGI` reader - Tx FIFO Get Index"]
pub type TFGI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TFQPI` reader - Tx FIFO/Queue Put Index"]
pub type TFQPI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TFQF` reader - Tx FIFO/Queue Full"]
pub type TFQF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:5 - Tx FIFO Free Level"]
    #[inline(always)]
    pub fn tffl(&self) -> TFFL_R {
        TFFL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:12 - Tx FIFO Get Index"]
    #[inline(always)]
    pub fn tfgi(&self) -> TFGI_R {
        TFGI_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Tx FIFO/Queue Put Index"]
    #[inline(always)]
    pub fn tfqpi(&self) -> TFQPI_R {
        TFQPI_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 21 - Tx FIFO/Queue Full"]
    #[inline(always)]
    pub fn tfqf(&self) -> TFQF_R {
        TFQF_R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[doc = "Transmit FIFO/Queue Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txfqs](index.html) module"]
pub struct TXFQS_SPEC;
impl crate::RegisterSpec for TXFQS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txfqs::R](R) reader structure"]
impl crate::Readable for TXFQS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TXFQS to value 0"]
impl crate::Resettable for TXFQS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
