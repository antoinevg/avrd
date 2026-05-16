//! The AVR AT90CAN64 microcontroller
//!
//! # Variants
//! |        | Pinout | Mcu age | Operating temperature | Operating voltage | Max speed |
//! |--------|--------|---------|-----------------------|-------------------|-----------|
//! | AT90CAN64-16AI | TQFP64 | TQFP64 | -40°C - 85°C | 2.7V - 5.5V | 16 MHz |
//! | AT90CAN64-16MI | QFN64 | QFN64 | -40°C - 85°C | 2.7V - 5.5V | 16 MHz |
//! | AT90CAN64-16AU | TQFP64 | TQFP64 | -40°C - 85°C | 2.7V - 5.5V | 16 MHz |
//! | AT90CAN64-16MU | QFN64 | QFN64 | -40°C - 85°C | 2.7V - 5.5V | 16 MHz |
//!

#![allow(non_upper_case_globals)]

/// `LOW` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CKOUT | 1000000 |
/// | CKDIV8 | 10000000 |
/// | SUT_CKSEL | 111111 |
pub const LOW: *mut u8 = 0x0 as *mut u8;

/// `LOCKBIT` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LB | 11 |
/// | BLB0 | 1100 |
/// | BLB1 | 110000 |
pub const LOCKBIT: *mut u8 = 0x0 as *mut u8;

/// `HIGH` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCDEN | 10000000 |
/// | BOOTSZ | 110 |
/// | SPIEN | 100000 |
/// | EESAVE | 1000 |
/// | BOOTRST | 1 |
/// | WDTON | 10000 |
/// | JTAGEN | 1000000 |
pub const HIGH: *mut u8 = 0x1 as *mut u8;

/// `EXTENDED` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BODLEVEL | 1110 |
/// | TA0SEL | 1 |
pub const EXTENDED: *mut u8 = 0x2 as *mut u8;

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

/// Port C Input Pins.
pub const PINC: *mut u8 = 0x26 as *mut u8;

/// Port C Data Direction Register.
pub const DDRC: *mut u8 = 0x27 as *mut u8;

/// Port C Data Register.
pub const PORTC: *mut u8 = 0x28 as *mut u8;

/// Port D Input Pins.
pub const PIND: *mut u8 = 0x29 as *mut u8;

/// Port D Data Direction Register.
pub const DDRD: *mut u8 = 0x2A as *mut u8;

/// Port D Data Register.
pub const PORTD: *mut u8 = 0x2B as *mut u8;

/// Input Pins, Port E.
pub const PINE: *mut u8 = 0x2C as *mut u8;

/// Data Direction Register, Port E.
pub const DDRE: *mut u8 = 0x2D as *mut u8;

/// Data Register, Port E.
pub const PORTE: *mut u8 = 0x2E as *mut u8;

/// Input Pins, Port F.
pub const PINF: *mut u8 = 0x2F as *mut u8;

/// Data Direction Register, Port F.
pub const DDRF: *mut u8 = 0x30 as *mut u8;

/// Data Register, Port F.
pub const PORTF: *mut u8 = 0x31 as *mut u8;

/// Input Pins, Port G.
pub const PING: *mut u8 = 0x32 as *mut u8;

/// Data Direction Register, Port G.
pub const DDRG: *mut u8 = 0x33 as *mut u8;

/// Data Register, Port G.
pub const PORTG: *mut u8 = 0x34 as *mut u8;

/// Timer/Counter0 Interrupt Flag register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TOV0 | 1 |
/// | OCF0A | 10 |
pub const TIFR0: *mut u8 = 0x35 as *mut u8;

/// Timer/Counter Interrupt Flag register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCF1C | 1000 |
/// | ICF1 | 100000 |
/// | OCF1A | 10 |
/// | OCF1B | 100 |
/// | TOV1 | 1 |
pub const TIFR1: *mut u8 = 0x36 as *mut u8;

/// Timer/Counter Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TOV2 | 1 |
/// | OCF2A | 10 |
pub const TIFR2: *mut u8 = 0x37 as *mut u8;

/// Timer/Counter Interrupt Flag register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCF3B | 100 |
/// | OCF3A | 10 |
/// | TOV3 | 1 |
/// | ICF3 | 100000 |
/// | OCF3C | 1000 |
pub const TIFR3: *mut u8 = 0x38 as *mut u8;

/// External Interrupt Flag Register.
pub const EIFR: *mut u8 = 0x3C as *mut u8;

/// External Interrupt Mask Register.
pub const EIMSK: *mut u8 = 0x3D as *mut u8;

/// General Purpose IO Register 0.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | GPIOR06 | 1000000 |
/// | GPIOR00 | 1 |
/// | GPIOR03 | 1000 |
/// | GPIOR01 | 10 |
/// | GPIOR05 | 100000 |
/// | GPIOR02 | 100 |
/// | GPIOR07 | 10000000 |
/// | GPIOR04 | 10000 |
pub const GPIOR0: *mut u8 = 0x3E as *mut u8;

/// EEPROM Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EEWE | 10 |
/// | EEMWE | 100 |
/// | EERE | 1 |
/// | EERIE | 1000 |
pub const EECR: *mut u8 = 0x3F as *mut u8;

/// EEPROM Data Register.
pub const EEDR: *mut u8 = 0x40 as *mut u8;

/// EEPROM Read/Write Access Bytes - Only bit 10..8 are used in AT90CAN64 - Only bit 9..8 are used in AT90CAN32 low byte.
pub const EEARL: *mut u8 = 0x41 as *mut u8;

/// EEPROM Read/Write Access Bytes - Only bit 10..8 are used in AT90CAN64 - Only bit 9..8 are used in AT90CAN32.
pub const EEAR: *mut u16 = 0x41 as *mut u16;

/// EEPROM Read/Write Access Bytes - Only bit 10..8 are used in AT90CAN64 - Only bit 9..8 are used in AT90CAN32 high byte.
pub const EEARH: *mut u8 = 0x42 as *mut u8;

/// General Timer/Counter Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PSR2 | 10 |
pub const GTCCR: *mut u8 = 0x43 as *mut u8;

/// Timer/Counter0 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WGM00 | 1000000 |
/// | COM0A | 110000 |
/// | WGM01 | 1000 |
/// | FOC0A | 10000000 |
/// | CS0 | 111 |
pub const TCCR0A: *mut u8 = 0x44 as *mut u8;

/// Timer/Counter0.
pub const TCNT0: *mut u8 = 0x46 as *mut u8;

/// Timer/Counter0 Output Compare Register.
pub const OCR0A: *mut u8 = 0x47 as *mut u8;

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
/// | SPE | 1000000 |
/// | CPOL | 1000 |
/// | CPHA | 100 |
/// | MSTR | 10000 |
/// | SPR | 11 |
/// | SPIE | 10000000 |
/// | DORD | 100000 |
pub const SPCR: *mut u8 = 0x4C as *mut u8;

/// SPI Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WCOL | 1000000 |
/// | SPI2X | 1 |
/// | SPIF | 10000000 |
pub const SPSR: *mut u8 = 0x4D as *mut u8;

/// SPI Data Register.
pub const SPDR: *mut u8 = 0x4E as *mut u8;

/// Analog Comparator Control And Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ACIE | 1000 |
/// | ACD | 10000000 |
/// | ACIS | 11 |
/// | ACI | 10000 |
/// | ACBG | 1000000 |
/// | ACIC | 100 |
/// | ACO | 100000 |
pub const ACSR: *mut u8 = 0x50 as *mut u8;

/// On-Chip Debug Related Register in I/O Memory.
pub const OCDR: *mut u8 = 0x51 as *mut u8;

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
/// | JTRF | 10000 |
/// | WDRF | 1000 |
/// | EXTRF | 10 |
/// | BORF | 100 |
/// | PORF | 1 |
pub const MCUSR: *mut u8 = 0x54 as *mut u8;

/// MCU Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PUD | 10000 |
/// | IVCE | 1 |
/// | IVSEL | 10 |
pub const MCUCR: *mut u8 = 0x55 as *mut u8;

