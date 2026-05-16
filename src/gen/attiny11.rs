//! The AVR ATtiny11 microcontroller
//!
//! # Variants
//! |        | Pinout | Mcu age | Operating temperature | Operating voltage | Max speed |
//! |--------|--------|---------|-----------------------|-------------------|-----------|
//! | standard |  |  | 0°C - 0°C | 2.7V - 5.5V | 0 MHz |
//!

#![allow(non_upper_case_globals)]

/// `LOW` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RSTDISBL | 1000 |
/// | FSTRT | 10000 |
/// | CKSEL | 111 |
pub const LOW: *mut u8 = 0x0 as *mut u8;

/// `LOCKBIT` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LB | 110 |
pub const LOCKBIT: *mut u8 = 0x0 as *mut u8;

/// Analog Comparator Control And Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ACIS | 11 |
/// | ACIE | 1000 |
/// | ACI | 10000 |
/// | ACO | 100000 |
/// | ACD | 10000000 |
pub const ACSR: *mut u8 = 0x8 as *mut u8;

/// Input Pins, Port B.
pub const PINB: *mut u8 = 0x16 as *mut u8;

/// Data Direction Register, Port B.
pub const DDRB: *mut u8 = 0x17 as *mut u8;

/// Data Register, Port B.
pub const PORTB: *mut u8 = 0x18 as *mut u8;

/// Watchdog Timer Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDTOE | 10000 |
/// | WDE | 1000 |
/// | WDP | 111 |
pub const WDTCR: *mut u8 = 0x21 as *mut u8;

/// Timer Counter 0.
pub const TCNT0: *mut u8 = 0x32 as *mut u8;

/// Timer/Counter0 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CS00 | 1 |
/// | CS01 | 10 |
/// | CS02 | 100 |
pub const TCCR0: *mut u8 = 0x33 as *mut u8;

/// MCU Status register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EXTRF | 10 |
/// | PORF | 1 |
pub const MCUSR: *mut u8 = 0x34 as *mut u8;

/// MCU Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SE | 100000 |
/// | ISC0 | 11 |
/// | SM | 10000 |
pub const MCUCR: *mut u8 = 0x35 as *mut u8;

/// Timer/Counter Interrupt Flag register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TOV0 | 10 |
pub const TIFR: *mut u8 = 0x38 as *mut u8;

/// Timer/Counter Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TOIE0 | 10 |
pub const TIMSK: *mut u8 = 0x39 as *mut u8;

/// General Interrupt Flag register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCIF | 100000 |
/// | INTF0 | 1000000 |
pub const GIFR: *mut u8 = 0x3A as *mut u8;

/// General Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | INT0 | 1000000 |
/// | PCIE | 100000 |
pub const GIMSK: *mut u8 = 0x3B as *mut u8;

/// Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | S | 10000 |
/// | T | 1000000 |
/// | H | 100000 |
/// | N | 100 |
/// | C | 1 |
/// | V | 1000 |
/// | I | 10000000 |
/// | Z | 10 |
pub const SREG: *mut u8 = 0x3F as *mut u8;

/// Bitfield on register `ACSR`
pub const ACIS: u8 = 0x3;

/// Bitfield on register `ACSR`
pub const ACIE: u8 = 0x8;

/// Bitfield on register `ACSR`
pub const ACI: u8 = 0x10;

/// Bitfield on register `ACSR`
pub const ACO: u8 = 0x20;

/// Bitfield on register `ACSR`
pub const ACD: u8 = 0x80;

/// Bitfield on register `GIFR`
pub const PCIF: u8 = 0x20;

/// Bitfield on register `GIFR`
pub const INTF0: u8 = 0x40;

/// Bitfield on register `GIMSK`
pub const INT0: u8 = 0x40;

/// Bitfield on register `GIMSK`
pub const PCIE: u8 = 0x20;

/// Bitfield on register `LOCKBIT`
pub const LB: u8 = 0x6;

/// Bitfield on register `LOW`
pub const RSTDISBL: u8 = 0x8;

/// Bitfield on register `LOW`
pub const FSTRT: u8 = 0x10;

/// Bitfield on register `LOW`
pub const CKSEL: u8 = 0x7;

/// Bitfield on register `MCUCR`
pub const SE: u8 = 0x20;

/// Bitfield on register `MCUCR`
pub const ISC0: u8 = 0x3;

/// Bitfield on register `MCUCR`
pub const SM: u8 = 0x10;

/// Bitfield on register `MCUSR`
pub const EXTRF: u8 = 0x2;

/// Bitfield on register `MCUSR`
pub const PORF: u8 = 0x1;

/// Bitfield on register `SREG`
pub const S: u8 = 0x10;

/// Bitfield on register `SREG`
pub const T: u8 = 0x40;

/// Bitfield on register `SREG`
pub const H: u8 = 0x20;

/// Bitfield on register `SREG`
pub const N: u8 = 0x4;

/// Bitfield on register `SREG`
pub const C: u8 = 0x1;

/// Bitfield on register `SREG`
pub const V: u8 = 0x8;

