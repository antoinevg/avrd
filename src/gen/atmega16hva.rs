//! The AVR ATmega16HVA microcontroller
//!
//! # Variants
//! |        | Pinout | Mcu age | Operating temperature | Operating voltage | Max speed |
//! |--------|--------|---------|-----------------------|-------------------|-----------|
//! | standard |  |  | 0°C - 0°C | 1.8V - 4.5V | 0 MHz |
//!

#![allow(non_upper_case_globals)]

/// `LOCKBIT` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LB | 11 |
pub const LOCKBIT: *mut u8 = 0x0 as *mut u8;

/// `LOW` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SUT | 111 |
/// | WDTON | 10000000 |
/// | SPIEN | 100000 |
/// | SELFPRGEN | 1000 |
/// | EESAVE | 1000000 |
/// | DWEN | 10000 |
pub const LOW: *mut u8 = 0x0 as *mut u8;

/// Port A Input Pins.
pub const PINA: *mut u8 = 0x20 as *mut u8;

/// Port A Data Direction Register.
pub const DDRA: *mut u8 = 0x21 as *mut u8;

/// Port A Data Register.
pub const PORTA: *mut u8 = 0x22 as *mut u8;

/// Input Pins, Port B.
pub const PINB: *mut u8 = 0x23 as *mut u8;

/// Data Direction Register, Port B.
pub const DDRB: *mut u8 = 0x24 as *mut u8;

/// Data Register, Port B.
pub const PORTB: *mut u8 = 0x25 as *mut u8;

/// Port C Input Pins.
pub const PINC: *mut u8 = 0x26 as *mut u8;

/// Port C Data Register.
pub const PORTC: *mut u8 = 0x28 as *mut u8;

/// Timer/Counter Interrupt Flag register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCF0A | 10 |
/// | ICF0 | 1000 |
/// | OCF0B | 100 |
/// | TOV0 | 1 |
pub const TIFR0: *mut u8 = 0x35 as *mut u8;

/// Timer/Counter Interrupt Flag register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ICF1 | 1000 |
/// | OCF1A | 10 |
/// | OCF1B | 100 |
/// | TOV1 | 1 |
pub const TIFR1: *mut u8 = 0x36 as *mut u8;

/// Oscillator Sampling Interface Control and Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OSIEN | 1 |
/// | OSISEL0 | 10000 |
/// | OSIST | 10 |
pub const OSICSR: *mut u8 = 0x37 as *mut u8;

/// External Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | INTF | 111 |
pub const EIFR: *mut u8 = 0x3C as *mut u8;

/// External Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | INT | 111 |
pub const EIMSK: *mut u8 = 0x3D as *mut u8;

/// General Purpose IO Register 0.
pub const GPIOR0: *mut u8 = 0x3E as *mut u8;

/// EEPROM Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EERE | 1 |
/// | EEPM | 110000 |
/// | EEMPE | 100 |
/// | EERIE | 1000 |
/// | EEPE | 10 |
pub const EECR: *mut u8 = 0x3F as *mut u8;

/// EEPROM Data Register.
pub const EEDR: *mut u8 = 0x40 as *mut u8;

/// EEPROM Read/Write Access.
pub const EEAR: *mut u8 = 0x41 as *mut u8;

/// General Timer/Counter Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PSRSYNC | 1 |
/// | TSM | 10000000 |
pub const GTCCR: *mut u8 = 0x43 as *mut u8;

/// Timer/Counter0 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WGM00 | 1 |
/// | TCW0 | 10000000 |
/// | ICEN0 | 1000000 |
/// | ICNC0 | 100000 |
/// | ICS0 | 1000 |
/// | ICES0 | 10000 |
pub const TCCR0A: *mut u8 = 0x44 as *mut u8;

/// Timer/Counter0 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CS00 | 1 |
/// | CS01 | 10 |
/// | CS02 | 100 |
pub const TCCR0B: *mut u8 = 0x45 as *mut u8;

/// Timer Counter 0  Bytes.
pub const TCNT0: *mut u16 = 0x46 as *mut u16;