/// Store Program Memory Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BLBSET | 1000 |
/// | PGWRT | 100 |
/// | RWWSB | 1000000 |
/// | SPMIE | 10000000 |
/// | RWWSRE | 10000 |
/// | SPMEN | 1 |
/// | PGERS | 10 |
pub const SPMCSR: *mut u8 = 0x57 as *mut u8;

/// RAM Page Z Select Register - Not used.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RAMPZ0 | 1 |
pub const RAMPZ: *mut u8 = 0x5B as *mut u8;

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
/// | S | 10000 |
/// | Z | 10 |
/// | V | 1000 |
/// | C | 1 |
/// | I | 10000000 |
/// | N | 100 |
/// | T | 1000000 |
/// | H | 100000 |
pub const SREG: *mut u8 = 0x5F as *mut u8;

/// Watchdog Timer Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDE | 1000 |
/// | WDP | 111 |
/// | WDCE | 10000 |
pub const WDTCR: *mut u8 = 0x60 as *mut u8;

/// Clock Prescale Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CLKPS | 1111 |
/// | CLKPCE | 10000000 |
pub const CLKPR: *mut u8 = 0x61 as *mut u8;

/// Oscillator Calibration Value.
pub const OSCCAL: *mut u8 = 0x66 as *mut u8;

/// External Interrupt Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ISC1 | 1100 |
/// | ISC2 | 110000 |
/// | ISC3 | 11000000 |
/// | ISC0 | 11 |
pub const EICRA: *mut u8 = 0x69 as *mut u8;

/// External Interrupt Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ISC6 | 110000 |
/// | ISC4 | 11 |
/// | ISC7 | 11000000 |
/// | ISC5 | 1100 |
pub const EICRB: *mut u8 = 0x6A as *mut u8;

/// Timer/Counter0 Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
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
/// | OCIE1B | 100 |
/// | TOIE1 | 1 |
/// | OCIE1C | 1000 |
/// | ICIE1 | 100000 |
pub const TIMSK1: *mut u8 = 0x6F as *mut u8;

/// Timer/Counter Interrupt Mask register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCIE2A | 10 |
/// | TOIE2 | 1 |
pub const TIMSK2: *mut u8 = 0x70 as *mut u8;

/// Timer/Counter Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ICIE3 | 100000 |
/// | OCIE3A | 10 |
/// | OCIE3C | 1000 |
/// | OCIE3B | 100 |
/// | TOIE3 | 1 |
pub const TIMSK3: *mut u8 = 0x71 as *mut u8;

/// External Memory Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SRL | 1110000 |
/// | SRW1 | 1100 |
/// | SRW0 | 11 |
/// | SRE | 10000000 |
pub const XMCRA: *mut u8 = 0x74 as *mut u8;

/// External Memory Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | XMBK | 10000000 |
/// | XMM | 111 |
pub const XMCRB: *mut u8 = 0x75 as *mut u8;

/// ADC Data Register  Bytes.
pub const ADC: *mut u16 = 0x78 as *mut u16;

/// ADC Data Register  Bytes low byte.
pub const ADCL: *mut u8 = 0x78 as *mut u8;

/// ADC Data Register  Bytes high byte.
pub const ADCH: *mut u8 = 0x79 as *mut u8;

/// The ADC Control and Status register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADEN | 10000000 |
/// | ADIE | 1000 |
/// | ADPS | 111 |
/// | ADIF | 10000 |
/// | ADATE | 100000 |
/// | ADSC | 1000000 |
pub const ADCSRA: *mut u8 = 0x7A as *mut u8;

/// ADC Control and Status Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ACME | 1000000 |
pub const ADCSRB: *mut u8 = 0x7B as *mut u8;

/// The ADC multiplexer Selection Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | MUX | 11111 |
/// | REFS | 11000000 |
/// | ADLAR | 100000 |
pub const ADMUX: *mut u8 = 0x7C as *mut u8;

/// Digital Input Disable Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADC1D | 10 |
/// | ADC0D | 1 |
/// | ADC4D | 10000 |
/// | ADC6D | 1000000 |
/// | ADC5D | 100000 |
/// | ADC3D | 1000 |
/// | ADC7D | 10000000 |
/// | ADC2D | 100 |
pub const DIDR0: *mut u8 = 0x7E as *mut u8;

/// `DIDR1` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AIN0D | 1 |
/// | AIN1D | 10 |
pub const DIDR1: *mut u8 = 0x7F as *mut u8;

/// Timer/Counter1 Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | COM1B | 110000 |
/// | COM1A | 11000000 |
/// | COM1C | 1100 |
pub const TCCR1A: *mut u8 = 0x80 as *mut u8;

/// Timer/Counter1 Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ICES1 | 1000000 |
/// | ICNC1 | 10000000 |
/// | CS1 | 111 |
pub const TCCR1B: *mut u8 = 0x81 as *mut u8;

/// Timer/Counter 1 Control Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FOC1B | 1000000 |
/// | FOC1C | 100000 |
/// | FOC1A | 10000000 |
pub const TCCR1C: *mut u8 = 0x82 as *mut u8;

/// Timer/Counter1  Bytes low byte.
pub const TCNT1L: *mut u8 = 0x84 as *mut u8;

/// Timer/Counter1  Bytes.
pub const TCNT1: *mut u16 = 0x84 as *mut u16;

/// Timer/Counter1  Bytes high byte.
pub const TCNT1H: *mut u8 = 0x85 as *mut u8;

/// Timer/Counter1 Input Capture Register  Bytes.
pub const ICR1: *mut u16 = 0x86 as *mut u16;

/// Timer/Counter1 Input Capture Register  Bytes low byte.
pub const ICR1L: *mut u8 = 0x86 as *mut u8;

/// Timer/Counter1 Input Capture Register  Bytes high byte.
pub const ICR1H: *mut u8 = 0x87 as *mut u8;

/// Timer/Counter1 Output Compare Register  Bytes.
pub const OCR1A: *mut u16 = 0x88 as *mut u16;

/// Timer/Counter1 Output Compare Register  Bytes low byte.
pub const OCR1AL: *mut u8 = 0x88 as *mut u8;

/// Timer/Counter1 Output Compare Register  Bytes high byte.
pub const OCR1AH: *mut u8 = 0x89 as *mut u8;

/// Timer/Counter1 Output Compare Register  Bytes.
pub const OCR1B: *mut u16 = 0x8A as *mut u16;

/// Timer/Counter1 Output Compare Register  Bytes low byte.
pub const OCR1BL: *mut u8 = 0x8A as *mut u8;

/// Timer/Counter1 Output Compare Register  Bytes high byte.
pub const OCR1BH: *mut u8 = 0x8B as *mut u8;

/// Timer/Counter1 Output Compare Register  Bytes.
pub const OCR1C: *mut u16 = 0x8C as *mut u16;

/// Timer/Counter1 Output Compare Register  Bytes low byte.
pub const OCR1CL: *mut u8 = 0x8C as *mut u8;

/// Timer/Counter1 Output Compare Register  Bytes high byte.
pub const OCR1CH: *mut u8 = 0x8D as *mut u8;

/// Timer/Counter3 Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | COM3A | 11000000 |
/// | COM3B | 110000 |
/// | COM3C | 1100 |
pub const TCCR3A: *mut u8 = 0x90 as *mut u8;

/// Timer/Counter3 Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ICNC3 | 10000000 |
/// | ICES3 | 1000000 |
/// | CS3 | 111 |
pub const TCCR3B: *mut u8 = 0x91 as *mut u8;

/// Timer/Counter 3 Control Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FOC3B | 1000000 |
/// | FOC3C | 100000 |
/// | FOC3A | 10000000 |
pub const TCCR3C: *mut u8 = 0x92 as *mut u8;

/// Timer/Counter3  Bytes low byte.
pub const TCNT3L: *mut u8 = 0x94 as *mut u8;

/// Timer/Counter3  Bytes.
pub const TCNT3: *mut u16 = 0x94 as *mut u16;

