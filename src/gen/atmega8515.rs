//! The AVR ATmega8515 microcontroller
//!
//! # Variants
//! |        | Pinout | Mcu age | Operating temperature | Operating voltage | Max speed |
//! |--------|--------|---------|-----------------------|-------------------|-----------|
//! | standard |  |  | 0°C - 0°C | 2.7V - 5.5V | 0 MHz |
//!

#![allow(non_upper_case_globals)]

/// `LOCKBIT` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LB | 11 |
/// | BLB1 | 110000 |
/// | BLB0 | 1100 |
pub const LOCKBIT: *mut u8 = 0x0 as *mut u8;

/// `LOW` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BODLEVEL | 10000000 |
/// | BODEN | 1000000 |
/// | SUT_CKSEL | 111111 |
pub const LOW: *mut u8 = 0x0 as *mut u8;

/// `HIGH` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CKOPT | 10000 |
/// | BOOTRST | 1 |
/// | BOOTSZ | 110 |
/// | S8515C | 10000000 |
/// | EESAVE | 1000 |
/// | WDTON | 1000000 |
/// | SPIEN | 100000 |
pub const HIGH: *mut u8 = 0x1 as *mut u8;

/// Oscillator Calibration Value.
pub const OSCCAL: *mut u8 = 0x24 as *mut u8;

/// Port E Input Pins.
pub const PINE: *mut u8 = 0x25 as *mut u8;

/// Port E Data Direction Register.
pub const DDRE: *mut u8 = 0x26 as *mut u8;

/// Port E Data Register.
pub const PORTE: *mut u8 = 0x27 as *mut u8;

/// Analog Comparator Control And Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ACI | 10000 |
/// | ACIC | 100 |
/// | ACIS | 11 |
/// | ACD | 10000000 |
/// | ACO | 100000 |
/// | ACIE | 1000 |
/// | ACBG | 1000000 |
pub const ACSR: *mut u8 = 0x28 as *mut u8;

/// USART Baud Rate Register Low Byte.
pub const UBRRL: *mut u8 = 0x29 as *mut u8;

/// USART Control and Status Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | UCSZ2 | 100 |
/// | RXCIE | 10000000 |
/// | RXEN | 10000 |
/// | TXEN | 1000 |
/// | TXB8 | 1 |
/// | UDRIE | 100000 |
/// | RXB8 | 10 |
/// | TXCIE | 1000000 |
pub const UCSRB: *mut u8 = 0x2A as *mut u8;

/// USART Control and Status Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TXC | 1000000 |
/// | DOR | 1000 |
/// | U2X | 10 |
/// | FE | 10000 |
/// | UDRE | 100000 |
/// | RXC | 10000000 |
/// | MPCM | 1 |
/// | UPE | 100 |
pub const UCSRA: *mut u8 = 0x2B as *mut u8;

/// USART I/O Data Register.
pub const UDR: *mut u8 = 0x2C as *mut u8;

/// SPI Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SPE | 1000000 |
/// | CPOL | 1000 |
/// | DORD | 100000 |
/// | SPR | 11 |
/// | CPHA | 100 |
/// | SPIE | 10000000 |
/// | MSTR | 10000 |
pub const SPCR: *mut u8 = 0x2D as *mut u8;

/// SPI Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SPI2X | 1 |
/// | WCOL | 1000000 |
/// | SPIF | 10000000 |
pub const SPSR: *mut u8 = 0x2E as *mut u8;

/// SPI Data Register.
pub const SPDR: *mut u8 = 0x2F as *mut u8;

/// Port D Input Pins.
pub const PIND: *mut u8 = 0x30 as *mut u8;

/// Port D Data Direction Register.
pub const DDRD: *mut u8 = 0x31 as *mut u8;

/// Port D Data Register.
pub const PORTD: *mut u8 = 0x32 as *mut u8;

/// Port C Input Pins.
pub const PINC: *mut u8 = 0x33 as *mut u8;

/// Port C Data Direction Register.
pub const DDRC: *mut u8 = 0x34 as *mut u8;

/// Port C Data Register.
pub const PORTC: *mut u8 = 0x35 as *mut u8;

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
/// | EEMWE | 100 |
/// | EERIE | 1000 |
/// | EEWE | 10 |
/// | EERE | 1 |
pub const EECR: *mut u8 = 0x3C as *mut u8;

/// EEPROM Data Register.
pub const EEDR: *mut u8 = 0x3D as *mut u8;

/// EEPROM Address Register  Bytes.
pub const EEAR: *mut u16 = 0x3E as *mut u16;

