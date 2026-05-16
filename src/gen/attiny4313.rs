//! The AVR ATtiny4313 microcontroller
//!
//! # Variants
//! |        | Pinout | Mcu age | Operating temperature | Operating voltage | Max speed |
//! |--------|--------|---------|-----------------------|-------------------|-----------|
//! | standard |  |  | 0°C - 0°C | 1.8V - 5.5V | 0 MHz |
//!

#![allow(non_upper_case_globals)]

/// `LOW` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CKDIV8 | 10000000 |
/// | SUT_CKSEL | 111111 |
/// | CKOUT | 1000000 |
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
/// | DWEN | 10000000 |
/// | WDTON | 10000 |
/// | EESAVE | 1000000 |
/// | RSTDISBL | 1 |
/// | BODLEVEL | 1110 |
/// | SPIEN | 100000 |
pub const HIGH: *mut u8 = 0x1 as *mut u8;

/// `EXTENDED` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SELFPRGEN | 1 |
pub const EXTENDED: *mut u8 = 0x2 as *mut u8;

/// Digital Input Disable Register 1.
pub const DIDR: *mut u8 = 0x21 as *mut u8;

/// USART Baud Rate Register High Byte.
pub const UBRRH: *mut u8 = 0x22 as *mut u8;

/// USART Control and Status Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | UMSEL | 11000000 |
/// | UPM | 110000 |
/// | UCSZ | 110 |
/// | UCPOL | 1 |
/// | USBS | 1000 |
pub const UCSRC: *mut u8 = 0x23 as *mut u8;

/// Pin Change Interrupt Mask Register 1.
pub const PCMSK1: *mut u8 = 0x24 as *mut u8;

/// Pin Change Interrupt Mask Register 2.
pub const PCMSK2: *mut u8 = 0x25 as *mut u8;

/// Power reduction register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRTIM | 1100 |
/// | PRUSART | 1 |
/// | PRUSI | 10 |
pub const PRR: *mut u8 = 0x26 as *mut u8;

/// BOD control register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BPDSE | 1 |
/// | BPDS | 10 |
pub const BODCR: *mut u8 = 0x27 as *mut u8;

/// Analog Comparator Control And Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ACO | 100000 |
/// | ACD | 10000000 |
/// | ACIE | 1000 |
/// | ACI | 10000 |
/// | ACIC | 100 |
/// | ACBG | 1000000 |
/// | ACIS | 11 |
pub const ACSR: *mut u8 = 0x28 as *mut u8;

/// USART Baud Rate Register Low Byte.
pub const UBRRL: *mut u8 = 0x29 as *mut u8;

/// USART Control and Status Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TXCIE | 1000000 |
/// | RXEN | 10000 |
/// | TXEN | 1000 |
/// | UCSZ2 | 100 |
/// | TXB8 | 1 |
/// | UDRIE | 100000 |
/// | RXB8 | 10 |
/// | RXCIE | 10000000 |
pub const UCSRB: *mut u8 = 0x2A as *mut u8;

/// USART Control and Status Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DOR | 1000 |
/// | MPCM | 1 |
/// | UDRE | 100000 |
/// | UPE | 100 |
/// | TXC | 1000000 |
/// | RXC | 10000000 |
/// | U2X | 10 |
/// | FE | 10000 |
pub const UCSRA: *mut u8 = 0x2B as *mut u8;

/// USART I/O Data Register.
pub const UDR: *mut u8 = 0x2C as *mut u8;

/// USI Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | USISIE | 10000000 |
/// | USIOIE | 1000000 |
/// | USIWM | 110000 |
/// | USITC | 1 |
/// | USICLK | 10 |
/// | USICS | 1100 |
pub const USICR: *mut u8 = 0x2D as *mut u8;

/// USI Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | USIPF | 100000 |
/// | USIOIF | 1000000 |
/// | USICNT | 1111 |
/// | USISIF | 10000000 |
/// | USIDC | 10000 |
pub const USISR: *mut u8 = 0x2E as *mut u8;

/// USI Data Register.
pub const USIDR: *mut u8 = 0x2F as *mut u8;

/// Input Pins, Port D.
pub const PIND: *mut u8 = 0x30 as *mut u8;

/// Data Direction Register, Port D.
pub const DDRD: *mut u8 = 0x31 as *mut u8;

