///Register `TOCV` reader
pub struct R(crate::R<TOCV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOCV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOCV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOCV_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TOCV` writer
pub struct W(crate::W<TOCV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOCV_SPEC>;
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
impl From<crate::W<TOCV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOCV_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TOC` reader - TOC
pub struct TOC_R(crate::FieldReader<u16, u16>);
impl TOC_R {
    pub(crate) fn new(bits: u16) -> Self {
        TOC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOC_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TOC` writer - TOC
pub struct TOC_W<'a> {
    w: &'a mut W,
}
impl<'a> TOC_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    ///Bits 0:15 - TOC
    #[inline(always)]
    pub fn toc(&self) -> TOC_R {
        TOC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - TOC
    #[inline(always)]
    pub fn toc(&mut self) -> TOC_W {
        TOC_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FDCAN Timeout Counter Value Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tocv](index.html) module
pub struct TOCV_SPEC;
impl crate::RegisterSpec for TOCV_SPEC {
    type Ux = u32;
}
///`read()` method returns [tocv::R](R) reader structure
impl crate::Readable for TOCV_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tocv::W](W) writer structure
impl crate::Writable for TOCV_SPEC {
    type Writer = W;
}
///`reset()` method sets TOCV to value 0xffff
impl crate::Resettable for TOCV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
