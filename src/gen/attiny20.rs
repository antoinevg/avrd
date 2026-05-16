//! The AVR ATtiny20 microcontroller
//!
//! # Variants
//! |        | Pinout | Mcu age | Operating temperature | Operating voltage | Max speed |
//! |--------|--------|---------|-----------------------|-------------------|-----------|
//! | ATtiny20-UUR | WLCSP | WLCSP12 | -40°C - 85°C | 1.8V - 5.5V | 12 MHz |
//! | ATtiny20-SSU | DIP | SOIC14 | -40°C - 85°C | 1.8V - 5.5V | 12 MHz |
//! | ATtiny20-XU | DIP | TSSOP14 | -40°C - 85°C | 1.8V - 5.5V | 12 MHz |
//! | ATtiny20-CCU | UFBGA | UFBGA15 | -40°C - 85°C | 1.8V - 5.5V | 12 MHz |
//! | ATtiny20-MMH | VQFN | VQFN20 | -40°C - 85°C | 1.8V - 5.5V | 12 MHz |
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

/// `BYTE0` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CKOUT | 100 |
/// | WDTON | 10 |
/// | BODLEVEL | 1110000 |
/// | RSTDISBL | 1 |
pub const BYTE0: *mut u8 = 0x0 as *mut u8;

/// Port A Input Pins.
pub const PINA: *mut u8 = 0x0 as *mut u8;

/// Data Direction Register, Port A.
pub const DDRA: *mut u8 = 0x1 as *mut u8;

/// Port A Data Register.
pub const PORTA: *mut u8 = 0x2 as *mut u8;

/// Pull-up Enable Control Register.
pub const PUEA: *mut u8 = 0x3 as *mut u8;

/// Port B Data register.
pub const PINB: *mut u8 = 0x4 as *mut u8;

/// Data Direction Register, Port B.
pub const DDRB: *mut u8 = 0x5 as *mut u8;

/// Input Pins, Port B.
pub const PORTB: *mut u8 = 0x6 as *mut u8;

/// Pull-up Enable Control Register.
pub const PUEB: *mut u8 = 0x7 as *mut u8;

/// Port Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BBMA | 1 |
pub const PORTCR: *mut u8 = 0x8 as *mut u8;

/// Pin Change Mask Register 0.
pub const PCMSK0: *mut u8 = 0x9 as *mut u8;

/// Pin Change Mask Register 1.
pub const PCMSK1: *mut u8 = 0xA as *mut u8;

/// General Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | INTF0 | 1 |
/// | PCIF | 110000 |
pub const GIFR: *mut u8 = 0xB as *mut u8;

/// General Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | INT0 | 1 |
/// | PCIE | 110000 |
pub const GIMSK: *mut u8 = 0xC as *mut u8;

/// Digital Input Disable Register 0.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADC5D | 100000 |
/// | ADC4D | 10000 |
/// | ADC6D | 1000000 |
/// | ADC7D | 10000000 |
/// | ADC3D | 1000 |
/// | ADC1D | 10 |
/// | ADC2D | 100 |
/// | ADC0D | 1 |
pub const DIDR0: *mut u8 = 0xD as *mut u8;

/// ADC Data Register  Bytes low byte.
pub const ADCL: *mut u8 = 0xE as *mut u8;

/// ADC Data Register  Bytes.
pub const ADC: *mut u16 = 0xE as *mut u16;

/// ADC Data Register  Bytes high byte.
pub const ADCH: *mut u8 = 0xF as *mut u8;

/// The ADC multiplexer Selection Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | REFS | 1000000 |
/// | MUX | 1111 |
pub const ADMUX: *mut u8 = 0x10 as *mut u8;

/// ADC Control and Status Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADLAR | 1000 |
/// | ADTS | 111 |
pub const ADCSRB: *mut u8 = 0x11 as *mut u8;

/// The ADC Control and Status register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADSC | 1000000 |
/// | ADEN | 10000000 |
/// | ADPS | 111 |
/// | ADATE | 100000 |
/// | ADIF | 10000 |
/// | ADIE | 1000 |
pub const ADCSRA: *mut u8 = 0x12 as *mut u8;

/// Analog Comparator Control And Status Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | HLEV | 1000000 |
/// | HSEL | 10000000 |
/// | ACME | 100 |
pub const ACSRB: *mut u8 = 0x13 as *mut u8;