/// Data Register, Port D.
pub const PORTD: *mut u8 = 0x32 as *mut u8;

/// General Purpose I/O Register 0.
pub const GPIOR0: *mut u8 = 0x33 as *mut u8;

/// General Purpose I/O Register 1.
pub const GPIOR1: *mut u8 = 0x34 as *mut u8;

/// General Purpose I/O Register 2.
pub const GPIOR2: *mut u8 = 0x35 as *mut u8;

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
/// | EEPE | 10 |
/// | EERIE | 1000 |
/// | EEPM | 110000 |
/// | EERE | 1 |
/// | EEMPE | 100 |
pub const EECR: *mut u8 = 0x3C as *mut u8;

/// EEPROM Data Register.
pub const EEDR: *mut u8 = 0x3D as *mut u8;

/// EEPROM Read/Write Access.
pub const EEAR: *mut u8 = 0x3E as *mut u8;

/// Pin Change Interrupt Mask Register 0.
pub const PCMSK0: *mut u8 = 0x40 as *mut u8;

/// Watchdog Timer Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDIE | 1000000 |
/// | WDIF | 10000000 |
/// | WDE | 1000 |
/// | WDCE | 10000 |
/// | WDP | 100111 |
pub const WDTCR: *mut u8 = 0x41 as *mut u8;

/// Timer/Counter1 Control Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FOC1A | 10000000 |
/// | FOC1B | 1000000 |
pub const TCCR1C: *mut u8 = 0x42 as *mut u8;

/// General Timer Counter Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PSR10 | 1 |
pub const GTCCR: *mut u8 = 0x43 as *mut u8;

/// Timer/Counter1 Input Capture Register  Bytes.
pub const ICR1: *mut u16 = 0x44 as *mut u16;

/// Timer/Counter1 Input Capture Register  Bytes low byte.
pub const ICR1L: *mut u8 = 0x44 as *mut u8;

/// Timer/Counter1 Input Capture Register  Bytes high byte.
pub const ICR1H: *mut u8 = 0x45 as *mut u8;

/// Clock Prescale Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CLKPCE | 10000000 |
/// | CLKPS | 1111 |
pub const CLKPR: *mut u8 = 0x46 as *mut u8;

/// Timer/Counter1 Output Compare Register  Bytes low byte.
pub const OCR1BL: *mut u8 = 0x48 as *mut u8;

/// Timer/Counter1 Output Compare Register  Bytes.
pub const OCR1B: *mut u16 = 0x48 as *mut u16;

/// Timer/Counter1 Output Compare Register  Bytes high byte.
pub const OCR1BH: *mut u8 = 0x49 as *mut u8;

/// Timer/Counter1 Output Compare Register  Bytes low byte.
pub const OCR1AL: *mut u8 = 0x4A as *mut u8;

/// Timer/Counter1 Output Compare Register  Bytes.
pub const OCR1A: *mut u16 = 0x4A as *mut u16;

/// Timer/Counter1 Output Compare Register  Bytes high byte.
pub const OCR1AH: *mut u8 = 0x4B as *mut u8;

/// Timer/Counter1  Bytes low byte.
pub const TCNT1L: *mut u8 = 0x4C as *mut u8;

/// Timer/Counter1  Bytes.
pub const TCNT1: *mut u16 = 0x4C as *mut u16;

/// Timer/Counter1  Bytes high byte.
pub const TCNT1H: *mut u8 = 0x4D as *mut u8;

/// Timer/Counter1 Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ICES1 | 1000000 |
/// | ICNC1 | 10000000 |
/// | CS1 | 111 |
pub const TCCR1B: *mut u8 = 0x4E as *mut u8;

/// Timer/Counter1 Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | COM1A | 11000000 |
/// | COM1B | 110000 |
pub const TCCR1A: *mut u8 = 0x4F as *mut u8;

/// Timer/Counter  Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | COM0B | 110000 |
/// | COM0A | 11000000 |
/// | WGM0 | 11 |
pub const TCCR0A: *mut u8 = 0x50 as *mut u8;

/// Oscillator Calibration Register.
pub const OSCCAL: *mut u8 = 0x51 as *mut u8;

/// Timer/Counter0.
pub const TCNT0: *mut u8 = 0x52 as *mut u8;

