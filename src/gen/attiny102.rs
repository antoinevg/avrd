//! The AVR ATtiny102 microcontroller
//!
//! # Variants
//! |        | Pinout | Mcu age | Operating temperature | Operating voltage | Max speed |
//! |--------|--------|---------|-----------------------|-------------------|-----------|
//! | ATtiny102-M7R  | SOIC-8 | UDFN8 | -40°C - 105°C | 1.8V - 5.5V | 12 MHz |
//! | ATtiny102F-M7R | SOIC-8 | UDFN8 | -40°C - 105°C | 1.8V - 5.5V | 12 MHz |
//! | ATtiny102-SSNR  | SOIC-8 | SOIC8 | -40°C - 105°C | 1.8V - 5.5V | 12 MHz |
//! | ATtiny102F-SSNR | SOIC-8 | SOIC8 | -40°C - 105°C | 1.8V - 5.5V | 12 MHz |
//! | ATtiny102-M8R  | SOIC-8 | UDFN8 | -40°C - 125°C | 1.8V - 5.5V | 12 MHz |
//! | ATtiny102F-M8R | SOIC-8 | UDFN8 | -40°C - 125°C | 1.8V - 5.5V | 12 MHz |
//! | ATtiny102-SSFR  | SOIC-8 | SOIC8 | -40°C - 125°C | 1.8V - 5.5V | 12 MHz |
//! | ATtiny102F-SSFR | SOIC-8 | SOIC8 | -40°C - 125°C | 1.8V - 5.5V | 12 MHz |
//!

#![allow(non_upper_case_globals)]

/// `BYTE0` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDTON | 10 |
/// | SELFPROGEN | 1000 |
/// | RSTDISBL | 1 |
/// | CKOUT | 100 |
pub const BYTE0: *mut u8 = 0x0 as *mut u8;

/// `LOCKBIT` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LB | 11 |
pub const LOCKBIT: *mut u8 = 0x0 as *mut u8;

/// Input Pins, Port A.
pub const PINA: *mut u8 = 0x0 as *mut u8;

/// Data Direction Register, Port A.
pub const DDRA: *mut u8 = 0x1 as *mut u8;

/// Port A Data register.
pub const PORTA: *mut u8 = 0x2 as *mut u8;

/// Pull-up Enable Control Register for PORTA.
pub const PUEA: *mut u8 = 0x3 as *mut u8;

/// Input Pins, Port B.
pub const PINB: *mut u8 = 0x4 as *mut u8;

/// Data Direction Register, Port B.
pub const DDRB: *mut u8 = 0x5 as *mut u8;

/// Port B Data register.
pub const PORTB: *mut u8 = 0x6 as *mut u8;

/// Pull-up Enable Control Register for PORTB.
pub const PUEB: *mut u8 = 0x7 as *mut u8;

/// USART I/O Data Register.
pub const UDR: *mut u8 = 0x8 as *mut u8;

/// USART Baud Rate Register  Bytes low byte.
pub const UBRRL: *mut u8 = 0x9 as *mut u8;

/// USART Baud Rate Register  Bytes.
pub const UBRR: *mut u16 = 0x9 as *mut u16;

/// USART Baud Rate Register  Bytes high byte.
pub const UBRRH: *mut u8 = 0xA as *mut u8;

/// USART Control and Status Register D.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RXSIE | 10000000 |
/// | SFDE | 100000 |
/// | RXS | 1000000 |
pub const UCSRD: *mut u8 = 0xB as *mut u8;

/// USART Control and Status Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | UCSZ | 110 |
/// | UCPOL | 1 |
/// | USBS | 1000 |
/// | UPM | 110000 |
/// | UMSEL | 11000000 |
pub const UCSRC: *mut u8 = 0xC as *mut u8;

/// USART Control and Status Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RXB8 | 10 |
/// | TXEN | 1000 |
/// | RXEN | 10000 |
/// | RXCIE | 10000000 |
/// | UCSZ2 | 100 |
/// | TXB8 | 1 |
/// | UDRIE | 100000 |
/// | TXCIE | 1000000 |
pub const UCSRB: *mut u8 = 0xD as *mut u8;