/// Timer Counter 0  Bytes low byte.
pub const TCNT0L: *mut u8 = 0x46 as *mut u8;

/// Timer Counter 0  Bytes high byte.
pub const TCNT0H: *mut u8 = 0x47 as *mut u8;

/// Output compare Register A.
pub const OCR0A: *mut u8 = 0x48 as *mut u8;

/// Output compare Register B.
pub const OCR0B: *mut u8 = 0x49 as *mut u8;

/// General Purpose IO Register 1.
pub const GPIOR1: *mut u8 = 0x4A as *mut u8;

/// General Purpose IO Register 2.
pub const GPIOR2: *mut u8 = 0x4B as *mut u8;

/// SPI Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CPHA | 100 |
/// | CPOL | 1000 |
/// | SPIE | 10000000 |
/// | MSTR | 10000 |
/// | SPR | 11 |
/// | SPE | 1000000 |
/// | DORD | 100000 |
pub const SPCR: *mut u8 = 0x4C as *mut u8;

/// SPI Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SPIF | 10000000 |
/// | WCOL | 1000000 |
/// | SPI2X | 1 |
pub const SPSR: *mut u8 = 0x4D as *mut u8;

/// SPI Data Register.
pub const SPDR: *mut u8 = 0x4E as *mut u8;

/// Sleep Mode Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SE | 1 |
/// | SM | 1110 |
pub const SMCR: *mut u8 = 0x53 as *mut u8;

/// MCU Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PORF | 1 |
/// | WDRF | 1000 |
/// | BODRF | 100 |
/// | OCDRF | 10000 |
/// | EXTRF | 10 |
pub const MCUSR: *mut u8 = 0x54 as *mut u8;

/// MCU Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CKOE | 100000 |
/// | PUD | 10000 |
pub const MCUCR: *mut u8 = 0x55 as *mut u8;

/// Store Program Memory Control and Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PGWRT | 100 |
/// | SPMEN | 1 |
/// | CTPB | 10000 |
/// | PGERS | 10 |
/// | SIGRD | 100000 |
/// | RFLB | 1000 |
pub const SPMCSR: *mut u8 = 0x57 as *mut u8;

/// Stack Pointer  low byte.
pub const SPL: *mut u8 = 0x5D as *mut u8;

/// Stack Pointer.
pub const SP: *mut u16 = 0x5D as *mut u16;

/// Stack Pointer  high byte.
pub const SPH: *mut u8 = 0x5E as *mut u8;

/// Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T | 1000000 |
/// | H | 100000 |
/// | V | 1000 |
/// | S | 10000 |
/// | N | 100 |
/// | C | 1 |
/// | Z | 10 |
/// | I | 10000000 |
pub const SREG: *mut u8 = 0x5F as *mut u8;

/// Watchdog Timer Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDP | 100111 |
/// | WDIE | 1000000 |
/// | WDE | 1000 |
/// | WDCE | 10000 |
/// | WDIF | 10000000 |
pub const WDTCSR: *mut u8 = 0x60 as *mut u8;

/// Clock Prescale Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CLKPCE | 10000000 |
/// | CLKPS | 11 |
pub const CLKPR: *mut u8 = 0x61 as *mut u8;

/// Power Reduction Register 0.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRTIM1 | 100 |
/// | PRVRM | 100000 |
/// | PRTIM0 | 10 |
/// | PRSPI | 1000 |
/// | PRVADC | 1 |
pub const PRR0: *mut u8 = 0x64 as *mut u8;

/// Fast Oscillator Calibration Value.
pub const FOSCCAL: *mut u8 = 0x66 as *mut u8;

/// External Interrupt Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ISC0 | 11 |
/// | ISC2 | 110000 |
/// | ISC1 | 1100 |
pub const EICRA: *mut u8 = 0x69 as *mut u8;

/// Timer/Counter Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCIE0A | 10 |
/// | OCIE0B | 100 |
/// | ICIE0 | 1000 |
/// | TOIE0 | 1 |
pub const TIMSK0: *mut u8 = 0x6E as *mut u8;