/// Timer/Counter Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CS0 | 111 |
/// | WGM02 | 1000 |
/// | FOC0B | 1000000 |
/// | FOC0A | 10000000 |
pub const TCCR0B: *mut u8 = 0x53 as *mut u8;

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
/// | ISC1 | 1100 |
/// | ISC0 | 11 |
/// | PUD | 10000000 |
/// | SE | 100000 |
/// | SM | 1010000 |
pub const MCUCR: *mut u8 = 0x55 as *mut u8;

/// Timer/Counter0 Output Compare Register.
pub const OCR0A: *mut u8 = 0x56 as *mut u8;

/// Store Program Memory Control and Status register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SPMEN | 1 |
/// | RFLB | 1000 |
/// | PGERS | 10 |
/// | CTPB | 10000 |
/// | PGWRT | 100 |
pub const SPMCSR: *mut u8 = 0x57 as *mut u8;

/// Timer/Counter Interrupt Flag register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCF1B | 100000 |
/// | OCF1A | 1000000 |
/// | TOV1 | 10000000 |
/// | ICF1 | 1000 |
pub const TIFR: *mut u8 = 0x58 as *mut u8;

/// Timer/Counter Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ICIE1 | 1000 |
/// | OCIE1B | 100000 |
/// | OCIE1A | 1000000 |
/// | TOIE1 | 10000000 |
pub const TIMSK: *mut u8 = 0x59 as *mut u8;

/// General Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | INTF | 11000000 |
/// | PCIF | 111000 |
pub const GIFR: *mut u8 = 0x5A as *mut u8;

/// General Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | INT | 11000000 |
/// | PCIE | 100000 |
pub const GIMSK: *mut u8 = 0x5B as *mut u8;

/// Timer/Counter0 Output Compare Register.
pub const OCR0B: *mut u8 = 0x5C as *mut u8;

/// Stack Pointer.
pub const SP: *mut u16 = 0x5D as *mut u16;

/// Stack Pointer  low byte.
pub const SPL: *mut u8 = 0x5D as *mut u8;

/// Stack Pointer  high byte.
pub const SPH: *mut u8 = 0x5E as *mut u8;

/// Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | S | 10000 |
/// | T | 1000000 |
/// | H | 100000 |
/// | V | 1000 |
/// | I | 10000000 |
/// | N | 100 |
/// | Z | 10 |
/// | C | 1 |
pub const SREG: *mut u8 = 0x5F as *mut u8;

/// Bitfield on register `ACSR`
pub const ACO: u8 = 0x20;

/// Bitfield on register `ACSR`
pub const ACD: u8 = 0x80;

/// Bitfield on register `ACSR`
pub const ACIE: u8 = 0x8;

/// Bitfield on register `ACSR`
pub const ACI: u8 = 0x10;

/// Bitfield on register `ACSR`
pub const ACIC: u8 = 0x4;

/// Bitfield on register `ACSR`
pub const ACBG: u8 = 0x40;

/// Bitfield on register `ACSR`
pub const ACIS: u8 = 0x3;

/// Bitfield on register `BODCR`
pub const BPDSE: u8 = 0x1;

/// Bitfield on register `BODCR`
pub const BPDS: u8 = 0x2;

/// Bitfield on register `CLKPR`
pub const CLKPCE: u8 = 0x80;

/// Bitfield on register `CLKPR`
pub const CLKPS: u8 = 0xF;

/// Bitfield on register `EECR`
pub const EEPE: u8 = 0x2;

/// Bitfield on register `EECR`
pub const EERIE: u8 = 0x8;

/// Bitfield on register `EECR`
pub const EEPM: u8 = 0x30;

/// Bitfield on register `EECR`
pub const EERE: u8 = 0x1;

/// Bitfield on register `EECR`
pub const EEMPE: u8 = 0x4;

/// Bitfield on register `EXTENDED`
pub const SELFPRGEN: u8 = 0x1;

/// Bitfield on register `GIFR`
pub const INTF: u8 = 0xC0;

/// Bitfield on register `GIFR`
pub const PCIF: u8 = 0x38;

/// Bitfield on register `GIMSK`
pub const INT: u8 = 0xC0;

/// Bitfield on register `GIMSK`
pub const PCIE: u8 = 0x20;

/// Bitfield on register `GTCCR`
pub const PSR10: u8 = 0x1;

/// Bitfield on register `HIGH`
pub const DWEN: u8 = 0x80;