/// USART Control and Status Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | U2X | 10 |
/// | RXC | 10000000 |
/// | DOR | 1000 |
/// | TXC | 1000000 |
/// | UDRE | 100000 |
/// | FE | 10000 |
/// | MPCM | 1 |
/// | UPE | 100 |
pub const UCSRA: *mut u8 = 0xE as *mut u8;

/// Pin Change Mask Register 0.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCINT5 | 100000 |
/// | PCINT6 | 1000000 |
/// | PCINT2 | 100 |
/// | PCINT1 | 10 |
/// | PCINT4 | 10000 |
/// | PCINT7 | 10000000 |
/// | PCINT3 | 1000 |
/// | PCINT0 | 1 |
pub const PCMSK0: *mut u8 = 0xF as *mut u8;

/// Pin Change Mask Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCINT9 | 10 |
/// | PCINT8 | 1 |
/// | PCINT10 | 100 |
/// | PCINT11 | 1000 |
pub const PCMSK1: *mut u8 = 0x10 as *mut u8;

/// Pin Change Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCIF0 | 1 |
/// | PCIF1 | 10 |
pub const PCIFR: *mut u8 = 0x11 as *mut u8;

/// Pin Change Interrupt Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCIE1 | 10 |
/// | PCIE0 | 1 |
pub const PCICR: *mut u8 = 0x12 as *mut u8;

/// External Interrupt Mask register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | INT0 | 1 |
pub const EIMSK: *mut u8 = 0x13 as *mut u8;

/// External Interrupt Flag register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | INTF0 | 1 |
pub const EIFR: *mut u8 = 0x14 as *mut u8;

/// External Interrupt Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ISC01 | 10 |
/// | ISC00 | 1 |
pub const EICRA: *mut u8 = 0x15 as *mut u8;

/// Port Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BBMA | 1 |
pub const PORTCR: *mut u8 = 0x16 as *mut u8;

/// `DIDR0` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AIN1D | 10 |
/// | AIN0D | 1 |
pub const DIDR0: *mut u8 = 0x17 as *mut u8;

/// ADC Data Register Low.
pub const ADCL: *mut u8 = 0x19 as *mut u8;

/// ADC Data Register High.
pub const ADCH: *mut u8 = 0x1A as *mut u8;

/// The ADC multiplexer Selection Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | REFS | 11000000 |
/// | MUX | 111 |
pub const ADMUX: *mut u8 = 0x1B as *mut u8;

/// The ADC Control and Status register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADTS | 111 |
/// | ADLAR | 10000000 |
pub const ADCSRB: *mut u8 = 0x1C as *mut u8;

/// The ADC Control and Status register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADIF | 10000 |
/// | ADSC | 1000000 |
/// | ADEN | 10000000 |
/// | ADPS | 111 |
/// | ADIE | 1000 |
/// | ADATE | 100000 |
pub const ADCSRA: *mut u8 = 0x1D as *mut u8;

/// Analog Comparator Control And Status Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ACPMUX | 1 |
/// | ACOE | 10 |
pub const ACSRB: *mut u8 = 0x1E as *mut u8;

/// Analog Comparator Control And Status Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ACIE | 1000 |
/// | ACO | 100000 |
/// | ACD | 10000000 |
/// | ACIC | 100 |
/// | ACBG | 1000000 |
/// | ACI | 10000 |
/// | ACIS | 11 |
pub const ACSRA: *mut u8 = 0x1F as *mut u8;

/// Input Capture Register  Bytes low byte.
pub const ICR0L: *mut u8 = 0x22 as *mut u8;

/// Input Capture Register  Bytes.
pub const ICR0: *mut u16 = 0x22 as *mut u16;

/// Input Capture Register  Bytes high byte.
pub const ICR0H: *mut u8 = 0x23 as *mut u8;

