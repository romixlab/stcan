///Register `PSR` reader
pub struct R(crate::R<PSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PSR` writer
pub struct W(crate::W<PSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSR_SPEC>;
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
impl From<crate::W<PSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LEC` reader - Last Error Code
pub struct LEC_R(crate::FieldReader<u8, u8>);
impl LEC_R {
    pub(crate) fn new(bits: u8) -> Self {
        LEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LEC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LEC` writer - Last Error Code
pub struct LEC_W<'a> {
    w: &'a mut W,
}
impl<'a> LEC_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
///Field `ACT` reader - Activity
pub struct ACT_R(crate::FieldReader<u8, u8>);
impl ACT_R {
    pub(crate) fn new(bits: u8) -> Self {
        ACT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ACT` writer - Activity
pub struct ACT_W<'a> {
    w: &'a mut W,
}
impl<'a> ACT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
///Field `EP` reader - Error Passive
pub struct EP_R(crate::FieldReader<bool, bool>);
impl EP_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EP` writer - Error Passive
pub struct EP_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
///Field `EW` reader - Warning Status
pub struct EW_R(crate::FieldReader<bool, bool>);
impl EW_R {
    pub(crate) fn new(bits: bool) -> Self {
        EW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EW` writer - Warning Status
pub struct EW_W<'a> {
    w: &'a mut W,
}
impl<'a> EW_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
///Field `BO` reader - Bus_Off Status
pub struct BO_R(crate::FieldReader<bool, bool>);
impl BO_R {
    pub(crate) fn new(bits: bool) -> Self {
        BO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `BO` writer - Bus_Off Status
pub struct BO_W<'a> {
    w: &'a mut W,
}
impl<'a> BO_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
///Field `DLEC` reader - Data Last Error Code
pub struct DLEC_R(crate::FieldReader<u8, u8>);
impl DLEC_R {
    pub(crate) fn new(bits: u8) -> Self {
        DLEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLEC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DLEC` writer - Data Last Error Code
pub struct DLEC_W<'a> {
    w: &'a mut W,
}
impl<'a> DLEC_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
///Field `RESI` reader - ESI flag of last received FDCAN Message
pub struct RESI_R(crate::FieldReader<bool, bool>);
impl RESI_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RESI` writer - ESI flag of last received FDCAN Message
pub struct RESI_W<'a> {
    w: &'a mut W,
}
impl<'a> RESI_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
///Field `RBRS` reader - BRS flag of last received FDCAN Message
pub struct RBRS_R(crate::FieldReader<bool, bool>);
impl RBRS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RBRS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBRS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RBRS` writer - BRS flag of last received FDCAN Message
pub struct RBRS_W<'a> {
    w: &'a mut W,
}
impl<'a> RBRS_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
///Field `REDL` reader - Received FDCAN Message
pub struct REDL_R(crate::FieldReader<bool, bool>);
impl REDL_R {
    pub(crate) fn new(bits: bool) -> Self {
        REDL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REDL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `REDL` writer - Received FDCAN Message
pub struct REDL_W<'a> {
    w: &'a mut W,
}
impl<'a> REDL_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
///Field `PXE` reader - Protocol Exception Event
pub struct PXE_R(crate::FieldReader<bool, bool>);
impl PXE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PXE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PXE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PXE` writer - Protocol Exception Event
pub struct PXE_W<'a> {
    w: &'a mut W,
}
impl<'a> PXE_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
///Field `TDCV` reader - Transmitter Delay Compensation Value
pub struct TDCV_R(crate::FieldReader<u8, u8>);
impl TDCV_R {
    pub(crate) fn new(bits: u8) -> Self {
        TDCV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDCV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TDCV` writer - Transmitter Delay Compensation Value
pub struct TDCV_W<'a> {
    w: &'a mut W,
}
impl<'a> TDCV_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | ((value as u32 & 0x7f) << 16);
        self.w
    }
}
impl R {
    ///Bits 0:2 - Last Error Code
    #[inline(always)]
    pub fn lec(&self) -> LEC_R {
        LEC_R::new((self.bits & 0x07) as u8)
    }
    ///Bits 3:4 - Activity
    #[inline(always)]
    pub fn act(&self) -> ACT_R {
        ACT_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    ///Bit 5 - Error Passive
    #[inline(always)]
    pub fn ep(&self) -> EP_R {
        EP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - Warning Status
    #[inline(always)]
    pub fn ew(&self) -> EW_R {
        EW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 7 - Bus_Off Status
    #[inline(always)]
    pub fn bo(&self) -> BO_R {
        BO_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bits 8:10 - Data Last Error Code
    #[inline(always)]
    pub fn dlec(&self) -> DLEC_R {
        DLEC_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    ///Bit 11 - ESI flag of last received FDCAN Message
    #[inline(always)]
    pub fn resi(&self) -> RESI_R {
        RESI_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 12 - BRS flag of last received FDCAN Message
    #[inline(always)]
    pub fn rbrs(&self) -> RBRS_R {
        RBRS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 13 - Received FDCAN Message
    #[inline(always)]
    pub fn redl(&self) -> REDL_R {
        REDL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - Protocol Exception Event
    #[inline(always)]
    pub fn pxe(&self) -> PXE_R {
        PXE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bits 16:22 - Transmitter Delay Compensation Value
    #[inline(always)]
    pub fn tdcv(&self) -> TDCV_R {
        TDCV_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    ///Bits 0:2 - Last Error Code
    #[inline(always)]
    pub fn lec(&mut self) -> LEC_W {
        LEC_W { w: self }
    }
    ///Bits 3:4 - Activity
    #[inline(always)]
    pub fn act(&mut self) -> ACT_W {
        ACT_W { w: self }
    }
    ///Bit 5 - Error Passive
    #[inline(always)]
    pub fn ep(&mut self) -> EP_W {
        EP_W { w: self }
    }
    ///Bit 6 - Warning Status
    #[inline(always)]
    pub fn ew(&mut self) -> EW_W {
        EW_W { w: self }
    }
    ///Bit 7 - Bus_Off Status
    #[inline(always)]
    pub fn bo(&mut self) -> BO_W {
        BO_W { w: self }
    }
    ///Bits 8:10 - Data Last Error Code
    #[inline(always)]
    pub fn dlec(&mut self) -> DLEC_W {
        DLEC_W { w: self }
    }
    ///Bit 11 - ESI flag of last received FDCAN Message
    #[inline(always)]
    pub fn resi(&mut self) -> RESI_W {
        RESI_W { w: self }
    }
    ///Bit 12 - BRS flag of last received FDCAN Message
    #[inline(always)]
    pub fn rbrs(&mut self) -> RBRS_W {
        RBRS_W { w: self }
    }
    ///Bit 13 - Received FDCAN Message
    #[inline(always)]
    pub fn redl(&mut self) -> REDL_W {
        REDL_W { w: self }
    }
    ///Bit 14 - Protocol Exception Event
    #[inline(always)]
    pub fn pxe(&mut self) -> PXE_W {
        PXE_W { w: self }
    }
    ///Bits 16:22 - Transmitter Delay Compensation Value
    #[inline(always)]
    pub fn tdcv(&mut self) -> TDCV_W {
        TDCV_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FDCAN Protocol Status Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [psr](index.html) module
pub struct PSR_SPEC;
impl crate::RegisterSpec for PSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [psr::R](R) reader structure
impl crate::Readable for PSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [psr::W](W) writer structure
impl crate::Writable for PSR_SPEC {
    type Writer = W;
}
///`reset()` method sets PSR to value 0
impl crate::Resettable for PSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