/// Analog Comparator Control And Status Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ACO | 100000 |
/// | ACBG | 1000000 |
/// | ACIS | 11 |
/// | ACI | 10000 |
/// | ACD | 10000000 |
/// | ACIE | 1000 |
/// | ACIC | 100 |
pub const ACSRA: *mut u8 = 0x14 as *mut u8;

/// Timer/Counter0 Output Compare Register.
pub const OCR0B: *mut u8 = 0x15 as *mut u8;

/// Timer/Counter0 Output Compare Register.
pub const OCR0A: *mut u8 = 0x16 as *mut u8;

/// Timer/Counter0.
pub const TCNT0: *mut u8 = 0x17 as *mut u8;

/// Timer/Counter 0 Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WGM02 | 1000 |
/// | CS0 | 111 |
/// | FOC0B | 1000000 |
/// | FOC0A | 10000000 |
pub const TCCR0B: *mut u8 = 0x18 as *mut u8;

/// Timer/Counter 0 Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WGM0 | 11 |
/// | COM0B | 110000 |
/// | COM0A | 11000000 |
pub const TCCR0A: *mut u8 = 0x19 as *mut u8;

/// Input Capture Register  Bytes.
pub const ICR1: *mut u16 = 0x1A as *mut u16;

/// Input Capture Register  Bytes low byte.
pub const ICR1L: *mut u8 = 0x1A as *mut u8;

/// Input Capture Register  Bytes high byte.
pub const ICR1H: *mut u8 = 0x1B as *mut u8;

/// Timer/Counter1 Output Compare Register B.
pub const OCR1B: *mut u16 = 0x1C as *mut u16;

/// Timer/Counter1 Output Compare Register B  low byte.
pub const OCR1BL: *mut u8 = 0x1C as *mut u8;

/// Timer/Counter1 Output Compare Register B  high byte.
pub const OCR1BH: *mut u8 = 0x1D as *mut u8;

/// Timer/Counter 1 Output Compare Register A.
pub const OCR1A: *mut u16 = 0x1E as *mut u16;

/// Timer/Counter 1 Output Compare Register A  low byte.
pub const OCR1AL: *mut u8 = 0x1E as *mut u8;

/// Timer/Counter 1 Output Compare Register A  high byte.
pub const OCR1AH: *mut u8 = 0x1F as *mut u8;

/// Timer/Counter1.
pub const TCNT1: *mut u16 = 0x20 as *mut u16;

/// Timer/Counter1  low byte.
pub const TCNT1L: *mut u8 = 0x20 as *mut u8;

/// Timer/Counter1  high byte.
pub const TCNT1H: *mut u8 = 0x21 as *mut u8;

/// Timer/Counter1 Control Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FOC1B | 1000000 |
/// | FOC1A | 10000000 |
pub const TCCR1C: *mut u8 = 0x22 as *mut u8;

/// Timer/Counter1 Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CS1 | 111 |
/// | ICES1 | 1000000 |
/// | ICNC1 | 10000000 |
pub const TCCR1B: *mut u8 = 0x23 as *mut u8;

/// Timer/Counter1 Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | COM1B | 110000 |
/// | COM1A | 11000000 |
pub const TCCR1A: *mut u8 = 0x24 as *mut u8;

/// Overflow Interrupt Enable.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCF1A | 10000 |
/// | OCF1B | 100000 |
/// | TOV | 1001 |
/// | ICF1 | 10000000 |
/// | OCF0A | 10 |
/// | OCF0B | 100 |
pub const TIFR: *mut u8 = 0x25 as *mut u8;

/// Timer Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TOIE | 1001 |
/// | OCIE0B | 100 |
/// | OCIE1A | 10000 |
/// | ICIE1 | 10000000 |
/// | OCIE1B | 100000 |
/// | OCIE0A | 10 |
pub const TIMSK: *mut u8 = 0x26 as *mut u8;

/// General Timer/Counter Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TSM | 10000000 |
/// | PSR | 1 |
pub const GTCCR: *mut u8 = 0x27 as *mut u8;

/// TWI Slave Data Register.
pub const TWSD: *mut u8 = 0x28 as *mut u8;

/// TWI Slave Address Mask Register.
pub const TWSAM: *mut u8 = 0x29 as *mut u8;

/// TWI Slave Address Register.
pub const TWSA: *mut u8 = 0x2A as *mut u8;