/// Timer/Counter Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TOIE1 | 1 |
/// | OCIE1B | 100 |
/// | OCIE1A | 10 |
/// | ICIE1 | 1000 |
pub const TIMSK1: *mut u8 = 0x6F as *mut u8;

/// VADC Data Register  Bytes.
pub const VADC: *mut u16 = 0x78 as *mut u16;

/// VADC Data Register  Bytes low byte.
pub const VADCL: *mut u8 = 0x78 as *mut u8;

/// VADC Data Register  Bytes high byte.
pub const VADCH: *mut u8 = 0x79 as *mut u8;

/// The VADC Control and Status register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | VADCCIF | 10 |
/// | VADCCIE | 1 |
/// | VADEN | 1000 |
/// | VADSC | 100 |
pub const VADCSR: *mut u8 = 0x7A as *mut u8;

/// The VADC multiplexer Selection Register.
pub const VADMUX: *mut u8 = 0x7C as *mut u8;

/// Digital Input Disable Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PA0DID | 1 |
/// | PA1DID | 10 |
pub const DIDR0: *mut u8 = 0x7E as *mut u8;

/// Timer/Counter 1 Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ICES1 | 10000 |
/// | ICEN1 | 1000000 |
/// | ICS1 | 1000 |
/// | WGM10 | 1 |
/// | TCW1 | 10000000 |
/// | ICNC1 | 100000 |
pub const TCCR1A: *mut u8 = 0x80 as *mut u8;

/// Timer/Counter1 Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CS | 111 |
pub const TCCR1B: *mut u8 = 0x81 as *mut u8;

/// Timer Counter 1  Bytes low byte.
pub const TCNT1L: *mut u8 = 0x84 as *mut u8;

/// Timer Counter 1  Bytes.
pub const TCNT1: *mut u16 = 0x84 as *mut u16;

/// Timer Counter 1  Bytes high byte.
pub const TCNT1H: *mut u8 = 0x85 as *mut u8;

/// Output Compare Register 1A.
pub const OCR1A: *mut u8 = 0x88 as *mut u8;

/// Output Compare Register B.
pub const OCR1B: *mut u8 = 0x89 as *mut u8;

/// Regulator Operating Condition Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ROCS | 10000000 |
/// | ROCWIE | 1 |
/// | ROCWIF | 10 |
pub const ROCR: *mut u8 = 0xC8 as *mut u8;

/// Bandgap Calibration Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BGD | 10000000 |
/// | BGCC | 111111 |
pub const BGCCR: *mut u8 = 0xD0 as *mut u8;

/// Bandgap Calibration of Resistor Ladder.
pub const BGCRR: *mut u8 = 0xD1 as *mut u8;

/// ADC Accumulate Current.
pub const CADAC0: *mut u8 = 0xE0 as *mut u8;

/// ADC Accumulate Current.
pub const CADAC1: *mut u8 = 0xE1 as *mut u8;

/// ADC Accumulate Current.
pub const CADAC2: *mut u8 = 0xE2 as *mut u8;

/// ADC Accumulate Current.
pub const CADAC3: *mut u8 = 0xE3 as *mut u8;

/// CC-ADC Control and Status Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CADEN | 10000000 |
/// | CADAS | 11000 |
/// | CADPOL | 1000000 |
/// | CADUB | 100000 |
/// | CADSI | 110 |
/// | CADSE | 1 |
pub const CADCSRA: *mut u8 = 0xE4 as *mut u8;

/// CC-ADC Control and Status Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CADRCIE | 100000 |
/// | CADICIE | 10000 |
/// | CADICIF | 1 |
/// | CADACIE | 1000000 |
/// | CADRCIF | 10 |
/// | CADACIF | 100 |
pub const CADCSRB: *mut u8 = 0xE5 as *mut u8;

/// CC-ADC Regular Current.
pub const CADRC: *mut u8 = 0xE6 as *mut u8;