/// Timer/Counter0 Output Compare Register B.
pub const OCR0B: *mut u16 = 0x24 as *mut u16;

/// Timer/Counter0 Output Compare Register B  low byte.
pub const OCR0BL: *mut u8 = 0x24 as *mut u8;

/// Timer/Counter0 Output Compare Register B  high byte.
pub const OCR0BH: *mut u8 = 0x25 as *mut u8;

/// Timer/Counter 0 Output Compare Register A.
pub const OCR0A: *mut u16 = 0x26 as *mut u16;

/// Timer/Counter 0 Output Compare Register A  low byte.
pub const OCR0AL: *mut u8 = 0x26 as *mut u8;

/// Timer/Counter 0 Output Compare Register A  high byte.
pub const OCR0AH: *mut u8 = 0x27 as *mut u8;

/// Timer/Counter0  low byte.
pub const TCNT0L: *mut u8 = 0x28 as *mut u8;

/// Timer/Counter0.
pub const TCNT0: *mut u16 = 0x28 as *mut u16;

/// Timer/Counter0  high byte.
pub const TCNT0H: *mut u8 = 0x29 as *mut u8;

/// Overflow Interrupt Enable.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCF0B | 100 |
/// | OCF0A | 10 |
/// | TOV0 | 1 |
/// | ICF0 | 100000 |
pub const TIFR0: *mut u8 = 0x2A as *mut u8;

/// Timer Interrupt Mask Register 0.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TOIE0 | 1 |
/// | ICIE0 | 100000 |
/// | OCIE0B | 100 |
/// | OCIE0A | 10 |
pub const TIMSK0: *mut u8 = 0x2B as *mut u8;

/// Timer/Counter 0 Control Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FOC0A | 10000000 |
/// | FOC0B | 1000000 |
pub const TCCR0C: *mut u8 = 0x2C as *mut u8;

/// Timer/Counter 0 Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ICES0 | 1000000 |
/// | ICNC0 | 10000000 |
/// | CS0 | 111 |
pub const TCCR0B: *mut u8 = 0x2D as *mut u8;

/// Timer/Counter 0 Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | COM0A | 11000000 |
/// | COM0B | 110000 |
pub const TCCR0A: *mut u8 = 0x2E as *mut u8;

/// General Timer/Counter Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TSM | 10000000 |
/// | PSR | 1 |
/// | REMAP | 10 |
pub const GTCCR: *mut u8 = 0x2F as *mut u8;

/// Watchdog Timer Control and Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDIE | 1000000 |
/// | WDP | 100111 |
/// | WDE | 1000 |
/// | WDIF | 10000000 |
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

/// Vcc Level Monitoring Control and Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | VLMF | 10000000 |
/// | VLM | 111 |
/// | VLMIE | 1000000 |
pub const VLMCSR: *mut u8 = 0x34 as *mut u8;

/// Power Reduction Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRUSART | 100 |
/// | PRTIM0 | 1 |
/// | PRADC | 10 |
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

/// Sleep Mode Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SE | 1 |
/// | SM | 1110 |
pub const SMCR: *mut u8 = 0x3A as *mut u8;

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

/// Stack Pointer  low byte.
pub const SPL: *mut u8 = 0x3D as *mut u8;

/// Stack Pointer.
pub const SP: *mut u16 = 0x3D as *mut u16;

/// Stack Pointer  high byte.
pub const SPH: *mut u8 = 0x3E as *mut u8;

/// Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | C | 1 |
/// | H | 100000 |
/// | S | 10000 |
/// | T | 1000000 |
/// | V | 1000 |
/// | Z | 10 |
/// | N | 100 |
/// | I | 10000000 |
pub const SREG: *mut u8 = 0x3F as *mut u8;

/// Bitfield on register `ACSRA`
pub const ACIE: u8 = 0x8;

/// Bitfield on register `ACSRA`
pub const ACO: u8 = 0x20;

/// Bitfield on register `ACSRA`
pub const ACD: u8 = 0x80;

