#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ExtendedMessageIdFilterF1 {
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

#[doc = "Extended filter type"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FilterType {
    /// Range filter from EF1ID to EF2ID (EF2ID >= EF1ID)
    RangeWithMask = 0b00,
    /// Dual ID filter for EF1ID or EF2ID
    DualId = 0b01,
    /// Classic filter: EF1ID = filter, EF2ID = mask
    Classic = 0b10,
    /// Range filter from EF1ID to EF2ID (EF2ID >= EF1ID), XIDAM mask not applied
    RangeWithoutMask = 0b11,
}
impl From<FilterType> for u8 {
    fn from(variant: FilterType) -> Self {
        variant as _
    }
}

/// Reader for xtended filter type
pub struct EFTIR {
    bits: u8,
}
impl EFTIR {
    pub fn variant(&self) -> FilterType {
        use FilterType::*;
        match self.bits {
            0b00 => RangeWithMask,
            0b01 => DualId,
            0b10 => Classic,
            0b11 => RangeWithoutMask,
            _ => unreachable!(),
        }
    }
}

/// Writer for extended filter type
pub struct EFTIW<'a> {
    w: &'a mut W,
}
impl<'a> EFTIW<'a> {
    #[doc = "Set raw bits"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0b111 << 29);
        self.w.bits |= ((value as u32) & 0b111) << 29;
        self.w
    }
    #[doc = "Set enumerated value"]
    #[inline(always)]
    pub fn variant(self, value: u8) -> &'a mut W {
        unsafe { self.bits(value.into()) }
    }
}

/// Reader for extended filter ID 2
pub struct EFID2R {
    bits: u32,
}
impl EFID2R {
    pub fn bits(&self) -> u32 {
        self.bits
    }
    pub fn extended_id(&self) -> u32 {
        self.bits
    }
}

/// Writer for extended filter ID 2
pub struct EFID2W<'a> {
    w: &'a mut W,
}
impl<'a> EFID2W<'a> {
    pub fn bits(self, value: u32) -> &'a mut W {
        self.w.bits &= !0x1F_FF_FF_FF;
        self.w.bits |= (value as u32) & 0x1F_FF_FF_FF;
        self.w
    }
    pub fn extended_id(self, value: u32) -> &'a mut W {
        self.bits(value)
    }
}

impl R {
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[inline(always)]
    pub fn efti(&self) -> EFTIR {
        let bits = ((self.bits >> 29) & 0b111) as u8;
        EFTIR { bits }
    }
    #[inline(always)]
    pub fn efid2(&self) -> EFID2R {
        let bits = ((self.bits >> 0) & 0x1F_FF_FF_FF) as u32;
        EFID2R { bits }
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
    pub fn efti(&mut self) -> EFTIW {
        EFTIW { w: self }
    }
    #[inline(always)]
    pub fn efid2(&mut self) -> EFID2W {
        EFID2W { w: self }
    }
}
