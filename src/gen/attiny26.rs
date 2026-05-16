//! The AVR ATtiny26 microcontroller
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
/// | CKOPT | 1000000 |
/// | PLLCK_SUT_CKSEL | 10111111 |
pub const LOW: *mut u8 = 0x0 as *mut u8;

/// `LOCKBIT` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LB | 11 |
pub const LOCKBIT: *mut u8 = 0x0 as *mut u8;

/// `HIGH` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RSTDISBL | 10000 |
/// | BODLEVEL | 10 |
/// | SPIEN | 1000 |
/// | EESAVE | 100 |
/// | BODEN | 1 |
pub const HIGH: *mut u8 = 0x1 as *mut u8;

/// ADC Data Register  Bytes low byte.
pub const ADCL: *mut u8 = 0x24 as *mut u8;

/// ADC Data Register  Bytes.
pub const ADC: *mut u16 = 0x24 as *mut u16;

/// ADC Data Register  Bytes high byte.
pub const ADCH: *mut u8 = 0x25 as *mut u8;

/// The ADC Control and Status register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADFR | 100000 |
/// | ADSC | 1000000 |
/// | ADIE | 1000 |
/// | ADPS | 111 |
/// | ADEN | 10000000 |
/// | ADIF | 10000 |
pub const ADCSR: *mut u8 = 0x26 as *mut u8;

/// The ADC multiplexer Selection Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADLAR | 100000 |
/// | REFS | 11000000 |
/// | MUX | 11111 |
pub const ADMUX: *mut u8 = 0x27 as *mut u8;

/// Analog Comparator Control And Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ACI | 10000 |
/// | ACD | 10000000 |
/// | ACO | 100000 |
/// | ACIE | 1000 |
/// | ACME | 100 |
/// | ACIS | 11 |
/// | ACBG | 1000000 |
pub const ACSR: *mut u8 = 0x28 as *mut u8;

/// USI Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | USISIE | 10000000 |
/// | USIOIE | 1000000 |
/// | USICLK | 10 |
/// | USIWM | 110000 |
/// | USICS | 1100 |
/// | USITC | 1 |
pub const USICR: *mut u8 = 0x2D as *mut u8;

/// USI Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | USIOIF | 1000000 |
/// | USISIF | 10000000 |
/// | USICNT | 1111 |
/// | USIPF | 100000 |
/// | USIDC | 10000 |
pub const USISR: *mut u8 = 0x2E as *mut u8;

/// USI Data Register.
pub const USIDR: *mut u8 = 0x2F as *mut u8;

/// Port B Input Pins.
pub const PINB: *mut u8 = 0x36 as *mut u8;

/// Port B Data Direction Register.
pub const DDRB: *mut u8 = 0x37 as *mut u8;

/// Port B Data Register.
pub const PORTB: *mut u8 = 0x38 as *mut u8;

/// Port A Input Pins.
pub const PINA: *mut u8 = 0x39 as *mut u8;

/// Port A Data Direction Register.
pub const DDRA: *mut u8 = 0x3A as *mut u8;

/// Port A Data Register.
pub const PORTA: *mut u8 = 0x3B as *mut u8;

/// EEPROM Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EERIE | 1000 |
/// | EERE | 1 |
/// | EEMWE | 100 |
/// | EEWE | 10 |
pub const EECR: *mut u8 = 0x3C as *mut u8;

/// EEPROM Data Register.
pub const EEDR: *mut u8 = 0x3D as *mut u8;

/// EEPROM Read/Write Access.
pub const EEAR: *mut u8 = 0x3E as *mut u8;

/// Watchdog Timer Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDE | 1000 |
/// | WDCE | 10000 |
/// | WDP | 111 |
pub const WDTCR: *mut u8 = 0x41 as *mut u8;

/// PLL Control and Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PLOCK | 1 |
/// | PLLE | 10 |
/// | PCKE | 100 |
pub const PLLCSR: *mut u8 = 0x49 as *mut u8;

