//! The AVR ATmega406 microcontroller
//!
//! # Variants
//! |        | Pinout | Mcu age | Operating temperature | Operating voltage | Max speed |
//! |--------|--------|---------|-----------------------|-------------------|-----------|
//! | standard |  |  | 0°C - 0°C | 4V - 5.5V | 0 MHz |
//!

#![allow(non_upper_case_globals)]

/// `LOW` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BOOTSZ | 110000 |
/// | BOOTRST | 1000 |
/// | EESAVE | 1000000 |
/// | SUT_CKSEL | 111 |
/// | WDTON | 10000000 |
pub const LOW: *mut u8 = 0x0 as *mut u8;

/// `LOCKBIT` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BLB0 | 1100 |
/// | BLB1 | 110000 |
/// | LB | 11 |
pub const LOCKBIT: *mut u8 = 0x0 as *mut u8;

/// `HIGH` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | JTAGEN | 1 |
/// | OCDEN | 10 |
pub const HIGH: *mut u8 = 0x1 as *mut u8;

/// Port A Input Pins.
pub const PINA: *mut u8 = 0x20 as *mut u8;

/// Port A Data Direction Register.
pub const DDRA: *mut u8 = 0x21 as *mut u8;

/// Port A Data Register.
pub const PORTA: *mut u8 = 0x22 as *mut u8;

/// Port B Input Pins.
pub const PINB: *mut u8 = 0x23 as *mut u8;

/// Port B Data Direction Register.
pub const DDRB: *mut u8 = 0x24 as *mut u8;

/// Port B Data Register.
pub const PORTB: *mut u8 = 0x25 as *mut u8;

/// Port C Data Register.
pub const PORTC: *mut u8 = 0x28 as *mut u8;

/// Input Pins, Port D.
pub const PIND: *mut u8 = 0x29 as *mut u8;

/// Data Direction Register, Port D.
pub const DDRD: *mut u8 = 0x2A as *mut u8;

/// Data Register, Port D.
pub const PORTD: *mut u8 = 0x2B as *mut u8;

/// Timer/Counter Interrupt Flag register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TOV0 | 1 |
/// | OCF0B | 100 |
/// | OCF0A | 10 |
pub const TIFR0: *mut u8 = 0x35 as *mut u8;

/// Timer/Counter Interrupt Flag register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCF1A | 10 |
/// | TOV1 | 1 |
pub const TIFR1: *mut u8 = 0x36 as *mut u8;

/// Pin Change Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCIF | 11 |
pub const PCIFR: *mut u8 = 0x3B as *mut u8;

/// External Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | INTF | 1111 |
pub const EIFR: *mut u8 = 0x3C as *mut u8;

/// External Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | INT | 1111 |
pub const EIMSK: *mut u8 = 0x3D as *mut u8;

/// General Purpose IO Register 0.
pub const GPIOR0: *mut u8 = 0x3E as *mut u8;

/// EEPROM Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EEPM | 110000 |
/// | EEPE | 10 |
/// | EERIE | 1000 |
/// | EERE | 1 |
/// | EEMPE | 100 |
pub const EECR: *mut u8 = 0x3F as *mut u8;

/// EEPROM Data Register.
pub const EEDR: *mut u8 = 0x40 as *mut u8;

/// EEPROM Address Register  Bytes low byte.
pub const EEARL: *mut u8 = 0x41 as *mut u8;

/// EEPROM Address Register  Bytes.
pub const EEAR: *mut u16 = 0x41 as *mut u16;

/// EEPROM Address Register  Bytes high byte.
pub const EEARH: *mut u8 = 0x42 as *mut u8;

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
/// | COM0A | 11000000 |
/// | COM0B | 110000 |
/// | WGM0 | 11 |
pub const TCCR0A: *mut u8 = 0x44 as *mut u8;

/// Timer/Counter0 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WGM02 | 1000 |
/// | CS0 | 111 |
/// | FOC0B | 1000000 |
/// | FOC0A | 10000000 |
pub const TCCR0B: *mut u8 = 0x45 as *mut u8;

/// Timer Counter 0.
pub const TCNT0: *mut u8 = 0x46 as *mut u8;

/// Output compare Register A.
pub const OCR0A: *mut u8 = 0x47 as *mut u8;

/// Output compare Register B.
pub const OCR0B: *mut u8 = 0x48 as *mut u8;

/// General Purpose IO Register 1.
pub const GPIOR1: *mut u8 = 0x4A as *mut u8;

/// General Purpose IO Register 2.
pub const GPIOR2: *mut u8 = 0x4B as *mut u8;

/// Sleep Mode Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SM | 1110 |
/// | SE | 1 |
pub const SMCR: *mut u8 = 0x53 as *mut u8;

/// MCU Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDRF | 1000 |
/// | PORF | 1 |
/// | BODRF | 100 |
/// | JTRF | 10000 |
/// | EXTRF | 10 |
pub const MCUSR: *mut u8 = 0x54 as *mut u8;