/// Bitfield on register `HIGH`
pub const WDTON: u8 = 0x10;

/// Bitfield on register `HIGH`
pub const EESAVE: u8 = 0x40;

/// Bitfield on register `HIGH`
pub const RSTDISBL: u8 = 0x1;

/// Bitfield on register `HIGH`
pub const BODLEVEL: u8 = 0xE;

/// Bitfield on register `HIGH`
pub const SPIEN: u8 = 0x20;

/// Bitfield on register `LOCKBIT`
pub const LB: u8 = 0x3;

/// Bitfield on register `LOW`
pub const CKDIV8: u8 = 0x80;

/// Bitfield on register `LOW`
pub const SUT_CKSEL: u8 = 0x3F;

/// Bitfield on register `LOW`
pub const CKOUT: u8 = 0x40;

/// Bitfield on register `MCUCR`
pub const ISC1: u8 = 0xC;

/// Bitfield on register `MCUCR`
pub const ISC0: u8 = 0x3;

/// Bitfield on register `MCUCR`
pub const PUD: u8 = 0x80;

/// Bitfield on register `MCUCR`
pub const SE: u8 = 0x20;

/// Bitfield on register `MCUCR`
pub const SM: u8 = 0x50;

/// Bitfield on register `MCUSR`
pub const EXTRF: u8 = 0x2;

/// Bitfield on register `MCUSR`
pub const WDRF: u8 = 0x8;

/// Bitfield on register `MCUSR`
pub const PORF: u8 = 0x1;

/// Bitfield on register `MCUSR`
pub const BORF: u8 = 0x4;

/// Bitfield on register `PRR`
pub const PRTIM: u8 = 0xC;

/// Bitfield on register `PRR`
pub const PRUSART: u8 = 0x1;

/// Bitfield on register `PRR`
pub const PRUSI: u8 = 0x2;

/// Bitfield on register `SPMCSR`
pub const SPMEN: u8 = 0x1;

/// Bitfield on register `SPMCSR`
pub const RFLB: u8 = 0x8;

/// Bitfield on register `SPMCSR`
pub const PGERS: u8 = 0x2;

/// Bitfield on register `SPMCSR`
pub const CTPB: u8 = 0x10;

/// Bitfield on register `SPMCSR`
pub const PGWRT: u8 = 0x4;

/// Bitfield on register `SREG`
pub const S: u8 = 0x10;

/// Bitfield on register `SREG`
pub const T: u8 = 0x40;

/// Bitfield on register `SREG`
pub const H: u8 = 0x20;

/// Bitfield on register `SREG`
pub const V: u8 = 0x8;

/// Bitfield on register `SREG`
pub const I: u8 = 0x80;

/// Bitfield on register `SREG`
pub const N: u8 = 0x4;

/// Bitfield on register `SREG`
pub const Z: u8 = 0x2;

/// Bitfield on register `SREG`
pub const C: u8 = 0x1;

/// Bitfield on register `TCCR0A`
pub const COM0B: u8 = 0x30;

/// Bitfield on register `TCCR0A`
pub const COM0A: u8 = 0xC0;

/// Bitfield on register `TCCR0A`
pub const WGM0: u8 = 0x3;

/// Bitfield on register `TCCR0B`
pub const CS0: u8 = 0x7;

/// Bitfield on register `TCCR0B`
pub const WGM02: u8 = 0x8;

/// Bitfield on register `TCCR0B`
pub const FOC0B: u8 = 0x40;

/// Bitfield on register `TCCR0B`
pub const FOC0A: u8 = 0x80;

/// Bitfield on register `TCCR1A`
pub const COM1A: u8 = 0xC0;

/// Bitfield on register `TCCR1A`
pub const COM1B: u8 = 0x30;

/// Bitfield on register `TCCR1B`
pub const ICES1: u8 = 0x40;

/// Bitfield on register `TCCR1B`
pub const ICNC1: u8 = 0x80;

/// Bitfield on register `TCCR1B`
pub const CS1: u8 = 0x7;

/// Bitfield on register `TCCR1C`
pub const FOC1A: u8 = 0x80;

/// Bitfield on register `TCCR1C`
pub const FOC1B: u8 = 0x40;

/// Bitfield on register `TIFR`
pub const OCF1B: u8 = 0x20;