/// Output Compare Register.
pub const OCR1C: *mut u8 = 0x4B as *mut u8;

/// Output Compare Register.
pub const OCR1B: *mut u8 = 0x4C as *mut u8;

/// Output Compare Register.
pub const OCR1A: *mut u8 = 0x4D as *mut u8;

/// Timer/Counter Register.
pub const TCNT1: *mut u8 = 0x4E as *mut u8;

/// Timer/Counter Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CTC1 | 10000000 |
/// | CS1 | 1111 |
/// | PSR1 | 1000000 |
pub const TCCR1B: *mut u8 = 0x4F as *mut u8;

/// Timer/Counter Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | COM1B | 110000 |
/// | COM1A | 11000000 |
/// | PWM1B | 1 |
/// | PWM1A | 10 |
/// | FOC1B | 100 |
/// | FOC1A | 1000 |
pub const TCCR1A: *mut u8 = 0x50 as *mut u8;

/// Status Register.
pub const OSCCAL: *mut u8 = 0x51 as *mut u8;

/// Timer Counter 0.
pub const TCNT0: *mut u8 = 0x52 as *mut u8;

/// Timer/Counter0 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CS0 | 111 |
/// | PSR0 | 1000 |
pub const TCCR0: *mut u8 = 0x53 as *mut u8;

/// MCU Status register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EXTRF | 10 |
/// | WDRF | 1000 |
/// | PORF | 1 |
/// | BORF | 100 |
pub const MCUSR: *mut u8 = 0x54 as *mut u8;

/// MCU Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SM | 11000 |
/// | PUD | 1000000 |
/// | ISC0 | 11 |
/// | SE | 100000 |
pub const MCUCR: *mut u8 = 0x55 as *mut u8;

/// Timer/Counter Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCF1A | 1000000 |
/// | OCF1B | 100000 |
/// | TOV1 | 100 |
pub const TIFR: *mut u8 = 0x58 as *mut u8;

/// Timer/Counter Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCIE1A | 1000000 |
/// | OCIE1B | 100000 |
/// | TOIE1 | 100 |
pub const TIMSK: *mut u8 = 0x59 as *mut u8;

/// General Interrupt Flag register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCIF | 100000 |
/// | INTF0 | 1000000 |
pub const GIFR: *mut u8 = 0x5A as *mut u8;

/// General Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCIE | 110000 |
/// | INT0 | 1000000 |
pub const GIMSK: *mut u8 = 0x5B as *mut u8;

/// Stack Pointer.
pub const SP: *mut u8 = 0x5D as *mut u8;

/// Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | I | 10000000 |
/// | H | 100000 |
/// | N | 100 |
/// | S | 10000 |
/// | T | 1000000 |
/// | Z | 10 |
/// | V | 1000 |
/// | C | 1 |
pub const SREG: *mut u8 = 0x5F as *mut u8;

/// Bitfield on register `ACSR`
pub const ACI: u8 = 0x10;

/// Bitfield on register `ACSR`
pub const ACD: u8 = 0x80;

/// Bitfield on register `ACSR`
pub const ACO: u8 = 0x20;

/// Bitfield on register `ACSR`
pub const ACIE: u8 = 0x8;

/// Bitfield on register `ACSR`
pub const ACME: u8 = 0x4;

/// Bitfield on register `ACSR`
pub const ACIS: u8 = 0x3;

/// Bitfield on register `ACSR`
pub const ACBG: u8 = 0x40;

/// Bitfield on register `ADCSR`
pub const ADFR: u8 = 0x20;

/// Bitfield on register `ADCSR`
pub const ADSC: u8 = 0x40;

/// Bitfield on register `ADCSR`
pub const ADIE: u8 = 0x8;

/// Bitfield on register `ADCSR`
pub const ADPS: u8 = 0x7;

/// Bitfield on register `ADCSR`
pub const ADEN: u8 = 0x80;

/// Bitfield on register `ADCSR`
pub const ADIF: u8 = 0x10;