/// Bitfield on register `ACSRA`
pub const ACIC: u8 = 0x4;

/// Bitfield on register `ACSRA`
pub const ACBG: u8 = 0x40;

/// Bitfield on register `ACSRA`
pub const ACI: u8 = 0x10;

/// Bitfield on register `ACSRA`
pub const ACIS: u8 = 0x3;

/// Bitfield on register `ACSRB`
pub const ACPMUX: u8 = 0x1;

/// Bitfield on register `ACSRB`
pub const ACOE: u8 = 0x2;

/// Bitfield on register `ADCSRA`
pub const ADIF: u8 = 0x10;

/// Bitfield on register `ADCSRA`
pub const ADSC: u8 = 0x40;

/// Bitfield on register `ADCSRA`
pub const ADEN: u8 = 0x80;

/// Bitfield on register `ADCSRA`
pub const ADPS: u8 = 0x7;

/// Bitfield on register `ADCSRA`
pub const ADIE: u8 = 0x8;

/// Bitfield on register `ADCSRA`
pub const ADATE: u8 = 0x20;

/// Bitfield on register `ADCSRB`
pub const ADTS: u8 = 0x7;

/// Bitfield on register `ADCSRB`
pub const ADLAR: u8 = 0x80;

/// Bitfield on register `ADMUX`
pub const REFS: u8 = 0xC0;

/// Bitfield on register `ADMUX`
pub const MUX: u8 = 0x7;

/// Bitfield on register `BYTE0`
pub const WDTON: u8 = 0x2;

/// Bitfield on register `BYTE0`
pub const SELFPROGEN: u8 = 0x8;

/// Bitfield on register `BYTE0`
pub const RSTDISBL: u8 = 0x1;

/// Bitfield on register `BYTE0`
pub const CKOUT: u8 = 0x4;

/// Bitfield on register `CLKMSR`
pub const CLKMS: u8 = 0x3;

/// Bitfield on register `CLKPSR`
pub const CLKPS: u8 = 0xF;

/// Bitfield on register `DIDR0`
pub const AIN1D: u8 = 0x2;

/// Bitfield on register `DIDR0`
pub const AIN0D: u8 = 0x1;

/// Bitfield on register `EICRA`
pub const ISC01: u8 = 0x2;

/// Bitfield on register `EICRA`
pub const ISC00: u8 = 0x1;

/// Bitfield on register `EIFR`
pub const INTF0: u8 = 0x1;

/// Bitfield on register `EIMSK`
pub const INT0: u8 = 0x1;

/// Bitfield on register `GTCCR`
pub const TSM: u8 = 0x80;

/// Bitfield on register `GTCCR`
pub const PSR: u8 = 0x1;

/// Bitfield on register `GTCCR`
pub const REMAP: u8 = 0x2;

/// Bitfield on register `LOCKBIT`
pub const LB: u8 = 0x3;

/// Bitfield on register `NVMCSR`
pub const NVMBSY: u8 = 0x80;

/// Bitfield on register `PCICR`
pub const PCIE1: u8 = 0x2;

/// Bitfield on register `PCICR`
pub const PCIE0: u8 = 0x1;

/// Bitfield on register `PCIFR`
pub const PCIF0: u8 = 0x1;

/// Bitfield on register `PCIFR`
pub const PCIF1: u8 = 0x2;

/// Bitfield on register `PCMSK0`
pub const PCINT5: u8 = 0x20;

/// Bitfield on register `PCMSK0`
pub const PCINT6: u8 = 0x40;

/// Bitfield on register `PCMSK0`
pub const PCINT2: u8 = 0x4;

/// Bitfield on register `PCMSK0`
pub const PCINT1: u8 = 0x2;

/// Bitfield on register `PCMSK0`
pub const PCINT4: u8 = 0x10;

/// Bitfield on register `PCMSK0`
pub const PCINT7: u8 = 0x80;

/// Bitfield on register `PCMSK0`
pub const PCINT3: u8 = 0x8;

