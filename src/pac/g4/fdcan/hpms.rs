///Register `HPMS` reader
pub struct R(crate::R<HPMS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HPMS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HPMS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HPMS_SPEC>) -> Self {
        R(reader)
    }
}
///Field `BIDX` reader - BIDX
pub struct BIDX_R(crate::FieldReader<u8, u8>);
impl BIDX_R {
    pub(crate) fn new(bits: u8) -> Self {
        BIDX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIDX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MSI` reader - MSI
pub struct MSI_R(crate::FieldReader<u8, u8>);
impl MSI_R {
    pub(crate) fn new(bits: u8) -> Self {
        MSI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `FIDX` reader - FIDX
pub struct FIDX_R(crate::FieldReader<u8, u8>);
impl FIDX_R {
    pub(crate) fn new(bits: u8) -> Self {
        FIDX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIDX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `FLST` reader - FLST
pub struct FLST_R(crate::FieldReader<bool, bool>);
impl FLST_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bits 0:5 - BIDX
    #[inline(always)]
    pub fn bidx(&self) -> BIDX_R {
        BIDX_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 6:7 - MSI
    #[inline(always)]
    pub fn msi(&self) -> MSI_R {
        MSI_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    ///Bits 8:14 - FIDX
    #[inline(always)]
    pub fn fidx(&self) -> FIDX_R {
        FIDX_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    ///Bit 15 - FLST
    #[inline(always)]
    pub fn flst(&self) -> FLST_R {
        FLST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
///This register is updated every time a Message ID filter element configured to generate a priority event match. This can be used to monitor the status of incoming high priority messages and to enable fast access to these messages.
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hpms](index.html) module
pub struct HPMS_SPEC;
impl crate::RegisterSpec for HPMS_SPEC {
    type Ux = u32;
}
///`read()` method returns [hpms::R](R) reader structure
impl crate::Readable for HPMS_SPEC {
    type Reader = R;
}
///`reset()` method sets HPMS to value 0
impl crate::Resettable for HPMS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