/// Bitfield on register `TIFR`
pub const OCF1A: u8 = 0x40;

/// Bitfield on register `TIFR`
pub const TOV1: u8 = 0x80;

/// Bitfield on register `TIFR`
pub const ICF1: u8 = 0x8;

/// Bitfield on register `TIMSK`
pub const ICIE1: u8 = 0x8;

/// Bitfield on register `TIMSK`
pub const OCIE1B: u8 = 0x20;

/// Bitfield on register `TIMSK`
pub const OCIE1A: u8 = 0x40;

/// Bitfield on register `TIMSK`
pub const TOIE1: u8 = 0x80;

/// Bitfield on register `UCSRA`
pub const DOR: u8 = 0x8;

/// Bitfield on register `UCSRA`
pub const MPCM: u8 = 0x1;

/// Bitfield on register `UCSRA`
pub const UDRE: u8 = 0x20;

/// Bitfield on register `UCSRA`
pub const UPE: u8 = 0x4;

/// Bitfield on register `UCSRA`
pub const TXC: u8 = 0x40;

/// Bitfield on register `UCSRA`
pub const RXC: u8 = 0x80;

/// Bitfield on register `UCSRA`
pub const U2X: u8 = 0x2;

/// Bitfield on register `UCSRA`
pub const FE: u8 = 0x10;

/// Bitfield on register `UCSRB`
pub const TXCIE: u8 = 0x40;

/// Bitfield on register `UCSRB`
pub const RXEN: u8 = 0x10;

/// Bitfield on register `UCSRB`
pub const TXEN: u8 = 0x8;

/// Bitfield on register `UCSRB`
pub const UCSZ2: u8 = 0x4;

/// Bitfield on register `UCSRB`
pub const TXB8: u8 = 0x1;

/// Bitfield on register `UCSRB`
pub const UDRIE: u8 = 0x20;

/// Bitfield on register `UCSRB`
pub const RXB8: u8 = 0x2;

/// Bitfield on register `UCSRB`
pub const RXCIE: u8 = 0x80;

/// Bitfield on register `UCSRC`
pub const UMSEL: u8 = 0xC0;

/// Bitfield on register `UCSRC`
pub const UPM: u8 = 0x30;

/// Bitfield on register `UCSRC`
pub const UCSZ: u8 = 0x6;

/// Bitfield on register `UCSRC`
pub const UCPOL: u8 = 0x1;

/// Bitfield on register `UCSRC`
pub const USBS: u8 = 0x8;

/// Bitfield on register `USICR`
pub const USISIE: u8 = 0x80;

/// Bitfield on register `USICR`
pub const USIOIE: u8 = 0x40;

/// Bitfield on register `USICR`
pub const USIWM: u8 = 0x30;

/// Bitfield on register `USICR`
pub const USITC: u8 = 0x1;

/// Bitfield on register `USICR`
pub const USICLK: u8 = 0x2;

/// Bitfield on register `USICR`
pub const USICS: u8 = 0xC;

/// Bitfield on register `USISR`
pub const USIPF: u8 = 0x20;

/// Bitfield on register `USISR`
pub const USIOIF: u8 = 0x40;

/// Bitfield on register `USISR`
pub const USICNT: u8 = 0xF;

/// Bitfield on register `USISR`
pub const USISIF: u8 = 0x80;

/// Bitfield on register `USISR`
pub const USIDC: u8 = 0x10;

/// Bitfield on register `WDTCR`
pub const WDIE: u8 = 0x40;

/// Bitfield on register `WDTCR`
pub const WDIF: u8 = 0x80;

/// Bitfield on register `WDTCR`
pub const WDE: u8 = 0x8;

/// Bitfield on register `WDTCR`
pub const WDCE: u8 = 0x10;

/// Bitfield on register `WDTCR`
pub const WDP: u8 = 0x27;

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

/// `COMM_STOP_BIT_SEL` value group
#[allow(non_upper_case_globals)]
pub mod comm_stop_bit_sel {
   /// 1-bit.
   pub const VAL_0x00: u32 = 0x0;
   /// 2-bit.
   pub const VAL_0x01: u32 = 0x1;
}