/// CC-ADC Instantaneous Current low byte.
pub const CADICL: *mut u8 = 0xE8 as *mut u8;

/// CC-ADC Instantaneous Current.
pub const CADIC: *mut u16 = 0xE8 as *mut u16;

/// CC-ADC Instantaneous Current high byte.
pub const CADICH: *mut u8 = 0xE9 as *mut u8;

/// FET Control and Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DUVRD | 1000 |
/// | CPS | 100 |
/// | DFE | 10 |
/// | CFE | 1 |
pub const FCSR: *mut u8 = 0xF0 as *mut u8;

/// Battery Protection Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DHCIE | 10 |
/// | CHCIE | 1 |
/// | DOCIE | 1000 |
/// | SCIE | 10000 |
/// | COCIE | 100 |
pub const BPIMSK: *mut u8 = 0xF2 as *mut u8;

/// Battery Protection Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | COCIF | 100 |
/// | SCIF | 10000 |
/// | DOCIF | 1000 |
/// | CHCIF | 1 |
/// | DHCIF | 10 |
pub const BPIFR: *mut u8 = 0xF3 as *mut u8;

/// Battery Protection Short-Circuit Detection Level Register.
pub const BPSCD: *mut u8 = 0xF5 as *mut u8;

/// Battery Protection Discharge-Over-current Detection Level Register.
pub const BPDOCD: *mut u8 = 0xF6 as *mut u8;

/// Battery Protection Charge-Over-current Detection Level Register.
pub const BPCOCD: *mut u8 = 0xF7 as *mut u8;

/// Battery Protection Discharge-High-current Detection Level Register.
pub const BPDHCD: *mut u8 = 0xF8 as *mut u8;

/// Battery Protection Charge-High-current Detection Level Register.
pub const BPCHCD: *mut u8 = 0xF9 as *mut u8;

/// Battery Protection Short-current Timing Register.
pub const BPSCTR: *mut u8 = 0xFA as *mut u8;

/// Battery Protection Over-current Timing Register.
pub const BPOCTR: *mut u8 = 0xFB as *mut u8;

/// Battery Protection Short-current Timing Register.
pub const BPHCTR: *mut u8 = 0xFC as *mut u8;

/// Battery Protection Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SCD | 10000 |
/// | COCD | 100 |
/// | DHCD | 10 |
/// | CHCD | 1 |
/// | DOCD | 1000 |
pub const BPCR: *mut u8 = 0xFD as *mut u8;

/// Battery Protection Parameter Lock Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BPPLE | 10 |
/// | BPPL | 1 |
pub const BPPLR: *mut u8 = 0xFE as *mut u8;

/// Bitfield on register `BGCCR`
pub const BGD: u8 = 0x80;

/// Bitfield on register `BGCCR`
pub const BGCC: u8 = 0x3F;

/// Bitfield on register `BPCR`
pub const SCD: u8 = 0x10;

/// Bitfield on register `BPCR`
pub const COCD: u8 = 0x4;

/// Bitfield on register `BPCR`
pub const DHCD: u8 = 0x2;

/// Bitfield on register `BPCR`
pub const CHCD: u8 = 0x1;

/// Bitfield on register `BPCR`
pub const DOCD: u8 = 0x8;

/// Bitfield on register `BPIFR`
pub const COCIF: u8 = 0x4;

/// Bitfield on register `BPIFR`
pub const SCIF: u8 = 0x10;

/// Bitfield on register `BPIFR`
pub const DOCIF: u8 = 0x8;

/// Bitfield on register `BPIFR`
pub const CHCIF: u8 = 0x1;

/// Bitfield on register `BPIFR`
pub const DHCIF: u8 = 0x2;

/// Bitfield on register `BPIMSK`
pub const DHCIE: u8 = 0x2;

/// Bitfield on register `BPIMSK`
pub const CHCIE: u8 = 0x1;

/// Bitfield on register `BPIMSK`
pub const DOCIE: u8 = 0x8;

/// Bitfield on register `BPIMSK`
pub const SCIE: u8 = 0x10;