/// Bitfield on register `ADMUX`
pub const ADLAR: u8 = 0x20;

/// Bitfield on register `ADMUX`
pub const REFS: u8 = 0xC0;

/// Bitfield on register `ADMUX`
pub const MUX: u8 = 0x1F;

/// Bitfield on register `EECR`
pub const EERIE: u8 = 0x8;

/// Bitfield on register `EECR`
pub const EERE: u8 = 0x1;

/// Bitfield on register `EECR`
pub const EEMWE: u8 = 0x4;

/// Bitfield on register `EECR`
pub const EEWE: u8 = 0x2;

/// Bitfield on register `GIFR`
pub const PCIF: u8 = 0x20;

/// Bitfield on register `GIFR`
pub const INTF0: u8 = 0x40;

/// Bitfield on register `GIMSK`
pub const PCIE: u8 = 0x30;

/// Bitfield on register `GIMSK`
pub const INT0: u8 = 0x40;

/// Bitfield on register `HIGH`
pub const RSTDISBL: u8 = 0x10;

/// Bitfield on register `HIGH`
pub const BODLEVEL: u8 = 0x2;

/// Bitfield on register `HIGH`
pub const SPIEN: u8 = 0x8;

/// Bitfield on register `HIGH`
pub const EESAVE: u8 = 0x4;

/// Bitfield on register `HIGH`
pub const BODEN: u8 = 0x1;

/// Bitfield on register `LOCKBIT`
pub const LB: u8 = 0x3;

/// Bitfield on register `LOW`
pub const CKOPT: u8 = 0x40;

/// Bitfield on register `LOW`
pub const PLLCK_SUT_CKSEL: u8 = 0xBF;

/// Bitfield on register `MCUCR`
pub const SM: u8 = 0x18;

/// Bitfield on register `MCUCR`
pub const PUD: u8 = 0x40;

/// Bitfield on register `MCUCR`
pub const ISC0: u8 = 0x3;

/// Bitfield on register `MCUCR`
pub const SE: u8 = 0x20;

/// Bitfield on register `MCUSR`
pub const EXTRF: u8 = 0x2;

/// Bitfield on register `MCUSR`
pub const WDRF: u8 = 0x8;

/// Bitfield on register `MCUSR`
pub const PORF: u8 = 0x1;

/// Bitfield on register `MCUSR`
pub const BORF: u8 = 0x4;

/// Bitfield on register `PLLCSR`
pub const PLOCK: u8 = 0x1;

/// Bitfield on register `PLLCSR`
pub const PLLE: u8 = 0x2;

/// Bitfield on register `PLLCSR`
pub const PCKE: u8 = 0x4;

/// Bitfield on register `SREG`
pub const I: u8 = 0x80;

/// Bitfield on register `SREG`
pub const H: u8 = 0x20;

/// Bitfield on register `SREG`
pub const N: u8 = 0x4;

/// Bitfield on register `SREG`
pub const S: u8 = 0x10;

/// Bitfield on register `SREG`
pub const T: u8 = 0x40;

/// Bitfield on register `SREG`
pub const Z: u8 = 0x2;

/// Bitfield on register `SREG`
pub const V: u8 = 0x8;

/// Bitfield on register `SREG`
pub const C: u8 = 0x1;

/// Bitfield on register `TCCR0`
pub const CS0: u8 = 0x7;

/// Bitfield on register `TCCR0`
pub const PSR0: u8 = 0x8;

/// Bitfield on register `TCCR1A`
pub const COM1B: u8 = 0x30;

/// Bitfield on register `TCCR1A`
pub const COM1A: u8 = 0xC0;

/// Bitfield on register `TCCR1A`
pub const PWM1B: u8 = 0x1;

/// Bitfield on register `TCCR1A`
pub const PWM1A: u8 = 0x2;

/// Bitfield on register `TCCR1A`
pub const FOC1B: u8 = 0x4;

/// Bitfield on register `TCCR1A`
pub const FOC1A: u8 = 0x8;

