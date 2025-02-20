#[doc = "Register `DMA_P_ADDR` reader"]
pub struct R(crate::R<DMA_P_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_P_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_P_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_P_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_P_ADDR` writer"]
pub struct W(crate::W<DMA_P_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_P_ADDR_SPEC>;
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
impl From<crate::W<DMA_P_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_P_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P_ADDR` reader - Preview Image Base Address"]
pub type P_ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `P_ADDR` writer - Preview Image Base Address"]
pub type P_ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMA_P_ADDR_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bits 2:31 - Preview Image Base Address"]
    #[inline(always)]
    pub fn p_addr(&self) -> P_ADDR_R {
        P_ADDR_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Preview Image Base Address"]
    #[inline(always)]
    #[must_use]
    pub fn p_addr(&mut self) -> P_ADDR_W<2> {
        P_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Preview Base Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_p_addr](index.html) module"]
pub struct DMA_P_ADDR_SPEC;
impl crate::RegisterSpec for DMA_P_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_p_addr::R](R) reader structure"]
impl crate::Readable for DMA_P_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_p_addr::W](W) writer structure"]
impl crate::Writable for DMA_P_ADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_P_ADDR to value 0"]
impl crate::Resettable for DMA_P_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