/// MCU Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | JTD | 10000000 |
/// | IVCE | 1 |
/// | PUD | 10000 |
/// | IVSEL | 10 |
pub const MCUCR: *mut u8 = 0x55 as *mut u8;

/// Store Program Memory Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PGWRT | 100 |
/// | RWWSB | 1000000 |
/// | RWWSRE | 10000 |
/// | SIGRD | 100000 |
/// | BLBSET | 1000 |
/// | PGERS | 10 |
/// | SPMEN | 1 |
/// | SPMIE | 10000000 |
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
/// | V | 1000 |
/// | C | 1 |
/// | N | 100 |
/// | T | 1000000 |
/// | H | 100000 |
/// | S | 10000 |
/// | Z | 10 |
/// | I | 10000000 |
pub const SREG: *mut u8 = 0x5F as *mut u8;

/// Watchdog Timer Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDCE | 10000 |
/// | WDE | 1000 |
/// | WDIF | 10000000 |
/// | WDP | 100111 |
/// | WDIE | 1000000 |
pub const WDTCSR: *mut u8 = 0x60 as *mut u8;

/// Wake-up Timer Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WUTCF | 100000 |
/// | WUTIE | 1000000 |
/// | WUTIF | 10000000 |
/// | WUTR | 10000 |
/// | WUTP | 111 |
/// | WUTE | 1000 |
pub const WUTCSR: *mut u8 = 0x62 as *mut u8;

/// Power Reduction Register 0.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRTIM0 | 10 |
/// | PRTWI | 1000 |
/// | PRVADC | 1 |
/// | PRTIM1 | 100 |
pub const PRR0: *mut u8 = 0x64 as *mut u8;

/// Fast Oscillator Calibration Value.
pub const FOSCCAL: *mut u8 = 0x66 as *mut u8;

/// Pin Change Interrupt Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCIE | 11 |
pub const PCICR: *mut u8 = 0x68 as *mut u8;

/// External Interrupt Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ISC1 | 1100 |
/// | ISC0 | 11 |
/// | ISC2 | 110000 |
/// | ISC3 | 11000000 |
pub const EICRA: *mut u8 = 0x69 as *mut u8;

/// Pin Change Enable Mask Register 0.
pub const PCMSK0: *mut u8 = 0x6B as *mut u8;

/// Pin Change Enable Mask Register 1.
pub const PCMSK1: *mut u8 = 0x6C as *mut u8;

/// Timer/Counter Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCIE0B | 100 |
/// | TOIE0 | 1 |
/// | OCIE0A | 10 |
pub const TIMSK0: *mut u8 = 0x6E as *mut u8;

/// Timer/Counter Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCIE1A | 10 |
/// | TOIE1 | 1 |
pub const TIMSK1: *mut u8 = 0x6F as *mut u8;

/// VADC Data Register  Bytes low byte.
pub const VADCL: *mut u8 = 0x78 as *mut u8;

/// VADC Data Register  Bytes.
pub const VADC: *mut u16 = 0x78 as *mut u16;

/// VADC Data Register  Bytes high byte.
pub const VADCH: *mut u8 = 0x79 as *mut u8;

/// The VADC Control and Status register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | VADCCIF | 10 |
/// | VADEN | 1000 |
/// | VADCCIE | 1 |
/// | VADSC | 100 |
pub const VADCSR: *mut u8 = 0x7A as *mut u8;

/// The VADC multiplexer Selection Register.
pub const VADMUX: *mut u8 = 0x7C as *mut u8;

/// Digital Input Disable Register.
pub const DIDR0: *mut u8 = 0x7E as *mut u8;

/// Timer/Counter1 Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CS1 | 111 |
/// | CTC1 | 1000 |
pub const TCCR1B: *mut u8 = 0x81 as *mut u8;

/// Timer Counter 1  Bytes.
pub const TCNT1: *mut u16 = 0x84 as *mut u16;

/// Timer Counter 1  Bytes low byte.
pub const TCNT1L: *mut u8 = 0x84 as *mut u8;

/// Timer Counter 1  Bytes high byte.
pub const TCNT1H: *mut u8 = 0x85 as *mut u8;

/// Output Compare Register 1A Low byte.
pub const OCR1AL: *mut u8 = 0x88 as *mut u8;

/// Output Compare Register 1A High byte.
pub const OCR1AH: *mut u8 = 0x89 as *mut u8;

/// TWI Bit Rate register.
pub const TWBR: *mut u8 = 0xB8 as *mut u8;

/// TWI Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TWPS | 11 |
/// | TWS | 11111000 |
pub const TWSR: *mut u8 = 0xB9 as *mut u8;

/// TWI (Slave) Address register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TWGCE | 1 |
/// | TWA | 11111110 |
pub const TWAR: *mut u8 = 0xBA as *mut u8;

/// TWI Data register.
pub const TWDR: *mut u8 = 0xBB as *mut u8;

/// TWI Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TWWC | 1000 |
/// | TWIE | 1 |
/// | TWEN | 100 |
/// | TWINT | 10000000 |
/// | TWEA | 1000000 |
/// | TWSTA | 100000 |
/// | TWSTO | 10000 |
pub const TWCR: *mut u8 = 0xBC as *mut u8;