/// EEPROM Address Register  Bytes low byte.
pub const EEARL: *mut u8 = 0x3E as *mut u8;

/// EEPROM Address Register  Bytes high byte.
pub const EEARH: *mut u8 = 0x3F as *mut u8;

/// USART Baud Rate Register High Byte.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | UBRR1 | 1100 |
/// | UBRR | 11 |
pub const UBRRH: *mut u8 = 0x40 as *mut u8;

/// USART Control and Status Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | USBS | 1000 |
/// | UPM | 110000 |
/// | UCSZ | 110 |
/// | UMSEL | 1000000 |
/// | UCPOL | 1 |
pub const UCSRC: *mut u8 = 0x40 as *mut u8;

/// Watchdog Timer Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDP | 111 |
/// | WDE | 1000 |
/// | WDCE | 10000 |
pub const WDTCR: *mut u8 = 0x41 as *mut u8;

/// Timer/Counter1 Input Capture Register  Bytes low byte.
pub const ICR1L: *mut u8 = 0x44 as *mut u8;

/// Timer/Counter1 Input Capture Register  Bytes.
pub const ICR1: *mut u16 = 0x44 as *mut u16;

/// Timer/Counter1 Input Capture Register  Bytes high byte.
pub const ICR1H: *mut u8 = 0x45 as *mut u8;

/// Timer/Counter1 Output Compare Register B  Bytes.
pub const OCR1B: *mut u16 = 0x48 as *mut u16;

/// Timer/Counter1 Output Compare Register B  Bytes low byte.
pub const OCR1BL: *mut u8 = 0x48 as *mut u8;

/// Timer/Counter1 Output Compare Register B  Bytes high byte.
pub const OCR1BH: *mut u8 = 0x49 as *mut u8;

/// Timer/Counter1 Output Compare Register A  Bytes.
pub const OCR1A: *mut u16 = 0x4A as *mut u16;

/// Timer/Counter1 Output Compare Register A  Bytes low byte.
pub const OCR1AL: *mut u8 = 0x4A as *mut u8;

/// Timer/Counter1 Output Compare Register A  Bytes high byte.
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
/// | CS1 | 111 |
/// | ICES1 | 1000000 |
/// | ICNC1 | 10000000 |
pub const TCCR1B: *mut u8 = 0x4E as *mut u8;

/// Timer/Counter1 Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | COM1B | 110000 |
/// | COM1A | 11000000 |
/// | FOC1A | 1000 |
/// | FOC1B | 100 |
pub const TCCR1A: *mut u8 = 0x4F as *mut u8;

/// Special Function IO Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | XMBK | 1000000 |
/// | PUD | 100 |
/// | XMM | 111000 |
/// | PSR10 | 1 |
pub const SFIOR: *mut u8 = 0x50 as *mut u8;

/// Timer/Counter 0 Output Compare Register.
pub const OCR0: *mut u8 = 0x51 as *mut u8;

/// Timer/Counter 0 Register.
pub const TCNT0: *mut u8 = 0x52 as *mut u8;

/// Timer/Counter 0 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | COM0 | 110000 |
/// | WGM01 | 1000 |
/// | WGM00 | 1000000 |
/// | FOC0 | 10000000 |
/// | CS0 | 111 |
pub const TCCR0: *mut u8 = 0x53 as *mut u8;

/// MCU Control And Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BORF | 100 |
/// | PORF | 1 |
/// | EXTRF | 10 |
/// | WDRF | 1000 |
/// | SM2 | 100000 |
pub const MCUCSR: *mut u8 = 0x54 as *mut u8;

/// MCU Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SM1 | 10000 |
/// | SRE | 10000000 |
/// | ISC1 | 1100 |
/// | SE | 100000 |
/// | ISC0 | 11 |
/// | SRW10 | 1000000 |
pub const MCUCR: *mut u8 = 0x55 as *mut u8;

/// Extended MCU Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SM0 | 10000000 |
/// | ISC2 | 1 |
/// | SRW11 | 10 |
/// | SRW0 | 1100 |
/// | SRL | 1110000 |
pub const EMCUCR: *mut u8 = 0x56 as *mut u8;

/// Store Program Memory Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PGWRT | 100 |
/// | SPMEN | 1 |
/// | PGERS | 10 |
/// | RWWSRE | 10000 |
/// | SPMIE | 10000000 |
/// | BLBSET | 1000 |
/// | RWWSB | 1000000 |
pub const SPMCR: *mut u8 = 0x57 as *mut u8;

