///Register `TTCTC` reader
pub struct R(crate::R<TTCTC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TTCTC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TTCTC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TTCTC_SPEC>) -> Self {
        R(reader)
    }
}
///Field `CT` reader - Cycle Time
pub struct CT_R(crate::FieldReader<u16, u16>);
impl CT_R {
    pub(crate) fn new(bits: u16) -> Self {
        CT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CC` reader - Cycle Count
pub struct CC_R(crate::FieldReader<u8, u8>);
impl CC_R {
    pub(crate) fn new(bits: u8) -> Self {
        CC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bits 0:15 - Cycle Time
    #[inline(always)]
    pub fn ct(&self) -> CT_R {
        CT_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:21 - Cycle Count
    #[inline(always)]
    pub fn cc(&self) -> CC_R {
        CC_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
///FDCAN TT Cycle Time and Count Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ttctc](index.html) module
pub struct TTCTC_SPEC;
impl crate::RegisterSpec for TTCTC_SPEC {
    type Ux = u32;
}
///`read()` method returns [ttctc::R](R) reader structure
impl crate::Readable for TTCTC_SPEC {
    type Reader = R;
}
///`reset()` method sets TTCTC to value 0
impl crate::Resettable for TTCTC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
