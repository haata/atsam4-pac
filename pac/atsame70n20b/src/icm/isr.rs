#[doc = "Register `ISR` reader"]
pub struct R(crate::R<ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RHC` reader - Region Hash Completed"]
pub type RHC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDM` reader - Region Digest Mismatch"]
pub type RDM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RBE` reader - Region Bus Error"]
pub type RBE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RWC` reader - Region Wrap Condition Detected"]
pub type RWC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REC` reader - Region End bit Condition Detected"]
pub type REC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSU` reader - Region Status Updated Detected"]
pub type RSU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `URAD` reader - Undefined Register Access Detection Status"]
pub type URAD_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:3 - Region Hash Completed"]
    #[inline(always)]
    pub fn rhc(&self) -> RHC_R {
        RHC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Region Digest Mismatch"]
    #[inline(always)]
    pub fn rdm(&self) -> RDM_R {
        RDM_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Region Bus Error"]
    #[inline(always)]
    pub fn rbe(&self) -> RBE_R {
        RBE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Region Wrap Condition Detected"]
    #[inline(always)]
    pub fn rwc(&self) -> RWC_R {
        RWC_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Region End bit Condition Detected"]
    #[inline(always)]
    pub fn rec(&self) -> REC_R {
        REC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Region Status Updated Detected"]
    #[inline(always)]
    pub fn rsu(&self) -> RSU_R {
        RSU_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Undefined Register Access Detection Status"]
    #[inline(always)]
    pub fn urad(&self) -> URAD_R {
        URAD_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](index.html) module"]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isr::R](R) reader structure"]
impl crate::Readable for ISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