/// TWI Slave Status Register A.
pub const TWSSRA: *mut u8 = 0x2B as *mut u8;

/// TWI Slave Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TWCMD | 11 |
/// | TWAA | 100 |
pub const TWSCRB: *mut u8 = 0x2C as *mut u8;

/// TWI Slave Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TWEN | 1000 |
/// | TWSIE | 100 |
/// | TWPME | 10 |
/// | TWSME | 1 |
/// | TWSHE | 10000000 |
/// | TWASIE | 10000 |
/// | TWDIE | 100000 |
pub const TWSCRA: *mut u8 = 0x2D as *mut u8;

/// SPI Data Register.
pub const SPDR: *mut u8 = 0x2E as *mut u8;

/// SPI Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WCOL | 1000000 |
/// | SPI2X | 1 |
/// | SPIF | 10000000 |
pub const SPSR: *mut u8 = 0x2F as *mut u8;

/// SPI Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SPIE | 10000000 |
/// | SPE | 1000000 |
/// | DORD | 100000 |
/// | MSTR | 10000 |
/// | CPHA | 100 |
/// | SPR | 11 |
/// | CPOL | 1000 |
pub const SPCR: *mut u8 = 0x30 as *mut u8;

/// Watchdog Timer Control and Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDE | 1000 |
/// | WDP | 100111 |
/// | WDIF | 10000000 |
/// | WDIE | 1000000 |
pub const WDTCSR: *mut u8 = 0x31 as *mut u8;

/// Non-Volatile Memory Control and Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | NVMBSY | 10000000 |
pub const NVMCSR: *mut u8 = 0x32 as *mut u8;

/// Non-Volatile Memory Command.
pub const NVMCMD: *mut u8 = 0x33 as *mut u8;

/// Power Reduction Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRADC | 1 |
/// | PRSPI | 1000 |
/// | PRTIM0 | 10 |
/// | PRTWI | 10000 |
/// | PRTIM1 | 100 |
pub const PRR: *mut u8 = 0x35 as *mut u8;

/// Clock Prescale Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CLKPS | 1111 |
pub const CLKPSR: *mut u8 = 0x36 as *mut u8;

/// Clock Main Settings Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CLKMS | 11 |
pub const CLKMSR: *mut u8 = 0x37 as *mut u8;

/// Oscillator Calibration Value.
pub const OSCCAL: *mut u8 = 0x39 as *mut u8;

/// MCU Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ISC01 | 10000000 |
/// | ISC00 | 1000000 |
pub const MCUCR: *mut u8 = 0x3A as *mut u8;

/// Reset Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PORF | 1 |
/// | EXTRF | 10 |
/// | WDRF | 1000 |
pub const RSTFLR: *mut u8 = 0x3B as *mut u8;

/// Configuration Change Protection.
pub const CCP: *mut u8 = 0x3C as *mut u8;

/// Stack Pointer.
pub const SP: *mut u16 = 0x3D as *mut u16;

/// Stack Pointer  low byte.
pub const SPL: *mut u8 = 0x3D as *mut u8;

/// Stack Pointer  high byte.
pub const SPH: *mut u8 = 0x3E as *mut u8;

/// Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | C | 1 |
/// | N | 100 |
/// | Z | 10 |
/// | H | 100000 |
/// | I | 10000000 |
/// | V | 1000 |
/// | T | 1000000 |
/// | S | 10000 |
pub const SREG: *mut u8 = 0x3F as *mut u8;

/// Bitfield on register `ACSRA`
pub const ACO: u8 = 0x20;

/// Bitfield on register `ACSRA`
pub const ACBG: u8 = 0x40;

/// Bitfield on register `ACSRA`
pub const ACIS: u8 = 0x3;

/// Bitfield on register `ACSRA`
pub const ACI: u8 = 0x10;

/// Bitfield on register `ACSRA`
pub const ACD: u8 = 0x80;

/// Bitfield on register `ACSRA`
pub const ACIE: u8 = 0x8;

/// Bitfield on register `ACSRA`
pub const ACIC: u8 = 0x4;

/// Bitfield on register `ACSRB`
pub const HLEV: u8 = 0x40;

/// Bitfield on register `ACSRB`
pub const HSEL: u8 = 0x80;

/// Bitfield on register `ACSRB`
pub const ACME: u8 = 0x4;

