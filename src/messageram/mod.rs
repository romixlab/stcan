/// Rx FIFO 0 and 1 R0 register
pub(crate) mod rx_fifo_element_r0;
/// Rx FIFO 0 and 1 R1 register
pub(crate) mod rx_fifo_element_r1;
/// Tx Buffer 0, 1, 2 T0 register
pub(crate) mod tx_buffer_element_t0;
pub(crate) mod tx_buffer_element_t1;

pub(crate) mod extended_message_id_filter_f0;
pub(crate) mod extended_message_id_filter_f1;
pub(crate) mod standart_message_id_filter;
pub(crate) mod tx_event_fifo_element_e0;
pub(crate) mod tx_event_fifo_element_e1;

#[repr(C)]
#[doc = "Standard message ID Filter element, up to 28 of such filters can be configured"]
pub struct StandartMessageIdFilter {
    register: vcell::VolatileCell<u32>,
}
#[repr(C)]
pub struct ExtendedMessageIdFilterF0 {
    register: vcell::VolatileCell<u32>,
}
#[repr(C)]
pub struct ExtendedMessageIdFilterF1 {
    register: vcell::VolatileCell<u32>,
}
#[repr(C)]
pub struct ExtendedMessageIdFilter {
    pub f0: ExtendedMessageIdFilterF0,
    pub f1: ExtendedMessageIdFilterF1,
}

#[repr(C)]
pub struct RxFifoElementR0 {
    register: vcell::VolatileCell<u32>,
}
#[repr(C)]
pub struct RxFifoElementR1 {
    register: vcell::VolatileCell<u32>,
}
#[repr(C)]
pub struct RxFifoElement {
    pub r0: RxFifoElementR0,
    pub r1: RxFifoElementR1,
    pub data: [vcell::VolatileCell<u32>; 16],
}
impl RxFifoElement {
    unsafe fn data(&self) -> &[u8] {
        let p = (self as *const _ as *const u8).offset(8);
        let dlc = self.r1.read().dlc().bits() as usize;
        core::slice::from_raw_parts(p, dlc)
    }
}

#[repr(C)]
pub struct TxEventFifoElementE0 {
    register: vcell::VolatileCell<u32>,
}
#[repr(C)]
pub struct TxEventFifoElementE1 {
    register: vcell::VolatileCell<u32>,
}

#[repr(C)]
pub struct TxEventFifoElement {
    pub e0: TxEventFifoElementE0,
    pub e1: TxEventFifoElementE1,
}

#[repr(C)]
pub struct TxBufferElementT0 {
    register: vcell::VolatileCell<u32>,
}
#[repr(C)]
pub struct TxBufferElementT1 {
    register: vcell::VolatileCell<u32>,
}
#[repr(C)]
pub struct TxBufferElement {
    pub t0: TxBufferElementT0,
    pub t1: TxBufferElementT1,
    pub data: [vcell::VolatileCell<u32>; 16],
}

/// Standard filter configuration
/// All enabled filter elements are used for acceptance filtering of standard frames.
/// Acceptance filtering stops at the first matching enabled filter element or when the end
/// of the filter list is reached. If SFEC = 100, 101 or 110 a match sets interrupt flag
/// IR.HPM and, if enabled, an interrupt is generated. In this case register HPMS is
/// updated with the status of the priority match.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FilterConfiguration {
    #[doc = "Disable filter element"]
    Disabled = 0b000,
    #[doc = "Store in Rx FIFO 0 if filter matches"]
    StoreIntoFifo0 = 0b001,
    #[doc = "Store in Rx FIFO 1 if filter matches"]
    StoreIntoFifo1 = 0b010,
    #[doc = "Reject ID if filter matches"]
    Reject = 0b011,
    #[doc = "Set priority if filter matches"]
    SetPriority = 0b100,
    #[doc = "Set priority and store in FIFO 0 if filter matches"]
    SetPriorityAndStoreIntoFifo0 = 0b101,
    #[doc = "Set priority and store in FIFO 1 if filter matches"]
    SetPriorityAndStoreIntoFifo1 = 0b110,
    #[cfg(any(feature = "g0", feature = "g4"))]
    #[doc = "Reserved"]
    _Reserved = 0b111,
    #[cfg(feature = "h7")]
    #[doc = "Store into Rx buffer or as debug message regardles of the filter configuration"]
    StoreIntoBuffer = 0b111,
}
impl From<FilterConfiguration> for u8 {
    fn from(variant: FilterConfiguration) -> Self {
        variant as _
    }
}