/// Timer/Counter Interrupt Flag register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCF1A | 1000000 |
/// | TOV1 | 10000000 |
/// | OCF1B | 100000 |
/// | ICF1 | 1000 |
pub const TIFR: *mut u8 = 0x58 as *mut u8;

/// Timer/Counter Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TOIE1 | 10000000 |
/// | OCIE1A | 1000000 |
/// | OCIE1B | 100000 |
/// | TICIE1 | 1000 |
pub const TIMSK: *mut u8 = 0x59 as *mut u8;

/// General Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | INTF | 11000000 |
/// | INTF2 | 100000 |
pub const GIFR: *mut u8 = 0x5A as *mut u8;

/// General Interrupt Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | IVCE | 1 |
/// | INT2 | 100000 |
/// | INT1 | 10000000 |
/// | INT0 | 1000000 |
/// | IVSEL | 10 |
pub const GICR: *mut u8 = 0x5B as *mut u8;

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
/// | C | 1 |
/// | V | 1000 |
/// | Z | 10 |
/// | H | 100000 |
/// | N | 100 |
/// | T | 1000000 |
/// | S | 10000 |
/// | I | 10000000 |
pub const SREG: *mut u8 = 0x5F as *mut u8;

/// Bitfield on register `ACSR`
pub const ACI: u8 = 0x10;

/// Bitfield on register `ACSR`
pub const ACIC: u8 = 0x4;

/// Bitfield on register `ACSR`
pub const ACIS: u8 = 0x3;

/// Bitfield on register `ACSR`
pub const ACD: u8 = 0x80;

/// Bitfield on register `ACSR`
pub const ACO: u8 = 0x20;

/// Bitfield on register `ACSR`
pub const ACIE: u8 = 0x8;

/// Bitfield on register `ACSR`
pub const ACBG: u8 = 0x40;

/// Bitfield on register `EECR`
pub const EEMWE: u8 = 0x4;

/// Bitfield on register `EECR`
pub const EERIE: u8 = 0x8;

/// Bitfield on register `EECR`
pub const EEWE: u8 = 0x2;

/// Bitfield on register `EECR`
pub const EERE: u8 = 0x1;

/// Bitfield on register `EMCUCR`
pub const SM0: u8 = 0x80;

/// Bitfield on register `EMCUCR`
pub const ISC2: u8 = 0x1;

/// Bitfield on register `EMCUCR`
pub const SRW11: u8 = 0x2;

/// Bitfield on register `EMCUCR`
pub const SRW0: u8 = 0xC;

/// Bitfield on register `EMCUCR`
pub const SRL: u8 = 0x70;

/// Bitfield on register `GICR`
pub const IVCE: u8 = 0x1;

/// Bitfield on register `GICR`
pub const INT2: u8 = 0x20;

/// Bitfield on register `GICR`
pub const INT1: u8 = 0x80;

/// Bitfield on register `GICR`
pub const INT0: u8 = 0x40;

/// Bitfield on register `GICR`
pub const IVSEL: u8 = 0x2;

/// Bitfield on register `GIFR`
pub const INTF: u8 = 0xC0;

/// Bitfield on register `GIFR`
pub const INTF2: u8 = 0x20;

/// Bitfield on register `HIGH`
pub const CKOPT: u8 = 0x10;

/// Bitfield on register `HIGH`
pub const BOOTRST: u8 = 0x1;

/// Bitfield on register `HIGH`
pub const BOOTSZ: u8 = 0x6;

/// Bitfield on register `HIGH`
pub const S8515C: u8 = 0x80;

/// Bitfield on register `HIGH`
pub const EESAVE: u8 = 0x8;

/// Bitfield on register `HIGH`
pub const WDTON: u8 = 0x40;

/// Bitfield on register `HIGH`
pub const SPIEN: u8 = 0x20;

/// Bitfield on register `LOCKBIT`
pub const LB: u8 = 0x3;

/// Bitfield on register `LOCKBIT`
pub const BLB1: u8 = 0x30;

/// Bitfield on register `LOCKBIT`
pub const BLB0: u8 = 0xC;

/// Bitfield on register `LOW`
pub const BODLEVEL: u8 = 0x80;

/// Bitfield on register `LOW`
pub const BODEN: u8 = 0x40;

/// Bitfield on register `LOW`
pub const SUT_CKSEL: u8 = 0x3F;

/// Bitfield on register `MCUCR`
pub const SM1: u8 = 0x10;

/// Bitfield on register `MCUCR`
pub const SRE: u8 = 0x80;