/// Bitfield on register `BPIMSK`
pub const COCIE: u8 = 0x4;

/// Bitfield on register `BPPLR`
pub const BPPLE: u8 = 0x2;

/// Bitfield on register `BPPLR`
pub const BPPL: u8 = 0x1;

/// Bitfield on register `CADCSRA`
pub const CADEN: u8 = 0x80;

/// Bitfield on register `CADCSRA`
pub const CADAS: u8 = 0x18;

/// Bitfield on register `CADCSRA`
pub const CADPOL: u8 = 0x40;

/// Bitfield on register `CADCSRA`
pub const CADUB: u8 = 0x20;

/// Bitfield on register `CADCSRA`
pub const CADSI: u8 = 0x6;

/// Bitfield on register `CADCSRA`
pub const CADSE: u8 = 0x1;

/// Bitfield on register `CADCSRB`
pub const CADRCIE: u8 = 0x20;

/// Bitfield on register `CADCSRB`
pub const CADICIE: u8 = 0x10;

/// Bitfield on register `CADCSRB`
pub const CADICIF: u8 = 0x1;

/// Bitfield on register `CADCSRB`
pub const CADACIE: u8 = 0x40;

/// Bitfield on register `CADCSRB`
pub const CADRCIF: u8 = 0x2;

/// Bitfield on register `CADCSRB`
pub const CADACIF: u8 = 0x4;

/// Bitfield on register `CLKPR`
pub const CLKPCE: u8 = 0x80;

/// Bitfield on register `CLKPR`
pub const CLKPS: u8 = 0x3;

/// Bitfield on register `DIDR0`
pub const PA0DID: u8 = 0x1;

/// Bitfield on register `DIDR0`
pub const PA1DID: u8 = 0x2;

/// Bitfield on register `EECR`
pub const EERE: u8 = 0x1;

/// Bitfield on register `EECR`
pub const EEPM: u8 = 0x30;

/// Bitfield on register `EECR`
pub const EEMPE: u8 = 0x4;

/// Bitfield on register `EECR`
pub const EERIE: u8 = 0x8;

/// Bitfield on register `EECR`
pub const EEPE: u8 = 0x2;

/// Bitfield on register `EICRA`
pub const ISC0: u8 = 0x3;

/// Bitfield on register `EICRA`
pub const ISC2: u8 = 0x30;

/// Bitfield on register `EICRA`
pub const ISC1: u8 = 0xC;

/// Bitfield on register `EIFR`
pub const INTF: u8 = 0x7;

/// Bitfield on register `EIMSK`
pub const INT: u8 = 0x7;

/// Bitfield on register `FCSR`
pub const DUVRD: u8 = 0x8;

/// Bitfield on register `FCSR`
pub const CPS: u8 = 0x4;

/// Bitfield on register `FCSR`
pub const DFE: u8 = 0x2;

/// Bitfield on register `FCSR`
pub const CFE: u8 = 0x1;

/// Bitfield on register `GTCCR`
pub const PSRSYNC: u8 = 0x1;

/// Bitfield on register `GTCCR`
pub const TSM: u8 = 0x80;

/// Bitfield on register `LOCKBIT`
pub const LB: u8 = 0x3;

/// Bitfield on register `LOW`
pub const SUT: u8 = 0x7;

/// Bitfield on register `LOW`
pub const WDTON: u8 = 0x80;

/// Bitfield on register `LOW`
pub const SPIEN: u8 = 0x20;

/// Bitfield on register `LOW`
pub const SELFPRGEN: u8 = 0x8;

/// Bitfield on register `LOW`
pub const EESAVE: u8 = 0x40;

/// Bitfield on register `LOW`
pub const DWEN: u8 = 0x10;

/// Bitfield on register `MCUCR`
pub const CKOE: u8 = 0x20;

/// Bitfield on register `MCUCR`
pub const PUD: u8 = 0x10;

/// Bitfield on register `MCUSR`
pub const PORF: u8 = 0x1;

