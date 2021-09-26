///Register `RWD` reader
pub struct R(crate::R<RWD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RWD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RWD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RWD_SPEC>) -> Self {
        R(reader)
    }
}
///Field `WDV` reader - Watchdog value
pub struct WDV_R(crate::FieldReader<u8, u8>);
impl WDV_R {
    pub(crate) fn new(bits: u8) -> Self {
        WDV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `WDC` reader - Watchdog configuration
pub struct WDC_R(crate::FieldReader<u8, u8>);
impl WDC_R {
    pub(crate) fn new(bits: u8) -> Self {
        WDC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bits 8:15 - Watchdog value
    #[inline(always)]
    pub fn wdv(&self) -> WDV_R {
        WDV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 0:7 - Watchdog configuration
    #[inline(always)]
    pub fn wdc(&self) -> WDC_R {
        WDC_R::new((self.bits & 0xff) as u8)
    }
}
///FDCAN RAM Watchdog Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rwd](index.html) module
pub struct RWD_SPEC;
impl crate::RegisterSpec for RWD_SPEC {
    type Ux = u32;
}
///`read()` method returns [rwd::R](R) reader structure
impl crate::Readable for RWD_SPEC {
    type Reader = R;
}
///`reset()` method sets RWD to value 0
impl crate::Resettable for RWD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