/// TWI (Slave) Address Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TWAM | 11111110 |
pub const TWAMR: *mut u8 = 0xBD as *mut u8;

/// TWI Bus Control and Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TWBDT | 110 |
/// | TWBCIF | 10000000 |
/// | TWBCIP | 1 |
/// | TWBCIE | 1000000 |
pub const TWBCSR: *mut u8 = 0xBE as *mut u8;

/// Clock Control and Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | XOE | 10 |
/// | ACS | 1 |
pub const CCSR: *mut u8 = 0xC0 as *mut u8;

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
/// | CADSE | 1 |
/// | CADUB | 100000 |
/// | CADAS | 11000 |
/// | CADSI | 110 |
pub const CADCSRA: *mut u8 = 0xE4 as *mut u8;

/// CC-ADC Control and Status Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CADICIE | 10000 |
/// | CADRCIE | 100000 |
/// | CADACIE | 1000000 |
/// | CADRCIF | 10 |
/// | CADICIF | 1 |
/// | CADACIF | 100 |
pub const CADCSRB: *mut u8 = 0xE5 as *mut u8;

/// CC-ADC Regular Charge Current.
pub const CADRCC: *mut u8 = 0xE6 as *mut u8;

/// CC-ADC Regular Discharge Current.
pub const CADRDC: *mut u8 = 0xE7 as *mut u8;

/// CC-ADC Instantaneous Current.
pub const CADIC: *mut u16 = 0xE8 as *mut u16;

/// CC-ADC Instantaneous Current low byte.
pub const CADICL: *mut u8 = 0xE8 as *mut u8;

/// CC-ADC Instantaneous Current high byte.
pub const CADICH: *mut u8 = 0xE9 as *mut u8;

/// `FCSR` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PWMOC | 100000 |
/// | PFD | 1 |
/// | CFE | 10 |
/// | PWMOPC | 10000 |
/// | DFE | 100 |
/// | CPS | 1000 |
pub const FCSR: *mut u8 = 0xF0 as *mut u8;

/// Cell Balancing Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CBE | 1111 |
pub const CBCR: *mut u8 = 0xF1 as *mut u8;

/// Battery Protection Interrupt Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | COCIE | 100 |
/// | DOCIE | 10 |
/// | DUVIF | 10000000 |
/// | DOCIF | 100000 |
/// | DUVIE | 1000 |
/// | SCIF | 10000 |
/// | COCIF | 1000000 |
/// | SCIE | 1 |
pub const BPIR: *mut u8 = 0xF2 as *mut u8;

/// Battery Protection Deep Under Voltage Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DUVT | 110000 |
/// | DUDL | 1111 |
pub const BPDUV: *mut u8 = 0xF3 as *mut u8;

/// Battery Protection Short-Circuit Detection Level Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SCDL | 1111 |
pub const BPSCD: *mut u8 = 0xF4 as *mut u8;

/// Battery Protection OverCurrent Detection Level Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CCDL | 1111 |
/// | DCDL | 11110000 |
pub const BPOCD: *mut u8 = 0xF5 as *mut u8;

/// Current Battery Protection Timing Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SCPT | 11110000 |
/// | OCPT | 1111 |
pub const CBPTR: *mut u8 = 0xF6 as *mut u8;

/// Battery Protection Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CCD | 1 |
/// | DCD | 10 |
/// | SCD | 100 |
/// | DUVD | 1000 |
pub const BPCR: *mut u8 = 0xF7 as *mut u8;

/// Battery Protection Parameter Lock Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BPPLE | 10 |
/// | BPPL | 1 |
pub const BPPLR: *mut u8 = 0xF8 as *mut u8;

/// Bitfield on register `BGCCR`
pub const BGD: u8 = 0x80;

/// Bitfield on register `BGCCR`
pub const BGCC: u8 = 0x3F;

/// Bitfield on register `BPCR`
pub const CCD: u8 = 0x1;

/// Bitfield on register `BPCR`
pub const DCD: u8 = 0x2;

/// Bitfield on register `BPCR`
pub const SCD: u8 = 0x4;

/// Bitfield on register `BPCR`
pub const DUVD: u8 = 0x8;

/// Bitfield on register `BPDUV`
pub const DUVT: u8 = 0x30;

/// Bitfield on register `BPDUV`
pub const DUDL: u8 = 0xF;

/// Bitfield on register `BPIR`
pub const COCIE: u8 = 0x4;

/// Bitfield on register `BPIR`
pub const DOCIE: u8 = 0x2;

/// Bitfield on register `BPIR`
pub const DUVIF: u8 = 0x80;

/// Bitfield on register `BPIR`
pub const DOCIF: u8 = 0x20;

/// Bitfield on register `BPIR`
pub const DUVIE: u8 = 0x8;

/// Bitfield on register `BPIR`
pub const SCIF: u8 = 0x10;

/// Bitfield on register `BPIR`
pub const COCIF: u8 = 0x40;

