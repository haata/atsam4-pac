#[doc = "Register `EFTSL` reader"]
pub struct R(crate::R<EFTSL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EFTSL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EFTSL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EFTSL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RUD` reader - Register Update"]
pub type RUD_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Register Update"]
    #[inline(always)]
    pub fn rud(&self) -> RUD_R {
        RUD_R::new(self.bits)
    }
}
#[doc = "PTP Event Frame Transmitted Seconds Low Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eftsl](index.html) module"]
pub struct EFTSL_SPEC;
impl crate::RegisterSpec for EFTSL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eftsl::R](R) reader structure"]
impl crate::Readable for EFTSL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EFTSL to value 0"]
impl crate::Resettable for EFTSL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