/// Bitfield on register `MCUCR`
pub const ISC1: u8 = 0xC;

/// Bitfield on register `MCUCR`
pub const SE: u8 = 0x20;

/// Bitfield on register `MCUCR`
pub const ISC0: u8 = 0x3;

/// Bitfield on register `MCUCR`
pub const SRW10: u8 = 0x40;

/// Bitfield on register `MCUCSR`
pub const BORF: u8 = 0x4;

/// Bitfield on register `MCUCSR`
pub const PORF: u8 = 0x1;

/// Bitfield on register `MCUCSR`
pub const EXTRF: u8 = 0x2;

/// Bitfield on register `MCUCSR`
pub const WDRF: u8 = 0x8;

/// Bitfield on register `MCUCSR`
pub const SM2: u8 = 0x20;

/// Bitfield on register `SFIOR`
pub const XMBK: u8 = 0x40;

/// Bitfield on register `SFIOR`
pub const PUD: u8 = 0x4;

/// Bitfield on register `SFIOR`
pub const XMM: u8 = 0x38;

/// Bitfield on register `SFIOR`
pub const PSR10: u8 = 0x1;

/// Bitfield on register `SPCR`
pub const SPE: u8 = 0x40;

/// Bitfield on register `SPCR`
pub const CPOL: u8 = 0x8;

/// Bitfield on register `SPCR`
pub const DORD: u8 = 0x20;

/// Bitfield on register `SPCR`
pub const SPR: u8 = 0x3;

/// Bitfield on register `SPCR`
pub const CPHA: u8 = 0x4;

/// Bitfield on register `SPCR`
pub const SPIE: u8 = 0x80;

/// Bitfield on register `SPCR`
pub const MSTR: u8 = 0x10;

/// Bitfield on register `SPMCR`
pub const PGWRT: u8 = 0x4;

/// Bitfield on register `SPMCR`
pub const SPMEN: u8 = 0x1;

/// Bitfield on register `SPMCR`
pub const PGERS: u8 = 0x2;

/// Bitfield on register `SPMCR`
pub const RWWSRE: u8 = 0x10;

/// Bitfield on register `SPMCR`
pub const SPMIE: u8 = 0x80;

/// Bitfield on register `SPMCR`
pub const BLBSET: u8 = 0x8;

/// Bitfield on register `SPMCR`
pub const RWWSB: u8 = 0x40;

/// Bitfield on register `SPSR`
pub const SPI2X: u8 = 0x1;

/// Bitfield on register `SPSR`
pub const WCOL: u8 = 0x40;

/// Bitfield on register `SPSR`
pub const SPIF: u8 = 0x80;

/// Bitfield on register `SREG`
pub const C: u8 = 0x1;

/// Bitfield on register `SREG`
pub const V: u8 = 0x8;

/// Bitfield on register `SREG`
pub const Z: u8 = 0x2;

/// Bitfield on register `SREG`
pub const H: u8 = 0x20;

/// Bitfield on register `SREG`
pub const N: u8 = 0x4;

/// Bitfield on register `SREG`
pub const T: u8 = 0x40;

/// Bitfield on register `SREG`
pub const S: u8 = 0x10;

/// Bitfield on register `SREG`
pub const I: u8 = 0x80;

/// Bitfield on register `TCCR0`
pub const COM0: u8 = 0x30;

/// Bitfield on register `TCCR0`
pub const WGM01: u8 = 0x8;

/// Bitfield on register `TCCR0`
pub const WGM00: u8 = 0x40;

/// Bitfield on register `TCCR0`
pub const FOC0: u8 = 0x80;

/// Bitfield on register `TCCR0`
pub const CS0: u8 = 0x7;

/// Bitfield on register `TCCR1A`
pub const COM1B: u8 = 0x30;

/// Bitfield on register `TCCR1A`
pub const COM1A: u8 = 0xC0;

/// Bitfield on register `TCCR1A`
pub const FOC1A: u8 = 0x8;

/// Bitfield on register `TCCR1A`
pub const FOC1B: u8 = 0x4;

/// Bitfield on register `TCCR1B`
pub const CS1: u8 = 0x7;

/// Bitfield on register `TCCR1B`
pub const ICES1: u8 = 0x40;

/// Bitfield on register `TCCR1B`
pub const ICNC1: u8 = 0x80;

/// Bitfield on register `TIFR`
pub const OCF1A: u8 = 0x40;

/// Bitfield on register `TIFR`
pub const TOV1: u8 = 0x80;

