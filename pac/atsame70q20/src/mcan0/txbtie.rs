#[doc = "Register `TXBTIE` reader"]
pub struct R(crate::R<TXBTIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXBTIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXBTIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXBTIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXBTIE` writer"]
pub struct W(crate::W<TXBTIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXBTIE_SPEC>;
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
impl From<crate::W<TXBTIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXBTIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIE0` reader - Transmission Interrupt Enable for Buffer 0"]
pub type TIE0_R = crate::BitReader<bool>;
#[doc = "Field `TIE0` writer - Transmission Interrupt Enable for Buffer 0"]
pub type TIE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBTIE_SPEC, bool, O>;
#[doc = "Field `TIE1` reader - Transmission Interrupt Enable for Buffer 1"]
pub type TIE1_R = crate::BitReader<bool>;
#[doc = "Field `TIE1` writer - Transmission Interrupt Enable for Buffer 1"]
pub type TIE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBTIE_SPEC, bool, O>;
#[doc = "Field `TIE2` reader - Transmission Interrupt Enable for Buffer 2"]
pub type TIE2_R = crate::BitReader<bool>;
#[doc = "Field `TIE2` writer - Transmission Interrupt Enable for Buffer 2"]
pub type TIE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBTIE_SPEC, bool, O>;
#[doc = "Field `TIE3` reader - Transmission Interrupt Enable for Buffer 3"]
pub type TIE3_R = crate::BitReader<bool>;
#[doc = "Field `TIE3` writer - Transmission Interrupt Enable for Buffer 3"]
pub type TIE3_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBTIE_SPEC, bool, O>;
#[doc = "Field `TIE4` reader - Transmission Interrupt Enable for Buffer 4"]
pub type TIE4_R = crate::BitReader<bool>;
#[doc = "Field `TIE4` writer - Transmission Interrupt Enable for Buffer 4"]
pub type TIE4_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBTIE_SPEC, bool, O>;
#[doc = "Field `TIE5` reader - Transmission Interrupt Enable for Buffer 5"]
pub type TIE5_R = crate::BitReader<bool>;
#[doc = "Field `TIE5` writer - Transmission Interrupt Enable for Buffer 5"]
pub type TIE5_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBTIE_SPEC, bool, O>;
#[doc = "Field `TIE6` reader - Transmission Interrupt Enable for Buffer 6"]
pub type TIE6_R = crate::BitReader<bool>;
#[doc = "Field `TIE6` writer - Transmission Interrupt Enable for Buffer 6"]
pub type TIE6_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBTIE_SPEC, bool, O>;
#[doc = "Field `TIE7` reader - Transmission Interrupt Enable for Buffer 7"]
pub type TIE7_R = crate::BitReader<bool>;
#[doc = "Field `TIE7` writer - Transmission Interrupt Enable for Buffer 7"]
pub type TIE7_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBTIE_SPEC, bool, O>;
#[doc = "Field `TIE8` reader - Transmission Interrupt Enable for Buffer 8"]
pub type TIE8_R = crate::BitReader<bool>;
#[doc = "Field `TIE8` writer - Transmission Interrupt Enable for Buffer 8"]
pub type TIE8_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBTIE_SPEC, bool, O>;
#[doc = "Field `TIE9` reader - Transmission Interrupt Enable for Buffer 9"]
pub type TIE9_R = crate::BitReader<bool>;
#[doc = "Field `TIE9` writer - Transmission Interrupt Enable for Buffer 9"]
pub type TIE9_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBTIE_SPEC, bool, O>;
#[doc = "Field `TIE10` reader - Transmission Interrupt Enable for Buffer 10"]
pub type TIE10_R = crate::BitReader<bool>;
#[doc = "Field `TIE10` writer - Transmission Interrupt Enable for Buffer 10"]
pub type TIE10_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBTIE_SPEC, bool, O>;
#[doc = "Field `TIE11` reader - Transmission Interrupt Enable for Buffer 11"]
pub type TIE11_R = crate::BitReader<bool>;
#[doc = "Field `TIE11` writer - Transmission Interrupt Enable for Buffer 11"]
pub type TIE11_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBTIE_SPEC, bool, O>;
#[doc = "Field `TIE12` reader - Transmission Interrupt Enable for Buffer 12"]
pub type TIE12_R = crate::BitReader<bool>;
#[doc = "Field `TIE12` writer - Transmission Interrupt Enable for Buffer 12"]
pub type TIE12_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBTIE_SPEC, bool, O>;
#[doc = "Field `TIE13` reader - Transmission Interrupt Enable for Buffer 13"]
pub type TIE13_R = crate::BitReader<bool>;
#[doc = "Field `TIE13` writer - Transmission Interrupt Enable for Buffer 13"]
pub type TIE13_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBTIE_SPEC, bool, O>;
#[doc = "Field `TIE14` reader - Transmission Interrupt Enable for Buffer 14"]
pub type TIE14_R = crate::BitReader<bool>;
#[doc = "Field `TIE14` writer - Transmission Interrupt Enable for Buffer 14"]
pub type TIE14_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBTIE_SPEC, bool, O>;
#[doc = "Field `TIE15` reader - Transmission Interrupt Enable for Buffer 15"]
pub type TIE15_R = crate::BitReader<bool>;
#[doc = "Field `TIE15` writer - Transmission Interrupt Enable for Buffer 15"]
pub type TIE15_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBTIE_SPEC, bool, O>;
#[doc = "Field `TIE16` reader - Transmission Interrupt Enable for Buffer 16"]
pub type TIE16_R = crate::BitReader<bool>;
#[doc = "Field `TIE16` writer - Transmission Interrupt Enable for Buffer 16"]
pub type TIE16_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBTIE_SPEC, bool, O>;
#[doc = "Field `TIE17` reader - Transmission Interrupt Enable for Buffer 17"]
pub type TIE17_R = crate::BitReader<bool>;
#[doc = "Field `TIE17` writer - Transmission Interrupt Enable for Buffer 17"]
pub type TIE17_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBTIE_SPEC, bool, O>;
#[doc = "Field `TIE18` reader - Transmission Interrupt Enable for Buffer 18"]
pub type TIE18_R = crate::BitReader<bool>;
#[doc = "Field `TIE18` writer - Transmission Interrupt Enable for Buffer 18"]
pub type TIE18_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBTIE_SPEC, bool, O>;
#[doc = "Field `TIE19` reader - Transmission Interrupt Enable for Buffer 19"]
pub type TIE19_R = crate::BitReader<bool>;
#[doc = "Field `TIE19` writer - Transmission Interrupt Enable for Buffer 19"]
pub type TIE19_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBTIE_SPEC, bool, O>;
#[doc = "Field `TIE20` reader - Transmission Interrupt Enable for Buffer 20"]
pub type TIE20_R = crate::BitReader<bool>;
#[doc = "Field `TIE20` writer - Transmission Interrupt Enable for Buffer 20"]
pub type TIE20_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBTIE_SPEC, bool, O>;
#[doc = "Field `TIE21` reader - Transmission Interrupt Enable for Buffer 21"]
pub type TIE21_R = crate::BitReader<bool>;
#[doc = "Field `TIE21` writer - Transmission Interrupt Enable for Buffer 21"]
pub type TIE21_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBTIE_SPEC, bool, O>;
#[doc = "Field `TIE22` reader - Transmission Interrupt Enable for Buffer 22"]
pub type TIE22_R = crate::BitReader<bool>;
#[doc = "Field `TIE22` writer - Transmission Interrupt Enable for Buffer 22"]
pub type TIE22_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBTIE_SPEC, bool, O>;
#[doc = "Field `TIE23` reader - Transmission Interrupt Enable for Buffer 23"]
pub type TIE23_R = crate::BitReader<bool>;
#[doc = "Field `TIE23` writer - Transmission Interrupt Enable for Buffer 23"]
pub type TIE23_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBTIE_SPEC, bool, O>;
#[doc = "Field `TIE24` reader - Transmission Interrupt Enable for Buffer 24"]
pub type TIE24_R = crate::BitReader<bool>;
#[doc = "Field `TIE24` writer - Transmission Interrupt Enable for Buffer 24"]
pub type TIE24_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBTIE_SPEC, bool, O>;
#[doc = "Field `TIE25` reader - Transmission Interrupt Enable for Buffer 25"]
pub type TIE25_R = crate::BitReader<bool>;
#[doc = "Field `TIE25` writer - Transmission Interrupt Enable for Buffer 25"]
pub type TIE25_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBTIE_SPEC, bool, O>;
#[doc = "Field `TIE26` reader - Transmission Interrupt Enable for Buffer 26"]
pub type TIE26_R = crate::BitReader<bool>;
#[doc = "Field `TIE26` writer - Transmission Interrupt Enable for Buffer 26"]
pub type TIE26_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBTIE_SPEC, bool, O>;
#[doc = "Field `TIE27` reader - Transmission Interrupt Enable for Buffer 27"]
pub type TIE27_R = crate::BitReader<bool>;
#[doc = "Field `TIE27` writer - Transmission Interrupt Enable for Buffer 27"]
pub type TIE27_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBTIE_SPEC, bool, O>;
#[doc = "Field `TIE28` reader - Transmission Interrupt Enable for Buffer 28"]
pub type TIE28_R = crate::BitReader<bool>;
#[doc = "Field `TIE28` writer - Transmission Interrupt Enable for Buffer 28"]
pub type TIE28_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBTIE_SPEC, bool, O>;
#[doc = "Field `TIE29` reader - Transmission Interrupt Enable for Buffer 29"]
pub type TIE29_R = crate::BitReader<bool>;
#[doc = "Field `TIE29` writer - Transmission Interrupt Enable for Buffer 29"]
pub type TIE29_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBTIE_SPEC, bool, O>;
#[doc = "Field `TIE30` reader - Transmission Interrupt Enable for Buffer 30"]
pub type TIE30_R = crate::BitReader<bool>;
#[doc = "Field `TIE30` writer - Transmission Interrupt Enable for Buffer 30"]
pub type TIE30_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBTIE_SPEC, bool, O>;
#[doc = "Field `TIE31` reader - Transmission Interrupt Enable for Buffer 31"]
pub type TIE31_R = crate::BitReader<bool>;
#[doc = "Field `TIE31` writer - Transmission Interrupt Enable for Buffer 31"]
pub type TIE31_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBTIE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Transmission Interrupt Enable for Buffer 0"]
    #[inline(always)]
    pub fn tie0(&self) -> TIE0_R {
        TIE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmission Interrupt Enable for Buffer 1"]
    #[inline(always)]
    pub fn tie1(&self) -> TIE1_R {
        TIE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmission Interrupt Enable for Buffer 2"]
    #[inline(always)]
    pub fn tie2(&self) -> TIE2_R {
        TIE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmission Interrupt Enable for Buffer 3"]
    #[inline(always)]
    pub fn tie3(&self) -> TIE3_R {
        TIE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmission Interrupt Enable for Buffer 4"]
    #[inline(always)]
    pub fn tie4(&self) -> TIE4_R {
        TIE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmission Interrupt Enable for Buffer 5"]
    #[inline(always)]
    pub fn tie5(&self) -> TIE5_R {
        TIE5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmission Interrupt Enable for Buffer 6"]
    #[inline(always)]
    pub fn tie6(&self) -> TIE6_R {
        TIE6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmission Interrupt Enable for Buffer 7"]
    #[inline(always)]
    pub fn tie7(&self) -> TIE7_R {
        TIE7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmission Interrupt Enable for Buffer 8"]
    #[inline(always)]
    pub fn tie8(&self) -> TIE8_R {
        TIE8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmission Interrupt Enable for Buffer 9"]
    #[inline(always)]
    pub fn tie9(&self) -> TIE9_R {
        TIE9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmission Interrupt Enable for Buffer 10"]
    #[inline(always)]
    pub fn tie10(&self) -> TIE10_R {
        TIE10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Transmission Interrupt Enable for Buffer 11"]
    #[inline(always)]
    pub fn tie11(&self) -> TIE11_R {
        TIE11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Transmission Interrupt Enable for Buffer 12"]
    #[inline(always)]
    pub fn tie12(&self) -> TIE12_R {
        TIE12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Transmission Interrupt Enable for Buffer 13"]
    #[inline(always)]
    pub fn tie13(&self) -> TIE13_R {
        TIE13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Transmission Interrupt Enable for Buffer 14"]
    #[inline(always)]
    pub fn tie14(&self) -> TIE14_R {
        TIE14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Transmission Interrupt Enable for Buffer 15"]
    #[inline(always)]
    pub fn tie15(&self) -> TIE15_R {
        TIE15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Transmission Interrupt Enable for Buffer 16"]
    #[inline(always)]
    pub fn tie16(&self) -> TIE16_R {
        TIE16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Transmission Interrupt Enable for Buffer 17"]
    #[inline(always)]
    pub fn tie17(&self) -> TIE17_R {
        TIE17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Transmission Interrupt Enable for Buffer 18"]
    #[inline(always)]
    pub fn tie18(&self) -> TIE18_R {
        TIE18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Transmission Interrupt Enable for Buffer 19"]
    #[inline(always)]
    pub fn tie19(&self) -> TIE19_R {
        TIE19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Transmission Interrupt Enable for Buffer 20"]
    #[inline(always)]
    pub fn tie20(&self) -> TIE20_R {
        TIE20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Transmission Interrupt Enable for Buffer 21"]
    #[inline(always)]
    pub fn tie21(&self) -> TIE21_R {
        TIE21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Transmission Interrupt Enable for Buffer 22"]
    #[inline(always)]
    pub fn tie22(&self) -> TIE22_R {
        TIE22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Transmission Interrupt Enable for Buffer 23"]
    #[inline(always)]
    pub fn tie23(&self) -> TIE23_R {
        TIE23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Transmission Interrupt Enable for Buffer 24"]
    #[inline(always)]
    pub fn tie24(&self) -> TIE24_R {
        TIE24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Transmission Interrupt Enable for Buffer 25"]
    #[inline(always)]
    pub fn tie25(&self) -> TIE25_R {
        TIE25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Transmission Interrupt Enable for Buffer 26"]
    #[inline(always)]
    pub fn tie26(&self) -> TIE26_R {
        TIE26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Transmission Interrupt Enable for Buffer 27"]
    #[inline(always)]
    pub fn tie27(&self) -> TIE27_R {
        TIE27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Transmission Interrupt Enable for Buffer 28"]
    #[inline(always)]
    pub fn tie28(&self) -> TIE28_R {
        TIE28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Transmission Interrupt Enable for Buffer 29"]
    #[inline(always)]
    pub fn tie29(&self) -> TIE29_R {
        TIE29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Transmission Interrupt Enable for Buffer 30"]
    #[inline(always)]
    pub fn tie30(&self) -> TIE30_R {
        TIE30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Transmission Interrupt Enable for Buffer 31"]
    #[inline(always)]
    pub fn tie31(&self) -> TIE31_R {
        TIE31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmission Interrupt Enable for Buffer 0"]
    #[inline(always)]
    #[must_use]
    pub fn tie0(&mut self) -> TIE0_W<0> {
        TIE0_W::new(self)
    }
    #[doc = "Bit 1 - Transmission Interrupt Enable for Buffer 1"]
    #[inline(always)]
    #[must_use]
    pub fn tie1(&mut self) -> TIE1_W<1> {
        TIE1_W::new(self)
    }
    #[doc = "Bit 2 - Transmission Interrupt Enable for Buffer 2"]
    #[inline(always)]
    #[must_use]
    pub fn tie2(&mut self) -> TIE2_W<2> {
        TIE2_W::new(self)
    }
    #[doc = "Bit 3 - Transmission Interrupt Enable for Buffer 3"]
    #[inline(always)]
    #[must_use]
    pub fn tie3(&mut self) -> TIE3_W<3> {
        TIE3_W::new(self)
    }
    #[doc = "Bit 4 - Transmission Interrupt Enable for Buffer 4"]
    #[inline(always)]
    #[must_use]
    pub fn tie4(&mut self) -> TIE4_W<4> {
        TIE4_W::new(self)
    }
    #[doc = "Bit 5 - Transmission Interrupt Enable for Buffer 5"]
    #[inline(always)]
    #[must_use]
    pub fn tie5(&mut self) -> TIE5_W<5> {
        TIE5_W::new(self)
    }
    #[doc = "Bit 6 - Transmission Interrupt Enable for Buffer 6"]
    #[inline(always)]
    #[must_use]
    pub fn tie6(&mut self) -> TIE6_W<6> {
        TIE6_W::new(self)
    }
    #[doc = "Bit 7 - Transmission Interrupt Enable for Buffer 7"]
    #[inline(always)]
    #[must_use]
    pub fn tie7(&mut self) -> TIE7_W<7> {
        TIE7_W::new(self)
    }
    #[doc = "Bit 8 - Transmission Interrupt Enable for Buffer 8"]
    #[inline(always)]
    #[must_use]
    pub fn tie8(&mut self) -> TIE8_W<8> {
        TIE8_W::new(self)
    }
    #[doc = "Bit 9 - Transmission Interrupt Enable for Buffer 9"]
    #[inline(always)]
    #[must_use]
    pub fn tie9(&mut self) -> TIE9_W<9> {
        TIE9_W::new(self)
    }
    #[doc = "Bit 10 - Transmission Interrupt Enable for Buffer 10"]
    #[inline(always)]
    #[must_use]
    pub fn tie10(&mut self) -> TIE10_W<10> {
        TIE10_W::new(self)
    }
    #[doc = "Bit 11 - Transmission Interrupt Enable for Buffer 11"]
    #[inline(always)]
    #[must_use]
    pub fn tie11(&mut self) -> TIE11_W<11> {
        TIE11_W::new(self)
    }
    #[doc = "Bit 12 - Transmission Interrupt Enable for Buffer 12"]
    #[inline(always)]
    #[must_use]
    pub fn tie12(&mut self) -> TIE12_W<12> {
        TIE12_W::new(self)
    }
    #[doc = "Bit 13 - Transmission Interrupt Enable for Buffer 13"]
    #[inline(always)]
    #[must_use]
    pub fn tie13(&mut self) -> TIE13_W<13> {
        TIE13_W::new(self)
    }
    #[doc = "Bit 14 - Transmission Interrupt Enable for Buffer 14"]
    #[inline(always)]
    #[must_use]
    pub fn tie14(&mut self) -> TIE14_W<14> {
        TIE14_W::new(self)
    }
    #[doc = "Bit 15 - Transmission Interrupt Enable for Buffer 15"]
    #[inline(always)]
    #[must_use]
    pub fn tie15(&mut self) -> TIE15_W<15> {
        TIE15_W::new(self)
    }
    #[doc = "Bit 16 - Transmission Interrupt Enable for Buffer 16"]
    #[inline(always)]
    #[must_use]
    pub fn tie16(&mut self) -> TIE16_W<16> {
        TIE16_W::new(self)
    }
    #[doc = "Bit 17 - Transmission Interrupt Enable for Buffer 17"]
    #[inline(always)]
    #[must_use]
    pub fn tie17(&mut self) -> TIE17_W<17> {
        TIE17_W::new(self)
    }
    #[doc = "Bit 18 - Transmission Interrupt Enable for Buffer 18"]
    #[inline(always)]
    #[must_use]
    pub fn tie18(&mut self) -> TIE18_W<18> {
        TIE18_W::new(self)
    }
    #[doc = "Bit 19 - Transmission Interrupt Enable for Buffer 19"]
    #[inline(always)]
    #[must_use]
    pub fn tie19(&mut self) -> TIE19_W<19> {
        TIE19_W::new(self)
    }
    #[doc = "Bit 20 - Transmission Interrupt Enable for Buffer 20"]
    #[inline(always)]
    #[must_use]
    pub fn tie20(&mut self) -> TIE20_W<20> {
        TIE20_W::new(self)
    }
    #[doc = "Bit 21 - Transmission Interrupt Enable for Buffer 21"]
    #[inline(always)]
    #[must_use]
    pub fn tie21(&mut self) -> TIE21_W<21> {
        TIE21_W::new(self)
    }
    #[doc = "Bit 22 - Transmission Interrupt Enable for Buffer 22"]
    #[inline(always)]
    #[must_use]
    pub fn tie22(&mut self) -> TIE22_W<22> {
        TIE22_W::new(self)
    }
    #[doc = "Bit 23 - Transmission Interrupt Enable for Buffer 23"]
    #[inline(always)]
    #[must_use]
    pub fn tie23(&mut self) -> TIE23_W<23> {
        TIE23_W::new(self)
    }
    #[doc = "Bit 24 - Transmission Interrupt Enable for Buffer 24"]
    #[inline(always)]
    #[must_use]
    pub fn tie24(&mut self) -> TIE24_W<24> {
        TIE24_W::new(self)
    }
    #[doc = "Bit 25 - Transmission Interrupt Enable for Buffer 25"]
    #[inline(always)]
    #[must_use]
    pub fn tie25(&mut self) -> TIE25_W<25> {
        TIE25_W::new(self)
    }
    #[doc = "Bit 26 - Transmission Interrupt Enable for Buffer 26"]
    #[inline(always)]
    #[must_use]
    pub fn tie26(&mut self) -> TIE26_W<26> {
        TIE26_W::new(self)
    }
    #[doc = "Bit 27 - Transmission Interrupt Enable for Buffer 27"]
    #[inline(always)]
    #[must_use]
    pub fn tie27(&mut self) -> TIE27_W<27> {
        TIE27_W::new(self)
    }
    #[doc = "Bit 28 - Transmission Interrupt Enable for Buffer 28"]
    #[inline(always)]
    #[must_use]
    pub fn tie28(&mut self) -> TIE28_W<28> {
        TIE28_W::new(self)
    }
    #[doc = "Bit 29 - Transmission Interrupt Enable for Buffer 29"]
    #[inline(always)]
    #[must_use]
    pub fn tie29(&mut self) -> TIE29_W<29> {
        TIE29_W::new(self)
    }
    #[doc = "Bit 30 - Transmission Interrupt Enable for Buffer 30"]
    #[inline(always)]
    #[must_use]
    pub fn tie30(&mut self) -> TIE30_W<30> {
        TIE30_W::new(self)
    }
    #[doc = "Bit 31 - Transmission Interrupt Enable for Buffer 31"]
    #[inline(always)]
    #[must_use]
    pub fn tie31(&mut self) -> TIE31_W<31> {
        TIE31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Buffer Transmission Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbtie](index.html) module"]
pub struct TXBTIE_SPEC;
impl crate::RegisterSpec for TXBTIE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txbtie::R](R) reader structure"]
impl crate::Readable for TXBTIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txbtie::W](W) writer structure"]
impl crate::Writable for TXBTIE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXBTIE to value 0"]
impl crate::Resettable for TXBTIE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