/// Bitfield on register `BPIR`
pub const SCIE: u8 = 0x1;

/// Bitfield on register `BPOCD`
pub const CCDL: u8 = 0xF;

/// Bitfield on register `BPOCD`
pub const DCDL: u8 = 0xF0;

/// Bitfield on register `BPPLR`
pub const BPPLE: u8 = 0x2;

/// Bitfield on register `BPPLR`
pub const BPPL: u8 = 0x1;

/// Bitfield on register `BPSCD`
pub const SCDL: u8 = 0xF;

/// Bitfield on register `CADCSRA`
pub const CADEN: u8 = 0x80;

/// Bitfield on register `CADCSRA`
pub const CADSE: u8 = 0x1;

/// Bitfield on register `CADCSRA`
pub const CADUB: u8 = 0x20;

/// Bitfield on register `CADCSRA`
pub const CADAS: u8 = 0x18;

/// Bitfield on register `CADCSRA`
pub const CADSI: u8 = 0x6;

/// Bitfield on register `CADCSRB`
pub const CADICIE: u8 = 0x10;

/// Bitfield on register `CADCSRB`
pub const CADRCIE: u8 = 0x20;

/// Bitfield on register `CADCSRB`
pub const CADACIE: u8 = 0x40;

/// Bitfield on register `CADCSRB`
pub const CADRCIF: u8 = 0x2;

/// Bitfield on register `CADCSRB`
pub const CADICIF: u8 = 0x1;

/// Bitfield on register `CADCSRB`
pub const CADACIF: u8 = 0x4;

/// Bitfield on register `CBCR`
pub const CBE: u8 = 0xF;

/// Bitfield on register `CBPTR`
pub const SCPT: u8 = 0xF0;

/// Bitfield on register `CBPTR`
pub const OCPT: u8 = 0xF;

/// Bitfield on register `CCSR`
pub const XOE: u8 = 0x2;

/// Bitfield on register `CCSR`
pub const ACS: u8 = 0x1;

/// Bitfield on register `EECR`
pub const EEPM: u8 = 0x30;

/// Bitfield on register `EECR`
pub const EEPE: u8 = 0x2;

/// Bitfield on register `EECR`
pub const EERIE: u8 = 0x8;

/// Bitfield on register `EECR`
pub const EERE: u8 = 0x1;

/// Bitfield on register `EECR`
pub const EEMPE: u8 = 0x4;

/// Bitfield on register `EICRA`
pub const ISC1: u8 = 0xC;

/// Bitfield on register `EICRA`
pub const ISC0: u8 = 0x3;

/// Bitfield on register `EICRA`
pub const ISC2: u8 = 0x30;

/// Bitfield on register `EICRA`
pub const ISC3: u8 = 0xC0;

/// Bitfield on register `EIFR`
pub const INTF: u8 = 0xF;

/// Bitfield on register `EIMSK`
pub const INT: u8 = 0xF;

/// Bitfield on register `FCSR`
pub const PWMOC: u8 = 0x20;

/// Bitfield on register `FCSR`
pub const PFD: u8 = 0x1;

/// Bitfield on register `FCSR`
pub const CFE: u8 = 0x2;

/// Bitfield on register `FCSR`
pub const PWMOPC: u8 = 0x10;

/// Bitfield on register `FCSR`
pub const DFE: u8 = 0x4;

/// Bitfield on register `FCSR`
pub const CPS: u8 = 0x8;

/// Bitfield on register `GTCCR`
pub const PSRSYNC: u8 = 0x1;

/// Bitfield on register `GTCCR`
pub const TSM: u8 = 0x80;

/// Bitfield on register `HIGH`
pub const JTAGEN: u8 = 0x1;

/// Bitfield on register `HIGH`
pub const OCDEN: u8 = 0x2;

/// Bitfield on register `LOCKBIT`
pub const BLB0: u8 = 0xC;

/// Bitfield on register `LOCKBIT`
pub const BLB1: u8 = 0x30;

/// Bitfield on register `LOCKBIT`
pub const LB: u8 = 0x3;

/// Bitfield on register `LOW`
pub const BOOTSZ: u8 = 0x30;

/// Bitfield on register `LOW`
pub const BOOTRST: u8 = 0x8;

/// Bitfield on register `LOW`
pub const EESAVE: u8 = 0x40;

/// Bitfield on register `LOW`
pub const SUT_CKSEL: u8 = 0x7;

/// Bitfield on register `LOW`
pub const WDTON: u8 = 0x80;

/// Bitfield on register `MCUCR`
pub const JTD: u8 = 0x80;

/// Bitfield on register `MCUCR`
pub const IVCE: u8 = 0x1;

/// Bitfield on register `MCUCR`
pub const PUD: u8 = 0x10;

/// Bitfield on register `MCUCR`
pub const IVSEL: u8 = 0x2;

/// Bitfield on register `MCUSR`
pub const WDRF: u8 = 0x8;

/// Bitfield on register `MCUSR`
pub const PORF: u8 = 0x1;

