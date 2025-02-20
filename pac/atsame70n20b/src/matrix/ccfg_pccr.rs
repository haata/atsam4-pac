#[doc = "Register `CCFG_PCCR` reader"]
pub struct R(crate::R<CCFG_PCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCFG_PCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCFG_PCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCFG_PCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCFG_PCCR` writer"]
pub struct W(crate::W<CCFG_PCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCFG_PCCR_SPEC>;
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
impl From<crate::W<CCFG_PCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCFG_PCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TC0CC` reader - TC0 Clock Configuration"]
pub type TC0CC_R = crate::BitReader<bool>;
#[doc = "Field `TC0CC` writer - TC0 Clock Configuration"]
pub type TC0CC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCFG_PCCR_SPEC, bool, O>;
#[doc = "Field `I2SC0CC` reader - I2SC0 Clock Configuration"]
pub type I2SC0CC_R = crate::BitReader<bool>;
#[doc = "Field `I2SC0CC` writer - I2SC0 Clock Configuration"]
pub type I2SC0CC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCFG_PCCR_SPEC, bool, O>;
#[doc = "Field `I2SC1CC` reader - I2SC1 Clock Configuration"]
pub type I2SC1CC_R = crate::BitReader<bool>;
#[doc = "Field `I2SC1CC` writer - I2SC1 Clock Configuration"]
pub type I2SC1CC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCFG_PCCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 20 - TC0 Clock Configuration"]
    #[inline(always)]
    pub fn tc0cc(&self) -> TC0CC_R {
        TC0CC_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2SC0 Clock Configuration"]
    #[inline(always)]
    pub fn i2sc0cc(&self) -> I2SC0CC_R {
        I2SC0CC_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2SC1 Clock Configuration"]
    #[inline(always)]
    pub fn i2sc1cc(&self) -> I2SC1CC_R {
        I2SC1CC_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 20 - TC0 Clock Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn tc0cc(&mut self) -> TC0CC_W<20> {
        TC0CC_W::new(self)
    }
    #[doc = "Bit 21 - I2SC0 Clock Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn i2sc0cc(&mut self) -> I2SC0CC_W<21> {
        I2SC0CC_W::new(self)
    }
    #[doc = "Bit 22 - I2SC1 Clock Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn i2sc1cc(&mut self) -> I2SC1CC_W<22> {
        I2SC1CC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Clock Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccfg_pccr](index.html) module"]
pub struct CCFG_PCCR_SPEC;
impl crate::RegisterSpec for CCFG_PCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccfg_pccr::R](R) reader structure"]
impl crate::Readable for CCFG_PCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccfg_pccr::W](W) writer structure"]
impl crate::Writable for CCFG_PCCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCFG_PCCR to value 0"]
impl crate::Resettable for CCFG_PCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
