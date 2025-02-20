#[doc = "Register `UIHVAL[%s]` writer"]
pub struct W(crate::W<UIHVAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UIHVAL_SPEC>;
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
impl From<crate::W<UIHVAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UIHVAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VAL` writer - Initial Hash Value"]
pub type VAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, UIHVAL_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Initial Hash Value"]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> VAL_W<0> {
        VAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "User Initial Hash Value 0 Register 0\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uihval](index.html) module"]
pub struct UIHVAL_SPEC;
impl crate::RegisterSpec for UIHVAL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [uihval::W](W) writer structure"]
impl crate::Writable for UIHVAL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UIHVAL[%s]
to value 0"]
impl crate::Resettable for UIHVAL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