/// Bitfield on register `PCMSK0`
pub const PCINT0: u8 = 0x1;

/// Bitfield on register `PCMSK1`
pub const PCINT9: u8 = 0x2;

/// Bitfield on register `PCMSK1`
pub const PCINT8: u8 = 0x1;

/// Bitfield on register `PCMSK1`
pub const PCINT10: u8 = 0x4;

/// Bitfield on register `PCMSK1`
pub const PCINT11: u8 = 0x8;

/// Bitfield on register `PORTCR`
pub const BBMA: u8 = 0x1;

/// Bitfield on register `PRR`
pub const PRUSART: u8 = 0x4;

/// Bitfield on register `PRR`
pub const PRTIM0: u8 = 0x1;

/// Bitfield on register `PRR`
pub const PRADC: u8 = 0x2;

/// Bitfield on register `RSTFLR`
pub const PORF: u8 = 0x1;

/// Bitfield on register `RSTFLR`
pub const EXTRF: u8 = 0x2;

/// Bitfield on register `RSTFLR`
pub const WDRF: u8 = 0x8;

/// Bitfield on register `SMCR`
pub const SE: u8 = 0x1;

/// Bitfield on register `SMCR`
pub const SM: u8 = 0xE;

/// Bitfield on register `SREG`
pub const C: u8 = 0x1;

/// Bitfield on register `SREG`
pub const H: u8 = 0x20;

/// Bitfield on register `SREG`
pub const S: u8 = 0x10;

/// Bitfield on register `SREG`
pub const T: u8 = 0x40;

/// Bitfield on register `SREG`
pub const V: u8 = 0x8;

/// Bitfield on register `SREG`
pub const Z: u8 = 0x2;

/// Bitfield on register `SREG`
pub const N: u8 = 0x4;

/// Bitfield on register `SREG`
pub const I: u8 = 0x80;

/// Bitfield on register `TCCR0A`
pub const COM0A: u8 = 0xC0;

/// Bitfield on register `TCCR0A`
pub const COM0B: u8 = 0x30;

/// Bitfield on register `TCCR0B`
pub const ICES0: u8 = 0x40;

/// Bitfield on register `TCCR0B`
pub const ICNC0: u8 = 0x80;

/// Bitfield on register `TCCR0B`
pub const CS0: u8 = 0x7;

/// Bitfield on register `TCCR0C`
pub const FOC0A: u8 = 0x80;

/// Bitfield on register `TCCR0C`
pub const FOC0B: u8 = 0x40;

/// Bitfield on register `TIFR0`
pub const OCF0B: u8 = 0x4;

/// Bitfield on register `TIFR0`
pub const OCF0A: u8 = 0x2;

/// Bitfield on register `TIFR0`
pub const TOV0: u8 = 0x1;

/// Bitfield on register `TIFR0`
pub const ICF0: u8 = 0x20;

/// Bitfield on register `TIMSK0`
pub const TOIE0: u8 = 0x1;

/// Bitfield on register `TIMSK0`
pub const ICIE0: u8 = 0x20;

/// Bitfield on register `TIMSK0`
pub const OCIE0B: u8 = 0x4;

/// Bitfield on register `TIMSK0`
pub const OCIE0A: u8 = 0x2;

/// Bitfield on register `UCSRA`
pub const U2X: u8 = 0x2;

/// Bitfield on register `UCSRA`
pub const RXC: u8 = 0x80;

/// Bitfield on register `UCSRA`
pub const DOR: u8 = 0x8;

/// Bitfield on register `UCSRA`
pub const TXC: u8 = 0x40;

/// Bitfield on register `UCSRA`
pub const UDRE: u8 = 0x20;

/// Bitfield on register `UCSRA`
pub const FE: u8 = 0x10;

/// Bitfield on register `UCSRA`
pub const MPCM: u8 = 0x1;

/// Bitfield on register `UCSRA`
pub const UPE: u8 = 0x4;

/// Bitfield on register `UCSRB`
pub const RXB8: u8 = 0x2;

