#[doc = "Register `US_FIDI_LON_MODE` reader"]
pub struct R(crate::R<US_FIDI_LON_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<US_FIDI_LON_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<US_FIDI_LON_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<US_FIDI_LON_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `US_FIDI_LON_MODE` writer"]
pub struct W(crate::W<US_FIDI_LON_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<US_FIDI_LON_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<US_FIDI_LON_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<US_FIDI_LON_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BETA2` reader - LON BETA2 Length"]
pub type BETA2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `BETA2` writer - LON BETA2 Length"]
pub type BETA2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, US_FIDI_LON_MODE_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:23 - LON BETA2 Length"]
    #[inline(always)]
    pub fn beta2(&self) -> BETA2_R {
        BETA2_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - LON BETA2 Length"]
    #[inline(always)]
    #[must_use]
    pub fn beta2(&mut self) -> BETA2_W<0> {
        BETA2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FI DI Ratio Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_fidi_lon_mode](index.html) module"]
pub struct US_FIDI_LON_MODE_SPEC;
impl crate::RegisterSpec for US_FIDI_LON_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [us_fidi_lon_mode::R](R) reader structure"]
impl crate::Readable for US_FIDI_LON_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [us_fidi_lon_mode::W](W) writer structure"]
impl crate::Writable for US_FIDI_LON_MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets US_FIDI_LON_MODE to value 0"]
impl crate::Resettable for US_FIDI_LON_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