/// Bitfield on register `SREG`
pub const I: u8 = 0x80;

/// Bitfield on register `SREG`
pub const Z: u8 = 0x2;

/// Bitfield on register `TCCR0`
pub const CS00: u8 = 0x1;

/// Bitfield on register `TCCR0`
pub const CS01: u8 = 0x2;

/// Bitfield on register `TCCR0`
pub const CS02: u8 = 0x4;

/// Bitfield on register `TIFR`
pub const TOV0: u8 = 0x2;

/// Bitfield on register `TIMSK`
pub const TOIE0: u8 = 0x2;

/// Bitfield on register `WDTCR`
pub const WDTOE: u8 = 0x10;

/// Bitfield on register `WDTCR`
pub const WDE: u8 = 0x8;

/// Bitfield on register `WDTCR`
pub const WDP: u8 = 0x7;

/// `ANALOG_COMP_INTERRUPT` value group
#[allow(non_upper_case_globals)]
pub mod analog_comp_interrupt {
   /// Interrupt on Toggle.
   pub const VAL_0x00: u32 = 0x0;
   /// Reserved.
   pub const VAL_0x01: u32 = 0x1;
   /// Interrupt on Falling Edge.
   pub const VAL_0x02: u32 = 0x2;
   /// Interrupt on Rising Edge.
   pub const VAL_0x03: u32 = 0x3;
}

/// `CLK_SEL_3BIT_EXT` value group
#[allow(non_upper_case_globals)]
pub mod clk_sel_3bit_ext {
   /// No Clock Source (Stopped).
   pub const VAL_0x00: u32 = 0x0;
   /// Running, No Prescaling.
   pub const VAL_0x01: u32 = 0x1;
   /// Running, CLK/8.
   pub const VAL_0x02: u32 = 0x2;
   /// Running, CLK/64.
   pub const VAL_0x03: u32 = 0x3;
   /// Running, CLK/256.
   pub const VAL_0x04: u32 = 0x4;
   /// Running, CLK/1024.
   pub const VAL_0x05: u32 = 0x5;
   /// Running, ExtClk Tx Falling Edge.
   pub const VAL_0x06: u32 = 0x6;
   /// Running, ExtClk Tx Rising Edge.
   pub const VAL_0x07: u32 = 0x7;
}

/// `CPU_SLEEP_MODE_1BIT` value group
#[allow(non_upper_case_globals)]
pub mod cpu_sleep_mode_1bit {
   /// Idle.
   pub const IDLE: u32 = 0x0;
   /// Power Down.
   pub const PDOWN: u32 = 0x1;
}

/// `ENUM_LB` value group
#[allow(non_upper_case_globals)]
pub mod enum_lb {
   /// Further programming and verification disabled.
   pub const PROG_VER_DISABLED: u32 = 0x0;
   /// Further programming disabled.
   pub const PROG_DISABLED: u32 = 0x1;
   /// No memory lock features enabled.
   pub const NO_LOCK: u32 = 0x3;
}

/// `ENUM_SUT_CKSEL` value group
#[allow(non_upper_case_globals)]
pub mod enum_sut_cksel {
   /// External Crystal / Ceramic Resonator.
   pub const EXTXTALCRES: u32 = 0x7;
   /// External Low-frequency Crystal.
   pub const EXTLOFXTAL: u32 = 0x6;
   /// External RC Oscillator.
   pub const EXTRCOSC: u32 = 0x5;
   /// Internal RC Oscillator.
   pub const INTRCOSC: u32 = 0x4;
   /// External Clock.
   pub const EXTCLK: u32 = 0x0;
}

/// `INTERRUPT_SENSE_CONTROL2` value group
#[allow(non_upper_case_globals)]
pub mod interrupt_sense_control2 {
   /// Low Level of INTX.
   pub const VAL_0x00: u32 = 0x0;
   /// Any Logical Change in INTX.
   pub const VAL_0x01: u32 = 0x1;
   /// Falling Edge of INTX.
   pub const VAL_0x02: u32 = 0x2;
   /// Rising Edge of INTX.
   pub const VAL_0x03: u32 = 0x3;
}

/// `WDOG_TIMER_PRESCALE_3BITS` value group
#[allow(non_upper_case_globals)]
pub mod wdog_timer_prescale_3bits {
   /// Oscillator Cycles 16K.
   pub const VAL_0x00: u32 = 0x0;
   /// Oscillator Cycles 32K.
   pub const VAL_0x01: u32 = 0x1;
   /// Oscillator Cycles 64K.
   pub const VAL_0x02: u32 = 0x2;
   /// Oscillator Cycles 128K.
   pub const VAL_0x03: u32 = 0x3;
   /// Oscillator Cycles 256K.
   pub const VAL_0x04: u32 = 0x4;
   /// Oscillator Cycles 512K.
   pub const VAL_0x05: u32 = 0x5;
   /// Oscillator Cycles 1024K.
   pub const VAL_0x06: u32 = 0x6;
   /// Oscillator Cycles 2048K.
   pub const VAL_0x07: u32 = 0x7;
}

