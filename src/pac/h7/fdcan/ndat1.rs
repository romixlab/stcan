///Register `NDAT1` reader
pub struct R(crate::R<NDAT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NDAT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NDAT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NDAT1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `NDAT1` writer
pub struct W(crate::W<NDAT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NDAT1_SPEC>;
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
impl From<crate::W<NDAT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NDAT1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ND0` reader - New data
pub struct ND0_R(crate::FieldReader<bool, bool>);
impl ND0_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ND0` writer - New data
pub struct ND0_W<'a> {
    w: &'a mut W,
}
impl<'a> ND0_W<'a> {
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
///Field `ND1` reader - New data
pub struct ND1_R(crate::FieldReader<bool, bool>);
impl ND1_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ND1` writer - New data
pub struct ND1_W<'a> {
    w: &'a mut W,
}
impl<'a> ND1_W<'a> {
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
///Field `ND2` reader - New data
pub struct ND2_R(crate::FieldReader<bool, bool>);
impl ND2_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ND2` writer - New data
pub struct ND2_W<'a> {
    w: &'a mut W,
}
impl<'a> ND2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
///Field `ND3` reader - New data
pub struct ND3_R(crate::FieldReader<bool, bool>);
impl ND3_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ND3` writer - New data
pub struct ND3_W<'a> {
    w: &'a mut W,
}
impl<'a> ND3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
///Field `ND4` reader - New data
pub struct ND4_R(crate::FieldReader<bool, bool>);
impl ND4_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ND4` writer - New data
pub struct ND4_W<'a> {
    w: &'a mut W,
}
impl<'a> ND4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
///Field `ND5` reader - New data
pub struct ND5_R(crate::FieldReader<bool, bool>);
impl ND5_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ND5` writer - New data
pub struct ND5_W<'a> {
    w: &'a mut W,
}
impl<'a> ND5_W<'a> {
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
///Field `ND6` reader - New data
pub struct ND6_R(crate::FieldReader<bool, bool>);
impl ND6_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ND6` writer - New data
pub struct ND6_W<'a> {
    w: &'a mut W,
}
impl<'a> ND6_W<'a> {
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
///Field `ND7` reader - New data
pub struct ND7_R(crate::FieldReader<bool, bool>);
impl ND7_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ND7` writer - New data
pub struct ND7_W<'a> {
    w: &'a mut W,
}
impl<'a> ND7_W<'a> {
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
///Field `ND8` reader - New data
pub struct ND8_R(crate::FieldReader<bool, bool>);
impl ND8_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ND8` writer - New data
pub struct ND8_W<'a> {
    w: &'a mut W,
}
impl<'a> ND8_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
///Field `ND9` reader - New data
pub struct ND9_R(crate::FieldReader<bool, bool>);
impl ND9_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ND9` writer - New data
pub struct ND9_W<'a> {
    w: &'a mut W,
}
impl<'a> ND9_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
///Field `ND10` reader - New data
pub struct ND10_R(crate::FieldReader<bool, bool>);
impl ND10_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ND10` writer - New data
pub struct ND10_W<'a> {
    w: &'a mut W,
}
impl<'a> ND10_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
///Field `ND11` reader - New data
pub struct ND11_R(crate::FieldReader<bool, bool>);
impl ND11_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ND11` writer - New data
pub struct ND11_W<'a> {
    w: &'a mut W,
}
impl<'a> ND11_W<'a> {
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
///Field `ND12` reader - New data
pub struct ND12_R(crate::FieldReader<bool, bool>);
impl ND12_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ND12` writer - New data
pub struct ND12_W<'a> {
    w: &'a mut W,
}
impl<'a> ND12_W<'a> {
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
///Field `ND13` reader - New data
pub struct ND13_R(crate::FieldReader<bool, bool>);
impl ND13_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ND13` writer - New data
pub struct ND13_W<'a> {
    w: &'a mut W,
}
impl<'a> ND13_W<'a> {
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
///Field `ND14` reader - New data
pub struct ND14_R(crate::FieldReader<bool, bool>);
impl ND14_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ND14` writer - New data
pub struct ND14_W<'a> {
    w: &'a mut W,
}
impl<'a> ND14_W<'a> {
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
///Field `ND15` reader - New data
pub struct ND15_R(crate::FieldReader<bool, bool>);
impl ND15_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ND15` writer - New data
pub struct ND15_W<'a> {
    w: &'a mut W,
}
impl<'a> ND15_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
///Field `ND16` reader - New data
pub struct ND16_R(crate::FieldReader<bool, bool>);
impl ND16_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ND16` writer - New data
pub struct ND16_W<'a> {
    w: &'a mut W,
}
impl<'a> ND16_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
///Field `ND17` reader - New data
pub struct ND17_R(crate::FieldReader<bool, bool>);
impl ND17_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ND17` writer - New data
pub struct ND17_W<'a> {
    w: &'a mut W,
}
impl<'a> ND17_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
///Field `ND18` reader - New data
pub struct ND18_R(crate::FieldReader<bool, bool>);
impl ND18_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ND18` writer - New data
pub struct ND18_W<'a> {
    w: &'a mut W,
}
impl<'a> ND18_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
///Field `ND19` reader - New data
pub struct ND19_R(crate::FieldReader<bool, bool>);
impl ND19_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ND19` writer - New data
pub struct ND19_W<'a> {
    w: &'a mut W,
}
impl<'a> ND19_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
///Field `ND20` reader - New data
pub struct ND20_R(crate::FieldReader<bool, bool>);
impl ND20_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ND20` writer - New data
pub struct ND20_W<'a> {
    w: &'a mut W,
}
impl<'a> ND20_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
///Field `ND21` reader - New data
pub struct ND21_R(crate::FieldReader<bool, bool>);
impl ND21_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ND21` writer - New data
pub struct ND21_W<'a> {
    w: &'a mut W,
}
impl<'a> ND21_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
///Field `ND22` reader - New data
pub struct ND22_R(crate::FieldReader<bool, bool>);
impl ND22_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ND22` writer - New data
pub struct ND22_W<'a> {
    w: &'a mut W,
}
impl<'a> ND22_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
///Field `ND23` reader - New data
pub struct ND23_R(crate::FieldReader<bool, bool>);
impl ND23_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ND23` writer - New data
pub struct ND23_W<'a> {
    w: &'a mut W,
}
impl<'a> ND23_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
///Field `ND24` reader - New data
pub struct ND24_R(crate::FieldReader<bool, bool>);
impl ND24_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ND24` writer - New data
pub struct ND24_W<'a> {
    w: &'a mut W,
}
impl<'a> ND24_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
///Field `ND25` reader - New data
pub struct ND25_R(crate::FieldReader<bool, bool>);
impl ND25_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ND25` writer - New data
pub struct ND25_W<'a> {
    w: &'a mut W,
}
impl<'a> ND25_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
///Field `ND26` reader - New data
pub struct ND26_R(crate::FieldReader<bool, bool>);
impl ND26_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ND26` writer - New data
pub struct ND26_W<'a> {
    w: &'a mut W,
}
impl<'a> ND26_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
///Field `ND27` reader - New data
pub struct ND27_R(crate::FieldReader<bool, bool>);
impl ND27_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ND27` writer - New data
pub struct ND27_W<'a> {
    w: &'a mut W,
}
impl<'a> ND27_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
///Field `ND28` reader - New data
pub struct ND28_R(crate::FieldReader<bool, bool>);
impl ND28_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND28_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ND28` writer - New data
pub struct ND28_W<'a> {
    w: &'a mut W,
}
impl<'a> ND28_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
///Field `ND29` reader - New data
pub struct ND29_R(crate::FieldReader<bool, bool>);
impl ND29_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND29_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ND29` writer - New data
pub struct ND29_W<'a> {
    w: &'a mut W,
}
impl<'a> ND29_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
///Field `ND30` reader - New data
pub struct ND30_R(crate::FieldReader<bool, bool>);
impl ND30_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ND30` writer - New data
pub struct ND30_W<'a> {
    w: &'a mut W,
}
impl<'a> ND30_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
///Field `ND31` reader - New data
pub struct ND31_R(crate::FieldReader<bool, bool>);
impl ND31_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ND31` writer - New data
pub struct ND31_W<'a> {
    w: &'a mut W,
}
impl<'a> ND31_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    ///Bit 0 - New data
    #[inline(always)]
    pub fn nd0(&self) -> ND0_R {
        ND0_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - New data
    #[inline(always)]
    pub fn nd1(&self) -> ND1_R {
        ND1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - New data
    #[inline(always)]
    pub fn nd2(&self) -> ND2_R {
        ND2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - New data
    #[inline(always)]
    pub fn nd3(&self) -> ND3_R {
        ND3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - New data
    #[inline(always)]
    pub fn nd4(&self) -> ND4_R {
        ND4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - New data
    #[inline(always)]
    pub fn nd5(&self) -> ND5_R {
        ND5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - New data
    #[inline(always)]
    pub fn nd6(&self) -> ND6_R {
        ND6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 7 - New data
    #[inline(always)]
    pub fn nd7(&self) -> ND7_R {
        ND7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 8 - New data
    #[inline(always)]
    pub fn nd8(&self) -> ND8_R {
        ND8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - New data
    #[inline(always)]
    pub fn nd9(&self) -> ND9_R {
        ND9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 10 - New data
    #[inline(always)]
    pub fn nd10(&self) -> ND10_R {
        ND10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 11 - New data
    #[inline(always)]
    pub fn nd11(&self) -> ND11_R {
        ND11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 12 - New data
    #[inline(always)]
    pub fn nd12(&self) -> ND12_R {
        ND12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 13 - New data
    #[inline(always)]
    pub fn nd13(&self) -> ND13_R {
        ND13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - New data
    #[inline(always)]
    pub fn nd14(&self) -> ND14_R {
        ND14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 15 - New data
    #[inline(always)]
    pub fn nd15(&self) -> ND15_R {
        ND15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 16 - New data
    #[inline(always)]
    pub fn nd16(&self) -> ND16_R {
        ND16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 17 - New data
    #[inline(always)]
    pub fn nd17(&self) -> ND17_R {
        ND17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 18 - New data
    #[inline(always)]
    pub fn nd18(&self) -> ND18_R {
        ND18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 19 - New data
    #[inline(always)]
    pub fn nd19(&self) -> ND19_R {
        ND19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    ///Bit 20 - New data
    #[inline(always)]
    pub fn nd20(&self) -> ND20_R {
        ND20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    ///Bit 21 - New data
    #[inline(always)]
    pub fn nd21(&self) -> ND21_R {
        ND21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    ///Bit 22 - New data
    #[inline(always)]
    pub fn nd22(&self) -> ND22_R {
        ND22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    ///Bit 23 - New data
    #[inline(always)]
    pub fn nd23(&self) -> ND23_R {
        ND23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    ///Bit 24 - New data
    #[inline(always)]
    pub fn nd24(&self) -> ND24_R {
        ND24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    ///Bit 25 - New data
    #[inline(always)]
    pub fn nd25(&self) -> ND25_R {
        ND25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    ///Bit 26 - New data
    #[inline(always)]
    pub fn nd26(&self) -> ND26_R {
        ND26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    ///Bit 27 - New data
    #[inline(always)]
    pub fn nd27(&self) -> ND27_R {
        ND27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    ///Bit 28 - New data
    #[inline(always)]
    pub fn nd28(&self) -> ND28_R {
        ND28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    ///Bit 29 - New data
    #[inline(always)]
    pub fn nd29(&self) -> ND29_R {
        ND29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    ///Bit 30 - New data
    #[inline(always)]
    pub fn nd30(&self) -> ND30_R {
        ND30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    ///Bit 31 - New data
    #[inline(always)]
    pub fn nd31(&self) -> ND31_R {
        ND31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - New data
    #[inline(always)]
    pub fn nd0(&mut self) -> ND0_W {
        ND0_W { w: self }
    }
    ///Bit 1 - New data
    #[inline(always)]
    pub fn nd1(&mut self) -> ND1_W {
        ND1_W { w: self }
    }
    ///Bit 2 - New data
    #[inline(always)]
    pub fn nd2(&mut self) -> ND2_W {
        ND2_W { w: self }
    }
    ///Bit 3 - New data
    #[inline(always)]
    pub fn nd3(&mut self) -> ND3_W {
        ND3_W { w: self }
    }
    ///Bit 4 - New data
    #[inline(always)]
    pub fn nd4(&mut self) -> ND4_W {
        ND4_W { w: self }
    }
    ///Bit 5 - New data
    #[inline(always)]
    pub fn nd5(&mut self) -> ND5_W {
        ND5_W { w: self }
    }
    ///Bit 6 - New data
    #[inline(always)]
    pub fn nd6(&mut self) -> ND6_W {
        ND6_W { w: self }
    }
    ///Bit 7 - New data
    #[inline(always)]
    pub fn nd7(&mut self) -> ND7_W {
        ND7_W { w: self }
    }
    ///Bit 8 - New data
    #[inline(always)]
    pub fn nd8(&mut self) -> ND8_W {
        ND8_W { w: self }
    }
    ///Bit 9 - New data
    #[inline(always)]
    pub fn nd9(&mut self) -> ND9_W {
        ND9_W { w: self }
    }
    ///Bit 10 - New data
    #[inline(always)]
    pub fn nd10(&mut self) -> ND10_W {
        ND10_W { w: self }
    }
    ///Bit 11 - New data
    #[inline(always)]
    pub fn nd11(&mut self) -> ND11_W {
        ND11_W { w: self }
    }
    ///Bit 12 - New data
    #[inline(always)]
    pub fn nd12(&mut self) -> ND12_W {
        ND12_W { w: self }
    }
    ///Bit 13 - New data
    #[inline(always)]
    pub fn nd13(&mut self) -> ND13_W {
        ND13_W { w: self }
    }
    ///Bit 14 - New data
    #[inline(always)]
    pub fn nd14(&mut self) -> ND14_W {
        ND14_W { w: self }
    }
    ///Bit 15 - New data
    #[inline(always)]
    pub fn nd15(&mut self) -> ND15_W {
        ND15_W { w: self }
    }
    ///Bit 16 - New data
    #[inline(always)]
    pub fn nd16(&mut self) -> ND16_W {
        ND16_W { w: self }
    }
    ///Bit 17 - New data
    #[inline(always)]
    pub fn nd17(&mut self) -> ND17_W {
        ND17_W { w: self }
    }
    ///Bit 18 - New data
    #[inline(always)]
    pub fn nd18(&mut self) -> ND18_W {
        ND18_W { w: self }
    }
    ///Bit 19 - New data
    #[inline(always)]
    pub fn nd19(&mut self) -> ND19_W {
        ND19_W { w: self }
    }
    ///Bit 20 - New data
    #[inline(always)]
    pub fn nd20(&mut self) -> ND20_W {
        ND20_W { w: self }
    }
    ///Bit 21 - New data
    #[inline(always)]
    pub fn nd21(&mut self) -> ND21_W {
        ND21_W { w: self }
    }
    ///Bit 22 - New data
    #[inline(always)]
    pub fn nd22(&mut self) -> ND22_W {
        ND22_W { w: self }
    }
    ///Bit 23 - New data
    #[inline(always)]
    pub fn nd23(&mut self) -> ND23_W {
        ND23_W { w: self }
    }
    ///Bit 24 - New data
    #[inline(always)]
    pub fn nd24(&mut self) -> ND24_W {
        ND24_W { w: self }
    }
    ///Bit 25 - New data
    #[inline(always)]
    pub fn nd25(&mut self) -> ND25_W {
        ND25_W { w: self }
    }
    ///Bit 26 - New data
    #[inline(always)]
    pub fn nd26(&mut self) -> ND26_W {
        ND26_W { w: self }
    }
    ///Bit 27 - New data
    #[inline(always)]
    pub fn nd27(&mut self) -> ND27_W {
        ND27_W { w: self }
    }
    ///Bit 28 - New data
    #[inline(always)]
    pub fn nd28(&mut self) -> ND28_W {
        ND28_W { w: self }
    }
    ///Bit 29 - New data
    #[inline(always)]
    pub fn nd29(&mut self) -> ND29_W {
        ND29_W { w: self }
    }
    ///Bit 30 - New data
    #[inline(always)]
    pub fn nd30(&mut self) -> ND30_W {
        ND30_W { w: self }
    }
    ///Bit 31 - New data
    #[inline(always)]
    pub fn nd31(&mut self) -> ND31_W {
        ND31_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FDCAN New Data 1 Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ndat1](index.html) module
pub struct NDAT1_SPEC;
impl crate::RegisterSpec for NDAT1_SPEC {
    type Ux = u32;
}
///`read()` method returns [ndat1::R](R) reader structure
impl crate::Readable for NDAT1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ndat1::W](W) writer structure
impl crate::Writable for NDAT1_SPEC {
    type Writer = W;
}
///`reset()` method sets NDAT1 to value 0
impl crate::Resettable for NDAT1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