/// Timer/Counter3  Bytes high byte.
pub const TCNT3H: *mut u8 = 0x95 as *mut u8;

/// Timer/Counter3 Input Capture Register  Bytes low byte.
pub const ICR3L: *mut u8 = 0x96 as *mut u8;

/// Timer/Counter3 Input Capture Register  Bytes.
pub const ICR3: *mut u16 = 0x96 as *mut u16;

/// Timer/Counter3 Input Capture Register  Bytes high byte.
pub const ICR3H: *mut u8 = 0x97 as *mut u8;

/// Timer/Counter3 Output Compare Register  Bytes low byte.
pub const OCR3AL: *mut u8 = 0x98 as *mut u8;

/// Timer/Counter3 Output Compare Register  Bytes.
pub const OCR3A: *mut u16 = 0x98 as *mut u16;

/// Timer/Counter3 Output Compare Register  Bytes high byte.
pub const OCR3AH: *mut u8 = 0x99 as *mut u8;

/// Timer/Counter3 Output Compare Register  Bytes low byte.
pub const OCR3BL: *mut u8 = 0x9A as *mut u8;

/// Timer/Counter3 Output Compare Register  Bytes.
pub const OCR3B: *mut u16 = 0x9A as *mut u16;

/// Timer/Counter3 Output Compare Register  Bytes high byte.
pub const OCR3BH: *mut u8 = 0x9B as *mut u8;

/// Timer/Counter3 Output Compare Register  Bytes.
pub const OCR3C: *mut u16 = 0x9C as *mut u16;

/// Timer/Counter3 Output Compare Register  Bytes low byte.
pub const OCR3CL: *mut u8 = 0x9C as *mut u8;

/// Timer/Counter3 Output Compare Register  Bytes high byte.
pub const OCR3CH: *mut u8 = 0x9D as *mut u8;

/// Timer/Counter2 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CS2 | 111 |
/// | COM2A | 110000 |
/// | WGM21 | 1000 |
/// | WGM20 | 1000000 |
/// | FOC2A | 10000000 |
pub const TCCR2A: *mut u8 = 0xB0 as *mut u8;

/// Timer/Counter2.
pub const TCNT2: *mut u8 = 0xB2 as *mut u8;

/// Timer/Counter2 Output Compare Register.
pub const OCR2A: *mut u8 = 0xB3 as *mut u8;

/// Asynchronous Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EXCLK | 10000 |
/// | TCN2UB | 100 |
/// | AS2 | 1000 |
/// | OCR2UB | 10 |
/// | TCR2UB | 1 |
pub const ASSR: *mut u8 = 0xB6 as *mut u8;

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
/// | TWEN | 100 |
/// | TWIE | 1 |
/// | TWINT | 10000000 |
/// | TWEA | 1000000 |
/// | TWSTO | 10000 |
/// | TWSTA | 100000 |
pub const TWCR: *mut u8 = 0xBC as *mut u8;

/// USART Control and Status Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | U2X0 | 10 |
/// | MPCM0 | 1 |
/// | TXC0 | 1000000 |
/// | RXC0 | 10000000 |
/// | DOR0 | 1000 |
/// | FE0 | 10000 |
/// | UPE0 | 100 |
/// | UDRE0 | 100000 |
pub const UCSR0A: *mut u8 = 0xC0 as *mut u8;

/// USART Control and Status Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TXB80 | 1 |
/// | TXEN0 | 1000 |
/// | TXCIE0 | 1000000 |
/// | RXEN0 | 10000 |
/// | UDRIE0 | 100000 |
/// | UCSZ02 | 100 |
/// | RXCIE0 | 10000000 |
/// | RXB80 | 10 |
pub const UCSR0B: *mut u8 = 0xC1 as *mut u8;

/// USART Control and Status Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | UMSEL0 | 1000000 |
/// | UPM0 | 110000 |
/// | UCSZ0 | 110 |
/// | USBS0 | 1000 |
/// | UCPOL0 | 1 |
pub const UCSR0C: *mut u8 = 0xC2 as *mut u8;

/// USART Baud Rate Register t Bytes.
pub const UBRR0: *mut u16 = 0xC4 as *mut u16;

/// USART Baud Rate Register t Bytes low byte.
pub const UBRR0L: *mut u8 = 0xC4 as *mut u8;

/// USART Baud Rate Register t Bytes high byte.
pub const UBRR0H: *mut u8 = 0xC5 as *mut u8;

/// USART I/O Data Register.
pub const UDR0: *mut u8 = 0xC6 as *mut u8;

/// USART Control and Status Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | U2X1 | 10 |
/// | DOR1 | 1000 |
/// | UDRE1 | 100000 |
/// | MPCM1 | 1 |
/// | UPE1 | 100 |
/// | FE1 | 10000 |
/// | RXC1 | 10000000 |
/// | TXC1 | 1000000 |
pub const UCSR1A: *mut u8 = 0xC8 as *mut u8;

/// USART Control and Status Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TXB81 | 1 |
/// | TXCIE1 | 1000000 |
/// | UDRIE1 | 100000 |
/// | RXCIE1 | 10000000 |
/// | UCSZ12 | 100 |
/// | RXB81 | 10 |
/// | RXEN1 | 10000 |
/// | TXEN1 | 1000 |
pub const UCSR1B: *mut u8 = 0xC9 as *mut u8;

/// USART Control and Status Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | UPM1 | 110000 |
/// | UCPOL1 | 1 |
/// | UCSZ1 | 110 |
/// | USBS1 | 1000 |
/// | UMSEL1 | 1000000 |
pub const UCSR1C: *mut u8 = 0xCA as *mut u8;

/// USART Baud Rate Register t Bytes.
pub const UBRR1: *mut u16 = 0xCC as *mut u16;

/// USART Baud Rate Register t Bytes low byte.
pub const UBRR1L: *mut u8 = 0xCC as *mut u8;

/// USART Baud Rate Register t Bytes high byte.
pub const UBRR1H: *mut u8 = 0xCD as *mut u8;

/// USART I/O Data Register.
pub const UDR1: *mut u8 = 0xCE as *mut u8;

/// CAN General Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OVRQ | 1000000 |
/// | TEST | 100 |
/// | LISTEN | 1000 |
/// | TTC | 100000 |
/// | SYNTTC | 10000 |
/// | ABRQ | 10000000 |
/// | SWRES | 1 |
/// | ENASTB | 10 |
pub const CANGCON: *mut u8 = 0xD8 as *mut u8;

/// CAN General Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ENFG | 100 |
/// | OVRG | 1000000 |
/// | TXBSY | 10000 |
/// | RXBSY | 1000 |
/// | ERRP | 1 |
/// | BOFF | 10 |
pub const CANGSTA: *mut u8 = 0xD9 as *mut u8;

/// CAN General Interrupt Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BOFFIT | 1000000 |
/// | AERG | 1 |
/// | CERG | 100 |
/// | FERG | 10 |
/// | SERG | 1000 |
/// | OVRTIM | 100000 |
/// | CANIT | 10000000 |
/// | BXOK | 10000 |
pub const CANGIT: *mut u8 = 0xDA as *mut u8;

/// CAN General Interrupt Enable Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ENERR | 1000 |
/// | ENBX | 100 |
/// | ENERG | 10 |
/// | ENOVRT | 1 |
/// | ENRX | 100000 |
/// | ENIT | 10000000 |
/// | ENBOFF | 1000000 |
/// | ENTX | 10000 |
pub const CANGIE: *mut u8 = 0xDB as *mut u8;

/// Enable MOb Register.
pub const CANEN2: *mut u8 = 0xDC as *mut u8;

/// Enable MOb Register.
pub const CANEN1: *mut u8 = 0xDD as *mut u8;

/// Enable Interrupt MOb Register.
pub const CANIE2: *mut u8 = 0xDE as *mut u8;

/// Enable Interrupt MOb Register.
pub const CANIE1: *mut u8 = 0xDF as *mut u8;

/// CAN Status Interrupt MOb Register.
pub const CANSIT2: *mut u8 = 0xE0 as *mut u8;