/// Bitfield on register `TIFR`
pub const OCF1B: u8 = 0x20;

/// Bitfield on register `TIFR`
pub const ICF1: u8 = 0x8;

/// Bitfield on register `TIMSK`
pub const TOIE1: u8 = 0x80;

/// Bitfield on register `TIMSK`
pub const OCIE1A: u8 = 0x40;

/// Bitfield on register `TIMSK`
pub const OCIE1B: u8 = 0x20;

/// Bitfield on register `TIMSK`
pub const TICIE1: u8 = 0x8;

/// Bitfield on register `UBRRH`
pub const UBRR1: u8 = 0xC;

/// Bitfield on register `UBRRH`
pub const UBRR: u8 = 0x3;

/// Bitfield on register `UCSRA`
pub const TXC: u8 = 0x40;

/// Bitfield on register `UCSRA`
pub const DOR: u8 = 0x8;

/// Bitfield on register `UCSRA`
pub const U2X: u8 = 0x2;

/// Bitfield on register `UCSRA`
pub const FE: u8 = 0x10;

/// Bitfield on register `UCSRA`
pub const UDRE: u8 = 0x20;

/// Bitfield on register `UCSRA`
pub const RXC: u8 = 0x80;

/// Bitfield on register `UCSRA`
pub const MPCM: u8 = 0x1;

/// Bitfield on register `UCSRA`
pub const UPE: u8 = 0x4;

/// Bitfield on register `UCSRB`
pub const UCSZ2: u8 = 0x4;

/// Bitfield on register `UCSRB`
pub const RXCIE: u8 = 0x80;

/// Bitfield on register `UCSRB`
pub const RXEN: u8 = 0x10;

/// Bitfield on register `UCSRB`
pub const TXEN: u8 = 0x8;

/// Bitfield on register `UCSRB`
pub const TXB8: u8 = 0x1;

/// Bitfield on register `UCSRB`
pub const UDRIE: u8 = 0x20;

/// Bitfield on register `UCSRB`
pub const RXB8: u8 = 0x2;

/// Bitfield on register `UCSRB`
pub const TXCIE: u8 = 0x40;

/// Bitfield on register `UCSRC`
pub const USBS: u8 = 0x8;

/// Bitfield on register `UCSRC`
pub const UPM: u8 = 0x30;

/// Bitfield on register `UCSRC`
pub const UCSZ: u8 = 0x6;

/// Bitfield on register `UCSRC`
pub const UMSEL: u8 = 0x40;

/// Bitfield on register `UCSRC`
pub const UCPOL: u8 = 0x1;

/// Bitfield on register `WDTCR`
pub const WDP: u8 = 0x7;

/// Bitfield on register `WDTCR`
pub const WDE: u8 = 0x8;

/// Bitfield on register `WDTCR`
pub const WDCE: u8 = 0x10;

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
}

/// `CPU_SECTOR_LIMITS` value group
#[allow(non_upper_case_globals)]
pub mod cpu_sector_limits {
   /// LS = N/A, US = 0x1100 - 0xFFFF.
   pub const VAL_0x00: u32 = 0x0;
   /// LS = 0x1100 - 0x1FFF, US = 0x2000 - 0xFFFF.
   pub const VAL_0x01: u32 = 0x1;
   /// LS = 0x1100 - 0x3FFF, US = 0x4000 - 0xFFFF.
   pub const VAL_0x02: u32 = 0x2;
   /// LS = 0x1100 - 0x5FFF, US = 0x6000 - 0xFFFF.
   pub const VAL_0x03: u32 = 0x3;
   /// LS = 0x1100 - 0x7FFF, US = 0x8000 - 0xFFFF.
   pub const VAL_0x04: u32 = 0x4;
   /// LS = 0x1100 - 0x9FFF, US = 0xA000 - 0xFFFF.
   pub const VAL_0x05: u32 = 0x5;
   /// LS = 0x1100 - 0xBFFF, US = 0xC000 - 0xFFFF.
   pub const VAL_0x06: u32 = 0x6;
   /// LS = 0x1100 - 0xDFFF, US = 0xE000 - 0xFFFF.
   pub const VAL_0x07: u32 = 0x7;
}

