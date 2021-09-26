#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RxFifoElementR1 {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}

/// Accepted non-matching frame flag
pub struct ANMFR {
    bits: bool,
}
impl ANMFR {
    #[inline(always)]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[inline(always)]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[inline(always)]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[inline(always)]
    pub fn matched(&self) -> bool {
        !self.bit()
    }
    #[inline(always)]
    pub fn not_matched(&self) -> bool {
        self.bit()
    }
}

/// Filter index
pub struct FIDXR {
    bits: u8,
}
impl FIDXR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    pub fn index(&self) -> u8 {
        self.bits
    }
}

/// FD format flag
pub struct FDFR {
    bits: bool,
}
impl FDFR {
    #[inline(always)]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[inline(always)]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[inline(always)]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn is_fd_frame(&self) -> bool {
        self.bit()
    }
    pub fn is_standard_frame(&self) -> bool {
        !self.bit()
    }
}

/// Bit rate switch flag
pub struct BRSR {
    bits: bool,
}
impl BRSR {
    #[inline(always)]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[inline(always)]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[inline(always)]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}

/// Data length code
pub struct DLCR {
    bits: u8,
}
impl DLCR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}

/// Rx timestamp
/// Timestamp Counter value captured on start of frame reception. Resolution depending
/// on configuration of the Timestamp Counter Prescaler TSCC.TCP.
pub struct RXTSR {
    bits: u16,
}
impl RXTSR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}

impl R {
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[inline(always)]
    pub fn anmf(&self) -> ANMFR {
        let bits = ((self.bits >> 31) & 0x01) != 0;
        ANMFR { bits }
    }
    #[inline(always)]
    pub fn fidx(&self) -> FIDXR {
        let bits = ((self.bits >> 24) & 0x7F) as u8;
        FIDXR { bits }
    }
    #[inline(always)]
    pub fn fdf(&self) -> FDFR {
        let bits = ((self.bits >> 21) & 0x01) != 0;
        FDFR { bits }
    }

    #[inline(always)]
    pub fn brs(&self) -> BRSR {
        let bits = ((self.bits >> 20) & 0x01) != 0;
        BRSR { bits }
    }

    #[inline(always)]
    pub fn dlc(&self) -> DLCR {
        let bits = ((self.bits >> 16) & 0x0F) as u8;
        DLCR { bits }
    }

    #[inline(always)]
    pub fn rxts(&self) -> RXTSR {
        let bits = ((self.bits >> 0) & 0xFFFF) as u16;
        RXTSR { bits }
    }
}