/// CAN Status Interrupt MOb Register.
pub const CANSIT1: *mut u8 = 0xE1 as *mut u8;

/// Bit Timing Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BRP | 1111110 |
pub const CANBT1: *mut u8 = 0xE2 as *mut u8;

/// Bit Timing Register 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SJW | 1100000 |
/// | PRS | 1110 |
pub const CANBT2: *mut u8 = 0xE3 as *mut u8;

/// Bit Timing Register 3.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PHS2 | 1110000 |
/// | PHS1 | 1110 |
/// | SMP | 1 |
pub const CANBT3: *mut u8 = 0xE4 as *mut u8;

/// Timer Control Register.
pub const CANTCON: *mut u8 = 0xE5 as *mut u8;

/// Timer Register.
pub const CANTIM: *mut u16 = 0xE6 as *mut u16;

/// Timer Register low byte.
pub const CANTIML: *mut u8 = 0xE6 as *mut u8;

/// Timer Register high byte.
pub const CANTIMH: *mut u8 = 0xE7 as *mut u8;

/// TTC Timer Register low byte.
pub const CANTTCL: *mut u8 = 0xE8 as *mut u8;

/// TTC Timer Register.
pub const CANTTC: *mut u16 = 0xE8 as *mut u16;

/// TTC Timer Register high byte.
pub const CANTTCH: *mut u8 = 0xE9 as *mut u8;

/// Transmit Error Counter Register.
pub const CANTEC: *mut u8 = 0xEA as *mut u8;

/// Receive Error Counter Register.
pub const CANREC: *mut u8 = 0xEB as *mut u8;

/// Highest Priority MOb Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CGP | 1111 |
/// | HPMOB | 11110000 |
pub const CANHPMOB: *mut u8 = 0xEC as *mut u8;

/// Page MOb Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | MOBNB | 11110000 |
/// | INDX | 111 |
/// | AINC | 1000 |
pub const CANPAGE: *mut u8 = 0xED as *mut u8;

/// MOb Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DLCW | 10000000 |
/// | SERR | 1000 |
/// | BERR | 10000 |
/// | TXOK | 1000000 |
/// | AERR | 1 |
/// | CERR | 100 |
/// | FERR | 10 |
/// | RXOK | 100000 |
pub const CANSTMOB: *mut u8 = 0xEE as *mut u8;

/// MOb Control and DLC Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CONMOB | 11000000 |
/// | RPLV | 100000 |
/// | DLC | 1111 |
/// | IDE | 10000 |
pub const CANCDMOB: *mut u8 = 0xEF as *mut u8;

/// Identifier Tag Register 4.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RTRTAG | 100 |
/// | RB1TAG | 10 |
/// | RB0TAG | 1 |
pub const CANIDT4: *mut u8 = 0xF0 as *mut u8;

/// Identifier Tag Register 3.
pub const CANIDT3: *mut u8 = 0xF1 as *mut u8;

/// Identifier Tag Register 2.
pub const CANIDT2: *mut u8 = 0xF2 as *mut u8;

/// Identifier Tag Register 1.
pub const CANIDT1: *mut u8 = 0xF3 as *mut u8;

/// Identifier Mask Register 4.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | IDEMSK | 1 |
/// | RTRMSK | 100 |
pub const CANIDM4: *mut u8 = 0xF4 as *mut u8;

/// Identifier Mask Register 3.
pub const CANIDM3: *mut u8 = 0xF5 as *mut u8;

/// Identifier Mask Register 2.
pub const CANIDM2: *mut u8 = 0xF6 as *mut u8;

/// Identifier Mask Register 1.
pub const CANIDM1: *mut u8 = 0xF7 as *mut u8;

/// Time Stamp Register.
pub const CANSTM: *mut u16 = 0xF8 as *mut u16;

/// Time Stamp Register low byte.
pub const CANSTML: *mut u8 = 0xF8 as *mut u8;

/// Time Stamp Register high byte.
pub const CANSTMH: *mut u8 = 0xF9 as *mut u8;

/// Message Data Register.
pub const CANMSG: *mut u8 = 0xFA as *mut u8;

/// Bitfield on register `ACSR`
pub const ACIE: u8 = 0x8;

/// Bitfield on register `ACSR`
pub const ACD: u8 = 0x80;

/// Bitfield on register `ACSR`
pub const ACIS: u8 = 0x3;

/// Bitfield on register `ACSR`
pub const ACI: u8 = 0x10;

/// Bitfield on register `ACSR`
pub const ACBG: u8 = 0x40;

/// Bitfield on register `ACSR`
pub const ACIC: u8 = 0x4;

/// Bitfield on register `ACSR`
pub const ACO: u8 = 0x20;

/// Bitfield on register `ADCSRA`
pub const ADEN: u8 = 0x80;

/// Bitfield on register `ADCSRA`
pub const ADIE: u8 = 0x8;

/// Bitfield on register `ADCSRA`
pub const ADPS: u8 = 0x7;

/// Bitfield on register `ADCSRA`
pub const ADIF: u8 = 0x10;

/// Bitfield on register `ADCSRA`
pub const ADATE: u8 = 0x20;

/// Bitfield on register `ADCSRA`
pub const ADSC: u8 = 0x40;

/// Bitfield on register `ADCSRB`
pub const ACME: u8 = 0x40;

/// Bitfield on register `ADMUX`
pub const MUX: u8 = 0x1F;

/// Bitfield on register `ADMUX`
pub const REFS: u8 = 0xC0;

/// Bitfield on register `ADMUX`
pub const ADLAR: u8 = 0x20;

/// Bitfield on register `ASSR`
pub const EXCLK: u8 = 0x10;

/// Bitfield on register `ASSR`
pub const TCN2UB: u8 = 0x4;

/// Bitfield on register `ASSR`
pub const AS2: u8 = 0x8;

/// Bitfield on register `ASSR`
pub const OCR2UB: u8 = 0x2;

/// Bitfield on register `ASSR`
pub const TCR2UB: u8 = 0x1;

/// Bitfield on register `CANBT1`
pub const BRP: u8 = 0x7E;

/// Bitfield on register `CANBT2`
pub const SJW: u8 = 0x60;

/// Bitfield on register `CANBT2`
pub const PRS: u8 = 0xE;

/// Bitfield on register `CANBT3`
pub const PHS2: u8 = 0x70;

/// Bitfield on register `CANBT3`
pub const PHS1: u8 = 0xE;

/// Bitfield on register `CANBT3`
pub const SMP: u8 = 0x1;

/// Bitfield on register `CANCDMOB`
pub const CONMOB: u8 = 0xC0;

/// Bitfield on register `CANCDMOB`
pub const RPLV: u8 = 0x20;

/// Bitfield on register `CANCDMOB`
pub const DLC: u8 = 0xF;

/// Bitfield on register `CANCDMOB`
pub const IDE: u8 = 0x10;

/// Bitfield on register `CANGCON`
pub const OVRQ: u8 = 0x40;

/// Bitfield on register `CANGCON`
pub const TEST: u8 = 0x4;

/// Bitfield on register `CANGCON`
pub const LISTEN: u8 = 0x8;

/// Bitfield on register `CANGCON`
pub const TTC: u8 = 0x20;

/// Bitfield on register `CANGCON`
pub const SYNTTC: u8 = 0x10;

/// Bitfield on register `CANGCON`
pub const ABRQ: u8 = 0x80;

/// Bitfield on register `CANGCON`
pub const SWRES: u8 = 0x1;

/// Bitfield on register `CANGCON`
pub const ENASTB: u8 = 0x2;

/// Bitfield on register `CANGIE`
pub const ENERR: u8 = 0x8;

/// Bitfield on register `CANGIE`
pub const ENBX: u8 = 0x4;

/// Bitfield on register `CANGIE`
pub const ENERG: u8 = 0x2;

/// Bitfield on register `CANGIE`
pub const ENOVRT: u8 = 0x1;

/// Bitfield on register `CANGIE`
pub const ENRX: u8 = 0x20;

/// Bitfield on register `CANGIE`
pub const ENIT: u8 = 0x80;

