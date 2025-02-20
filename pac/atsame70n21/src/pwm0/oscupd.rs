#[doc = "Register `OSCUPD` writer"]
pub struct W(crate::W<OSCUPD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSCUPD_SPEC>;
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
impl From<crate::W<OSCUPD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSCUPD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OSCUPH0` writer - Output Selection Clear for PWMH output of the channel 0"]
pub type OSCUPH0_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCUPD_SPEC, bool, O>;
#[doc = "Field `OSCUPH1` writer - Output Selection Clear for PWMH output of the channel 1"]
pub type OSCUPH1_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCUPD_SPEC, bool, O>;
#[doc = "Field `OSCUPH2` writer - Output Selection Clear for PWMH output of the channel 2"]
pub type OSCUPH2_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCUPD_SPEC, bool, O>;
#[doc = "Field `OSCUPH3` writer - Output Selection Clear for PWMH output of the channel 3"]
pub type OSCUPH3_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCUPD_SPEC, bool, O>;
#[doc = "Field `OSCUPL0` writer - Output Selection Clear for PWML output of the channel 0"]
pub type OSCUPL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCUPD_SPEC, bool, O>;
#[doc = "Field `OSCUPL1` writer - Output Selection Clear for PWML output of the channel 1"]
pub type OSCUPL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCUPD_SPEC, bool, O>;
#[doc = "Field `OSCUPL2` writer - Output Selection Clear for PWML output of the channel 2"]
pub type OSCUPL2_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCUPD_SPEC, bool, O>;
#[doc = "Field `OSCUPL3` writer - Output Selection Clear for PWML output of the channel 3"]
pub type OSCUPL3_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCUPD_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Output Selection Clear for PWMH output of the channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn oscuph0(&mut self) -> OSCUPH0_W<0> {
        OSCUPH0_W::new(self)
    }
    #[doc = "Bit 1 - Output Selection Clear for PWMH output of the channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn oscuph1(&mut self) -> OSCUPH1_W<1> {
        OSCUPH1_W::new(self)
    }
    #[doc = "Bit 2 - Output Selection Clear for PWMH output of the channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn oscuph2(&mut self) -> OSCUPH2_W<2> {
        OSCUPH2_W::new(self)
    }
    #[doc = "Bit 3 - Output Selection Clear for PWMH output of the channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn oscuph3(&mut self) -> OSCUPH3_W<3> {
        OSCUPH3_W::new(self)
    }
    #[doc = "Bit 16 - Output Selection Clear for PWML output of the channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn oscupl0(&mut self) -> OSCUPL0_W<16> {
        OSCUPL0_W::new(self)
    }
    #[doc = "Bit 17 - Output Selection Clear for PWML output of the channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn oscupl1(&mut self) -> OSCUPL1_W<17> {
        OSCUPL1_W::new(self)
    }
    #[doc = "Bit 18 - Output Selection Clear for PWML output of the channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn oscupl2(&mut self) -> OSCUPL2_W<18> {
        OSCUPL2_W::new(self)
    }
    #[doc = "Bit 19 - Output Selection Clear for PWML output of the channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn oscupl3(&mut self) -> OSCUPL3_W<19> {
        OSCUPL3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Output Selection Clear Update Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oscupd](index.html) module"]
pub struct OSCUPD_SPEC;
impl crate::RegisterSpec for OSCUPD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [oscupd::W](W) writer structure"]
impl crate::Writable for OSCUPD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSCUPD to value 0"]
impl crate::Resettable for OSCUPD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