/// Bitfield on register `TCCR1B`
pub const CTC1: u8 = 0x80;

/// Bitfield on register `TCCR1B`
pub const CS1: u8 = 0xF;

/// Bitfield on register `TCCR1B`
pub const PSR1: u8 = 0x40;

/// Bitfield on register `TIFR`
pub const OCF1A: u8 = 0x40;

/// Bitfield on register `TIFR`
pub const OCF1B: u8 = 0x20;

/// Bitfield on register `TIFR`
pub const TOV1: u8 = 0x4;

/// Bitfield on register `TIMSK`
pub const OCIE1A: u8 = 0x40;

/// Bitfield on register `TIMSK`
pub const OCIE1B: u8 = 0x20;

/// Bitfield on register `TIMSK`
pub const TOIE1: u8 = 0x4;

/// Bitfield on register `USICR`
pub const USISIE: u8 = 0x80;

/// Bitfield on register `USICR`
pub const USIOIE: u8 = 0x40;

/// Bitfield on register `USICR`
pub const USICLK: u8 = 0x2;

/// Bitfield on register `USICR`
pub const USIWM: u8 = 0x30;

/// Bitfield on register `USICR`
pub const USICS: u8 = 0xC;

/// Bitfield on register `USICR`
pub const USITC: u8 = 0x1;

/// Bitfield on register `USISR`
pub const USIOIF: u8 = 0x40;

/// Bitfield on register `USISR`
pub const USISIF: u8 = 0x80;

/// Bitfield on register `USISR`
pub const USICNT: u8 = 0xF;

/// Bitfield on register `USISR`
pub const USIPF: u8 = 0x20;

/// Bitfield on register `USISR`
pub const USIDC: u8 = 0x10;

/// Bitfield on register `WDTCR`
pub const WDE: u8 = 0x8;

/// Bitfield on register `WDTCR`
pub const WDCE: u8 = 0x10;

/// Bitfield on register `WDTCR`
pub const WDP: u8 = 0x7;

/// `ANALOG_ADC_PRESCALER` value group
#[allow(non_upper_case_globals)]
pub mod analog_adc_prescaler {
   /// 2.
   pub const VAL_0x00: u32 = 0x0;
   /// 2.
   pub const VAL_0x01: u32 = 0x1;
   /// 4.
   pub const VAL_0x02: u32 = 0x2;
   /// 8.
   pub const VAL_0x03: u32 = 0x3;
   /// 16.
   pub const VAL_0x04: u32 = 0x4;
   /// 32.
   pub const VAL_0x05: u32 = 0x5;
   /// 64.
   pub const VAL_0x06: u32 = 0x6;
   /// 128.
   pub const VAL_0x07: u32 = 0x7;
}

/// `ANALOG_ADC_V_REF` value group
#[allow(non_upper_case_globals)]
pub mod analog_adc_v_ref {
   /// AVCC.
   pub const VAL_0x00: u32 = 0x0;
   /// AREF.
   pub const VAL_0x01: u32 = 0x1;
   /// Internal Voltage Referance.
   pub const VAL_0x02: u32 = 0x2;
   /// Internal Voltage Reference With External Capacitor at AREF Pin.
   pub const VAL_0x03: u32 = 0x3;
}

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