/// Bitfield on register `ADCSRA`
pub const ADSC: u8 = 0x40;

/// Bitfield on register `ADCSRA`
pub const ADEN: u8 = 0x80;

/// Bitfield on register `ADCSRA`
pub const ADPS: u8 = 0x7;

/// Bitfield on register `ADCSRA`
pub const ADATE: u8 = 0x20;

/// Bitfield on register `ADCSRA`
pub const ADIF: u8 = 0x10;

/// Bitfield on register `ADCSRA`
pub const ADIE: u8 = 0x8;

/// Bitfield on register `ADCSRB`
pub const ADLAR: u8 = 0x8;

/// Bitfield on register `ADCSRB`
pub const ADTS: u8 = 0x7;

/// Bitfield on register `ADMUX`
pub const REFS: u8 = 0x40;

/// Bitfield on register `ADMUX`
pub const MUX: u8 = 0xF;

/// Bitfield on register `BYTE0`
pub const CKOUT: u8 = 0x4;

/// Bitfield on register `BYTE0`
pub const WDTON: u8 = 0x2;

/// Bitfield on register `BYTE0`
pub const BODLEVEL: u8 = 0x70;

/// Bitfield on register `BYTE0`
pub const RSTDISBL: u8 = 0x1;

/// Bitfield on register `CLKMSR`
pub const CLKMS: u8 = 0x3;

/// Bitfield on register `CLKPSR`
pub const CLKPS: u8 = 0xF;

/// Bitfield on register `DIDR0`
pub const ADC5D: u8 = 0x20;

/// Bitfield on register `DIDR0`
pub const ADC4D: u8 = 0x10;

/// Bitfield on register `DIDR0`
pub const ADC6D: u8 = 0x40;

/// Bitfield on register `DIDR0`
pub const ADC7D: u8 = 0x80;

/// Bitfield on register `DIDR0`
pub const ADC3D: u8 = 0x8;

/// Bitfield on register `DIDR0`
pub const ADC1D: u8 = 0x2;

/// Bitfield on register `DIDR0`
pub const ADC2D: u8 = 0x4;

/// Bitfield on register `DIDR0`
pub const ADC0D: u8 = 0x1;

/// Bitfield on register `GIFR`
pub const INTF0: u8 = 0x1;

/// Bitfield on register `GIFR`
pub const PCIF: u8 = 0x30;

/// Bitfield on register `GIMSK`
pub const INT0: u8 = 0x1;

/// Bitfield on register `GIMSK`
pub const PCIE: u8 = 0x30;

/// Bitfield on register `GTCCR`
pub const TSM: u8 = 0x80;

/// Bitfield on register `GTCCR`
pub const PSR: u8 = 0x1;

/// Bitfield on register `LOCKBIT`
pub const LB: u8 = 0x3;

/// Bitfield on register `MCUCR`
pub const ISC01: u8 = 0x80;

/// Bitfield on register `MCUCR`
pub const ISC00: u8 = 0x40;

/// Bitfield on register `NVMCSR`
pub const NVMBSY: u8 = 0x80;

/// Bitfield on register `PORTCR`
pub const BBMA: u8 = 0x1;

/// Bitfield on register `PRR`
pub const PRADC: u8 = 0x1;

/// Bitfield on register `PRR`
pub const PRSPI: u8 = 0x8;

/// Bitfield on register `PRR`
pub const PRTIM0: u8 = 0x2;

/// Bitfield on register `PRR`
pub const PRTWI: u8 = 0x10;

/// Bitfield on register `PRR`
pub const PRTIM1: u8 = 0x4;

/// Bitfield on register `RSTFLR`
pub const PORF: u8 = 0x1;

/// Bitfield on register `RSTFLR`
pub const EXTRF: u8 = 0x2;

/// Bitfield on register `RSTFLR`
pub const WDRF: u8 = 0x8;

/// Bitfield on register `SPCR`
pub const SPIE: u8 = 0x80;

/// Bitfield on register `SPCR`
pub const SPE: u8 = 0x40;

/// Bitfield on register `SPCR`
pub const DORD: u8 = 0x20;

/// Bitfield on register `SPCR`
pub const MSTR: u8 = 0x10;

/// Bitfield on register `SPCR`
pub const CPHA: u8 = 0x4;

/// Bitfield on register `SPCR`
pub const SPR: u8 = 0x3;