/// Bitfield on register `MCUSR`
pub const BODRF: u8 = 0x4;

/// Bitfield on register `MCUSR`
pub const JTRF: u8 = 0x10;

/// Bitfield on register `MCUSR`
pub const EXTRF: u8 = 0x2;

/// Bitfield on register `PCICR`
pub const PCIE: u8 = 0x3;

/// Bitfield on register `PCIFR`
pub const PCIF: u8 = 0x3;

/// Bitfield on register `PRR0`
pub const PRTIM0: u8 = 0x2;

/// Bitfield on register `PRR0`
pub const PRTWI: u8 = 0x8;

/// Bitfield on register `PRR0`
pub const PRVADC: u8 = 0x1;

/// Bitfield on register `PRR0`
pub const PRTIM1: u8 = 0x4;

/// Bitfield on register `SMCR`
pub const SM: u8 = 0xE;

/// Bitfield on register `SMCR`
pub const SE: u8 = 0x1;

/// Bitfield on register `SPMCSR`
pub const PGWRT: u8 = 0x4;

/// Bitfield on register `SPMCSR`
pub const RWWSB: u8 = 0x40;

/// Bitfield on register `SPMCSR`
pub const RWWSRE: u8 = 0x10;

/// Bitfield on register `SPMCSR`
pub const SIGRD: u8 = 0x20;

/// Bitfield on register `SPMCSR`
pub const BLBSET: u8 = 0x8;

/// Bitfield on register `SPMCSR`
pub const PGERS: u8 = 0x2;

/// Bitfield on register `SPMCSR`
pub const SPMEN: u8 = 0x1;

/// Bitfield on register `SPMCSR`
pub const SPMIE: u8 = 0x80;

/// Bitfield on register `SREG`
pub const V: u8 = 0x8;

/// Bitfield on register `SREG`
pub const C: u8 = 0x1;

/// Bitfield on register `SREG`
pub const N: u8 = 0x4;

/// Bitfield on register `SREG`
pub const T: u8 = 0x40;

/// Bitfield on register `SREG`
pub const H: u8 = 0x20;

/// Bitfield on register `SREG`
pub const S: u8 = 0x10;

/// Bitfield on register `SREG`
pub const Z: u8 = 0x2;

/// Bitfield on register `SREG`
pub const I: u8 = 0x80;

/// Bitfield on register `TCCR0A`
pub const COM0A: u8 = 0xC0;

/// Bitfield on register `TCCR0A`
pub const COM0B: u8 = 0x30;

/// Bitfield on register `TCCR0A`
pub const WGM0: u8 = 0x3;

/// Bitfield on register `TCCR0B`
pub const WGM02: u8 = 0x8;

/// Bitfield on register `TCCR0B`
pub const CS0: u8 = 0x7;

/// Bitfield on register `TCCR0B`
pub const FOC0B: u8 = 0x40;

/// Bitfield on register `TCCR0B`
pub const FOC0A: u8 = 0x80;

/// Bitfield on register `TCCR1B`
pub const CS1: u8 = 0x7;

/// Bitfield on register `TCCR1B`
pub const CTC1: u8 = 0x8;

/// Bitfield on register `TIFR0`
pub const TOV0: u8 = 0x1;

/// Bitfield on register `TIFR0`
pub const OCF0B: u8 = 0x4;

/// Bitfield on register `TIFR0`
pub const OCF0A: u8 = 0x2;

/// Bitfield on register `TIFR1`
pub const OCF1A: u8 = 0x2;

/// Bitfield on register `TIFR1`
pub const TOV1: u8 = 0x1;

/// Bitfield on register `TIMSK0`
pub const OCIE0B: u8 = 0x4;

/// Bitfield on register `TIMSK0`
pub const TOIE0: u8 = 0x1;

/// Bitfield on register `TIMSK0`
pub const OCIE0A: u8 = 0x2;

/// Bitfield on register `TIMSK1`
pub const OCIE1A: u8 = 0x2;

/// Bitfield on register `TIMSK1`
pub const TOIE1: u8 = 0x1;

/// Bitfield on register `TWAMR`
pub const TWAM: u8 = 0xFE;

/// Bitfield on register `TWAR`
pub const TWGCE: u8 = 0x1;

/// Bitfield on register `TWAR`
pub const TWA: u8 = 0xFE;

/// Bitfield on register `TWBCSR`
pub const TWBDT: u8 = 0x6;

/// Bitfield on register `TWBCSR`
pub const TWBCIF: u8 = 0x80;

/// Bitfield on register `TWBCSR`
pub const TWBCIP: u8 = 0x1;

/// Bitfield on register `TWBCSR`
pub const TWBCIE: u8 = 0x40;

/// Bitfield on register `TWCR`
pub const TWWC: u8 = 0x8;

/// Bitfield on register `TWCR`
pub const TWIE: u8 = 0x1;

/// Bitfield on register `TWCR`
pub const TWEN: u8 = 0x4;

/// Bitfield on register `TWCR`
pub const TWINT: u8 = 0x80;

