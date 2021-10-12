#[doc = "Register `UNAME2` reader"]
pub struct R(crate::R<UNAME2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UNAME2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UNAME2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UNAME2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `UNAME2` reader - IP Name Part Two"]
pub struct UNAME2_R(crate::FieldReader<u32, u32>);
impl UNAME2_R {
    pub(crate) fn new(bits: u32) -> Self {
        UNAME2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UNAME2_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - IP Name Part Two"]
    #[inline(always)]
    pub fn uname2(&self) -> UNAME2_R {
        UNAME2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "IP Name Part Two: HOST\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uname2](index.html) module"]
pub struct UNAME2_SPEC;
impl crate::RegisterSpec for UNAME2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uname2::R](R) reader structure"]
impl crate::Readable for UNAME2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UNAME2 to value 0x484f_5354"]
impl crate::Resettable for UNAME2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x484f_5354
    }
}
