///Register `ENDN` reader
pub struct R(crate::R<ENDN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENDN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENDN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENDN_SPEC>) -> Self {
        R(reader)
    }
}
///Field `ETV` reader - ETV
pub struct ETV_R(crate::FieldReader<u32, u32>);
impl ETV_R {
    pub(crate) fn new(bits: u32) -> Self {
        ETV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETV_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bits 0:31 - ETV
    #[inline(always)]
    pub fn etv(&self) -> ETV_R {
        ETV_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
///FDCAN Core Release Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [endn](index.html) module
pub struct ENDN_SPEC;
impl crate::RegisterSpec for ENDN_SPEC {
    type Ux = u32;
}
///`read()` method returns [endn::R](R) reader structure
impl crate::Readable for ENDN_SPEC {
    type Reader = R;
}
///`reset()` method sets ENDN to value 0x8765_4321
impl crate::Resettable for ENDN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8765_4321
    }
}