/// Bitfield on register `UCSRB`
pub const TXEN: u8 = 0x8;

/// Bitfield on register `UCSRB`
pub const RXEN: u8 = 0x10;

/// Bitfield on register `UCSRB`
pub const RXCIE: u8 = 0x80;

/// Bitfield on register `UCSRB`
pub const UCSZ2: u8 = 0x4;

/// Bitfield on register `UCSRB`
pub const TXB8: u8 = 0x1;

/// Bitfield on register `UCSRB`
pub const UDRIE: u8 = 0x20;

/// Bitfield on register `UCSRB`
pub const TXCIE: u8 = 0x40;

/// Bitfield on register `UCSRC`
pub const UCSZ: u8 = 0x6;

/// Bitfield on register `UCSRC`
pub const UCPOL: u8 = 0x1;

/// Bitfield on register `UCSRC`
pub const USBS: u8 = 0x8;

/// Bitfield on register `UCSRC`
pub const UPM: u8 = 0x30;

/// Bitfield on register `UCSRC`
pub const UMSEL: u8 = 0xC0;

/// Bitfield on register `UCSRD`
pub const RXSIE: u8 = 0x80;

/// Bitfield on register `UCSRD`
pub const SFDE: u8 = 0x20;

/// Bitfield on register `UCSRD`
pub const RXS: u8 = 0x40;

/// Bitfield on register `VLMCSR`
pub const VLMF: u8 = 0x80;

/// Bitfield on register `VLMCSR`
pub const VLM: u8 = 0x7;

/// Bitfield on register `VLMCSR`
pub const VLMIE: u8 = 0x40;

/// Bitfield on register `WDTCSR`
pub const WDIE: u8 = 0x40;

/// Bitfield on register `WDTCSR`
pub const WDP: u8 = 0x27;

/// Bitfield on register `WDTCSR`
pub const WDE: u8 = 0x8;

/// Bitfield on register `WDTCSR`
pub const WDIF: u8 = 0x80;

/// `ADC_MUX_TINY10X` value group
#[allow(non_upper_case_globals)]
pub mod adc_mux_tiny10x {
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
}

/// `ANALOG_ADC_AUTO_TRIGGER_T10` value group
#[allow(non_upper_case_globals)]
pub mod analog_adc_auto_trigger_t10 {
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
   /// Timer/Counter0 Compare Match B.
   pub const VAL_0x05: u32 = 0x5;
   /// Pin Change Interrupt 0 Request.
   pub const VAL_0x06: u32 = 0x6;
   /// Timer/Counter0 Capture Event.
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

/// `ANALOG_ADC_V_REF_TINY10X` value group
#[allow(non_upper_case_globals)]
pub mod analog_adc_v_ref_tiny10x {
   /// Vcc.
   pub const VAL_0x00: u32 = 0x0;
   /// Internal 1.1V Referemce.
   pub const VAL_0x01: u32 = 0x1;
   /// Internal 2.2V Referemce.
   pub const VAL_0x02: u32 = 0x2;
   /// Internal 4.3V Referemce.
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
}

/// CCP signature select
#[allow(non_upper_case_globals)]
pub mod cpu_ccp {
   /// SPM Instruction Protection.
   pub const SPM: u32 = 0xE7;
   /// IO Register Protection.
   pub const IOREG: u32 = 0xD8;
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
pub mod cpu_sleep_mode_3bits {
   /// Idle.
   pub const IDLE: u32 = 0x0;
   /// ADC Noise Reduction.
   pub const ADC: u32 = 0x1;
   /// Power Down.
   pub const PDOWN: u32 = 0x2;
   /// Reserved.
   pub const VAL_0x03: u32 = 0x3;
   /// Standby.
   pub const STDBY: u32 = 0x4;
   /// Reserved.
   pub const VAL_0x05: u32 = 0x5;
   /// Reserved.
   pub const VAL_0x06: u32 = 0x6;
   /// Reserved.
   pub const VAL_0x07: u32 = 0x7;
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

