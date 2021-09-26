#![deny(missing_docs)]

use vhrdcan::id::{FrameId, StandardId, ExtendedId};

#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RxFifoElementR0 {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}

/// Transmitting node error state indicator (FD only)
pub struct ESIR {
    bits: bool,
}
impl ESIR {
    /// Value of the bit
    #[inline(always)]
    pub fn bit(&self) -> bool {
        self.bits
    }
    /// Checks whether bit is 0
    #[inline(always)]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    /// Checks whether bit is 1
    #[inline(always)]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    /// Checks whether transmitting node is in error passive state
    #[inline(always)]
    pub fn tx_node_is_error_passive(&self) -> bool {
        self.bit()
    }
}

/// Extended identifier flag
pub struct XTDR {
    bits: bool,
}
impl XTDR {
    /// Value of the bit
    #[inline(always)]
    pub fn bit(&self) -> bool {
        self.bits
    }
    /// Checks whether bit is 0
    #[inline(always)]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    /// Checks whether bit is 1
    #[inline(always)]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    /// Checks whether received message has standard ID
    pub fn is_standard(&self) -> bool {
        !self.bit()
    }
    /// Checks whether received message has extended ID
    pub fn is_extended(&self) -> bool {
        self.bit()
    }
}

/// Remote transmission request
pub struct RTRR {
    bits: bool,
}
impl RTRR {
    /// Value of the bit
    #[inline(always)]
    pub fn bit(&self) -> bool {
        self.bits
    }
    /// Checks whether bit is 0 (data frame)
    #[inline(always)]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    /// Checks whether bit is 1 (remote frame)
    #[inline(always)]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}

/// Identifier of received message
pub struct IDR {
    is_extended: bool,
    bits: u32,
}
impl IDR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub unsafe fn bits(&self) -> u32 {
        self.bits
    }
    /// Returns frame id enum
    pub fn frame_id(&self) -> FrameId {
        if self.is_extended {
            unsafe { FrameId::Extended(ExtendedId::new_unchecked(self.bits)) }
        } else {
            unsafe { FrameId::Standard(StandardId::new_unchecked(self.bits as u16)) } // correct, shifted below
        }
    }
}

impl R {
    /// Raw bits of the whole register
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    /// Bit 31 - error state indicator
    #[inline(always)]
    pub fn esi(&self) -> ESIR {
        let bits = ((self.bits >> 31) & 0x01) != 0;
        ESIR { bits }
    }
    /// Bit 30 - extended identifier
    #[inline(always)]
    pub fn xtd(&self) -> XTDR {
        let bits = ((self.bits >> 30) & 0x01) != 0;
        XTDR { bits }
    }
    /// Bit 29 - remote transmission request
    #[inline(always)]
    pub fn rtr(&self) -> RTRR {
        let bits = ((self.bits >> 29) & 0x01) != 0;
        RTRR { bits }
    }
    /// Bits 28:0 - identifier (standard identifier in bits 28:18)
    #[inline(always)]
    pub fn id(&self) -> IDR {
        let is_extended = self.xtd().is_extended();
        let bits = if is_extended {
            self.bits & 0x1F_FF_FF_FF
        } else {
            ((self.bits >> 18) & 0x3_FF) as u32
        };
        IDR { is_extended, bits }
    }
}