/// Bitfield on register `MCUSR`
pub const WDRF: u8 = 0x8;

/// Bitfield on register `MCUSR`
pub const BODRF: u8 = 0x4;

/// Bitfield on register `MCUSR`
pub const OCDRF: u8 = 0x10;

/// Bitfield on register `MCUSR`
pub const EXTRF: u8 = 0x2;

/// Bitfield on register `OSICSR`
pub const OSIEN: u8 = 0x1;

/// Bitfield on register `OSICSR`
pub const OSISEL0: u8 = 0x10;

/// Bitfield on register `OSICSR`
pub const OSIST: u8 = 0x2;

/// Bitfield on register `PRR0`
pub const PRTIM1: u8 = 0x4;

/// Bitfield on register `PRR0`
pub const PRVRM: u8 = 0x20;

/// Bitfield on register `PRR0`
pub const PRTIM0: u8 = 0x2;

/// Bitfield on register `PRR0`
pub const PRSPI: u8 = 0x8;

/// Bitfield on register `PRR0`
pub const PRVADC: u8 = 0x1;

/// Bitfield on register `ROCR`
pub const ROCS: u8 = 0x80;

/// Bitfield on register `ROCR`
pub const ROCWIE: u8 = 0x1;

/// Bitfield on register `ROCR`
pub const ROCWIF: u8 = 0x2;

/// Bitfield on register `SMCR`
pub const SE: u8 = 0x1;

/// Bitfield on register `SMCR`
pub const SM: u8 = 0xE;

/// Bitfield on register `SPCR`
pub const CPHA: u8 = 0x4;

/// Bitfield on register `SPCR`
pub const CPOL: u8 = 0x8;

/// Bitfield on register `SPCR`
pub const SPIE: u8 = 0x80;

/// Bitfield on register `SPCR`
pub const MSTR: u8 = 0x10;

/// Bitfield on register `SPCR`
pub const SPR: u8 = 0x3;

/// Bitfield on register `SPCR`
pub const SPE: u8 = 0x40;

/// Bitfield on register `SPCR`
pub const DORD: u8 = 0x20;

/// Bitfield on register `SPMCSR`
pub const PGWRT: u8 = 0x4;

/// Bitfield on register `SPMCSR`
pub const SPMEN: u8 = 0x1;

/// Bitfield on register `SPMCSR`
pub const CTPB: u8 = 0x10;

/// Bitfield on register `SPMCSR`
pub const PGERS: u8 = 0x2;

/// Bitfield on register `SPMCSR`
pub const SIGRD: u8 = 0x20;

/// Bitfield on register `SPMCSR`
pub const RFLB: u8 = 0x8;

/// Bitfield on register `SPSR`
pub const SPIF: u8 = 0x80;

/// Bitfield on register `SPSR`
pub const WCOL: u8 = 0x40;

/// Bitfield on register `SPSR`
pub const SPI2X: u8 = 0x1;

/// Bitfield on register `SREG`
pub const T: u8 = 0x40;

/// Bitfield on register `SREG`
pub const H: u8 = 0x20;

/// Bitfield on register `SREG`
pub const V: u8 = 0x8;

/// Bitfield on register `SREG`
pub const S: u8 = 0x10;

/// Bitfield on register `SREG`
pub const N: u8 = 0x4;

/// Bitfield on register `SREG`
pub const C: u8 = 0x1;

/// Bitfield on register `SREG`
pub const Z: u8 = 0x2;

/// Bitfield on register `SREG`
pub const I: u8 = 0x80;

/// Bitfield on register `TCCR0A`
pub const WGM00: u8 = 0x1;

/// Bitfield on register `TCCR0A`
pub const TCW0: u8 = 0x80;

/// Bitfield on register `TCCR0A`
pub const ICEN0: u8 = 0x40;

/// Bitfield on register `TCCR0A`
pub const ICNC0: u8 = 0x20;

/// Bitfield on register `TCCR0A`
pub const ICS0: u8 = 0x8;

