#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::StandartMessageIdFilter {
    #[doc = r"Modifies the contents of the register"]
    #[inline(always)]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        self.register.set(f(&R { bits }, &mut W { bits }).bits);
    }
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r"Writes to the register"]
    #[inline(always)]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        self.register.set(
            f(&mut W {
                bits: Self::reset_value(),
            })
            .bits,
        );
    }
    #[doc = r"Reset value of the register"]
    #[inline(always)]
    pub const fn reset_value() -> u32 {
        0x0000_0000
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}

#[doc = "Standard filter type"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FilterType {
    #[doc = "Range filter from SFID1 to SFID2"]
    RangeFilter = 0b00,
    #[doc = "Dual ID filter for SFID1 or SFID2"]
    DualIdFilter = 0b01,
    #[doc = "Classic filter: SFID1 = filter, SFID2 = mask"]
    ClassicFilter = 0b10,
    #[doc = "Filter element disabled"]
    DisabledFilter = 0b11,
}
impl From<FilterType> for u8 {
    fn from(variant: FilterType) -> Self {
        variant as _
    }
}
#[doc = "Reader of standard filter type field"]
pub struct SFTR {
    bits: u8,
}
impl SFTR {
    #[doc = "Get enumerated value"]
    #[inline(always)]
    pub fn variant(&self) -> FilterType {
        use FilterType::*;
        match self.bits {
            0b00 => RangeFilter,
            0b01 => DualIdFilter,
            0b10 => ClassicFilter,
            0b11 => DisabledFilter,
            _ => unreachable!(),
        }
    }
}

#[doc = "Writer for standard filter type field"]
pub struct SFTW<'a> {
    w: &'a mut W,
}
impl<'a> SFTW<'a> {
    #[doc = "Set raw bits"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0b11 << 30);
        self.w.bits |= ((value as u32) & 0b11) << 30;
        self.w
    }
    #[doc = "Set enumerated value"]
    #[inline(always)]
    pub fn variant(self, variant: FilterType) -> &'a mut W {
        unsafe {
            self.bits(variant.into())
        }
    }
}

use super::FilterConfiguration;
#[doc = "Reader of standard filter element configuration field"]
pub struct SFECR {
    bits: u8,
}
impl SFECR {
    pub fn variant(&self) -> FilterConfiguration {
        use FilterConfiguration::*;
        match self.bits {
            0b000 => Disabled,
            0b001 => StoreIntoFifo0,
            0b010 => StoreIntoFifo1,
            0b011 => Reject,
            0b100 => SetPriority,
            0b101 => SetPriorityAndStoreIntoFifo0,
            0b110 => SetPriorityAndStoreIntoFifo1,
            #[cfg(any(feature = "g0", feature = "g4"))]
            0b111 => FilterConfiguration::_Reserved,
            #[cfg(feature = "h7")]
            0b111 => FilterConfiguration::StoreIntoBuffer,
            _ => unreachable!(),
        }
    }
}

#[doc = "Writer of standard filter element configuration field"]
pub struct SFECW<'a> {
    w: &'a mut W,
}
impl<'a> SFECW<'a> {
    #[doc = "Set raw bits"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0b111 << 27);
        self.w.bits |= ((value as u32) & 0b111) << 27;
        self.w
    }
    #[doc = "Set enumerated value"]
    #[inline(always)]
    pub fn variant(self, variant: FilterConfiguration) -> &'a mut W {
        unsafe {
            self.bits(variant.into())
        }
    }
}

#[doc = "Reader of standard filter ID 1"]
pub struct SFID1R {
    bits: u16,
}
impl SFID1R {
    pub fn bits(&self) -> u16 {
        self.bits
    }
    pub fn standard_id(&self) -> u16 {
        self.bits
    }
}

#[doc = "Writer of standard filter ID 1"]
pub struct SFID1W<'a> {
    w: &'a mut W,
}
impl<'a> SFID1W<'a> {
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits &= !(0b0111_1111_1111 << 16);
        self.w.bits |= ((value as u32) & 0b0111_1111_1111) << 16;
        self.w
    }
    pub fn standard_id(self, value: u16) -> &'a mut W {
        self.bits(value)
    }
}

#[doc = "Reader of standard filter ID 2"]
pub struct SFID2R {
    bits: u16,
}
impl SFID2R {
    pub fn bits(&self) -> u16 {
        self.bits
    }
    pub fn standard_id(&self) -> u16 {
        self.bits
    }
}

#[doc = "Writer of standard filter ID 2"]
pub struct SFID2W<'a> {
    w: &'a mut W,
}
impl<'a> SFID2W<'a> {
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits &= !0b0111_1111_1111;
        self.w.bits |= (value as u32) & 0b0111_1111_1111;
        self.w
    }
    pub fn standard_id(self, value: u16) -> &'a mut W {
        self.bits(value)
    }
}
impl R {
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[inline(always)]
    pub fn sft(&self) -> SFTR {
        let bits = ((self.bits >> 30) & 0b11) as u8;
        SFTR { bits }
    }
    #[inline(always)]
    pub fn sfec(&self) -> SFECR {
        let bits = ((self.bits >> 27) & 0b111) as u8;
        SFECR { bits }
    }
    #[inline(always)]
    pub fn sfid1(&self) -> SFID1R {
        let bits = ((self.bits >> 16) & 0b0111_1111_1111) as u16;
        SFID1R { bits }
    }

    #[inline(always)]
    pub fn sfid2(&self) -> SFID2R {
        let bits = ((self.bits >> 0) & 0b0111_1111_1111) as u16;
        SFID2R { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[inline(always)]
    pub fn sft(&mut self) -> SFTW {
        SFTW { w: self }
    }
    #[inline(always)]
    pub fn sfec(&mut self) -> SFECW {
        SFECW { w: self }
    }
    #[inline(always)]
    pub fn sfid1(&mut self) -> SFID1W {
        SFID1W { w: self }
    }
    pub fn sfid2(&mut self) -> SFID2W {
        SFID2W { w: self }
    }
}
