#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

mod messageram;
pub mod pac;
pub mod bit_timing;
use pac::RegisterBlock;
use pac::generic::*;
use crate::bit_timing::{DataBitTiming, NominalBitTiming, TransmitterDelayCompensation};

pub unsafe trait Instance {
    /// Pointer to the instance's register block.
    const REGISTERS: *mut RegisterBlock;

    fn registers(&self) -> &RegisterBlock {
        unsafe { &*Self::REGISTERS }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum OperationMode {
    /// Normal operation mode
    Normal,
    /// Intended for applications that adapt themselves to different CAN bitrates.
    /// Frames can be sent and received, acknowledges are given to valid received frames.
    /// No error or overload frames are sent.
    /// Entered manually in initialization, when tx handler was not able to read message RAM in time
    /// or when clock calibration is in progress.
    /// Can be exited at any time.
    Restricted,
    /// Only recessive bits are sent on the bus. Frames can only be received.
    /// Can be used to analyze traffic on the bus without affecting it in any way.
    /// Entered manually in initialization or on error level S3 (TTOST.EL=11)
    /// Can be exited at any time.
    BusMonitoring,
    ExternalLoopback,
    InternalLoopback
}

pub struct CommonCanConfig<I: Instance> {
    instance: I
}

impl<I: Instance> CommonCanConfig<I> {
    /// Enter initialization mode
    pub fn new(instance: I) -> Self {
        instance.registers().cccr.modify(|_, w| w.init().set_bit());
        while instance.registers().cccr.read().init().bit_is_clear() {}
        instance.registers().cccr.modify(|_, w| w.cce().set_bit());
        while instance.registers().cccr.read().cce().bit_is_clear() {}
        instance.registers().cccr.reset(); // write 0b11
        Self {
            instance
        }
    }

    pub fn set_bit_timing(self, nbtp: NominalBitTiming) -> Self {
        self.instance.registers().nbtp.modify(|_, w| w
            .nbrp().bits(nbtp.brp)
            .ntseg1().bits(nbtp.tseg1)
            .ntseg2().bits(nbtp.tseg2)
            .nsjw().bits(nbtp.sjw)
        );
        self
    }

    /// Set operation mode to enter after enable() is called.
    pub fn set_operation_mode(self, mode: OperationMode) -> Self {
        match mode {
            OperationMode::Normal => {},
            OperationMode::Restricted => {
                self.instance.registers().cccr.modify(|_, w| w.asm().set_bit());
            },
            OperationMode::BusMonitoring => {
                self.instance.registers().cccr.modify(|_, w| w.mon().set_bit());
            },
            OperationMode::ExternalLoopback | OperationMode::InternalLoopback => {
                self.instance.registers().cccr.modify(|_, w| w.test().set_bit());
                self.instance.registers().test.modify(|_, w| w.lbck().set_bit());
                if mode == OperationMode::InternalLoopback {
                    self.instance.registers().test.modify(|_, w| w.tx().recessive());
                }
            },
        }
        self
    }

    pub fn set_transmit_pause(self, enabled: bool) -> Self {
        if enabled {
            self.instance.registers().cccr.modify(|_, w| w.txp().set_bit());
        }
        self
    }

    pub fn set_automatic_retransmission(self, enabled: bool) -> Self {
        if !enabled {
            self.instance.registers().cccr.modify(|_, w| w.dar().set_bit());
        }
        self
    }

    pub fn enable_fd_mode(self) -> FdCanConfig<I> {
        self.instance.registers().cccr.modify(|_, w| w.fdoe().set_bit());
        FdCanConfig {
            instance: self.instance
        }
    }

    /// Leave initialization mode and enable
    pub fn enable(mut self) -> FdCan<I> {
        self.instance.registers().cccr.modify(|_, w| w.init().clear_bit());
        while self.instance.registers().cccr.read().init().bit_is_set() {}
        FdCan {
            instance: self.instance
        }
    }
}



pub struct FdCanConfig<I: Instance> {
    instance: I
}

impl<I: Instance> FdCanConfig<I> {
    pub fn set_data_bit_timing(self, dbtp: DataBitTiming) -> Self {
        self.instance.registers().dbtp.modify(|_, w| w
            .dbrp().bits(dbtp.brp)
            .dtseg1().bits(dbtp.tseg1)
            .dtseg2().bits(dbtp.tseg2)
            .dsjw().bits(dbtp.sjw)
        );
        self
    }

    pub fn enable_transceiver_delay_compensation(self, tdc: TransmitterDelayCompensation) -> Self {
        self.instance.registers().dbtp.modify(|_, w| w.tdc().set_bit());
        self.instance.registers().tdcr.write(|w| w
            .tdco().bits(tdc.tdco)
            .tdcf().bits(tdc.tdcf)
        );
        self
    }

    pub fn set_bitrate_switching(self, enabled: bool) -> Self {
        if enabled {
            self.instance.registers().cccr.modify(|_, w| w.bse().set_bit());
        }
        self
    }

    pub fn set_non_iso(self, enabled: bool) -> Self {
        if enabled {
            self.instance.registers().cccr.modify(|_, w| w.niso().set_bit());
        }
        self
    }

    /// Leave initialization mode and enable
    pub fn enable(mut self) -> FdCan<I> {
        self.instance.registers().cccr.modify(|_, w| w.init().clear_bit());
        while self.instance.registers().cccr.read().init().bit_is_set() {}
        FdCan {
            instance: self.instance
        }
    }
}

pub struct FdCan<I: Instance> {
    instance: I
}

impl<I: Instance> FdCan<I> {

    /// Entered when tx handler ? was not able to read data from the message RAM in time.
    /// Also entered when clock calibration is in progress, if enabled.
    /// Can be enabled manually only during configuration phase.
    /// Application can test different bitrates and leave this mode after receiving a valid frame.
    /// Frames can be sent? and received, acknowledge is only given to valid frames, no error or
    /// overload frames are sent.
    pub fn is_restricted_operation_mode(&self) -> bool {
        self.instance.registers().cccr.read().asm().bit_is_set()
    }

    /// Leave restricted operation mode, can be called at any time.
    pub fn leave_restricted_operation_mode(&mut self) {
        self.instance.registers().cccr.modify(|_, w| w.asm().clear_bit());
    }

    /// Leave bus monitoring mode, can be called at any time.
    pub fn leave_bus_monitoring_mode(&mut self) {
        self.instance.registers().cccr.modify(|_, w| w.mon().clear_bit());
    }
}