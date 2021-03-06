///Register `TTLGT` reader
pub struct R(crate::R<TTLGT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TTLGT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TTLGT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TTLGT_SPEC>) -> Self {
        R(reader)
    }
}
///Field `LT` reader - Local Time
pub struct LT_R(crate::FieldReader<u16, u16>);
impl LT_R {
    pub(crate) fn new(bits: u16) -> Self {
        LT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `GT` reader - Global Time
pub struct GT_R(crate::FieldReader<u16, u16>);
impl GT_R {
    pub(crate) fn new(bits: u16) -> Self {
        GT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bits 0:15 - Local Time
    #[inline(always)]
    pub fn lt(&self) -> LT_R {
        LT_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Global Time
    #[inline(always)]
    pub fn gt(&self) -> GT_R {
        GT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
///FDCAN TT Local and Global Time Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ttlgt](index.html) module
pub struct TTLGT_SPEC;
impl crate::RegisterSpec for TTLGT_SPEC {
    type Ux = u32;
}
///`read()` method returns [ttlgt::R](R) reader structure
impl crate::Readable for TTLGT_SPEC {
    type Reader = R;
}
///`reset()` method sets TTLGT to value 0
impl crate::Resettable for TTLGT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