/// Bitfield on register `SPCR`
pub const CPOL: u8 = 0x8;

/// Bitfield on register `SPSR`
pub const WCOL: u8 = 0x40;

/// Bitfield on register `SPSR`
pub const SPI2X: u8 = 0x1;

/// Bitfield on register `SPSR`
pub const SPIF: u8 = 0x80;

/// Bitfield on register `SREG`
pub const C: u8 = 0x1;

/// Bitfield on register `SREG`
pub const N: u8 = 0x4;

/// Bitfield on register `SREG`
pub const Z: u8 = 0x2;

/// Bitfield on register `SREG`
pub const H: u8 = 0x20;

/// Bitfield on register `SREG`
pub const I: u8 = 0x80;

/// Bitfield on register `SREG`
pub const V: u8 = 0x8;

/// Bitfield on register `SREG`
pub const T: u8 = 0x40;

/// Bitfield on register `SREG`
pub const S: u8 = 0x10;

/// Bitfield on register `TCCR0A`
pub const WGM0: u8 = 0x3;

/// Bitfield on register `TCCR0A`
pub const COM0B: u8 = 0x30;

/// Bitfield on register `TCCR0A`
pub const COM0A: u8 = 0xC0;

/// Bitfield on register `TCCR0B`
pub const WGM02: u8 = 0x8;

/// Bitfield on register `TCCR0B`
pub const CS0: u8 = 0x7;

/// Bitfield on register `TCCR0B`
pub const FOC0B: u8 = 0x40;

/// Bitfield on register `TCCR0B`
pub const FOC0A: u8 = 0x80;

/// Bitfield on register `TCCR1A`
pub const COM1B: u8 = 0x30;

/// Bitfield on register `TCCR1A`
pub const COM1A: u8 = 0xC0;

/// Bitfield on register `TCCR1B`
pub const CS1: u8 = 0x7;

/// Bitfield on register `TCCR1B`
pub const ICES1: u8 = 0x40;

/// Bitfield on register `TCCR1B`
pub const ICNC1: u8 = 0x80;

/// Bitfield on register `TCCR1C`
pub const FOC1B: u8 = 0x40;

/// Bitfield on register `TCCR1C`
pub const FOC1A: u8 = 0x80;

/// Bitfield on register `TIFR`
pub const OCF1A: u8 = 0x10;

/// Bitfield on register `TIFR`
pub const OCF1B: u8 = 0x20;

/// Bitfield on register `TIFR`
pub const TOV: u8 = 0x9;

/// Bitfield on register `TIFR`
pub const ICF1: u8 = 0x80;

/// Bitfield on register `TIFR`
pub const OCF0A: u8 = 0x2;

/// Bitfield on register `TIFR`
pub const OCF0B: u8 = 0x4;

/// Bitfield on register `TIMSK`
pub const TOIE: u8 = 0x9;

/// Bitfield on register `TIMSK`
pub const OCIE0B: u8 = 0x4;

/// Bitfield on register `TIMSK`
pub const OCIE1A: u8 = 0x10;

/// Bitfield on register `TIMSK`
pub const ICIE1: u8 = 0x80;

/// Bitfield on register `TIMSK`
pub const OCIE1B: u8 = 0x20;

/// Bitfield on register `TIMSK`
pub const OCIE0A: u8 = 0x2;

/// Bitfield on register `TWSCRA`
pub const TWEN: u8 = 0x8;

/// Bitfield on register `TWSCRA`
pub const TWSIE: u8 = 0x4;

/// Bitfield on register `TWSCRA`
pub const TWPME: u8 = 0x2;

/// Bitfield on register `TWSCRA`
pub const TWSME: u8 = 0x1;

/// Bitfield on register `TWSCRA`
pub const TWSHE: u8 = 0x80;

/// Bitfield on register `TWSCRA`
pub const TWASIE: u8 = 0x10;

/// Bitfield on register `TWSCRA`
pub const TWDIE: u8 = 0x20;

/// Bitfield on register `TWSCRB`
pub const TWCMD: u8 = 0x3;

/// Bitfield on register `TWSCRB`
pub const TWAA: u8 = 0x4;

/// Bitfield on register `WDTCSR`
pub const WDE: u8 = 0x8;

/// Bitfield on register `WDTCSR`
pub const WDP: u8 = 0x27;

