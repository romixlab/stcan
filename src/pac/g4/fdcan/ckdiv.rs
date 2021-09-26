///Register `CKDIV` reader
pub struct R(crate::R<CKDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CKDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CKDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CKDIV_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CKDIV` writer
pub struct W(crate::W<CKDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CKDIV_SPEC>;
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
impl From<crate::W<CKDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CKDIV_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PDIV` reader - input clock divider. the APB clock could be divided prior to be used by the CAN sub
pub struct PDIV_R(crate::FieldReader<u8, u8>);
impl PDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        PDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PDIV` writer - input clock divider. the APB clock could be divided prior to be used by the CAN sub
pub struct PDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIV_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    ///Bits 0:3 - input clock divider. the APB clock could be divided prior to be used by the CAN sub
    #[inline(always)]
    pub fn pdiv(&self) -> PDIV_R {
        PDIV_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:3 - input clock divider. the APB clock could be divided prior to be used by the CAN sub
    #[inline(always)]
    pub fn pdiv(&mut self) -> PDIV_W {
        PDIV_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FDCAN CFG clock divider register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ckdiv](index.html) module
pub struct CKDIV_SPEC;
impl crate::RegisterSpec for CKDIV_SPEC {
    type Ux = u32;
}
///`read()` method returns [ckdiv::R](R) reader structure
impl crate::Readable for CKDIV_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ckdiv::W](W) writer structure
impl crate::Writable for CKDIV_SPEC {
    type Writer = W;
}
///`reset()` method sets CKDIV to value 0
impl crate::Resettable for CKDIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
