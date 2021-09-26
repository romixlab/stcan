use vhrdcan::id::{FrameId, StandardId, ExtendedId};

#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TxBufferElementT0 {
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

/// Reader for error state indicator (FD only)
pub struct ESIR {
    bits: bool,
}
impl ESIR {
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
    pub fn if_recessive_forced(&self) -> bool {
        self.bit()
    }
}

/// Writer for error state indicator flag (FD only)
pub struct ESIW<'a> {
    w: &'a mut W,
}
impl<'a> ESIW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(0x01 << 31);
        self.w.bits |= ((value as u32) & 0x01) << 31;
        self.w
    }
    #[doc = "Transmit ESI bit as recessive"]
    #[inline(always)]
    pub fn force_recessive(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = "ESI bit dependes on error passive flag"]
    #[inline(always)]
    pub fn use_error_passive_flag(self) -> &'a mut W {
        self.bit(false)
    }
}

/// Reader for extended identifier flag
pub struct XTDR {
    bits: bool,
}
impl XTDR {
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
    pub fn is_standard(&self) -> bool {
        !self.bit()
    }
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        self.bit()
    }
}

pub struct XTDW<'a> {
    w: &'a mut W,
}
impl<'a> XTDW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub unsafe fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub unsafe fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(0x01 << 30);
        self.w.bits |= ((value as u32) & 0x01) << 30;
        self.w
    }
    #[doc = "Transmit message using 11 bit ID"]
    #[inline(always)]
    pub unsafe fn use_standard_id(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = "Transmit message using 29 bit ID"]
    #[inline(always)]
    pub unsafe fn use_extended_id(self) -> &'a mut W {
        self.bit(true)
    }
}

/// Reader for remote transmission request
pub struct RTRR {
    bits: bool,
}
impl RTRR {
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
    pub fn is_data_frame(&self) -> bool {
        !self.bit()
    }
    #[inline(always)]
    pub fn is_remote_frame(&self) -> bool {
        self.bit()
    }
}

/// Writer for remote transmission request flag
pub struct RTRW<'a> {
    w: &'a mut W,
}
impl<'a> RTRW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(0x01 << 29);
        self.w.bits |= ((value as u32) & 0x01) << 29;
        self.w
    }
    #[doc = r"Transmit data frame"]
    #[inline(always)]
    pub fn transmit_data_frame(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Transmit remote frame"]
    #[inline(always)]
    pub fn transmit_remote_frame(self) -> &'a mut W {
        self.bit(true)
    }
}

/// Reader for frame ID
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
    pub fn frame_id(&self) -> FrameId {
        if self.is_extended {
            unsafe { FrameId::Extended(ExtendedId::new_unchecked(self.bits)) }
        } else {
            unsafe { FrameId::Standard(StandardId::new_unchecked(self.bits as u16)) } // correct, shifted below
        }
    }
}

/// Writer for frame ID
pub struct IDW<'a> {
    w: &'a mut W,
}
impl<'a> IDW<'a> {
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits &= !(0x1F_FF_FF_FF << 0);
        self.w.bits |= ((value as u32) & 0x1F_FF_FF_FF) << 0;
        self.w
    }
    pub fn standard_id(self, value: u16) -> &'a mut W {
        self.w.bits &= !(0x01 << 30);
        unsafe {
            self.bits(((value & 0x3F_FF) as u32) << 18)
        }
    }
    pub fn extended_id(self, value: u32) -> &'a mut W {
        self.w.bits |= 0x01 << 30;
        unsafe { self.bits(value) }
    }
    pub fn frame_id(self, value: FrameId) -> &'a mut W {
        match value {
            FrameId::Standard(sid) => self.standard_id(sid.id()),
            FrameId::Extended(eid) => self.extended_id(eid.id()),
        }
    }
}

impl R {
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[inline(always)]
    pub fn esi(&self) -> ESIR {
        let bits = ((self.bits >> 31) & 0x01) != 0;
        ESIR { bits }
    }
    #[inline(always)]
    pub fn xtd(&self) -> XTDR {
        let bits = ((self.bits >> 30) & 0x01) != 0;
        XTDR { bits }
    }
    #[inline(always)]
    pub fn rtr(&self) -> RTRR {
        let bits = ((self.bits >> 29) & 0x01) != 0;
        RTRR { bits }
    }

    #[inline(always)]
    pub fn id(&self) -> IDR {
        let is_extended = self.xtd().is_extended();
        let bits = if is_extended {
            self.bits & 0x1F_FF_FF_FF
        } else {
            ((self.bits >> 18) & 0x3F_FF) as u32
        };
        IDR { is_extended, bits }
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
    pub fn esi(&mut self) -> ESIW {
        ESIW { w: self }
    }
    #[inline(always)]
    pub fn xtd(&mut self) -> XTDW {
        XTDW { w: self }
    }
    #[inline(always)]
    pub fn rtr(&mut self) -> RTRW {
        RTRW { w: self }
    }
    #[inline(always)]
    pub fn id(&mut self) -> IDW {
        IDW { w: self }
    }
}