/// Bitfield on register `WDTCSR`
pub const WDIF: u8 = 0x80;

/// Bitfield on register `WDTCSR`
pub const WDIE: u8 = 0x40;

/// `ADC_MUX` value group
#[allow(non_upper_case_globals)]
pub mod adc_mux {
   /// ADC0.
   pub const ADC0: u32 = 0x0;
   /// ADC1.
   pub const ADC1: u32 = 0x1;
   /// ADC2.
   pub const ADC2: u32 = 0x2;
   /// ADC3.
   pub const ADC3: u32 = 0x3;
   /// ADC4.
   pub const ADC4: u32 = 0x4;
   /// ADC5.
   pub const ADC5: u32 = 0x5;
   /// ADC6.
   pub const ADC6: u32 = 0x6;
   /// ADC7.
   pub const ADC7: u32 = 0x7;
   /// 0V (AGND).
   pub const ADC_GND: u32 = 0x8;
   /// 1.1V Internal Reference.
   pub const ADC_VBG: u32 = 0x9;
   /// Temperature sensor.
   pub const TEMPSENS: u32 = 0xA;
}

/// `ANALOG_ADC_AUTO_TRIGGER3` value group
#[allow(non_upper_case_globals)]
pub mod analog_adc_auto_trigger3 {
   /// Free Running mode.
   pub const VAL_0x00: u32 = 0x0;
   /// Analog Comparator.
   pub const VAL_0x01: u32 = 0x1;
   /// External Interrupt Request 0.
   pub const VAL_0x02: u32 = 0x2;
   /// Timer/Counter0 Compare Match A.
   pub const VAL_0x03: u32 = 0x3;
   /// Timer/Counter0 Overflow.
   pub const VAL_0x04: u32 = 0x4;
   /// Timer/Counter1 Compare Match B.
   pub const VAL_0x05: u32 = 0x5;
   /// Timer/Counter1 Overflow.
   pub const VAL_0x06: u32 = 0x6;
   /// Watchdog Interrupt Request.
   pub const VAL_0x07: u32 = 0x7;
}

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
   /// Vcc used as internal reference.
   pub const VAL_0x00: u32 = 0x0;
   /// Internal 1.1V Referemce.
   pub const VAL_0x01: u32 = 0x1;
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

/// `COMM_SCK_RATE` value group
#[allow(non_upper_case_globals)]
pub mod comm_sck_rate {
   /// fcl/4.
   pub const VAL_0x00: u32 = 0x0;
   /// fcl/16.
   pub const VAL_0x01: u32 = 0x1;
   /// fcl/64.
   pub const VAL_0x02: u32 = 0x2;
   /// fcl/128.
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

/// Sleep Mode
#[allow(non_upper_case_globals)]
pub mod cpu_sleep_mode {
   /// Idle.
   pub const IDLE: u32 = 0x0;
   /// ADC noise reduction.
   pub const ADC: u32 = 0x1;
   /// Power-down.
   pub const PDOWN: u32 = 0x2;
   /// Standby.
   pub const STDBY: u32 = 0x4;
}

/// `ENUM_BODLEVEL` value group
#[allow(non_upper_case_globals)]
pub mod enum_bodlevel {
   /// Brown-out detection disabled; \[BODLEVEL=111\].
   pub const DISABLED: u32 = 0x7;
   /// Brown-out detection at VCC=1.8 V.
   pub const _1V8: u32 = 0x6;
   /// Brown-out detection at VCC=2.7 V.
   pub const _2V7: u32 = 0x5;
   /// Brown-out detection at VCC=4.3 V.
   pub const _4V3: u32 = 0x4;
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

/// Interrupt Sense Control
#[allow(non_upper_case_globals)]
pub mod interrupt_sense_control {
   /// Low Level of INTX.
   pub const VAL_0x00: u32 = 0x0;
   /// Any logical change on INTX.
   pub const VAL_0x01: u32 = 0x1;
   /// Falling Edge of INTX.
   pub const VAL_0x02: u32 = 0x2;
   /// Rising Edge of INTX.
   pub const VAL_0x03: u32 = 0x3;
}

/// Oscillator Calibration Values
#[allow(non_upper_case_globals)]
pub mod osccal_value_addresses {
   /// 8.0 MHz.
   pub const _8_0_MHz: u32 = 0x0;
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

