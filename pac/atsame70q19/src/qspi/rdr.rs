#[doc = "Register `RDR` reader"]
pub struct R(crate::R<RDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RD` reader - Receive Data"]
pub struct RD_R(crate::FieldReader<u16, u16>);
impl RD_R {
    pub(crate) fn new(bits: u16) -> Self {
        RD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Receive Data"]
    #[inline(always)]
    pub fn rd(&self) -> RD_R {
        RD_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Receive Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdr](index.html) module"]
pub struct RDR_SPEC;
impl crate::RegisterSpec for RDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rdr::R](R) reader structure"]
impl crate::Readable for RDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RDR to value 0"]
impl crate::Resettable for RDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