/// `COMM_UPM_PARITY_MODE` value group
#[allow(non_upper_case_globals)]
pub mod comm_upm_parity_mode {
   /// Disabled.
   pub const VAL_0x00: u32 = 0x0;
   /// Reserved.
   pub const VAL_0x01: u32 = 0x1;
   /// Enabled, Even Parity.
   pub const VAL_0x02: u32 = 0x2;
   /// Enabled, Odd Parity.
   pub const VAL_0x03: u32 = 0x3;
}

/// `COMM_USART_MODE` value group
#[allow(non_upper_case_globals)]
pub mod comm_usart_mode {
   /// Asynchronous Operation.
   pub const VAL_0x00: u32 = 0x0;
   /// Synchronous Operation.
   pub const VAL_0x01: u32 = 0x1;
   /// Master SPI.
   pub const VAL_0x03: u32 = 0x3;
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

/// `CPU_CLK_PRESCALE_4_BITS_SMALL` value group
#[allow(non_upper_case_globals)]
pub mod cpu_clk_prescale_4_bits_small {
   /// 1.
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
   /// 256.
   pub const VAL_0x08: u32 = 0x8;
}

/// `CPU_SLEEP_MODE3` value group
#[allow(non_upper_case_globals)]
pub mod cpu_sleep_mode3 {
   /// Idle.
   pub const IDLE: u32 = 0x0;
   /// Power Down.
   pub const PDOWN2: u32 = 0x1;
   /// Standby.
   pub const STDBY: u32 = 0x4;
   /// Power Down.
   pub const PDOWN: u32 = 0x5;
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

/// `ENUM_EESAVE_BODLEVEL` value group
#[allow(non_upper_case_globals)]
pub mod enum_eesave_bodlevel {
   /// Brown-out detection at VCC=4.3 V.
   pub const _4V3: u32 = 0x4;
   /// Brown-out detection at VCC=2.7 V.
   pub const _2V7: u32 = 0x5;
   /// Brown-out detection at VCC=1.8 V.
   pub const _1V8: u32 = 0x6;
   /// Brown-out detection disabled.
   pub const DISABLED: u32 = 0x7;
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
   /// Ext. Clock; Start-up time: 14 CK + 0   ms.
   pub const EXTCLK_14CK_0MS: u32 = 0x0;
   /// Ext. Clock; Start-up time: 14 CK + 4.1 ms.
   pub const EXTCLK_14CK_4MS1: u32 = 0x10;
   /// Ext. Clock; Start-up time: 14 CK + 65  ms.
   pub const EXTCLK_14CK_65MS: u32 = 0x20;
   /// Int. RC Osc. 4 MHz; Start-up time: 14 CK + 0   ms.
   pub const INTRCOSC_4MHZ_14CK_0MS: u32 = 0x2;
   /// Int. RC Osc. 4 MHz; Start-up time: 14 CK + 4.1 ms.
   pub const INTRCOSC_4MHZ_14CK_4MS1: u32 = 0x12;
   /// Int. RC Osc. 4 MHz; Start-up time: 14 CK + 65  ms.
   pub const INTRCOSC_4MHZ_14CK_65MS: u32 = 0x22;
   /// Int. RC Osc. 8 MHz; Start-up time: 14 CK + 0   ms.
   pub const INTRCOSC_8MHZ_14CK_0MS: u32 = 0x4;
   /// Int. RC Osc. 8 MHz; Start-up time: 14 CK + 4.1 ms.
   pub const INTRCOSC_8MHZ_14CK_4MS1: u32 = 0x14;
   /// Int. RC Osc. 8 MHz; Start-up time: 14 CK + 65  ms.
   pub const INTRCOSC_8MHZ_14CK_65MS: u32 = 0x24;
   /// Int. RC Osc. 128 kHz; Start-up time: 14 CK + 0 ms.
   pub const INTRCOSC_128KHZ_14CK_0MS: u32 = 0x6;
   /// Int. RC Osc. 128 kHz; Start-up time: 14 CK + 4 ms.
   pub const INTRCOSC_128KHZ_14CK_4MS: u32 = 0x16;
   /// Int. RC Osc. 128 kHz; Start-up time: 14 CK + 64 ms.
   pub const INTRCOSC_128KHZ_14CK_64MS: u32 = 0x26;
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
   /// 8 MHz.
   pub const _8_MHz: u32 = 0x0;
   /// 4 MHz.
   pub const _4_MHz: u32 = 0x1;
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