/// Bitfield on register `TCCR0A`
pub const ICES0: u8 = 0x10;

/// Bitfield on register `TCCR0B`
pub const CS00: u8 = 0x1;

/// Bitfield on register `TCCR0B`
pub const CS01: u8 = 0x2;

/// Bitfield on register `TCCR0B`
pub const CS02: u8 = 0x4;

/// Bitfield on register `TCCR1A`
pub const ICES1: u8 = 0x10;

/// Bitfield on register `TCCR1A`
pub const ICEN1: u8 = 0x40;

/// Bitfield on register `TCCR1A`
pub const ICS1: u8 = 0x8;

/// Bitfield on register `TCCR1A`
pub const WGM10: u8 = 0x1;

/// Bitfield on register `TCCR1A`
pub const TCW1: u8 = 0x80;

/// Bitfield on register `TCCR1A`
pub const ICNC1: u8 = 0x20;

/// Bitfield on register `TCCR1B`
pub const CS: u8 = 0x7;

/// Bitfield on register `TIFR0`
pub const OCF0A: u8 = 0x2;

/// Bitfield on register `TIFR0`
pub const ICF0: u8 = 0x8;

/// Bitfield on register `TIFR0`
pub const OCF0B: u8 = 0x4;

/// Bitfield on register `TIFR0`
pub const TOV0: u8 = 0x1;

/// Bitfield on register `TIFR1`
pub const ICF1: u8 = 0x8;

/// Bitfield on register `TIFR1`
pub const OCF1A: u8 = 0x2;

/// Bitfield on register `TIFR1`
pub const OCF1B: u8 = 0x4;

/// Bitfield on register `TIFR1`
pub const TOV1: u8 = 0x1;

/// Bitfield on register `TIMSK0`
pub const OCIE0A: u8 = 0x2;

/// Bitfield on register `TIMSK0`
pub const OCIE0B: u8 = 0x4;

/// Bitfield on register `TIMSK0`
pub const ICIE0: u8 = 0x8;

/// Bitfield on register `TIMSK0`
pub const TOIE0: u8 = 0x1;

/// Bitfield on register `TIMSK1`
pub const TOIE1: u8 = 0x1;

/// Bitfield on register `TIMSK1`
pub const OCIE1B: u8 = 0x4;

/// Bitfield on register `TIMSK1`
pub const OCIE1A: u8 = 0x2;

/// Bitfield on register `TIMSK1`
pub const ICIE1: u8 = 0x8;

/// Bitfield on register `VADCSR`
pub const VADCCIF: u8 = 0x2;

/// Bitfield on register `VADCSR`
pub const VADCCIE: u8 = 0x1;

/// Bitfield on register `VADCSR`
pub const VADEN: u8 = 0x8;

/// Bitfield on register `VADCSR`
pub const VADSC: u8 = 0x4;

/// Bitfield on register `WDTCSR`
pub const WDP: u8 = 0x27;

/// Bitfield on register `WDTCSR`
pub const WDIE: u8 = 0x40;

/// Bitfield on register `WDTCSR`
pub const WDE: u8 = 0x8;

/// Bitfield on register `WDTCSR`
pub const WDCE: u8 = 0x10;

/// Bitfield on register `WDTCSR`
pub const WDIF: u8 = 0x80;

/// `COMM_SCK_RATE_3BIT` value group
#[allow(non_upper_case_globals)]
pub mod comm_sck_rate_3bit {
   /// fosc/4.
   pub const VAL_0x00: u32 = 0x0;
   /// fosc/16.
   pub const VAL_0x01: u32 = 0x1;
   /// fosc/64.
   pub const VAL_0x02: u32 = 0x2;
   /// fosc/128.
   pub const VAL_0x03: u32 = 0x3;
   /// fosc/2.
   pub const VAL_0x04: u32 = 0x4;
   /// fosc/8.
   pub const VAL_0x05: u32 = 0x5;
   /// fosc/32.
   pub const VAL_0x06: u32 = 0x6;
   /// fosc/64.
   pub const VAL_0x07: u32 = 0x7;
}