/// `CLK_COMP_MATCH_OUT_MODE` value group
#[allow(non_upper_case_globals)]
pub mod clk_comp_match_out_mode {
   /// Disconnected from OCn/PWMn.
   pub const VAL_0x00: u32 = 0x0;
   /// Toggle OCn/PWMn output line.
   pub const VAL_0x01: u32 = 0x1;
   /// Clear OCn/PWMn output line.
   pub const VAL_0x02: u32 = 0x2;
   /// set OCn/PWMn output line.
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

/// `CLK_SEL_4BIT` value group
#[allow(non_upper_case_globals)]
pub mod clk_sel_4bit {
   /// No Clock Source (Stopped).
   pub const VAL_0x00: u32 = 0x0;
   /// Running, No Prescaling.
   pub const VAL_0x01: u32 = 0x1;
   /// Running, CLK/2.
   pub const VAL_0x02: u32 = 0x2;
   /// Running, CLK/4.
   pub const VAL_0x03: u32 = 0x3;
   /// Running, CLK/8.
   pub const VAL_0x04: u32 = 0x4;
   /// Running, CLK/16.
   pub const VAL_0x05: u32 = 0x5;
   /// Running, CLK/32.
   pub const VAL_0x06: u32 = 0x6;
   /// Running, CLK/64.
   pub const VAL_0x07: u32 = 0x7;
   /// Running, CLK/128.
   pub const VAL_0x08: u32 = 0x8;
   /// Running, CLK/256.
   pub const VAL_0x09: u32 = 0x9;
   /// Running, CLK/512.
   pub const VAL_0x0A: u32 = 0xA;
   /// Running, CLK/1024.
   pub const VAL_0x0B: u32 = 0xB;
   /// Running, CLK/2048.
   pub const VAL_0x0C: u32 = 0xC;
   /// Running, CLK/4096.
   pub const VAL_0x0D: u32 = 0xD;
   /// Running, CLK/8192.
   pub const VAL_0x0E: u32 = 0xE;
   /// Running, CLK/16384.
   pub const VAL_0x0F: u32 = 0xF;
}

/// `COMM_USI_OP` value group
#[allow(non_upper_case_globals)]
pub mod comm_usi_op {
   /// Normal Operation.
   pub const VAL_0x00: u32 = 0x0;
   /// Three-Wire Mode.
   pub const VAL_0x01: u32 = 0x1;
   /// Two-Wire Mode.
   pub const VAL_0x02: u32 = 0x2;
   /// Two-Wire Mode Held Low.
   pub const VAL_0x03: u32 = 0x3;
}

/// `CPU_SLEEP_MODE` value group
#[allow(non_upper_case_globals)]
pub mod cpu_sleep_mode {
   /// Idle.
   pub const IDLE: u32 = 0x0;
   /// ADC Noise Reduction (If Available).
   pub const ADC: u32 = 0x1;
   /// Power Down.
   pub const PDOWN: u32 = 0x2;
   /// Standby.
   pub const STDBY: u32 = 0x3;
}

/// `ENUM_BODLEVEL` value group
#[allow(non_upper_case_globals)]
pub mod enum_bodlevel {
   /// Brown-out detection at VCC=4.0 V.
   pub const _4V0: u32 = 0x0;
   /// Brown-out detection at VCC=2.7 V.
   pub const _2V7: u32 = 0x1;
}

/// `ENUM_LB` value group
#[allow(non_upper_case_globals)]
pub mod enum_lb {
   /// Further programming and verification disabled.
   pub const PROG_VER_DISABLED: u32 = 0x0;
   /// Further programming disabled.
   pub const PROG_DISABLED: u32 = 0x2;
   /// No memory lock features enabled.
   pub const NO_LOCK: u32 = 0x3;
}

/// `ENUM_PLLCK_SUT_CKSEL` value group
#[allow(non_upper_case_globals)]
pub mod enum_pllck_sut_cksel {
   /// PLL Clock; Start-up time: 1K CK + 0 ms.
   pub const PLLCLK_1KCK_0MS: u32 = 0x1;
   /// PLL Clock; Start-up time: 1K CK + 4 ms.
   pub const PLLCLK_1KCK_4MS: u32 = 0x11;
   /// PLL Clock; Start-up time: 1K CK + 64 ms.
   pub const PLLCLK_1KCK_64MS: u32 = 0x21;
   /// PLL Clock; Start-up time: 16K CK + 64 ms.
   pub const PLLCLK_16KCK_64MS: u32 = 0x31;
   /// Ext. Clock; Start-up time: 6 CK + 0 ms.
   pub const EXTCLK_6CK_0MS: u32 = 0x80;
   /// Ext. Clock; Start-up time: 6 CK + 4 ms.
   pub const EXTCLK_6CK_4MS: u32 = 0x90;
   /// Ext. Clock; Start-up time: 6 CK + 64 ms.
   pub const EXTCLK_6CK_64MS: u32 = 0xA0;
   /// Int. RC Osc. 1 MHz; Start-up time: 6 CK + 0 ms.
   pub const INTRCOSC_1MHZ_6CK_0MS: u32 = 0x81;
   /// Int. RC Osc. 1 MHz; Start-up time: 6 CK + 4 ms.
   pub const INTRCOSC_1MHZ_6CK_4MS: u32 = 0x91;
   /// Int. RC Osc. 1 MHz; Start-up time: 6 CK + 64 ms.
   pub const INTRCOSC_1MHZ_6CK_64MS: u32 = 0xA1;
   /// Int. RC Osc. 2 MHz; Start-up time: 6 CK + 0 ms.
   pub const INTRCOSC_2MHZ_6CK_0MS: u32 = 0x82;
   /// Int. RC Osc. 2 MHz; Start-up time: 6 CK + 4 ms.
   pub const INTRCOSC_2MHZ_6CK_4MS: u32 = 0x92;
   /// Int. RC Osc. 2 MHz; Start-up time: 6 CK + 64 ms.
   pub const INTRCOSC_2MHZ_6CK_64MS: u32 = 0xA2;
   /// Int. RC Osc. 4 MHz; Start-up time: 6 CK + 0 ms.
   pub const INTRCOSC_4MHZ_6CK_0MS: u32 = 0x83;
   /// Int. RC Osc. 4 MHz; Start-up time: 6 CK + 4 ms.
   pub const INTRCOSC_4MHZ_6CK_4MS: u32 = 0x93;
   /// Int. RC Osc. 4 MHz; Start-up time: 6 CK + 64 ms.
   pub const INTRCOSC_4MHZ_6CK_64MS: u32 = 0xA3;
   /// Int. RC Osc. 8 MHz; Start-up time: 6 CK + 0 ms.
   pub const INTRCOSC_8MHZ_6CK_0MS: u32 = 0x84;
   /// Int. RC Osc. 8 MHz; Start-up time: 6 CK + 4 ms.
   pub const INTRCOSC_8MHZ_6CK_4MS: u32 = 0x94;
   /// Int. RC Osc. 8 MHz; Start-up time: 6 CK + 64 ms.
   pub const INTRCOSC_8MHZ_6CK_64MS: u32 = 0xA4;
   /// Ext. RC Osc.         -  0.9 MHz; Start-up time: 18 CK + 0 ms.
   pub const EXTRCOSC_XX_0MHZ9_18CK_0MS: u32 = 0x85;
   /// Ext. RC Osc.         -  0.9 MHz; Start-up time: 18 CK + 4 ms.
   pub const EXTRCOSC_XX_0MHZ9_18CK_4MS: u32 = 0x95;
   /// Ext. RC Osc.         -  0.9 MHz; Start-up time: 18 CK + 64 ms.
   pub const EXTRCOSC_XX_0MHZ9_18CK_64MS: u32 = 0xA5;
   /// Ext. RC Osc.         -  0.9 MHz; Start-up time: 6 CK + 4 ms.
   pub const EXTRCOSC_XX_0MHZ9_6CK_4MS: u32 = 0xB5;
   /// Ext. RC Osc. 0.9 MHz -  3.0 MHz; Start-up time: 18 CK + 0 ms.
   pub const EXTRCOSC_0MHZ9_3MHZ_18CK_0MS: u32 = 0x86;
   /// Ext. RC Osc. 0.9 MHz -  3.0 MHz; Start-up time: 18 CK + 4 ms.
   pub const EXTRCOSC_0MHZ9_3MHZ_18CK_4MS: u32 = 0x96;
   /// Ext. RC Osc. 0.9 MHz -  3.0 MHz; Start-up time: 18 CK + 64 ms.
   pub const EXTRCOSC_0MHZ9_3MHZ_18CK_64MS: u32 = 0xA6;
   /// Ext. RC Osc. 0.9 MHz -  3.0 MHz; Start-up time: 6 CK + 4 ms.
   pub const EXTRCOSC_0MHZ9_3MHZ_6CK_4MS: u32 = 0xB6;
   /// Ext. RC Osc. 3.0 MHz -  8.0 MHz; Start-up time: 18 CK + 0 ms.
   pub const EXTRCOSC_3MHZ_8MHZ_18CK_0MS: u32 = 0x87;
   /// Ext. RC Osc. 3.0 MHz -  8.0 MHz; Start-up time: 18 CK + 4 ms.
   pub const EXTRCOSC_3MHZ_8MHZ_18CK_4MS: u32 = 0x97;
   /// Ext. RC Osc. 3.0 MHz -  8.0 MHz; Start-up time: 18 CK + 64 ms.
   pub const EXTRCOSC_3MHZ_8MHZ_18CK_64MS: u32 = 0xA7;
   /// Ext. RC Osc. 3.0 MHz -  8.0 MHz; Start-up time: 6 CK + 4 ms.
   pub const EXTRCOSC_3MHZ_8MHZ_6CK_4MS: u32 = 0xB7;
   /// Ext. RC Osc. 8.0 MHz - 12.0 MHz; Start-up time: 18 CK + 0 ms.
   pub const EXTRCOSC_8MHZ_12MHZ_18CK_0MS: u32 = 0x88;
   /// Ext. RC Osc. 8.0 MHz - 12.0 MHz; Start-up time: 18 CK + 4 ms.
   pub const EXTRCOSC_8MHZ_12MHZ_18CK_4MS: u32 = 0x98;
   /// Ext. RC Osc. 8.0 MHz - 12.0 MHz; Start-up time: 18 CK + 64 ms.
   pub const EXTRCOSC_8MHZ_12MHZ_18CK_64MS: u32 = 0xA8;
   /// Ext. RC Osc. 8.0 MHz - 12.0 MHz; Start-up time: 6 CK + 4 ms.
   pub const EXTRCOSC_8MHZ_12MHZ_6CK_4MS: u32 = 0xB8;
   /// Ext. Low-Freq. Crystal; Start-up time: 1K CK + 4 ms.
   pub const EXTLOFXTAL_1KCK_4MS: u32 = 0x89;
   /// Ext. Low-Freq. Crystal; Start-up time: 1K CK + 64 ms.
   pub const EXTLOFXTAL_1KCK_64MS: u32 = 0x99;
   /// Ext. Low-Freq. Crystal; Start-up time: 32K CK + 64 ms.
   pub const EXTLOFXTAL_32KCK_64MS: u32 = 0xA9;
   /// Ext. Crystal/Resonator Low Freq.; Start-up time: 258 CK + 4 ms.
   pub const EXTLOFXTALRES_258CK_4MS: u32 = 0x8A;
   /// Ext. Crystal/Resonator Low Freq.; Start-up time: 258 CK + 64 ms.
   pub const EXTLOFXTALRES_258CK_64MS: u32 = 0x9A;
   /// Ext. Crystal/Resonator Low Freq.; Start-up time: 1K CK + 0 ms.
   pub const EXTLOFXTALRES_1KCK_0MS: u32 = 0xAA;
   /// Ext. Crystal/Resonator Low Freq.; Start-up time: 1K CK + 4 ms.
   pub const EXTLOFXTALRES_1KCK_4MS: u32 = 0xBA;
   /// Ext. Crystal/Resonator Low Freq.; Start-up time: 1K CK + 64 ms.
   pub const EXTLOFXTALRES_1KCK_64MS: u32 = 0x8B;
   /// Ext. Crystal/Resonator Low Freq.; Start-up time: 16K CK + 0 ms.
   pub const EXTLOFXTALRES_16KCK_0MS: u32 = 0x9B;
   /// Ext. Crystal/Resonator Low Freq.; Start-up time: 16K CK + 4 ms.
   pub const EXTLOFXTALRES_16KCK_4MS: u32 = 0xAB;
   /// Ext. Crystal/Resonator Low Freq.; Start-up time: 16K CK + 64 ms.
   pub const EXTLOFXTALRES_16KCK_64MS: u32 = 0xBB;
   /// Ext. Crystal/Resonator Medium Freq.; Start-up time: 258 CK + 4 ms.
   pub const EXTMEDFXTALRES_258CK_4MS: u32 = 0x8C;
   /// Ext. Crystal/Resonator Medium Freq.; Start-up time: 258 CK + 64 ms.
   pub const EXTMEDFXTALRES_258CK_64MS: u32 = 0x9C;
   /// Ext. Crystal/Resonator Medium Freq.; Start-up time: 1K CK + 0 ms.
   pub const EXTMEDFXTALRES_1KCK_0MS: u32 = 0xAC;
   /// Ext. Crystal/Resonator Medium Freq.; Start-up time: 1K CK + 4 ms.
   pub const EXTMEDFXTALRES_1KCK_4MS: u32 = 0xBC;
   /// Ext. Crystal/Resonator Medium Freq.; Start-up time: 1K CK + 64 ms.
   pub const EXTMEDFXTALRES_1KCK_64MS: u32 = 0x8D;
   /// Ext. Crystal/Resonator Medium Freq.; Start-up time: 16K CK + 0 ms.
   pub const EXTMEDFXTALRES_16KCK_0MS: u32 = 0x9D;
   /// Ext. Crystal/Resonator Medium Freq.; Start-up time: 16K CK + 4 ms.
   pub const EXTMEDFXTALRES_16KCK_4MS: u32 = 0xAD;
   /// Ext. Crystal/Resonator Medium Freq.; Start-up time: 16K CK + 64 ms.
   pub const EXTMEDFXTALRES_16KCK_64MS: u32 = 0xBD;
   /// Ext. Crystal/Resonator High Freq.; Start-up time: 258 CK + 4 ms.
   pub const EXTHIFXTALRES_258CK_4MS: u32 = 0x8E;
   /// Ext. Crystal/Resonator High Freq.; Start-up time: 258 CK + 64 ms.
   pub const EXTHIFXTALRES_258CK_64MS: u32 = 0x9E;
   /// Ext. Crystal/Resonator High Freq.; Start-up time: 1K CK + 0 ms.
   pub const EXTHIFXTALRES_1KCK_0MS: u32 = 0xAE;
   /// Ext. Crystal/Resonator High Freq.; Start-up time: 1K CK + 4 ms.
   pub const EXTHIFXTALRES_1KCK_4MS: u32 = 0xBE;
   /// Ext. Crystal/Resonator High Freq.; Start-up time: 1K CK + 64 ms.
   pub const EXTHIFXTALRES_1KCK_64MS: u32 = 0x8F;
   /// Ext. Crystal/Resonator High Freq.; Start-up time: 16K CK + 0 ms.
   pub const EXTHIFXTALRES_16KCK_0MS: u32 = 0x9F;
   /// Ext. Crystal/Resonator High Freq.; Start-up time: 16K CK + 4 ms.
   pub const EXTHIFXTALRES_16KCK_4MS: u32 = 0xAF;
   /// Ext. Crystal/Resonator High Freq.; Start-up time: 16K CK + 64 ms.
   pub const EXTHIFXTALRES_16KCK_64MS: u32 = 0xBF;
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

/// Oscillator Calibration Values
#[allow(non_upper_case_globals)]
pub mod osccal_value_addresses {
   /// 1.0 MHz.
   pub const _1_0_MHz: u32 = 0x0;
   /// 2.0 MHz.
   pub const _2_0_MHz: u32 = 0x1;
   /// 4.0 MHz.
   pub const _4_0_MHz: u32 = 0x2;
   /// 8.0 MHz.
   pub const _8_0_MHz: u32 = 0x3;
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