/// Bitfield on register `TWCR`
pub const TWEA: u8 = 0x40;

/// Bitfield on register `TWCR`
pub const TWSTA: u8 = 0x20;

/// Bitfield on register `TWCR`
pub const TWSTO: u8 = 0x10;

/// Bitfield on register `TWSR`
pub const TWPS: u8 = 0x3;

/// Bitfield on register `TWSR`
pub const TWS: u8 = 0xF8;

/// Bitfield on register `VADCSR`
pub const VADCCIF: u8 = 0x2;

/// Bitfield on register `VADCSR`
pub const VADEN: u8 = 0x8;

/// Bitfield on register `VADCSR`
pub const VADCCIE: u8 = 0x1;

/// Bitfield on register `VADCSR`
pub const VADSC: u8 = 0x4;

/// Bitfield on register `WDTCSR`
pub const WDCE: u8 = 0x10;

/// Bitfield on register `WDTCSR`
pub const WDE: u8 = 0x8;

/// Bitfield on register `WDTCSR`
pub const WDIF: u8 = 0x80;

/// Bitfield on register `WDTCSR`
pub const WDP: u8 = 0x27;

/// Bitfield on register `WDTCSR`
pub const WDIE: u8 = 0x40;

/// Bitfield on register `WUTCSR`
pub const WUTCF: u8 = 0x20;

/// Bitfield on register `WUTCSR`
pub const WUTIE: u8 = 0x40;

/// Bitfield on register `WUTCSR`
pub const WUTIF: u8 = 0x80;

/// Bitfield on register `WUTCSR`
pub const WUTR: u8 = 0x10;

/// Bitfield on register `WUTCSR`
pub const WUTP: u8 = 0x7;

/// Bitfield on register `WUTCSR`
pub const WUTE: u8 = 0x8;

/// `ANALOG_CADA_ACC_TIME` value group
#[allow(non_upper_case_globals)]
pub mod analog_cada_acc_time {
   /// 125ms.
   pub const VAL_0x00: u32 = 0x0;
   /// 250ms.
   pub const VAL_0x01: u32 = 0x1;
   /// 500ms.
   pub const VAL_0x02: u32 = 0x2;
   /// 1000ms.
   pub const VAL_0x03: u32 = 0x3;
}

/// `BAT_DEEP_UNDER_DELAY` value group
#[allow(non_upper_case_globals)]
pub mod bat_deep_under_delay {
   /// 750ms.
   pub const VAL_0x00: u32 = 0x0;
   /// 1000ms.
   pub const VAL_0x01: u32 = 0x1;
   /// 1250ms.
   pub const VAL_0x02: u32 = 0x2;
   /// 1500ms.
   pub const VAL_0x03: u32 = 0x3;
}

/// `BAT_DEEP_UNDER_LEVEL` value group
#[allow(non_upper_case_globals)]
pub mod bat_deep_under_level {
   /// 4.71V.
   pub const VAL_0x00: u32 = 0x0;
   /// 5.03V.
   pub const VAL_0x01: u32 = 0x1;
   /// 5.34V.
   pub const VAL_0x02: u32 = 0x2;
   /// 5.66V.
   pub const VAL_0x03: u32 = 0x3;
   /// 5.97V.
   pub const VAL_0x04: u32 = 0x4;
   /// 6.29V.
   pub const VAL_0x05: u32 = 0x5;
   /// 6.60V.
   pub const VAL_0x06: u32 = 0x6;
   /// 6.91V.
   pub const VAL_0x07: u32 = 0x7;
   /// 7.23V.
   pub const VAL_0x08: u32 = 0x8;
   /// 7.54V.
   pub const VAL_0x09: u32 = 0x9;
   /// 7.86V.
   pub const VAL_0x0A: u32 = 0xA;
   /// 8.17V.
   pub const VAL_0x0B: u32 = 0xB;
   /// 8.49V.
   pub const VAL_0x0C: u32 = 0xC;
   /// 8.80V.
   pub const VAL_0x0D: u32 = 0xD;
   /// 9.11V.
   pub const VAL_0x0E: u32 = 0xE;
   /// 9.43V.
   pub const VAL_0x0F: u32 = 0xF;
}

/// `BAT_OVER_CURRENT_DELAY` value group
#[allow(non_upper_case_globals)]
pub mod bat_over_current_delay {
   /// 1 ms.
   pub const VAL_0x00: u32 = 0x0;
   /// 2 ms.
   pub const VAL_0x01: u32 = 0x1;
   /// 4 ms.
   pub const VAL_0x02: u32 = 0x2;
   /// 6 ms.
   pub const VAL_0x03: u32 = 0x3;
   /// 8 ms.
   pub const VAL_0x04: u32 = 0x4;
   /// 10 ms.
   pub const VAL_0x05: u32 = 0x5;
   /// 12 ms.
   pub const VAL_0x06: u32 = 0x6;
   /// 14 ms.
   pub const VAL_0x07: u32 = 0x7;
   /// 16 ms.
   pub const VAL_0x08: u32 = 0x8;
   /// 18 ms.
   pub const VAL_0x09: u32 = 0x9;
   /// 20 ms.
   pub const VAL_0x0A: u32 = 0xA;
   /// 22 ms.
   pub const VAL_0x0B: u32 = 0xB;
   /// 24 ms.
   pub const VAL_0x0C: u32 = 0xC;
   /// 26 ms.
   pub const VAL_0x0D: u32 = 0xD;
   /// 28 ms.
   pub const VAL_0x0E: u32 = 0xE;
   /// 30 ms.
   pub const VAL_0x0F: u32 = 0xF;
}

