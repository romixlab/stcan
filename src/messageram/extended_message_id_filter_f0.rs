#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ExtendedMessageIdFilterF0 {
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

use super::FilterConfiguration;
#[doc = "Reader of extended filter element configuration"]
pub struct EFECR {
    bits: u8,
}
impl EFECR {
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

#[doc = "Writer of extended filter element configuration"]
pub struct EFECW<'a> {
    w: &'a mut W,
}
impl<'a> EFECW<'a> {
    #[doc = "Set raw bits"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0b111 << 29);
        self.w.bits |= ((value as u32) & 0b111) << 29;
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

#[doc = "Reader of extended filter ID 1"]
pub struct EFID1R {
    bits: u32,
}
impl EFID1R {
    pub fn bits(&self) -> u32 {
        self.bits
    }
    pub fn extended_id(&self) -> u32 {
        self.bits
    }
}

pub struct EFID1W<'a> {
    w: &'a mut W,
}
impl<'a> EFID1W<'a> {
    pub fn bits(self, value: u32) -> &'a mut W {
        self.w.bits &= !(0x1F_FF_FF_FF << 0);
        self.w.bits |= ((value as u32) & 0x1F_FF_FF_FF) << 0;
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
    pub fn efec(&self) -> EFECR {
        let bits = ((self.bits >> 29) & 0b111) as u8;
        EFECR { bits }
    }
    #[inline(always)]
    pub fn efid1(&self) -> EFID1R {
        let bits = (self.bits & 0x1F_FF_FF_FF) as u32;
        EFID1R { bits }
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
    pub fn efec(&mut self) -> EFECW {
        EFECW { w: self }
    }
    #[inline(always)]
    pub fn efid1(&mut self) -> EFID1W {
        EFID1W { w: self }
    }
}