/// `CPU_SLEEP_MODE_3BITS` value group
#[allow(non_upper_case_globals)]
pub mod cpu_sleep_mode_3bits {
   /// Idle.
   pub const IDLE: u32 = 0x0;
   /// ADC Noise Reduction (If Available).
   pub const ADC: u32 = 0x1;
   /// Reserved.
   pub const VAL_0x02: u32 = 0x2;
   /// Power Save.
   pub const PSAVE: u32 = 0x3;
   /// Power Off.
   pub const POFF: u32 = 0x4;
   /// Reserved.
   pub const VAL_0x05: u32 = 0x5;
   /// Reserved.
   pub const VAL_0x06: u32 = 0x6;
   /// Reserved.
   pub const VAL_0x07: u32 = 0x7;
}

/// `EEP_MODE` value group
#[allow(non_upper_case_globals)]
pub mod eep_mode {
   /// Erase and Write in one operation.
   pub const VAL_0x00: u32 = 0x0;
   /// Erase Only.
   pub const VAL_0x01: u32 = 0x1;
   /// Write Only.
   pub const VAL_0x02: u32 = 0x2;
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

/// `ENUM_SUT` value group
#[allow(non_upper_case_globals)]
pub mod enum_sut {
   /// Start-up time 6 CK/14 CK + 4 ms.
   pub const _6CK_14CK_4MS: u32 = 0x0;
   /// Start-up time 6 CK/14 CK + 8 ms.
   pub const _6CK_14CK_8MS: u32 = 0x1;
   /// Start-up time 6 CK/14 CK + 16 ms.
   pub const _6CK_14CK_16MS: u32 = 0x2;
   /// Start-up time 6 CK/14 CK + 32 ms.
   pub const _6CK_14CK_32MS: u32 = 0x3;
   /// Start-up time 6 CK/14 CK + 64 ms.
   pub const _6CK_14CK_64MS: u32 = 0x4;
   /// Start-up time 6 CK/14 CK + 128 ms.
   pub const _6CK_14CK_128MS: u32 = 0x5;
   /// Start-up time 6 CK/14 CK + 256 ms.
   pub const _6CK_14CK_256MS: u32 = 0x6;
   /// Start-up time 6 CK/14 CK + 512 ms.
   pub const _6CK_14CK_512MS: u32 = 0x7;
}

/// Interrupt Sense Control
#[allow(non_upper_case_globals)]
pub mod interrupt_sense_control {
   /// Low Level of INTX.
   pub const VAL_0x00: u32 = 0x0;
   /// Any Logical Change of INTX.
   pub const VAL_0x01: u32 = 0x1;
   /// Falling Edge of INTX.
   pub const VAL_0x02: u32 = 0x2;
   /// Rising Edge of INTX.
   pub const VAL_0x03: u32 = 0x3;
}

/// `WDOG_TIMER_PRESCALE_4BITS` value group
#[allow(non_upper_case_globals)]
pub mod wdog_timer_prescale_4bits {
   /// Oscillator Cycles 2K.
   pub const VAL_0x00: u32 = 0x0;
   /// Oscillator Cycles 4K.
   pub const VAL_0x01: u32 = 0x1;
   /// Oscillator Cycles 8K.
   pub const VAL_0x02: u32 = 0x2;
   /// Oscillator Cycles 16K.
   pub const VAL_0x03: u32 = 0x3;
   /// Oscillator Cycles 32K.
   pub const VAL_0x04: u32 = 0x4;
   /// Oscillator Cycles 64K.
   pub const VAL_0x05: u32 = 0x5;
   /// Oscillator Cycles 128K.
   pub const VAL_0x06: u32 = 0x6;
   /// Oscillator Cycles 256K.
   pub const VAL_0x07: u32 = 0x7;
   /// Oscillator Cycles 512K.
   pub const VAL_0x08: u32 = 0x8;
   /// Oscillator Cycles 1024K.
   pub const VAL_0x09: u32 = 0x9;
}

