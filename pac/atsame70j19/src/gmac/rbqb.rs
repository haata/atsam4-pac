#[doc = "Register `RBQB` reader"]
pub struct R(crate::R<RBQB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RBQB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RBQB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RBQB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RBQB` writer"]
pub struct W(crate::W<RBQB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RBQB_SPEC>;
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
impl From<crate::W<RBQB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RBQB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - Receive Buffer Queue Base Address"]
pub type ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADDR` writer - Receive Buffer Queue Base Address"]
pub type ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RBQB_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bits 2:31 - Receive Buffer Queue Base Address"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Receive Buffer Queue Base Address"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<2> {
        ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Buffer Queue Base Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbqb](index.html) module"]
pub struct RBQB_SPEC;
impl crate::RegisterSpec for RBQB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rbqb::R](R) reader structure"]
impl crate::Readable for RBQB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rbqb::W](W) writer structure"]
impl crate::Writable for RBQB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RBQB to value 0"]
impl crate::Resettable for RBQB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