/// Bitfield on register `CANGIE`
pub const ENBOFF: u8 = 0x40;

/// Bitfield on register `CANGIE`
pub const ENTX: u8 = 0x10;

/// Bitfield on register `CANGIT`
pub const BOFFIT: u8 = 0x40;

/// Bitfield on register `CANGIT`
pub const AERG: u8 = 0x1;

/// Bitfield on register `CANGIT`
pub const CERG: u8 = 0x4;

/// Bitfield on register `CANGIT`
pub const FERG: u8 = 0x2;

/// Bitfield on register `CANGIT`
pub const SERG: u8 = 0x8;

/// Bitfield on register `CANGIT`
pub const OVRTIM: u8 = 0x20;

/// Bitfield on register `CANGIT`
pub const CANIT: u8 = 0x80;

/// Bitfield on register `CANGIT`
pub const BXOK: u8 = 0x10;

/// Bitfield on register `CANGSTA`
pub const ENFG: u8 = 0x4;

/// Bitfield on register `CANGSTA`
pub const OVRG: u8 = 0x40;

/// Bitfield on register `CANGSTA`
pub const TXBSY: u8 = 0x10;

/// Bitfield on register `CANGSTA`
pub const RXBSY: u8 = 0x8;

/// Bitfield on register `CANGSTA`
pub const ERRP: u8 = 0x1;

/// Bitfield on register `CANGSTA`
pub const BOFF: u8 = 0x2;

/// Bitfield on register `CANHPMOB`
pub const CGP: u8 = 0xF;

/// Bitfield on register `CANHPMOB`
pub const HPMOB: u8 = 0xF0;

/// Bitfield on register `CANIDM4`
pub const IDEMSK: u8 = 0x1;

/// Bitfield on register `CANIDM4`
pub const RTRMSK: u8 = 0x4;

/// Bitfield on register `CANIDT4`
pub const RTRTAG: u8 = 0x4;

/// Bitfield on register `CANIDT4`
pub const RB1TAG: u8 = 0x2;

/// Bitfield on register `CANIDT4`
pub const RB0TAG: u8 = 0x1;

/// Bitfield on register `CANPAGE`
pub const MOBNB: u8 = 0xF0;

/// Bitfield on register `CANPAGE`
pub const INDX: u8 = 0x7;

/// Bitfield on register `CANPAGE`
pub const AINC: u8 = 0x8;

/// Bitfield on register `CANSTMOB`
pub const DLCW: u8 = 0x80;

/// Bitfield on register `CANSTMOB`
pub const SERR: u8 = 0x8;

/// Bitfield on register `CANSTMOB`
pub const BERR: u8 = 0x10;

/// Bitfield on register `CANSTMOB`
pub const TXOK: u8 = 0x40;

/// Bitfield on register `CANSTMOB`
pub const AERR: u8 = 0x1;

/// Bitfield on register `CANSTMOB`
pub const CERR: u8 = 0x4;

/// Bitfield on register `CANSTMOB`
pub const FERR: u8 = 0x2;

/// Bitfield on register `CANSTMOB`
pub const RXOK: u8 = 0x20;

/// Bitfield on register `CLKPR`
pub const CLKPS: u8 = 0xF;

/// Bitfield on register `CLKPR`
pub const CLKPCE: u8 = 0x80;

/// Bitfield on register `DIDR0`
pub const ADC1D: u8 = 0x2;

/// Bitfield on register `DIDR0`
pub const ADC0D: u8 = 0x1;

/// Bitfield on register `DIDR0`
pub const ADC4D: u8 = 0x10;

/// Bitfield on register `DIDR0`
pub const ADC6D: u8 = 0x40;

/// Bitfield on register `DIDR0`
pub const ADC5D: u8 = 0x20;

/// Bitfield on register `DIDR0`
pub const ADC3D: u8 = 0x8;

/// Bitfield on register `DIDR0`
pub const ADC7D: u8 = 0x80;

/// Bitfield on register `DIDR0`
pub const ADC2D: u8 = 0x4;

/// Bitfield on register `DIDR1`
pub const AIN0D: u8 = 0x1;

/// Bitfield on register `DIDR1`
pub const AIN1D: u8 = 0x2;

/// Bitfield on register `EECR`
pub const EEWE: u8 = 0x2;

/// Bitfield on register `EECR`
pub const EEMWE: u8 = 0x4;

/// Bitfield on register `EECR`
pub const EERE: u8 = 0x1;

/// Bitfield on register `EECR`
pub const EERIE: u8 = 0x8;

/// Bitfield on register `EICRA`
pub const ISC1: u8 = 0xC;

/// Bitfield on register `EICRA`
pub const ISC2: u8 = 0x30;

/// Bitfield on register `EICRA`
pub const ISC3: u8 = 0xC0;

/// Bitfield on register `EICRA`
pub const ISC0: u8 = 0x3;

/// Bitfield on register `EICRB`
pub const ISC6: u8 = 0x30;

/// Bitfield on register `EICRB`
pub const ISC4: u8 = 0x3;

/// Bitfield on register `EICRB`
pub const ISC7: u8 = 0xC0;

/// Bitfield on register `EICRB`
pub const ISC5: u8 = 0xC;

/// Bitfield on register `EXTENDED`
pub const BODLEVEL: u8 = 0xE;

/// Bitfield on register `EXTENDED`
pub const TA0SEL: u8 = 0x1;

/// Bitfield on register `GPIOR0`
pub const GPIOR06: u8 = 0x40;

/// Bitfield on register `GPIOR0`
pub const GPIOR00: u8 = 0x1;

/// Bitfield on register `GPIOR0`
pub const GPIOR03: u8 = 0x8;

/// Bitfield on register `GPIOR0`
pub const GPIOR01: u8 = 0x2;

/// Bitfield on register `GPIOR0`
pub const GPIOR05: u8 = 0x20;

/// Bitfield on register `GPIOR0`
pub const GPIOR02: u8 = 0x4;

/// Bitfield on register `GPIOR0`
pub const GPIOR07: u8 = 0x80;

/// Bitfield on register `GPIOR0`
pub const GPIOR04: u8 = 0x10;

/// Bitfield on register `GTCCR`
pub const PSR2: u8 = 0x2;

/// Bitfield on register `HIGH`
pub const OCDEN: u8 = 0x80;

/// Bitfield on register `HIGH`
pub const BOOTSZ: u8 = 0x6;

/// Bitfield on register `HIGH`
pub const SPIEN: u8 = 0x20;

/// Bitfield on register `HIGH`
pub const EESAVE: u8 = 0x8;

/// Bitfield on register `HIGH`
pub const BOOTRST: u8 = 0x1;

/// Bitfield on register `HIGH`
pub const WDTON: u8 = 0x10;

/// Bitfield on register `HIGH`
pub const JTAGEN: u8 = 0x40;

/// Bitfield on register `LOCKBIT`
pub const LB: u8 = 0x3;

/// Bitfield on register `LOCKBIT`
pub const BLB0: u8 = 0xC;

/// Bitfield on register `LOCKBIT`
pub const BLB1: u8 = 0x30;

/// Bitfield on register `LOW`
pub const CKOUT: u8 = 0x40;

/// Bitfield on register `LOW`
pub const CKDIV8: u8 = 0x80;

/// Bitfield on register `LOW`
pub const SUT_CKSEL: u8 = 0x3F;

/// Bitfield on register `MCUCR`
pub const PUD: u8 = 0x10;

/// Bitfield on register `MCUCR`
pub const IVCE: u8 = 0x1;

/// Bitfield on register `MCUCR`
pub const IVSEL: u8 = 0x2;

/// Bitfield on register `MCUSR`
pub const JTRF: u8 = 0x10;

/// Bitfield on register `MCUSR`
pub const WDRF: u8 = 0x8;

/// Bitfield on register `MCUSR`
pub const EXTRF: u8 = 0x2;

/// Bitfield on register `MCUSR`
pub const BORF: u8 = 0x4;

/// Bitfield on register `MCUSR`
pub const PORF: u8 = 0x1;