/// `BAT_SHORT_CIRC_DELAY` value group
#[allow(non_upper_case_globals)]
pub mod bat_short_circ_delay {
   /// 61 us.
   pub const VAL_0x00: u32 = 0x0;
   /// 122 us.
   pub const VAL_0x01: u32 = 0x1;
   /// 183 us.
   pub const VAL_0x02: u32 = 0x2;
   /// 244 us.
   pub const VAL_0x03: u32 = 0x3;
   /// 305 us.
   pub const VAL_0x04: u32 = 0x4;
   /// 366 us.
   pub const VAL_0x05: u32 = 0x5;
   /// 427 us.
   pub const VAL_0x06: u32 = 0x6;
   /// 488 us.
   pub const VAL_0x07: u32 = 0x7;
   /// 610 us.
   pub const VAL_0x08: u32 = 0x8;
   /// 732 us.
   pub const VAL_0x09: u32 = 0x9;
   /// 854 us.
   pub const VAL_0x0A: u32 = 0xA;
   /// 976 us.
   pub const VAL_0x0B: u32 = 0xB;
   /// 1098 us.
   pub const VAL_0x0C: u32 = 0xC;
   /// 1220 us.
   pub const VAL_0x0D: u32 = 0xD;
   /// 1342 us.
   pub const VAL_0x0E: u32 = 0xE;
   /// 1464 us.
   pub const VAL_0x0F: u32 = 0xF;
}

/// `BAT_VOLT_SENSE` value group
#[allow(non_upper_case_globals)]
pub mod bat_volt_sense {
   /// 0.050V.
   pub const VAL_0x00: u32 = 0x0;
   /// 0.055V.
   pub const VAL_0x01: u32 = 0x1;
   /// 0.060V.
   pub const VAL_0x02: u32 = 0x2;
   /// 0.065V.
   pub const VAL_0x03: u32 = 0x3;
   /// 0.070V.
   pub const VAL_0x04: u32 = 0x4;
   /// 0.080V.
   pub const VAL_0x05: u32 = 0x5;
   /// 0.090V.
   pub const VAL_0x06: u32 = 0x6;
   /// 0.100V.
   pub const VAL_0x07: u32 = 0x7;
   /// 0.110V.
   pub const VAL_0x08: u32 = 0x8;
   /// 0.120V.
   pub const VAL_0x09: u32 = 0x9;
   /// 0.130V.
   pub const VAL_0x0A: u32 = 0xA;
   /// 0.140V.
   pub const VAL_0x0B: u32 = 0xB;
   /// 0.160V.
   pub const VAL_0x0C: u32 = 0xC;
   /// 0.180V.
   pub const VAL_0x0D: u32 = 0xD;
   /// 0.200V.
   pub const VAL_0x0E: u32 = 0xE;
   /// 0.220V.
   pub const VAL_0x0F: u32 = 0xF;
}

/// `BAT_VOLT_SENSE2` value group
#[allow(non_upper_case_globals)]
pub mod bat_volt_sense2 {
   /// 0.100V.
   pub const VAL_0x00: u32 = 0x0;
   /// 0.110V.
   pub const VAL_0x01: u32 = 0x1;
   /// 0.120V.
   pub const VAL_0x02: u32 = 0x2;
   /// 0.130V.
   pub const VAL_0x03: u32 = 0x3;
   /// 0.140V.
   pub const VAL_0x04: u32 = 0x4;
   /// 0.160V.
   pub const VAL_0x05: u32 = 0x5;
   /// 0.180V.
   pub const VAL_0x06: u32 = 0x6;
   /// 0.200V.
   pub const VAL_0x07: u32 = 0x7;
   /// 0.220V.
   pub const VAL_0x08: u32 = 0x8;
   /// 0.240V.
   pub const VAL_0x09: u32 = 0x9;
   /// 0.260V.
   pub const VAL_0x0A: u32 = 0xA;
   /// 0.280V.
   pub const VAL_0x0B: u32 = 0xB;
   /// 0.320V.
   pub const VAL_0x0C: u32 = 0xC;
   /// 0.360V.
   pub const VAL_0x0D: u32 = 0xD;
   /// 0.400V.
   pub const VAL_0x0E: u32 = 0xE;
   /// 0.440V.
   pub const VAL_0x0F: u32 = 0xF;
}

