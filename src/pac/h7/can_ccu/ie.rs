///Register `IE` reader
pub struct R(crate::R<IE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IE_SPEC>) -> Self {
        R(reader)
    }
}
///Register `IE` writer
pub struct W(crate::W<IE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IE_SPEC>;
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
impl From<crate::W<IE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IE_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CWEE` reader - Calibration Watchdog Event Enable
pub struct CWEE_R(crate::FieldReader<bool, bool>);
impl CWEE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CWEE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CWEE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CWEE` writer - Calibration Watchdog Event Enable
pub struct CWEE_W<'a> {
    w: &'a mut W,
}
impl<'a> CWEE_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
///Field `CSCE` reader - Calibration State Changed Enable
pub struct CSCE_R(crate::FieldReader<bool, bool>);
impl CSCE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CSCE` writer - Calibration State Changed Enable
pub struct CSCE_W<'a> {
    w: &'a mut W,
}
impl<'a> CSCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
impl R {
    ///Bit 0 - Calibration Watchdog Event Enable
    #[inline(always)]
    pub fn cwee(&self) -> CWEE_R {
        CWEE_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Calibration State Changed Enable
    #[inline(always)]
    pub fn csce(&self) -> CSCE_R {
        CSCE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Calibration Watchdog Event Enable
    #[inline(always)]
    pub fn cwee(&mut self) -> CWEE_W {
        CWEE_W { w: self }
    }
    ///Bit 1 - Calibration State Changed Enable
    #[inline(always)]
    pub fn csce(&mut self) -> CSCE_W {
        CSCE_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Clock Calibration Unit Interrupt Enable Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ie](index.html) module
pub struct IE_SPEC;
impl crate::RegisterSpec for IE_SPEC {
    type Ux = u32;
}
///`read()` method returns [ie::R](R) reader structure
impl crate::Readable for IE_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ie::W](W) writer structure
impl crate::Writable for IE_SPEC {
    type Writer = W;
}
///`reset()` method sets IE to value 0
impl crate::Resettable for IE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