/// Bitfield on register `RAMPZ`
pub const RAMPZ0: u8 = 0x1;

/// Bitfield on register `SMCR`
pub const SE: u8 = 0x1;

/// Bitfield on register `SMCR`
pub const SM: u8 = 0xE;

/// Bitfield on register `SPCR`
pub const SPE: u8 = 0x40;

/// Bitfield on register `SPCR`
pub const CPOL: u8 = 0x8;

/// Bitfield on register `SPCR`
pub const CPHA: u8 = 0x4;

/// Bitfield on register `SPCR`
pub const MSTR: u8 = 0x10;

/// Bitfield on register `SPCR`
pub const SPR: u8 = 0x3;

/// Bitfield on register `SPCR`
pub const SPIE: u8 = 0x80;

/// Bitfield on register `SPCR`
pub const DORD: u8 = 0x20;

/// Bitfield on register `SPMCSR`
pub const BLBSET: u8 = 0x8;

/// Bitfield on register `SPMCSR`
pub const PGWRT: u8 = 0x4;

/// Bitfield on register `SPMCSR`
pub const RWWSB: u8 = 0x40;

/// Bitfield on register `SPMCSR`
pub const SPMIE: u8 = 0x80;

/// Bitfield on register `SPMCSR`
pub const RWWSRE: u8 = 0x10;

/// Bitfield on register `SPMCSR`
pub const SPMEN: u8 = 0x1;

/// Bitfield on register `SPMCSR`
pub const PGERS: u8 = 0x2;

/// Bitfield on register `SPSR`
pub const WCOL: u8 = 0x40;

/// Bitfield on register `SPSR`
pub const SPI2X: u8 = 0x1;

/// Bitfield on register `SPSR`
pub const SPIF: u8 = 0x80;

/// Bitfield on register `SREG`
pub const S: u8 = 0x10;

/// Bitfield on register `SREG`
pub const Z: u8 = 0x2;

/// Bitfield on register `SREG`
pub const V: u8 = 0x8;

/// Bitfield on register `SREG`
pub const C: u8 = 0x1;

/// Bitfield on register `SREG`
pub const I: u8 = 0x80;

/// Bitfield on register `SREG`
pub const N: u8 = 0x4;

/// Bitfield on register `SREG`
pub const T: u8 = 0x40;

/// Bitfield on register `SREG`
pub const H: u8 = 0x20;

/// Bitfield on register `TCCR0A`
pub const WGM00: u8 = 0x40;

/// Bitfield on register `TCCR0A`
pub const COM0A: u8 = 0x30;

/// Bitfield on register `TCCR0A`
pub const WGM01: u8 = 0x8;

/// Bitfield on register `TCCR0A`
pub const FOC0A: u8 = 0x80;

/// Bitfield on register `TCCR0A`
pub const CS0: u8 = 0x7;

/// Bitfield on register `TCCR1A`
pub const COM1B: u8 = 0x30;

/// Bitfield on register `TCCR1A`
pub const COM1A: u8 = 0xC0;

/// Bitfield on register `TCCR1A`
pub const COM1C: u8 = 0xC;

/// Bitfield on register `TCCR1B`
pub const ICES1: u8 = 0x40;

/// Bitfield on register `TCCR1B`
pub const ICNC1: u8 = 0x80;

/// Bitfield on register `TCCR1B`
pub const CS1: u8 = 0x7;

/// Bitfield on register `TCCR1C`
pub const FOC1B: u8 = 0x40;

/// Bitfield on register `TCCR1C`
pub const FOC1C: u8 = 0x20;

/// Bitfield on register `TCCR1C`
pub const FOC1A: u8 = 0x80;

/// Bitfield on register `TCCR2A`
pub const CS2: u8 = 0x7;

/// Bitfield on register `TCCR2A`
pub const COM2A: u8 = 0x30;

/// Bitfield on register `TCCR2A`
pub const WGM21: u8 = 0x8;

/// Bitfield on register `TCCR2A`
pub const WGM20: u8 = 0x40;

/// Bitfield on register `TCCR2A`
pub const FOC2A: u8 = 0x80;

/// Bitfield on register `TCCR3A`
pub const COM3A: u8 = 0xC0;

/// Bitfield on register `TCCR3A`
pub const COM3B: u8 = 0x30;

/// Bitfield on register `TCCR3A`
pub const COM3C: u8 = 0xC;

/// Bitfield on register `TCCR3B`
pub const ICNC3: u8 = 0x80;

/// Bitfield on register `TCCR3B`
pub const ICES3: u8 = 0x40;

/// Bitfield on register `TCCR3B`
pub const CS3: u8 = 0x7;

/// Bitfield on register `TCCR3C`
pub const FOC3B: u8 = 0x40;

/// Bitfield on register `TCCR3C`
pub const FOC3C: u8 = 0x20;

/// Bitfield on register `TCCR3C`
pub const FOC3A: u8 = 0x80;

/// Bitfield on register `TIFR0`
pub const TOV0: u8 = 0x1;

/// Bitfield on register `TIFR0`
pub const OCF0A: u8 = 0x2;

/// Bitfield on register `TIFR1`
pub const OCF1C: u8 = 0x8;

/// Bitfield on register `TIFR1`
pub const ICF1: u8 = 0x20;

/// Bitfield on register `TIFR1`
pub const OCF1A: u8 = 0x2;

/// Bitfield on register `TIFR1`
pub const OCF1B: u8 = 0x4;

/// Bitfield on register `TIFR1`
pub const TOV1: u8 = 0x1;

/// Bitfield on register `TIFR2`
pub const TOV2: u8 = 0x1;

/// Bitfield on register `TIFR2`
pub const OCF2A: u8 = 0x2;

/// Bitfield on register `TIFR3`
pub const OCF3B: u8 = 0x4;

/// Bitfield on register `TIFR3`
pub const OCF3A: u8 = 0x2;

/// Bitfield on register `TIFR3`
pub const TOV3: u8 = 0x1;

/// Bitfield on register `TIFR3`
pub const ICF3: u8 = 0x20;

/// Bitfield on register `TIFR3`
pub const OCF3C: u8 = 0x8;

/// Bitfield on register `TIMSK0`
pub const TOIE0: u8 = 0x1;

/// Bitfield on register `TIMSK0`
pub const OCIE0A: u8 = 0x2;

/// Bitfield on register `TIMSK1`
pub const OCIE1A: u8 = 0x2;

/// Bitfield on register `TIMSK1`
pub const OCIE1B: u8 = 0x4;

/// Bitfield on register `TIMSK1`
pub const TOIE1: u8 = 0x1;

/// Bitfield on register `TIMSK1`
pub const OCIE1C: u8 = 0x8;

/// Bitfield on register `TIMSK1`
pub const ICIE1: u8 = 0x20;

/// Bitfield on register `TIMSK2`
pub const OCIE2A: u8 = 0x2;

/// Bitfield on register `TIMSK2`
pub const TOIE2: u8 = 0x1;

/// Bitfield on register `TIMSK3`
pub const ICIE3: u8 = 0x20;

/// Bitfield on register `TIMSK3`
pub const OCIE3A: u8 = 0x2;

/// Bitfield on register `TIMSK3`
pub const OCIE3C: u8 = 0x8;

/// Bitfield on register `TIMSK3`
pub const OCIE3B: u8 = 0x4;

/// Bitfield on register `TIMSK3`
pub const TOIE3: u8 = 0x1;

/// Bitfield on register `TWAR`
pub const TWGCE: u8 = 0x1;

/// Bitfield on register `TWAR`
pub const TWA: u8 = 0xFE;

/// Bitfield on register `TWCR`
pub const TWWC: u8 = 0x8;

/// Bitfield on register `TWCR`
pub const TWEN: u8 = 0x4;

/// Bitfield on register `TWCR`
pub const TWIE: u8 = 0x1;

/// Bitfield on register `TWCR`
pub const TWINT: u8 = 0x80;

/// Bitfield on register `TWCR`
pub const TWEA: u8 = 0x40;