/// `CLK_SEL_3BIT` value group
#[allow(non_upper_case_globals)]
pub mod clk_sel_3bit {
   /// No Clock Source (Stopped).
   pub const VAL_0x00: u32 = 0x0;
   /// Running, No Prescaling.
   pub const VAL_0x01: u32 = 0x1;
   /// Running, CLK/8.
   pub const VAL_0x02: u32 = 0x2;
   /// Running, CLK/32.
   pub const VAL_0x03: u32 = 0x3;
   /// Running, CLK/64.
   pub const VAL_0x04: u32 = 0x4;
   /// Running, CLK/128.
   pub const VAL_0x05: u32 = 0x5;
   /// Running, CLK/256.
   pub const VAL_0x06: u32 = 0x6;
   /// Running, CLK/1024.
   pub const VAL_0x07: u32 = 0x7;
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

/// `CLK_SEL_3BIT_ONLY_PRESCALE` value group
#[allow(non_upper_case_globals)]
pub mod clk_sel_3bit_only_prescale {
   /// 4K(Slow RC) / 1K (32kHz).
   pub const VAL_0x00: u32 = 0x0;
   /// 8K(Slow RC) / 2K (32kHz).
   pub const VAL_0x01: u32 = 0x1;
   /// 16K(Slow RC) / 4K (32kHz).
   pub const VAL_0x02: u32 = 0x2;
   /// 32K(Slow RC) / 8K (32kHz).
   pub const VAL_0x03: u32 = 0x3;
   /// 64K(Slow RC) / 16K (32kHz).
   pub const VAL_0x04: u32 = 0x4;
   /// 128K(Slow RC) / 32K (32kHz).
   pub const VAL_0x05: u32 = 0x5;
   /// 256K(Slow RC) / 64K (32kHz).
   pub const VAL_0x06: u32 = 0x6;
   /// 512K(Slow RC) / 128K (32kHz).
   pub const VAL_0x07: u32 = 0x7;
}

/// `COMM_TWI_PRESACLE` value group
#[allow(non_upper_case_globals)]
pub mod comm_twi_presacle {
   /// 1.
   pub const VAL_0x00: u32 = 0x0;
   /// 4.
   pub const VAL_0x01: u32 = 0x1;
   /// 16.
   pub const VAL_0x02: u32 = 0x2;
   /// 64.
   pub const VAL_0x03: u32 = 0x3;
}

/// `COMM_TW_BUS_TIMEOUT` value group
#[allow(non_upper_case_globals)]
pub mod comm_tw_bus_timeout {
   /// 250ms.
   pub const VAL_0x00: u32 = 0x0;
   /// 500ms.
   pub const VAL_0x01: u32 = 0x1;
   /// 1000ms.
   pub const VAL_0x02: u32 = 0x2;
   /// 2000ms.
   pub const VAL_0x03: u32 = 0x3;
}

/// `CPU_SLEEP_MODE_3BITS3` value group
#[allow(non_upper_case_globals)]
pub mod cpu_sleep_mode_3bits3 {
   /// Idle.
   pub const IDLE: u32 = 0x0;
   /// ADC Noise Reduction (If Available).
   pub const ADC: u32 = 0x1;
   /// Power Down.
   pub const PDOWN: u32 = 0x2;
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

/// `ENUM_BLB` value group
#[allow(non_upper_case_globals)]
pub mod enum_blb {
   /// LPM and SPM prohibited in Application Section.
   pub const LPM_SPM_DISABLE: u32 = 0x0;
   /// LPM prohibited in Application Section.
   pub const LPM_DISABLE: u32 = 0x1;
   /// SPM prohibited in Application Section.
   pub const SPM_DISABLE: u32 = 0x2;
   /// No lock on SPM and LPM in Application Section.
   pub const NO_LOCK: u32 = 0x3;
}

/// `ENUM_BLB2` value group
#[allow(non_upper_case_globals)]
pub mod enum_blb2 {
   /// LPM and SPM prohibited in Boot Section.
   pub const LPM_SPM_DISABLE: u32 = 0x0;
   /// LPM prohibited in Boot Section.
   pub const LPM_DISABLE: u32 = 0x1;
   /// SPM prohibited in Boot Section.
   pub const SPM_DISABLE: u32 = 0x2;
   /// No lock on SPM and LPM in Boot Section.
   pub const NO_LOCK: u32 = 0x3;
}

/// `ENUM_BOOTSZ` value group
#[allow(non_upper_case_globals)]
pub mod enum_bootsz {
   /// Boot Flash size=256 words Boot address=$4F00.
   pub const _256W_4F00: u32 = 0x3;
   /// Boot Flash size=512 words Boot address=$4E00.
   pub const _512W_4E00: u32 = 0x2;
   /// Boot Flash size=1024 words Boot address=$4C00.
   pub const _1024W_4C00: u32 = 0x1;
   /// Boot Flash size=2048 words Boot address=$4800.
   pub const _2048W_4800: u32 = 0x0;
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

/// `ENUM_SUT_CKSEL` value group
#[allow(non_upper_case_globals)]
pub mod enum_sut_cksel {
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

