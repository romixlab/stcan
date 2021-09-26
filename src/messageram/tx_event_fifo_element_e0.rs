#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TxEventFifoElementE0 {
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

pub struct ESIR{
    bits: bool,
}
impl ESIR{
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

pub struct _ESIW<'a> {
    w: &'a mut W,
}
impl<'a> _ESIW<'a> {
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
}

pub struct XTDR{
    bits: bool,
}
impl XTDR{
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

pub struct _XTDW<'a> {
    w: &'a mut W,
}
impl<'a> _XTDW<'a> {
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
        self.w.bits &= !(0x01 << 30);
        self.w.bits |= ((value as u32) & 0x01) << 30;
        self.w
    }
}
pub struct RTRR{
    bits: bool,
}
impl RTRR{
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

pub struct _RTRW<'a> {
    w: &'a mut W,
}
impl<'a> _RTRW<'a> {
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
}

pub struct IDR{
    bits: u32,
}
impl IDR{
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}

pub struct _IDW<'a> {
    w: &'a mut W,
}
impl<'a> _IDW<'a> {
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits &= !(0x1FFFFFFF << 0);
        self.w.bits |= ((value as u32) & 0x1FFFFFFF) << 0;
        self.w
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
        let bits = ((self.bits >> 0) & 0x1FFFFFFF) as u32;
        IDR { bits }
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
    pub fn esi(&mut self) -> _ESIW {
        _ESIW { w: self }
    }
    #[inline(always)]
    pub fn xtd(&mut self) -> _XTDW {
        _XTDW { w: self }
    }
    #[inline(always)]
    pub fn rtr(&mut self) -> _RTRW {
        _RTRW { w: self }
    }
    #[inline(always)]
    pub fn id(&mut self) -> _IDW {
        _IDW { w: self }
    }
}