/// Bitfield on register `TWCR`
pub const TWSTO: u8 = 0x10;

/// Bitfield on register `TWCR`
pub const TWSTA: u8 = 0x20;

/// Bitfield on register `TWSR`
pub const TWPS: u8 = 0x3;

/// Bitfield on register `TWSR`
pub const TWS: u8 = 0xF8;

/// Bitfield on register `UCSR0A`
pub const U2X0: u8 = 0x2;

/// Bitfield on register `UCSR0A`
pub const MPCM0: u8 = 0x1;

/// Bitfield on register `UCSR0A`
pub const TXC0: u8 = 0x40;

/// Bitfield on register `UCSR0A`
pub const RXC0: u8 = 0x80;

/// Bitfield on register `UCSR0A`
pub const DOR0: u8 = 0x8;

/// Bitfield on register `UCSR0A`
pub const FE0: u8 = 0x10;

/// Bitfield on register `UCSR0A`
pub const UPE0: u8 = 0x4;

/// Bitfield on register `UCSR0A`
pub const UDRE0: u8 = 0x20;

/// Bitfield on register `UCSR0B`
pub const TXB80: u8 = 0x1;

/// Bitfield on register `UCSR0B`
pub const TXEN0: u8 = 0x8;

/// Bitfield on register `UCSR0B`
pub const TXCIE0: u8 = 0x40;

/// Bitfield on register `UCSR0B`
pub const RXEN0: u8 = 0x10;

/// Bitfield on register `UCSR0B`
pub const UDRIE0: u8 = 0x20;

/// Bitfield on register `UCSR0B`
pub const UCSZ02: u8 = 0x4;

/// Bitfield on register `UCSR0B`
pub const RXCIE0: u8 = 0x80;

/// Bitfield on register `UCSR0B`
pub const RXB80: u8 = 0x2;

/// Bitfield on register `UCSR0C`
pub const UMSEL0: u8 = 0x40;

/// Bitfield on register `UCSR0C`
pub const UPM0: u8 = 0x30;

/// Bitfield on register `UCSR0C`
pub const UCSZ0: u8 = 0x6;

/// Bitfield on register `UCSR0C`
pub const USBS0: u8 = 0x8;

/// Bitfield on register `UCSR0C`
pub const UCPOL0: u8 = 0x1;

/// Bitfield on register `UCSR1A`
pub const U2X1: u8 = 0x2;

/// Bitfield on register `UCSR1A`
pub const DOR1: u8 = 0x8;

/// Bitfield on register `UCSR1A`
pub const UDRE1: u8 = 0x20;

/// Bitfield on register `UCSR1A`
pub const MPCM1: u8 = 0x1;

/// Bitfield on register `UCSR1A`
pub const UPE1: u8 = 0x4;

/// Bitfield on register `UCSR1A`
pub const FE1: u8 = 0x10;

/// Bitfield on register `UCSR1A`
pub const RXC1: u8 = 0x80;

/// Bitfield on register `UCSR1A`
pub const TXC1: u8 = 0x40;

/// Bitfield on register `UCSR1B`
pub const TXB81: u8 = 0x1;

/// Bitfield on register `UCSR1B`
pub const TXCIE1: u8 = 0x40;

/// Bitfield on register `UCSR1B`
pub const UDRIE1: u8 = 0x20;

/// Bitfield on register `UCSR1B`
pub const RXCIE1: u8 = 0x80;

/// Bitfield on register `UCSR1B`
pub const UCSZ12: u8 = 0x4;

/// Bitfield on register `UCSR1B`
pub const RXB81: u8 = 0x2;

/// Bitfield on register `UCSR1B`
pub const RXEN1: u8 = 0x10;

/// Bitfield on register `UCSR1B`
pub const TXEN1: u8 = 0x8;

/// Bitfield on register `UCSR1C`
pub const UPM1: u8 = 0x30;

/// Bitfield on register `UCSR1C`
pub const UCPOL1: u8 = 0x1;

/// Bitfield on register `UCSR1C`
pub const UCSZ1: u8 = 0x6;

/// Bitfield on register `UCSR1C`
pub const USBS1: u8 = 0x8;

/// Bitfield on register `UCSR1C`
pub const UMSEL1: u8 = 0x40;

/// Bitfield on register `WDTCR`
pub const WDE: u8 = 0x8;

/// Bitfield on register `WDTCR`
pub const WDP: u8 = 0x7;

/// Bitfield on register `WDTCR`
pub const WDCE: u8 = 0x10;

/// Bitfield on register `XMCRA`
pub const SRL: u8 = 0x70;

/// Bitfield on register `XMCRA`
pub const SRW1: u8 = 0xC;

/// Bitfield on register `XMCRA`
pub const SRW0: u8 = 0x3;

/// Bitfield on register `XMCRA`
pub const SRE: u8 = 0x80;

/// Bitfield on register `XMCRB`
pub const XMBK: u8 = 0x80;

/// Bitfield on register `XMCRB`
pub const XMM: u8 = 0x7;