/// `CPU_WAIT_STATES` value group
#[allow(non_upper_case_globals)]
pub mod cpu_wait_states {
   /// No wait-states.
   pub const VAL_0x00: u32 = 0x0;
   /// Wait one cycle during read/write strobe.
   pub const VAL_0x01: u32 = 0x1;
   /// Wait two cycles during read/write strobe.
   pub const VAL_0x02: u32 = 0x2;
   /// Wait two cycles during read/write and wait one cycle before driving out new address.
   pub const VAL_0x03: u32 = 0x3;
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

/// `ENUM_BODLEVEL` value group
#[allow(non_upper_case_globals)]
pub mod enum_bodlevel {
   /// Brown-out detection at VCC=4.0 V.
   pub const _4V0: u32 = 0x0;
   /// Brown-out detection at VCC=2.7 V.
   pub const _2V7: u32 = 0x1;
}

/// `ENUM_BOOTSZ` value group
#[allow(non_upper_case_globals)]
pub mod enum_bootsz {
   /// Boot Flashsize=128 words Boot address=$0F80.
   pub const _128W_0F80: u32 = 0x3;
   /// Boot Flash size=256 words Boot address=$0F00.
   pub const _256W_0F00: u32 = 0x2;
   /// Boot Flash size=512 words Boot address=$0E00.
   pub const _512W_0E00: u32 = 0x1;
   /// Boot Flash size=1024 words Boot address=$0C00.
   pub const _1024W_0C00: u32 = 0x0;
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
   /// Ext. Clock; Start-up time: 6 CK + 0 ms.
   pub const EXTCLK_6CK_0MS: u32 = 0x0;
   /// Ext. Clock; Start-up time: 6 CK + 4 ms.
   pub const EXTCLK_6CK_4MS: u32 = 0x10;
   /// Ext. Clock; Start-up time: 6 CK + 64 ms.
   pub const EXTCLK_6CK_64MS: u32 = 0x20;
   /// Int. RC Osc. 1 MHz; Start-up time: 6 CK + 0 ms.
   pub const INTRCOSC_1MHZ_6CK_0MS: u32 = 0x1;
   /// Int. RC Osc. 1 MHz; Start-up time: 6 CK + 4 ms.
   pub const INTRCOSC_1MHZ_6CK_4MS: u32 = 0x11;
   /// Int. RC Osc. 1 MHz; Start-up time: 6 CK + 64 ms.
   pub const INTRCOSC_1MHZ_6CK_64MS: u32 = 0x21;
   /// Int. RC Osc. 2 MHz; Start-up time: 6 CK + 0 ms.
   pub const INTRCOSC_2MHZ_6CK_0MS: u32 = 0x2;
   /// Int. RC Osc. 2 MHz; Start-up time: 6 CK + 4 ms.
   pub const INTRCOSC_2MHZ_6CK_4MS: u32 = 0x12;
   /// Int. RC Osc. 2 MHz; Start-up time: 6 CK + 64 ms.
   pub const INTRCOSC_2MHZ_6CK_64MS: u32 = 0x22;
   /// Int. RC Osc. 4 MHz; Start-up time: 6 CK + 0 ms.
   pub const INTRCOSC_4MHZ_6CK_0MS: u32 = 0x3;
   /// Int. RC Osc. 4 MHz; Start-up time: 6 CK + 4 ms.
   pub const INTRCOSC_4MHZ_6CK_4MS: u32 = 0x13;
   /// Int. RC Osc. 4 MHz; Start-up time: 6 CK + 64 ms.
   pub const INTRCOSC_4MHZ_6CK_64MS: u32 = 0x23;
   /// Int. RC Osc. 8 MHz; Start-up time: 6 CK + 0 ms.
   pub const INTRCOSC_8MHZ_6CK_0MS: u32 = 0x4;
   /// Int. RC Osc. 8 MHz; Start-up time: 6 CK + 4 ms.
   pub const INTRCOSC_8MHZ_6CK_4MS: u32 = 0x14;
   /// Int. RC Osc. 8 MHz; Start-up time: 6 CK + 64 ms.
   pub const INTRCOSC_8MHZ_6CK_64MS: u32 = 0x24;
   /// Ext. RC Osc.         -  0.9 MHz; Start-up time: 18 CK + 0 ms.
   pub const EXTRCOSC_XX_0MHZ9_18CK_0MS: u32 = 0x5;
   /// Ext. RC Osc.         -  0.9 MHz; Start-up time: 18 CK + 4 ms.
   pub const EXTRCOSC_XX_0MHZ9_18CK_4MS: u32 = 0x15;
   /// Ext. RC Osc.         -  0.9 MHz; Start-up time: 18 CK + 64 ms.
   pub const EXTRCOSC_XX_0MHZ9_18CK_64MS: u32 = 0x25;
   /// Ext. RC Osc.         -  0.9 MHz; Start-up time: 6 CK + 4 ms.
   pub const EXTRCOSC_XX_0MHZ9_6CK_4MS: u32 = 0x35;
   /// Ext. RC Osc. 0.9 MHz -  3.0 MHz; Start-up time: 18 CK + 0 ms.
   pub const EXTRCOSC_0MHZ9_3MHZ_18CK_0MS: u32 = 0x6;
   /// Ext. RC Osc. 0.9 MHz -  3.0 MHz; Start-up time: 18 CK + 4 ms.
   pub const EXTRCOSC_0MHZ9_3MHZ_18CK_4MS: u32 = 0x16;
   /// Ext. RC Osc. 0.9 MHz -  3.0 MHz; Start-up time: 18 CK + 64 ms.
   pub const EXTRCOSC_0MHZ9_3MHZ_18CK_64MS: u32 = 0x26;
   /// Ext. RC Osc. 0.9 MHz -  3.0 MHz; Start-up time: 6 CK + 4 ms.
   pub const EXTRCOSC_0MHZ9_3MHZ_6CK_4MS: u32 = 0x36;
   /// Ext. RC Osc. 3.0 MHz -  8.0 MHz; Start-up time: 18 CK + 0 ms.
   pub const EXTRCOSC_3MHZ_8MHZ_18CK_0MS: u32 = 0x7;
   /// Ext. RC Osc. 3.0 MHz -  8.0 MHz; Start-up time: 18 CK + 4 ms.
   pub const EXTRCOSC_3MHZ_8MHZ_18CK_4MS: u32 = 0x17;
   /// Ext. RC Osc. 3.0 MHz -  8.0 MHz; Start-up time: 18 CK + 64 ms.
   pub const EXTRCOSC_3MHZ_8MHZ_18CK_64MS: u32 = 0x27;
   /// Ext. RC Osc. 3.0 MHz -  8.0 MHz; Start-up time: 6 CK + 4 ms.
   pub const EXTRCOSC_3MHZ_8MHZ_6CK_4MS: u32 = 0x37;
   /// Ext. RC Osc. 8.0 MHz - 12.0 MHz; Start-up time: 18 CK + 0 ms.
   pub const EXTRCOSC_8MHZ_12MHZ_18CK_0MS: u32 = 0x8;
   /// Ext. RC Osc. 8.0 MHz - 12.0 MHz; Start-up time: 18 CK + 4 ms.
   pub const EXTRCOSC_8MHZ_12MHZ_18CK_4MS: u32 = 0x18;
   /// Ext. RC Osc. 8.0 MHz - 12.0 MHz; Start-up time: 18 CK + 64 ms.
   pub const EXTRCOSC_8MHZ_12MHZ_18CK_64MS: u32 = 0x28;
   /// Ext. RC Osc. 8.0 MHz - 12.0 MHz; Start-up time: 6 CK + 4 ms.
   pub const EXTRCOSC_8MHZ_12MHZ_6CK_4MS: u32 = 0x38;
   /// Ext. Low-Freq. Crystal; Start-up time: 1K CK + 4 ms.
   pub const EXTLOFXTAL_1KCK_4MS: u32 = 0x9;
   /// Ext. Low-Freq. Crystal; Start-up time: 1K CK + 64 ms.
   pub const EXTLOFXTAL_1KCK_64MS: u32 = 0x19;
   /// Ext. Low-Freq. Crystal; Start-up time: 32K CK + 64 ms.
   pub const EXTLOFXTAL_32KCK_64MS: u32 = 0x29;
   /// Ext. Crystal/Resonator Low Freq.; Start-up time: 258 CK + 4 ms.
   pub const EXTLOFXTALRES_258CK_4MS: u32 = 0xA;
   /// Ext. Crystal/Resonator Low Freq.; Start-up time: 258 CK + 64 ms.
   pub const EXTLOFXTALRES_258CK_64MS: u32 = 0x1A;
   /// Ext. Crystal/Resonator Low Freq.; Start-up time: 1K CK + 0 ms.
   pub const EXTLOFXTALRES_1KCK_0MS: u32 = 0x2A;
   /// Ext. Crystal/Resonator Low Freq.; Start-up time: 1K CK + 4 ms.
   pub const EXTLOFXTALRES_1KCK_4MS: u32 = 0x3A;
   /// Ext. Crystal/Resonator Low Freq.; Start-up time: 1K CK + 64 ms.
   pub const EXTLOFXTALRES_1KCK_64MS: u32 = 0xB;
   /// Ext. Crystal/Resonator Low Freq.; Start-up time: 16K CK + 0 ms.
   pub const EXTLOFXTALRES_16KCK_0MS: u32 = 0x1B;
   /// Ext. Crystal/Resonator Low Freq.; Start-up time: 16K CK + 4 ms.
   pub const EXTLOFXTALRES_16KCK_4MS: u32 = 0x2B;
   /// Ext. Crystal/Resonator Low Freq.; Start-up time: 16K CK + 64 ms.
   pub const EXTLOFXTALRES_16KCK_64MS: u32 = 0x3B;
   /// Ext. Crystal/Resonator Medium Freq.; Start-up time: 258 CK + 4 ms.
   pub const EXTMEDFXTALRES_258CK_4MS: u32 = 0xC;
   /// Ext. Crystal/Resonator Medium Freq.; Start-up time: 258 CK + 64 ms.
   pub const EXTMEDFXTALRES_258CK_64MS: u32 = 0x1C;
   /// Ext. Crystal/Resonator Medium Freq.; Start-up time: 1K CK + 0 ms.
   pub const EXTMEDFXTALRES_1KCK_0MS: u32 = 0x2C;
   /// Ext. Crystal/Resonator Medium Freq.; Start-up time: 1K CK + 4 ms.
   pub const EXTMEDFXTALRES_1KCK_4MS: u32 = 0x3C;
   /// Ext. Crystal/Resonator Medium Freq.; Start-up time: 1K CK + 64 ms.
   pub const EXTMEDFXTALRES_1KCK_64MS: u32 = 0xD;
   /// Ext. Crystal/Resonator Medium Freq.; Start-up time: 16K CK + 0 ms.
   pub const EXTMEDFXTALRES_16KCK_0MS: u32 = 0x1D;
   /// Ext. Crystal/Resonator Medium Freq.; Start-up time: 16K CK + 4 ms.
   pub const EXTMEDFXTALRES_16KCK_4MS: u32 = 0x2D;
   /// Ext. Crystal/Resonator Medium Freq.; Start-up time: 16K CK + 64 ms.
   pub const EXTMEDFXTALRES_16KCK_64MS: u32 = 0x3D;
   /// Ext. Crystal/Resonator High Freq.; Start-up time: 258 CK + 4 ms.
   pub const EXTHIFXTALRES_258CK_4MS: u32 = 0xE;
   /// Ext. Crystal/Resonator High Freq.; Start-up time: 258 CK + 64 ms.
   pub const EXTHIFXTALRES_258CK_64MS: u32 = 0x1E;
   /// Ext. Crystal/Resonator High Freq.; Start-up time: 1K CK + 0 ms.
   pub const EXTHIFXTALRES_1KCK_0MS: u32 = 0x2E;
   /// Ext. Crystal/Resonator High Freq.; Start-up time: 1K CK + 4 ms.
   pub const EXTHIFXTALRES_1KCK_4MS: u32 = 0x3E;
   /// Ext. Crystal/Resonator High Freq.; Start-up time: 1K CK + 64 ms.
   pub const EXTHIFXTALRES_1KCK_64MS: u32 = 0xF;
   /// Ext. Crystal/Resonator High Freq.; Start-up time: 16K CK + 0 ms.
   pub const EXTHIFXTALRES_16KCK_0MS: u32 = 0x1F;
   /// Ext. Crystal/Resonator High Freq.; Start-up time: 16K CK + 4 ms.
   pub const EXTHIFXTALRES_16KCK_4MS: u32 = 0x2F;
   /// Ext. Crystal/Resonator High Freq.; Start-up time: 16K CK + 64 ms.
   pub const EXTHIFXTALRES_16KCK_64MS: u32 = 0x3F;
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
   /// 1 Mhz.
   pub const _1_Mhz: u32 = 0x0;
   /// 2 Mhz.
   pub const _2_Mhz: u32 = 0x1;
   /// 4 Mhz.
   pub const _4_Mhz: u32 = 0x2;
   /// 8 Mhz.
   pub const _8_Mhz: u32 = 0x3;
}

/// `WAVEFORM_GEN_MODE` value group
#[allow(non_upper_case_globals)]
pub mod waveform_gen_mode {
   /// Normal.
   pub const VAL_0x00: u32 = 0x0;
   /// PWM, Phase Correct.
   pub const VAL_0x02: u32 = 0x2;
   /// CTC.
   pub const VAL_0x01: u32 = 0x1;
   /// Fast PWM.
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

