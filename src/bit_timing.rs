pub struct NominalBitTiming {
    pub(crate) sjw: u8,
    pub(crate) brp: u16,
    pub(crate) tseg1: u8,
    pub(crate) tseg2: u8
}

impl NominalBitTiming {
    pub const fn new(sjw: u8, brp: u16, tseg1: u8, tseg2: u8) -> Option<Self> {
        if sjw > 127 || sjw >= tseg2 || brp > 511 || tseg2 > 127 {
            None
        } else {
            Some(Self {
                sjw, brp, tseg1, tseg2
            })
        }
    }
}

pub struct DataBitTiming {
    pub(crate) sjw: u8,
    pub(crate) brp: u8,
    pub(crate) tseg1: u8,
    pub(crate) tseg2: u8
}

impl DataBitTiming {
    pub const fn new(sjw: u8, brp: u8, tseg1: u8, tseg2: u8) -> Option<Self> {
        if brp > 31 || sjw > 15 || tseg1 > 31 || tseg2 > 15 {
            None
        } else {
            Some(Self {
                sjw, brp, tseg1, tseg2
            })
        }
    }
}

pub struct TransmitterDelayCompensation {
    pub(crate) tdco: u8,
    pub(crate) tdcf: u8
}

impl TransmitterDelayCompensation {
    pub const fn new(tdco: u8, tdcf: u8) -> Option<Self> {
        if tdco > 127 || tdcf > 127 {
            None
        } else {
            Some(Self {
                tdco, tdcf
            })
        }
    }
}