/// `ANALOG_ADC_AUTO_TRIGGER2` value group
#[allow(non_upper_case_globals)]
pub mod analog_adc_auto_trigger2 {
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
   /// Timer/Counter1 Capture Event.
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

/// `ANALOG_ADC_V_REF2` value group
#[allow(non_upper_case_globals)]
pub mod analog_adc_v_ref2 {
   /// AREF, Internal Vref turned off.
   pub const VAL_0x00: u32 = 0x0;
   /// AVCC with external capacitor at AREF pin.
   pub const VAL_0x01: u32 = 0x1;
   /// Reserved.
   pub const VAL_0x02: u32 = 0x2;
   /// Internal 2.56V Voltage Reference with external capacitor at AREF pin.
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

/// `CPU_SECTOR_LIMITS_XMEM` value group
#[allow(non_upper_case_globals)]
pub mod cpu_sector_limits_xmem {
   /// LS = N/A, US = 0x1100 - 0xFFFF.
   pub const VAL_0x00: u32 = 0x0;
   /// LS = XMEMSTART - 0x1FFF, US = 0x2000 - 0xFFFF.
   pub const VAL_0x01: u32 = 0x1;
   /// LS = XMEMSTART - 0x3FFF, US = 0x4000 - 0xFFFF.
   pub const VAL_0x02: u32 = 0x2;
   /// LS = XMEMSTART - 0x5FFF, US = 0x6000 - 0xFFFF.
   pub const VAL_0x03: u32 = 0x3;
   /// LS = XMEMSTART - 0x7FFF, US = 0x8000 - 0xFFFF.
   pub const VAL_0x04: u32 = 0x4;
   /// LS = XMEMSTART - 0x9FFF, US = 0xA000 - 0xFFFF.
   pub const VAL_0x05: u32 = 0x5;
   /// LS = XMEMSTART - 0xBFFF, US = 0xC000 - 0xFFFF.
   pub const VAL_0x06: u32 = 0x6;
   /// LS = XMEMSTART - 0xDFFF, US = 0xE000 - 0xFFFF.
   pub const VAL_0x07: u32 = 0x7;
}

/// `CPU_SLEEP_MODE_3BITS2` value group
#[allow(non_upper_case_globals)]
pub mod cpu_sleep_mode_3bits2 {
   /// Idle.
   pub const IDLE: u32 = 0x0;
   /// ADC Noise Reduction (If Available).
   pub const ADC: u32 = 0x1;
   /// Power Down.
   pub const PDOWN: u32 = 0x2;
   /// Power Save.
   pub const PSAVE: u32 = 0x3;
   /// Reserved.
   pub const VAL_0x04: u32 = 0x4;
   /// Reserved.
   pub const VAL_0x05: u32 = 0x5;
   /// Standby.
   pub const STDBY: u32 = 0x6;
   /// Reserved.
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
   /// Brown-out detection disabled.
   pub const DISABLED: u32 = 0x7;
   /// Brown-out detection level at VCC=4.1 V.
   pub const _4V1: u32 = 0x6;
   /// Brown-out detection level at VCC=4.0 V.
   pub const _4V0: u32 = 0x5;
   /// Brown-out detection level at VCC=3.9 V.
   pub const _3V9: u32 = 0x4;
   /// Brown-out detection level at VCC=3.8 V.
   pub const _3V8: u32 = 0x3;
   /// Brown-out detection level at VCC=2.7 V.
   pub const _2V7: u32 = 0x2;
   /// Brown-out detection level at VCC=2.6 V.
   pub const _2V6: u32 = 0x1;
   /// Brown-out detection level at VCC=2.5 V.
   pub const _2V5: u32 = 0x0;
}

/// `ENUM_BOOTSZ` value group
#[allow(non_upper_case_globals)]
pub mod enum_bootsz {
   /// Boot Flash size=512 words start address=$7E00.
   pub const _512W_7E00: u32 = 0x3;
   /// Boot Flash size=1024 words start address=$7C00.
   pub const _1024W_7C00: u32 = 0x2;
   /// Boot Flash size=2048 words start address=$7800.
   pub const _2048W_7800: u32 = 0x1;
   /// Boot Flash size=4096 words start address=$7000.
   pub const _4096W_7000: u32 = 0x0;
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
   /// Ext. Clock; Start-up time: 6 CK + 4.1 ms.
   pub const EXTCLK_6CK_4MS1: u32 = 0x10;
   /// Ext. Clock; Start-up time: 6 CK + 65 ms.
   pub const EXTCLK_6CK_65MS: u32 = 0x20;
   /// Int. RC Osc.; Start-up time: 6 CK + 0 ms.
   pub const INTRCOSC_6CK_0MS: u32 = 0x2;
   /// Int. RC Osc.; Start-up time: 6 CK + 4.1 ms.
   pub const INTRCOSC_6CK_4MS1: u32 = 0x12;
   /// Int. RC Osc.; Start-up time: 6 CK + 65 ms.
   pub const INTRCOSC_6CK_65MS: u32 = 0x22;
   /// Ext. Low-Freq. Crystal; Start-up time: 32K CK + 0 ms; Int. Cap.
   pub const EXTLOFXTAL_32KCK_0MS_INTCAP: u32 = 0x7;
   /// Ext. Low-Freq. Crystal; Start-up time: 32K CK + 4.1 ms; Int. Cap.
   pub const EXTLOFXTAL_32KCK_4MS1_INTCAP: u32 = 0x17;
   /// Ext. Low-Freq. Crystal; Start-up time: 32K CK + 65 ms; Int. Cap.
   pub const EXTLOFXTAL_32KCK_65MS_INTCAP: u32 = 0x27;
   /// Ext. Low-Freq. Crystal; Start-up time: 1K CK + 0 ms; Int. Cap.
   pub const EXTLOFXTAL_1KCK_0MS_INTCAP: u32 = 0x6;
   /// Ext. Low-Freq. Crystal; Start-up time: 1K CK + 4.1 ms; Int. Cap.
   pub const EXTLOFXTAL_1KCK_4MS1_INTCAP: u32 = 0x16;
   /// Ext. Low-Freq. Crystal; Start-up time: 1K CK + 65 ms; Int. Cap.
   pub const EXTLOFXTAL_1KCK_65MS_INTCAP: u32 = 0x26;
   /// Ext. Low-Freq. Crystal; Start-up time: 32K CK + 0 ms.
   pub const EXTLOFXTAL_32KCK_0MS: u32 = 0x5;
   /// Ext. Low-Freq. Crystal; Start-up time: 32K CK + 4.1 ms.
   pub const EXTLOFXTAL_32KCK_4MS1: u32 = 0x15;
   /// Ext. Low-Freq. Crystal; Start-up time: 32K CK + 65 ms.
   pub const EXTLOFXTAL_32KCK_65MS: u32 = 0x25;
   /// Ext. Low-Freq. Crystal; Start-up time: 1K CK + 0 ms.
   pub const EXTLOFXTAL_1KCK_0MS: u32 = 0x4;
   /// Ext. Low-Freq. Crystal; Start-up time: 1K CK + 4.1 ms.
   pub const EXTLOFXTAL_1KCK_4MS1: u32 = 0x14;
   /// Ext. Low-Freq. Crystal; Start-up time: 1K CK + 65 ms.
   pub const EXTLOFXTAL_1KCK_65MS: u32 = 0x24;
   /// Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time: 258 CK + 4.1 ms.
   pub const EXTXOSC_0MHZ4_0MHZ9_258CK_4MS1: u32 = 0x8;
   /// Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time: 258 CK + 65 ms.
   pub const EXTXOSC_0MHZ4_0MHZ9_258CK_65MS: u32 = 0x18;
   /// Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time: 1K CK + 0 ms.
   pub const EXTXOSC_0MHZ4_0MHZ9_1KCK_0MS: u32 = 0x28;
   /// Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time: 1K CK + 4.1 ms.
   pub const EXTXOSC_0MHZ4_0MHZ9_1KCK_4MS1: u32 = 0x38;
   /// Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time: 1K CK + 65 ms.
   pub const EXTXOSC_0MHZ4_0MHZ9_1KCK_65MS: u32 = 0x9;
   /// Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time: 258 CK + 4.1 ms.
   pub const EXTXOSC_0MHZ9_3MHZ_258CK_4MS1: u32 = 0xA;
   /// Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time: 258 CK + 65 ms.
   pub const EXTXOSC_0MHZ9_3MHZ_258CK_65MS: u32 = 0x1A;
   /// Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time: 1K CK + 0 ms.
   pub const EXTXOSC_0MHZ9_3MHZ_1KCK_0MS: u32 = 0x2A;
   /// Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time: 1K CK + 4.1 ms.
   pub const EXTXOSC_0MHZ9_3MHZ_1KCK_4MS1: u32 = 0x3A;
   /// Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time: 1K CK + 65 ms.
   pub const EXTXOSC_0MHZ9_3MHZ_1KCK_65MS: u32 = 0xB;
   /// Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time: 258 CK + 4.1 ms.
   pub const EXTXOSC_3MHZ_8MHZ_258CK_4MS1: u32 = 0xC;
   /// Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time: 258 CK + 65 ms.
   pub const EXTXOSC_3MHZ_8MHZ_258CK_65MS: u32 = 0x1C;
   /// Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time: 1K CK + 0 ms.
   pub const EXTXOSC_3MHZ_8MHZ_1KCK_0MS: u32 = 0x2C;
   /// Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time: 1K CK + 4.1 ms.
   pub const EXTXOSC_3MHZ_8MHZ_1KCK_4MS1: u32 = 0x3C;
   /// Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time: 1K CK + 65 ms.
   pub const EXTXOSC_3MHZ_8MHZ_1KCK_65MS: u32 = 0xD;
   /// Ext. Crystal Osc. 8.0-16.0 MHz; Start-up time: 258 CK + 4.1 ms.
   pub const EXTXOSC_8MHZ_16MHZ_258CK_4MS1: u32 = 0xE;
   /// Ext. Crystal Osc. 8.0-16.0 MHz; Start-up time: 258 CK + 65 ms.
   pub const EXTXOSC_8MHZ_16MHZ_258CK_65MS: u32 = 0x1E;
   /// Ext. Crystal Osc. 8.0-16.0 MHz; Start-up time: 1K CK + 0 ms.
   pub const EXTXOSC_8MHZ_16MHZ_1KCK_0MS: u32 = 0x2E;
   /// Ext. Crystal Osc. 8.0-16.0 MHz; Start-up time: 1K CK + 4.1 ms.
   pub const EXTXOSC_8MHZ_16MHZ_1KCK_4MS1: u32 = 0x3E;
   /// Ext. Crystal Osc. 8.0-16.0 MHz; Start-up time: 1K CK + 65 ms.
   pub const EXTXOSC_8MHZ_16MHZ_1KCK_65MS: u32 = 0xF;
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

/// Oscillator Calibration Values
#[allow(non_upper_case_globals)]
pub mod osccal_value_addresses {
   /// 8.0 MHz.
   pub const _8_0_MHz: u32 = 0x0;
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

