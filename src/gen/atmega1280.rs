//! The AVR ATmega1280 microcontroller
//!
//! # Variants
//! |        | Pinout | Mcu age | Operating temperature | Operating voltage | Max speed |
//! |--------|--------|---------|-----------------------|-------------------|-----------|
//! | ATmega1280V-8AU | TQFP | TQFP100 | -40°C - 85°C | 1.8V - 5.5V | 8 MHz |
//! | ATmega1280V-8CU | CBGA | CBGA100 | -40°C - 85°C | 1.8V - 5.5V | 8 MHz |
//! | ATmega1280-16AU | TQFP | TQFP100 | -40°C - 85°C | 2.7V - 5.5V | 16 MHz |
//! | ATmega1280-16CU | CBGA | CBGA100 | -40°C - 85°C | 2.7V - 5.5V | 16 MHz |
//!

#![allow(non_upper_case_globals)]

/// `LOW` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SUT_CKSEL | 111111 |
/// | CKOUT | 1000000 |
/// | CKDIV8 | 10000000 |
pub const LOW: *mut u8 = 0x0 as *mut u8;

/// `LOCKBIT` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BLB1 | 110000 |
/// | BLB0 | 1100 |
/// | LB | 11 |
pub const LOCKBIT: *mut u8 = 0x0 as *mut u8;

/// `HIGH` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EESAVE | 1000 |
/// | BOOTSZ | 110 |
/// | OCDEN | 10000000 |
/// | JTAGEN | 1000000 |
/// | WDTON | 10000 |
/// | BOOTRST | 1 |
/// | SPIEN | 100000 |
pub const HIGH: *mut u8 = 0x1 as *mut u8;

/// `EXTENDED` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BODLEVEL | 111 |
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
/// | OCF0A | 10 |
/// | OCF0B | 100 |
/// | TOV0 | 1 |
pub const TIFR0: *mut u8 = 0x35 as *mut u8;

/// Timer/Counter1 Interrupt Flag register.
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
/// | OCF2B | 100 |
pub const TIFR2: *mut u8 = 0x37 as *mut u8;

/// Timer/Counter3 Interrupt Flag register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCF3A | 10 |
/// | OCF3C | 1000 |
/// | OCF3B | 100 |
/// | TOV3 | 1 |
/// | ICF3 | 100000 |
pub const TIFR3: *mut u8 = 0x38 as *mut u8;

/// Timer/Counter4 Interrupt Flag register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ICF4 | 100000 |
/// | TOV4 | 1 |
/// | OCF4B | 100 |
/// | OCF4A | 10 |
/// | OCF4C | 1000 |
pub const TIFR4: *mut u8 = 0x39 as *mut u8;

/// Timer/Counter5 Interrupt Flag register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCF5B | 100 |
/// | OCF5A | 10 |
/// | OCF5C | 1000 |
/// | ICF5 | 100000 |
/// | TOV5 | 1 |
pub const TIFR5: *mut u8 = 0x3A as *mut u8;

/// Pin Change Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCIF | 111 |
pub const PCIFR: *mut u8 = 0x3B as *mut u8;

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
/// | GPIOR04 | 10000 |
/// | GPIOR07 | 10000000 |
/// | GPIOR06 | 1000000 |
/// | GPIOR00 | 1 |
/// | GPIOR01 | 10 |
/// | GPIOR02 | 100 |
/// | GPIOR05 | 100000 |
/// | GPIOR03 | 1000 |
pub const GPIOR0: *mut u8 = 0x3E as *mut u8;

/// EEPROM Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EERIE | 1000 |
/// | EERE | 1 |
/// | EEPM | 110000 |
/// | EEMPE | 100 |
/// | EEPE | 10 |
pub const EECR: *mut u8 = 0x3F as *mut u8;

/// EEPROM Data Register.
pub const EEDR: *mut u8 = 0x40 as *mut u8;

/// EEPROM Address Register Low Bytes.
pub const EEAR: *mut u16 = 0x41 as *mut u16;

/// EEPROM Address Register Low Bytes low byte.
pub const EEARL: *mut u8 = 0x41 as *mut u8;

/// EEPROM Address Register Low Bytes high byte.
pub const EEARH: *mut u8 = 0x42 as *mut u8;

/// General Timer Counter Control register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TSM | 10000000 |
/// | PSRASY | 10 |
pub const GTCCR: *mut u8 = 0x43 as *mut u8;

/// Timer/Counter  Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WGM0 | 11 |
/// | COM0A | 11000000 |
/// | COM0B | 110000 |
pub const TCCR0A: *mut u8 = 0x44 as *mut u8;

/// Timer/Counter Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WGM02 | 1000 |
/// | CS0 | 111 |
/// | FOC0A | 10000000 |
/// | FOC0B | 1000000 |
pub const TCCR0B: *mut u8 = 0x45 as *mut u8;

/// Timer/Counter0.
pub const TCNT0: *mut u8 = 0x46 as *mut u8;

/// Timer/Counter0 Output Compare Register.
pub const OCR0A: *mut u8 = 0x47 as *mut u8;

/// Timer/Counter0 Output Compare Register.
pub const OCR0B: *mut u8 = 0x48 as *mut u8;

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
/// | SPIE | 10000000 |
/// | SPE | 1000000 |
/// | MSTR | 10000 |
/// | DORD | 100000 |
/// | SPR | 11 |
/// | CPOL | 1000 |
pub const SPCR: *mut u8 = 0x4C as *mut u8;

/// SPI Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WCOL | 1000000 |
/// | SPIF | 10000000 |
/// | SPI2X | 1 |
pub const SPSR: *mut u8 = 0x4D as *mut u8;

/// SPI Data Register.
pub const SPDR: *mut u8 = 0x4E as *mut u8;

/// Analog Comparator Control And Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ACI | 10000 |
/// | ACIC | 100 |
/// | ACD | 10000000 |
/// | ACIS | 11 |
/// | ACO | 100000 |
/// | ACIE | 1000 |
/// | ACBG | 1000000 |
pub const ACSR: *mut u8 = 0x50 as *mut u8;

/// On-Chip Debug Related Register in I/O Memory.
pub const OCDR: *mut u8 = 0x51 as *mut u8;

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
/// | PORF | 1 |
/// | WDRF | 1000 |
/// | EXTRF | 10 |
/// | JTRF | 10000 |
/// | BORF | 100 |
pub const MCUSR: *mut u8 = 0x54 as *mut u8;

/// MCU Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | IVCE | 1 |
/// | PUD | 10000 |
/// | JTD | 10000000 |
/// | IVSEL | 10 |
pub const MCUCR: *mut u8 = 0x55 as *mut u8;

/// Store Program Memory Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BLBSET | 1000 |
/// | PGERS | 10 |
/// | RWWSRE | 10000 |
/// | SIGRD | 100000 |
/// | SPMEN | 1 |
/// | RWWSB | 1000000 |
/// | PGWRT | 100 |
/// | SPMIE | 10000000 |
pub const SPMCSR: *mut u8 = 0x57 as *mut u8;

/// RAM Page Z Select Register.
pub const RAMPZ: *mut u8 = 0x5B as *mut u8;

/// Extended Indirect Register.
pub const EIND: *mut u8 = 0x5C as *mut u8;

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
/// | I | 10000000 |
/// | T | 1000000 |
/// | V | 1000 |
/// | C | 1 |
/// | S | 10000 |
/// | N | 100 |
/// | Z | 10 |
/// | H | 100000 |
pub const SREG: *mut u8 = 0x5F as *mut u8;

/// Watchdog Timer Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDP | 100111 |
/// | WDE | 1000 |
/// | WDCE | 10000 |
/// | WDIF | 10000000 |
/// | WDIE | 1000000 |
pub const WDTCSR: *mut u8 = 0x60 as *mut u8;

/// `CLKPR` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CLKPCE | 10000000 |
/// | CLKPS | 1111 |
pub const CLKPR: *mut u8 = 0x61 as *mut u8;

/// Power Reduction Register0.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRTIM0 | 100000 |
/// | PRTIM1 | 1000 |
/// | PRTWI | 10000000 |
/// | PRTIM2 | 1000000 |
/// | PRSPI | 100 |
/// | PRADC | 1 |
/// | PRUSART0 | 10 |
pub const PRR0: *mut u8 = 0x64 as *mut u8;

/// Power Reduction Register1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRTIM3 | 1000 |
/// | PRTIM4 | 10000 |
/// | PRTIM5 | 100000 |
/// | PRUSART3 | 100 |
/// | PRUSART2 | 10 |
/// | PRUSART1 | 1 |
pub const PRR1: *mut u8 = 0x65 as *mut u8;

/// Oscillator Calibration Value.
pub const OSCCAL: *mut u8 = 0x66 as *mut u8;

/// Pin Change Interrupt Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCIE | 111 |
pub const PCICR: *mut u8 = 0x68 as *mut u8;

/// External Interrupt Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ISC3 | 11000000 |
/// | ISC1 | 1100 |
/// | ISC2 | 110000 |
/// | ISC0 | 11 |
pub const EICRA: *mut u8 = 0x69 as *mut u8;

/// External Interrupt Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ISC5 | 1100 |
/// | ISC7 | 11000000 |
/// | ISC6 | 110000 |
/// | ISC4 | 11 |
pub const EICRB: *mut u8 = 0x6A as *mut u8;

/// Pin Change Mask Register 0.
pub const PCMSK0: *mut u8 = 0x6B as *mut u8;

/// Pin Change Mask Register 1.
pub const PCMSK1: *mut u8 = 0x6C as *mut u8;

/// Pin Change Mask Register 2.
pub const PCMSK2: *mut u8 = 0x6D as *mut u8;

/// Timer/Counter0 Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCIE0A | 10 |
/// | OCIE0B | 100 |
/// | TOIE0 | 1 |
pub const TIMSK0: *mut u8 = 0x6E as *mut u8;

/// Timer/Counter1 Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TOIE1 | 1 |
/// | OCIE1C | 1000 |
/// | ICIE1 | 100000 |
/// | OCIE1A | 10 |
/// | OCIE1B | 100 |
pub const TIMSK1: *mut u8 = 0x6F as *mut u8;

/// Timer/Counter Interrupt Mask register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCIE2A | 10 |
/// | OCIE2B | 100 |
/// | TOIE2 | 1 |
pub const TIMSK2: *mut u8 = 0x70 as *mut u8;

/// Timer/Counter3 Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCIE3C | 1000 |
/// | OCIE3A | 10 |
/// | OCIE3B | 100 |
/// | TOIE3 | 1 |
/// | ICIE3 | 100000 |
pub const TIMSK3: *mut u8 = 0x71 as *mut u8;

/// Timer/Counter4 Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCIE4A | 10 |
/// | OCIE4B | 100 |
/// | TOIE4 | 1 |
/// | ICIE4 | 100000 |
/// | OCIE4C | 1000 |
pub const TIMSK4: *mut u8 = 0x72 as *mut u8;

/// Timer/Counter5 Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ICIE5 | 100000 |
/// | OCIE5B | 100 |
/// | OCIE5A | 10 |
/// | OCIE5C | 1000 |
/// | TOIE5 | 1 |
pub const TIMSK5: *mut u8 = 0x73 as *mut u8;

/// External Memory Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SRL | 1110000 |
/// | SRW0 | 11 |
/// | SRW1 | 1100 |
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

/// The ADC Control and Status register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADIE | 1000 |
/// | ADIF | 10000 |
/// | ADPS | 111 |
/// | ADEN | 10000000 |
/// | ADATE | 100000 |
/// | ADSC | 1000000 |
pub const ADCSRA: *mut u8 = 0x7A as *mut u8;

/// The ADC Control and Status register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | MUX5 | 1000 |
/// | ADTS | 111 |
/// | ACME | 1000000 |
pub const ADCSRB: *mut u8 = 0x7B as *mut u8;

/// The ADC multiplexer Selection Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADLAR | 100000 |
/// | REFS | 11000000 |
/// | MUX | 11111 |
pub const ADMUX: *mut u8 = 0x7C as *mut u8;

/// Digital Input Disable Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADC8D | 1 |
/// | ADC15D | 10000000 |
/// | ADC13D | 100000 |
/// | ADC9D | 10 |
/// | ADC11D | 1000 |
/// | ADC12D | 10000 |
/// | ADC14D | 1000000 |
/// | ADC10D | 100 |
pub const DIDR2: *mut u8 = 0x7D as *mut u8;

/// Digital Input Disable Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADC2D | 100 |
/// | ADC3D | 1000 |
/// | ADC4D | 10000 |
/// | ADC5D | 100000 |
/// | ADC1D | 10 |
/// | ADC6D | 1000000 |
/// | ADC0D | 1 |
/// | ADC7D | 10000000 |
pub const DIDR0: *mut u8 = 0x7E as *mut u8;

/// Digital Input Disable Register 1.
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
/// | CS1 | 111 |
/// | ICNC1 | 10000000 |
pub const TCCR1B: *mut u8 = 0x81 as *mut u8;

/// Timer/Counter 1 Control Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FOC1A | 10000000 |
/// | FOC1C | 100000 |
/// | FOC1B | 1000000 |
pub const TCCR1C: *mut u8 = 0x82 as *mut u8;

/// Timer/Counter1  Bytes low byte.
pub const TCNT1L: *mut u8 = 0x84 as *mut u8;

/// Timer/Counter1  Bytes.
pub const TCNT1: *mut u16 = 0x84 as *mut u16;

/// Timer/Counter1  Bytes high byte.
pub const TCNT1H: *mut u8 = 0x85 as *mut u8;

/// Timer/Counter1 Input Capture Register  Bytes low byte.
pub const ICR1L: *mut u8 = 0x86 as *mut u8;

/// Timer/Counter1 Input Capture Register  Bytes.
pub const ICR1: *mut u16 = 0x86 as *mut u16;

/// Timer/Counter1 Input Capture Register  Bytes high byte.
pub const ICR1H: *mut u8 = 0x87 as *mut u8;

/// Timer/Counter1 Output Compare Register A  Bytes low byte.
pub const OCR1AL: *mut u8 = 0x88 as *mut u8;

/// Timer/Counter1 Output Compare Register A  Bytes.
pub const OCR1A: *mut u16 = 0x88 as *mut u16;

/// Timer/Counter1 Output Compare Register A  Bytes high byte.
pub const OCR1AH: *mut u8 = 0x89 as *mut u8;

/// Timer/Counter1 Output Compare Register B  Bytes low byte.
pub const OCR1BL: *mut u8 = 0x8A as *mut u8;

/// Timer/Counter1 Output Compare Register B  Bytes.
pub const OCR1B: *mut u16 = 0x8A as *mut u16;

/// Timer/Counter1 Output Compare Register B  Bytes high byte.
pub const OCR1BH: *mut u8 = 0x8B as *mut u8;

/// Timer/Counter1 Output Compare Register C  Bytes low byte.
pub const OCR1CL: *mut u8 = 0x8C as *mut u8;

/// Timer/Counter1 Output Compare Register C  Bytes.
pub const OCR1C: *mut u16 = 0x8C as *mut u16;

/// Timer/Counter1 Output Compare Register C  Bytes high byte.
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
/// | ICES3 | 1000000 |
/// | ICNC3 | 10000000 |
/// | CS3 | 111 |
pub const TCCR3B: *mut u8 = 0x91 as *mut u8;

/// Timer/Counter 3 Control Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FOC3A | 10000000 |
/// | FOC3C | 100000 |
/// | FOC3B | 1000000 |
pub const TCCR3C: *mut u8 = 0x92 as *mut u8;

/// Timer/Counter3  Bytes low byte.
pub const TCNT3L: *mut u8 = 0x94 as *mut u8;

/// Timer/Counter3  Bytes.
pub const TCNT3: *mut u16 = 0x94 as *mut u16;

/// Timer/Counter3  Bytes high byte.
pub const TCNT3H: *mut u8 = 0x95 as *mut u8;

/// Timer/Counter3 Input Capture Register  Bytes.
pub const ICR3: *mut u16 = 0x96 as *mut u16;

/// Timer/Counter3 Input Capture Register  Bytes low byte.
pub const ICR3L: *mut u8 = 0x96 as *mut u8;

/// Timer/Counter3 Input Capture Register  Bytes high byte.
pub const ICR3H: *mut u8 = 0x97 as *mut u8;

/// Timer/Counter3 Output Compare Register A  Bytes low byte.
pub const OCR3AL: *mut u8 = 0x98 as *mut u8;

/// Timer/Counter3 Output Compare Register A  Bytes.
pub const OCR3A: *mut u16 = 0x98 as *mut u16;

/// Timer/Counter3 Output Compare Register A  Bytes high byte.
pub const OCR3AH: *mut u8 = 0x99 as *mut u8;

/// Timer/Counter3 Output Compare Register B  Bytes.
pub const OCR3B: *mut u16 = 0x9A as *mut u16;

/// Timer/Counter3 Output Compare Register B  Bytes low byte.
pub const OCR3BL: *mut u8 = 0x9A as *mut u8;

/// Timer/Counter3 Output Compare Register B  Bytes high byte.
pub const OCR3BH: *mut u8 = 0x9B as *mut u8;

/// Timer/Counter3 Output Compare Register B  Bytes low byte.
pub const OCR3CL: *mut u8 = 0x9C as *mut u8;

/// Timer/Counter3 Output Compare Register B  Bytes.
pub const OCR3C: *mut u16 = 0x9C as *mut u16;

/// Timer/Counter3 Output Compare Register B  Bytes high byte.
pub const OCR3CH: *mut u8 = 0x9D as *mut u8;

/// Timer/Counter4 Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | COM4C | 1100 |
/// | COM4A | 11000000 |
/// | COM4B | 110000 |
pub const TCCR4A: *mut u8 = 0xA0 as *mut u8;

/// Timer/Counter4 Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CS4 | 111 |
/// | ICES4 | 1000000 |
/// | ICNC4 | 10000000 |
pub const TCCR4B: *mut u8 = 0xA1 as *mut u8;

/// Timer/Counter 4 Control Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FOC4B | 1000000 |
/// | FOC4C | 100000 |
/// | FOC4A | 10000000 |
pub const TCCR4C: *mut u8 = 0xA2 as *mut u8;

/// Timer/Counter4  Bytes low byte.
pub const TCNT4L: *mut u8 = 0xA4 as *mut u8;

/// Timer/Counter4  Bytes.
pub const TCNT4: *mut u16 = 0xA4 as *mut u16;

/// Timer/Counter4  Bytes high byte.
pub const TCNT4H: *mut u8 = 0xA5 as *mut u8;

/// Timer/Counter4 Input Capture Register  Bytes low byte.
pub const ICR4L: *mut u8 = 0xA6 as *mut u8;

/// Timer/Counter4 Input Capture Register  Bytes.
pub const ICR4: *mut u16 = 0xA6 as *mut u16;

/// Timer/Counter4 Input Capture Register  Bytes high byte.
pub const ICR4H: *mut u8 = 0xA7 as *mut u8;

/// Timer/Counter4 Output Compare Register A  Bytes low byte.
pub const OCR4AL: *mut u8 = 0xA8 as *mut u8;

/// Timer/Counter4 Output Compare Register A  Bytes.
pub const OCR4A: *mut u16 = 0xA8 as *mut u16;

/// Timer/Counter4 Output Compare Register A  Bytes high byte.
pub const OCR4AH: *mut u8 = 0xA9 as *mut u8;

/// Timer/Counter4 Output Compare Register B  Bytes low byte.
pub const OCR4BL: *mut u8 = 0xAA as *mut u8;

/// Timer/Counter4 Output Compare Register B  Bytes.
pub const OCR4B: *mut u16 = 0xAA as *mut u16;

/// Timer/Counter4 Output Compare Register B  Bytes high byte.
pub const OCR4BH: *mut u8 = 0xAB as *mut u8;

/// Timer/Counter4 Output Compare Register B  Bytes low byte.
pub const OCR4CL: *mut u8 = 0xAC as *mut u8;

/// Timer/Counter4 Output Compare Register B  Bytes.
pub const OCR4C: *mut u16 = 0xAC as *mut u16;

/// Timer/Counter4 Output Compare Register B  Bytes high byte.
pub const OCR4CH: *mut u8 = 0xAD as *mut u8;

/// Timer/Counter2 Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | COM2A | 11000000 |
/// | WGM2 | 11 |
/// | COM2B | 110000 |
pub const TCCR2A: *mut u8 = 0xB0 as *mut u8;

/// Timer/Counter2 Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FOC2B | 1000000 |
/// | CS2 | 111 |
/// | FOC2A | 10000000 |
/// | WGM22 | 1000 |
pub const TCCR2B: *mut u8 = 0xB1 as *mut u8;

/// Timer/Counter2.
pub const TCNT2: *mut u8 = 0xB2 as *mut u8;

/// Timer/Counter2 Output Compare Register A.
pub const OCR2A: *mut u8 = 0xB3 as *mut u8;

/// Timer/Counter2 Output Compare Register B.
pub const OCR2B: *mut u8 = 0xB4 as *mut u8;

/// Asynchronous Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCR2BUB | 100 |
/// | EXCLK | 1000000 |
/// | TCR2BUB | 1 |
/// | TCN2UB | 10000 |
/// | TCR2AUB | 10 |
/// | OCR2AUB | 1000 |
/// | AS2 | 100000 |
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
/// | TWEN | 100 |
/// | TWINT | 10000000 |
/// | TWSTA | 100000 |
/// | TWSTO | 10000 |
/// | TWWC | 1000 |
/// | TWEA | 1000000 |
/// | TWIE | 1 |
pub const TWCR: *mut u8 = 0xBC as *mut u8;

/// TWI (Slave) Address Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TWAM | 11111110 |
pub const TWAMR: *mut u8 = 0xBD as *mut u8;

/// USART Control and Status Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RXC0 | 10000000 |
/// | UPE0 | 100 |
/// | FE0 | 10000 |
/// | UDRE0 | 100000 |
/// | TXC0 | 1000000 |
/// | DOR0 | 1000 |
/// | MPCM0 | 1 |
/// | U2X0 | 10 |
pub const UCSR0A: *mut u8 = 0xC0 as *mut u8;

/// USART Control and Status Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RXCIE0 | 10000000 |
/// | TXEN0 | 1000 |
/// | UDRIE0 | 100000 |
/// | TXCIE0 | 1000000 |
/// | UCSZ02 | 100 |
/// | RXEN0 | 10000 |
/// | RXB80 | 10 |
/// | TXB80 | 1 |
pub const UCSR0B: *mut u8 = 0xC1 as *mut u8;

/// USART Control and Status Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | UPM0 | 110000 |
/// | UMSEL0 | 11000000 |
/// | UCPOL0 | 1 |
/// | UCSZ0 | 110 |
/// | USBS0 | 1000 |
pub const UCSR0C: *mut u8 = 0xC2 as *mut u8;

/// USART Baud Rate Register  Bytes.
pub const UBRR0: *mut u16 = 0xC4 as *mut u16;

/// USART Baud Rate Register  Bytes low byte.
pub const UBRR0L: *mut u8 = 0xC4 as *mut u8;

/// USART Baud Rate Register  Bytes high byte.
pub const UBRR0H: *mut u8 = 0xC5 as *mut u8;

/// USART I/O Data Register.
pub const UDR0: *mut u8 = 0xC6 as *mut u8;

/// USART Control and Status Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DOR1 | 1000 |
/// | UPE1 | 100 |
/// | FE1 | 10000 |
/// | UDRE1 | 100000 |
/// | U2X1 | 10 |
/// | TXC1 | 1000000 |
/// | RXC1 | 10000000 |
/// | MPCM1 | 1 |
pub const UCSR1A: *mut u8 = 0xC8 as *mut u8;

/// USART Control and Status Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RXB81 | 10 |
/// | UDRIE1 | 100000 |
/// | UCSZ12 | 100 |
/// | TXEN1 | 1000 |
/// | RXEN1 | 10000 |
/// | TXB81 | 1 |
/// | TXCIE1 | 1000000 |
/// | RXCIE1 | 10000000 |
pub const UCSR1B: *mut u8 = 0xC9 as *mut u8;

/// USART Control and Status Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | UMSEL1 | 11000000 |
/// | UCPOL1 | 1 |
/// | UPM1 | 110000 |
/// | USBS1 | 1000 |
/// | UCSZ1 | 110 |
pub const UCSR1C: *mut u8 = 0xCA as *mut u8;

/// USART Baud Rate Register  Bytes.
pub const UBRR1: *mut u16 = 0xCC as *mut u16;

/// USART Baud Rate Register  Bytes low byte.
pub const UBRR1L: *mut u8 = 0xCC as *mut u8;

/// USART Baud Rate Register  Bytes high byte.
pub const UBRR1H: *mut u8 = 0xCD as *mut u8;

/// USART I/O Data Register.
pub const UDR1: *mut u8 = 0xCE as *mut u8;

/// USART Control and Status Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RXC2 | 10000000 |
/// | TXC2 | 1000000 |
/// | UPE2 | 100 |
/// | DOR2 | 1000 |
/// | U2X2 | 10 |
/// | FE2 | 10000 |
/// | UDRE2 | 100000 |
/// | MPCM2 | 1 |
pub const UCSR2A: *mut u8 = 0xD0 as *mut u8;

/// USART Control and Status Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RXEN2 | 10000 |
/// | RXCIE2 | 10000000 |
/// | UDRIE2 | 100000 |
/// | RXB82 | 10 |
/// | TXEN2 | 1000 |
/// | TXCIE2 | 1000000 |
/// | TXB82 | 1 |
/// | UCSZ22 | 100 |
pub const UCSR2B: *mut u8 = 0xD1 as *mut u8;

/// USART Control and Status Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | USBS2 | 1000 |
/// | UCPOL2 | 1 |
/// | UMSEL2 | 11000000 |
/// | UPM2 | 110000 |
/// | UCSZ2 | 110 |
pub const UCSR2C: *mut u8 = 0xD2 as *mut u8;

/// USART Baud Rate Register  Bytes.
pub const UBRR2: *mut u16 = 0xD4 as *mut u16;

/// USART Baud Rate Register  Bytes low byte.
pub const UBRR2L: *mut u8 = 0xD4 as *mut u8;

/// USART Baud Rate Register  Bytes high byte.
pub const UBRR2H: *mut u8 = 0xD5 as *mut u8;

/// USART I/O Data Register.
pub const UDR2: *mut u8 = 0xD6 as *mut u8;

/// PORT H Input Pins.
pub const PINH: *mut u8 = 0x100 as *mut u8;

/// PORT H Data Direction Register.
pub const DDRH: *mut u8 = 0x101 as *mut u8;

/// PORT H Data Register.
pub const PORTH: *mut u8 = 0x102 as *mut u8;

/// PORT J Input Pins.
pub const PINJ: *mut u8 = 0x103 as *mut u8;

/// PORT J Data Direction Register.
pub const DDRJ: *mut u8 = 0x104 as *mut u8;

/// PORT J Data Register.
pub const PORTJ: *mut u8 = 0x105 as *mut u8;

/// PORT K Input Pins.
pub const PINK: *mut u8 = 0x106 as *mut u8;

/// PORT K Data Direction Register.
pub const DDRK: *mut u8 = 0x107 as *mut u8;

/// PORT K Data Register.
pub const PORTK: *mut u8 = 0x108 as *mut u8;

/// PORT L Input Pins.
pub const PINL: *mut u8 = 0x109 as *mut u8;

/// PORT L Data Direction Register.
pub const DDRL: *mut u8 = 0x10A as *mut u8;

/// PORT L Data Register.
pub const PORTL: *mut u8 = 0x10B as *mut u8;

/// Timer/Counter5 Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | COM5C | 1100 |
/// | COM5A | 11000000 |
/// | COM5B | 110000 |
pub const TCCR5A: *mut u8 = 0x120 as *mut u8;

/// Timer/Counter5 Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ICES5 | 1000000 |
/// | CS5 | 111 |
/// | ICNC5 | 10000000 |
pub const TCCR5B: *mut u8 = 0x121 as *mut u8;

/// Timer/Counter 5 Control Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FOC5B | 1000000 |
/// | FOC5A | 10000000 |
/// | FOC5C | 100000 |
pub const TCCR5C: *mut u8 = 0x122 as *mut u8;

/// Timer/Counter5  Bytes low byte.
pub const TCNT5L: *mut u8 = 0x124 as *mut u8;

/// Timer/Counter5  Bytes.
pub const TCNT5: *mut u16 = 0x124 as *mut u16;

/// Timer/Counter5  Bytes high byte.
pub const TCNT5H: *mut u8 = 0x125 as *mut u8;

/// Timer/Counter5 Input Capture Register  Bytes.
pub const ICR5: *mut u16 = 0x126 as *mut u16;

/// Timer/Counter5 Input Capture Register  Bytes low byte.
pub const ICR5L: *mut u8 = 0x126 as *mut u8;

/// Timer/Counter5 Input Capture Register  Bytes high byte.
pub const ICR5H: *mut u8 = 0x127 as *mut u8;

/// Timer/Counter5 Output Compare Register A  Bytes low byte.
pub const OCR5AL: *mut u8 = 0x128 as *mut u8;

/// Timer/Counter5 Output Compare Register A  Bytes.
pub const OCR5A: *mut u16 = 0x128 as *mut u16;

/// Timer/Counter5 Output Compare Register A  Bytes high byte.
pub const OCR5AH: *mut u8 = 0x129 as *mut u8;

/// Timer/Counter5 Output Compare Register B  Bytes low byte.
pub const OCR5BL: *mut u8 = 0x12A as *mut u8;

/// Timer/Counter5 Output Compare Register B  Bytes.
pub const OCR5B: *mut u16 = 0x12A as *mut u16;

/// Timer/Counter5 Output Compare Register B  Bytes high byte.
pub const OCR5BH: *mut u8 = 0x12B as *mut u8;

/// Timer/Counter5 Output Compare Register B  Bytes.
pub const OCR5C: *mut u16 = 0x12C as *mut u16;

/// Timer/Counter5 Output Compare Register B  Bytes low byte.
pub const OCR5CL: *mut u8 = 0x12C as *mut u8;

/// Timer/Counter5 Output Compare Register B  Bytes high byte.
pub const OCR5CH: *mut u8 = 0x12D as *mut u8;

/// USART Control and Status Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | U2X3 | 10 |
/// | FE3 | 10000 |
/// | MPCM3 | 1 |
/// | TXC3 | 1000000 |
/// | UPE3 | 100 |
/// | UDRE3 | 100000 |
/// | DOR3 | 1000 |
/// | RXC3 | 10000000 |
pub const UCSR3A: *mut u8 = 0x130 as *mut u8;

/// USART Control and Status Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RXB83 | 10 |
/// | UDRIE3 | 100000 |
/// | TXCIE3 | 1000000 |
/// | RXEN3 | 10000 |
/// | UCSZ32 | 100 |
/// | TXB83 | 1 |
/// | TXEN3 | 1000 |
/// | RXCIE3 | 10000000 |
pub const UCSR3B: *mut u8 = 0x131 as *mut u8;

/// USART Control and Status Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | UPM3 | 110000 |
/// | UCSZ3 | 110 |
/// | UMSEL3 | 11000000 |
/// | USBS3 | 1000 |
/// | UCPOL3 | 1 |
pub const UCSR3C: *mut u8 = 0x132 as *mut u8;

/// USART Baud Rate Register  Bytes low byte.
pub const UBRR3L: *mut u8 = 0x134 as *mut u8;

/// USART Baud Rate Register  Bytes.
pub const UBRR3: *mut u16 = 0x134 as *mut u16;

/// USART Baud Rate Register  Bytes high byte.
pub const UBRR3H: *mut u8 = 0x135 as *mut u8;

/// USART I/O Data Register.
pub const UDR3: *mut u8 = 0x136 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACI: u8 = 0x10;

/// Bitfield on register `ACSR`
pub const ACIC: u8 = 0x4;

/// Bitfield on register `ACSR`
pub const ACD: u8 = 0x80;

/// Bitfield on register `ACSR`
pub const ACIS: u8 = 0x3;

/// Bitfield on register `ACSR`
pub const ACO: u8 = 0x20;

/// Bitfield on register `ACSR`
pub const ACIE: u8 = 0x8;

/// Bitfield on register `ACSR`
pub const ACBG: u8 = 0x40;

/// Bitfield on register `ADCSRA`
pub const ADIE: u8 = 0x8;

/// Bitfield on register `ADCSRA`
pub const ADIF: u8 = 0x10;

/// Bitfield on register `ADCSRA`
pub const ADPS: u8 = 0x7;

/// Bitfield on register `ADCSRA`
pub const ADEN: u8 = 0x80;

/// Bitfield on register `ADCSRA`
pub const ADATE: u8 = 0x20;

/// Bitfield on register `ADCSRA`
pub const ADSC: u8 = 0x40;

/// Bitfield on register `ADCSRB`
pub const MUX5: u8 = 0x8;

/// Bitfield on register `ADCSRB`
pub const ADTS: u8 = 0x7;

/// Bitfield on register `ADCSRB`
pub const ACME: u8 = 0x40;

/// Bitfield on register `ADMUX`
pub const ADLAR: u8 = 0x20;

/// Bitfield on register `ADMUX`
pub const REFS: u8 = 0xC0;

/// Bitfield on register `ADMUX`
pub const MUX: u8 = 0x1F;

/// Bitfield on register `ASSR`
pub const OCR2BUB: u8 = 0x4;

/// Bitfield on register `ASSR`
pub const EXCLK: u8 = 0x40;

/// Bitfield on register `ASSR`
pub const TCR2BUB: u8 = 0x1;

/// Bitfield on register `ASSR`
pub const TCN2UB: u8 = 0x10;

/// Bitfield on register `ASSR`
pub const TCR2AUB: u8 = 0x2;

/// Bitfield on register `ASSR`
pub const OCR2AUB: u8 = 0x8;

/// Bitfield on register `ASSR`
pub const AS2: u8 = 0x20;

/// Bitfield on register `CLKPR`
pub const CLKPCE: u8 = 0x80;

/// Bitfield on register `CLKPR`
pub const CLKPS: u8 = 0xF;

/// Bitfield on register `DIDR0`
pub const ADC2D: u8 = 0x4;

/// Bitfield on register `DIDR0`
pub const ADC3D: u8 = 0x8;

/// Bitfield on register `DIDR0`
pub const ADC4D: u8 = 0x10;

/// Bitfield on register `DIDR0`
pub const ADC5D: u8 = 0x20;

/// Bitfield on register `DIDR0`
pub const ADC1D: u8 = 0x2;

/// Bitfield on register `DIDR0`
pub const ADC6D: u8 = 0x40;

/// Bitfield on register `DIDR0`
pub const ADC0D: u8 = 0x1;

/// Bitfield on register `DIDR0`
pub const ADC7D: u8 = 0x80;

/// Bitfield on register `DIDR1`
pub const AIN0D: u8 = 0x1;

/// Bitfield on register `DIDR1`
pub const AIN1D: u8 = 0x2;

/// Bitfield on register `DIDR2`
pub const ADC8D: u8 = 0x1;

/// Bitfield on register `DIDR2`
pub const ADC15D: u8 = 0x80;

/// Bitfield on register `DIDR2`
pub const ADC13D: u8 = 0x20;

/// Bitfield on register `DIDR2`
pub const ADC9D: u8 = 0x2;

/// Bitfield on register `DIDR2`
pub const ADC11D: u8 = 0x8;

/// Bitfield on register `DIDR2`
pub const ADC12D: u8 = 0x10;

/// Bitfield on register `DIDR2`
pub const ADC14D: u8 = 0x40;

/// Bitfield on register `DIDR2`
pub const ADC10D: u8 = 0x4;

/// Bitfield on register `EECR`
pub const EERIE: u8 = 0x8;

/// Bitfield on register `EECR`
pub const EERE: u8 = 0x1;

/// Bitfield on register `EECR`
pub const EEPM: u8 = 0x30;

/// Bitfield on register `EECR`
pub const EEMPE: u8 = 0x4;

/// Bitfield on register `EECR`
pub const EEPE: u8 = 0x2;

/// Bitfield on register `EICRA`
pub const ISC3: u8 = 0xC0;

/// Bitfield on register `EICRA`
pub const ISC1: u8 = 0xC;

/// Bitfield on register `EICRA`
pub const ISC2: u8 = 0x30;

/// Bitfield on register `EICRA`
pub const ISC0: u8 = 0x3;

/// Bitfield on register `EICRB`
pub const ISC5: u8 = 0xC;

/// Bitfield on register `EICRB`
pub const ISC7: u8 = 0xC0;

/// Bitfield on register `EICRB`
pub const ISC6: u8 = 0x30;

/// Bitfield on register `EICRB`
pub const ISC4: u8 = 0x3;

/// Bitfield on register `EXTENDED`
pub const BODLEVEL: u8 = 0x7;

/// Bitfield on register `GPIOR0`
pub const GPIOR04: u8 = 0x10;

/// Bitfield on register `GPIOR0`
pub const GPIOR07: u8 = 0x80;

/// Bitfield on register `GPIOR0`
pub const GPIOR06: u8 = 0x40;

/// Bitfield on register `GPIOR0`
pub const GPIOR00: u8 = 0x1;

/// Bitfield on register `GPIOR0`
pub const GPIOR01: u8 = 0x2;

/// Bitfield on register `GPIOR0`
pub const GPIOR02: u8 = 0x4;

/// Bitfield on register `GPIOR0`
pub const GPIOR05: u8 = 0x20;

/// Bitfield on register `GPIOR0`
pub const GPIOR03: u8 = 0x8;

/// Bitfield on register `GTCCR`
pub const TSM: u8 = 0x80;

/// Bitfield on register `GTCCR`
pub const PSRASY: u8 = 0x2;

/// Bitfield on register `HIGH`
pub const EESAVE: u8 = 0x8;

/// Bitfield on register `HIGH`
pub const BOOTSZ: u8 = 0x6;

/// Bitfield on register `HIGH`
pub const OCDEN: u8 = 0x80;

/// Bitfield on register `HIGH`
pub const JTAGEN: u8 = 0x40;

/// Bitfield on register `HIGH`
pub const WDTON: u8 = 0x10;

/// Bitfield on register `HIGH`
pub const BOOTRST: u8 = 0x1;

/// Bitfield on register `HIGH`
pub const SPIEN: u8 = 0x20;

/// Bitfield on register `LOCKBIT`
pub const BLB1: u8 = 0x30;

/// Bitfield on register `LOCKBIT`
pub const BLB0: u8 = 0xC;

/// Bitfield on register `LOCKBIT`
pub const LB: u8 = 0x3;

/// Bitfield on register `LOW`
pub const SUT_CKSEL: u8 = 0x3F;

/// Bitfield on register `LOW`
pub const CKOUT: u8 = 0x40;

/// Bitfield on register `LOW`
pub const CKDIV8: u8 = 0x80;

/// Bitfield on register `MCUCR`
pub const IVCE: u8 = 0x1;

/// Bitfield on register `MCUCR`
pub const PUD: u8 = 0x10;

/// Bitfield on register `MCUCR`
pub const JTD: u8 = 0x80;

/// Bitfield on register `MCUCR`
pub const IVSEL: u8 = 0x2;

/// Bitfield on register `MCUSR`
pub const PORF: u8 = 0x1;

/// Bitfield on register `MCUSR`
pub const WDRF: u8 = 0x8;

/// Bitfield on register `MCUSR`
pub const EXTRF: u8 = 0x2;

/// Bitfield on register `MCUSR`
pub const JTRF: u8 = 0x10;

/// Bitfield on register `MCUSR`
pub const BORF: u8 = 0x4;

/// Bitfield on register `PCICR`
pub const PCIE: u8 = 0x7;

/// Bitfield on register `PCIFR`
pub const PCIF: u8 = 0x7;

/// Bitfield on register `PRR0`
pub const PRTIM0: u8 = 0x20;

/// Bitfield on register `PRR0`
pub const PRTIM1: u8 = 0x8;

/// Bitfield on register `PRR0`
pub const PRTWI: u8 = 0x80;

/// Bitfield on register `PRR0`
pub const PRTIM2: u8 = 0x40;

/// Bitfield on register `PRR0`
pub const PRSPI: u8 = 0x4;

/// Bitfield on register `PRR0`
pub const PRADC: u8 = 0x1;

/// Bitfield on register `PRR0`
pub const PRUSART0: u8 = 0x2;

/// Bitfield on register `PRR1`
pub const PRTIM3: u8 = 0x8;

/// Bitfield on register `PRR1`
pub const PRTIM4: u8 = 0x10;

/// Bitfield on register `PRR1`
pub const PRTIM5: u8 = 0x20;

/// Bitfield on register `PRR1`
pub const PRUSART3: u8 = 0x4;

/// Bitfield on register `PRR1`
pub const PRUSART2: u8 = 0x2;

/// Bitfield on register `PRR1`
pub const PRUSART1: u8 = 0x1;

/// Bitfield on register `SMCR`
pub const SM: u8 = 0xE;

/// Bitfield on register `SMCR`
pub const SE: u8 = 0x1;

/// Bitfield on register `SPCR`
pub const CPHA: u8 = 0x4;

/// Bitfield on register `SPCR`
pub const SPIE: u8 = 0x80;

/// Bitfield on register `SPCR`
pub const SPE: u8 = 0x40;

/// Bitfield on register `SPCR`
pub const MSTR: u8 = 0x10;

/// Bitfield on register `SPCR`
pub const DORD: u8 = 0x20;

/// Bitfield on register `SPCR`
pub const SPR: u8 = 0x3;

/// Bitfield on register `SPCR`
pub const CPOL: u8 = 0x8;

/// Bitfield on register `SPMCSR`
pub const BLBSET: u8 = 0x8;

/// Bitfield on register `SPMCSR`
pub const PGERS: u8 = 0x2;

/// Bitfield on register `SPMCSR`
pub const RWWSRE: u8 = 0x10;

/// Bitfield on register `SPMCSR`
pub const SIGRD: u8 = 0x20;

/// Bitfield on register `SPMCSR`
pub const SPMEN: u8 = 0x1;

/// Bitfield on register `SPMCSR`
pub const RWWSB: u8 = 0x40;

/// Bitfield on register `SPMCSR`
pub const PGWRT: u8 = 0x4;

/// Bitfield on register `SPMCSR`
pub const SPMIE: u8 = 0x80;

/// Bitfield on register `SPSR`
pub const WCOL: u8 = 0x40;

/// Bitfield on register `SPSR`
pub const SPIF: u8 = 0x80;

/// Bitfield on register `SPSR`
pub const SPI2X: u8 = 0x1;

/// Bitfield on register `SREG`
pub const I: u8 = 0x80;

/// Bitfield on register `SREG`
pub const T: u8 = 0x40;

/// Bitfield on register `SREG`
pub const V: u8 = 0x8;

/// Bitfield on register `SREG`
pub const C: u8 = 0x1;

/// Bitfield on register `SREG`
pub const S: u8 = 0x10;

/// Bitfield on register `SREG`
pub const N: u8 = 0x4;

/// Bitfield on register `SREG`
pub const Z: u8 = 0x2;

/// Bitfield on register `SREG`
pub const H: u8 = 0x20;

/// Bitfield on register `TCCR0A`
pub const WGM0: u8 = 0x3;

/// Bitfield on register `TCCR0A`
pub const COM0A: u8 = 0xC0;

/// Bitfield on register `TCCR0A`
pub const COM0B: u8 = 0x30;

/// Bitfield on register `TCCR0B`
pub const WGM02: u8 = 0x8;

/// Bitfield on register `TCCR0B`
pub const CS0: u8 = 0x7;

/// Bitfield on register `TCCR0B`
pub const FOC0A: u8 = 0x80;

/// Bitfield on register `TCCR0B`
pub const FOC0B: u8 = 0x40;

/// Bitfield on register `TCCR1A`
pub const COM1B: u8 = 0x30;

/// Bitfield on register `TCCR1A`
pub const COM1A: u8 = 0xC0;

/// Bitfield on register `TCCR1A`
pub const COM1C: u8 = 0xC;

/// Bitfield on register `TCCR1B`
pub const ICES1: u8 = 0x40;

/// Bitfield on register `TCCR1B`
pub const CS1: u8 = 0x7;

/// Bitfield on register `TCCR1B`
pub const ICNC1: u8 = 0x80;

/// Bitfield on register `TCCR1C`
pub const FOC1A: u8 = 0x80;

/// Bitfield on register `TCCR1C`
pub const FOC1C: u8 = 0x20;

/// Bitfield on register `TCCR1C`
pub const FOC1B: u8 = 0x40;

/// Bitfield on register `TCCR2A`
pub const COM2A: u8 = 0xC0;

/// Bitfield on register `TCCR2A`
pub const WGM2: u8 = 0x3;

/// Bitfield on register `TCCR2A`
pub const COM2B: u8 = 0x30;

/// Bitfield on register `TCCR2B`
pub const FOC2B: u8 = 0x40;

/// Bitfield on register `TCCR2B`
pub const CS2: u8 = 0x7;

/// Bitfield on register `TCCR2B`
pub const FOC2A: u8 = 0x80;

/// Bitfield on register `TCCR2B`
pub const WGM22: u8 = 0x8;

/// Bitfield on register `TCCR3A`
pub const COM3A: u8 = 0xC0;

/// Bitfield on register `TCCR3A`
pub const COM3B: u8 = 0x30;

/// Bitfield on register `TCCR3A`
pub const COM3C: u8 = 0xC;

/// Bitfield on register `TCCR3B`
pub const ICES3: u8 = 0x40;

/// Bitfield on register `TCCR3B`
pub const ICNC3: u8 = 0x80;

/// Bitfield on register `TCCR3B`
pub const CS3: u8 = 0x7;

/// Bitfield on register `TCCR3C`
pub const FOC3A: u8 = 0x80;

/// Bitfield on register `TCCR3C`
pub const FOC3C: u8 = 0x20;

/// Bitfield on register `TCCR3C`
pub const FOC3B: u8 = 0x40;

/// Bitfield on register `TCCR4A`
pub const COM4C: u8 = 0xC;

/// Bitfield on register `TCCR4A`
pub const COM4A: u8 = 0xC0;

/// Bitfield on register `TCCR4A`
pub const COM4B: u8 = 0x30;

/// Bitfield on register `TCCR4B`
pub const CS4: u8 = 0x7;

/// Bitfield on register `TCCR4B`
pub const ICES4: u8 = 0x40;

/// Bitfield on register `TCCR4B`
pub const ICNC4: u8 = 0x80;

/// Bitfield on register `TCCR4C`
pub const FOC4B: u8 = 0x40;

/// Bitfield on register `TCCR4C`
pub const FOC4C: u8 = 0x20;

/// Bitfield on register `TCCR4C`
pub const FOC4A: u8 = 0x80;

/// Bitfield on register `TCCR5A`
pub const COM5C: u8 = 0xC;

/// Bitfield on register `TCCR5A`
pub const COM5A: u8 = 0xC0;

/// Bitfield on register `TCCR5A`
pub const COM5B: u8 = 0x30;

/// Bitfield on register `TCCR5B`
pub const ICES5: u8 = 0x40;

/// Bitfield on register `TCCR5B`
pub const CS5: u8 = 0x7;

/// Bitfield on register `TCCR5B`
pub const ICNC5: u8 = 0x80;

/// Bitfield on register `TCCR5C`
pub const FOC5B: u8 = 0x40;

/// Bitfield on register `TCCR5C`
pub const FOC5A: u8 = 0x80;

/// Bitfield on register `TCCR5C`
pub const FOC5C: u8 = 0x20;

/// Bitfield on register `TIFR0`
pub const OCF0A: u8 = 0x2;

/// Bitfield on register `TIFR0`
pub const OCF0B: u8 = 0x4;

/// Bitfield on register `TIFR0`
pub const TOV0: u8 = 0x1;

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

/// Bitfield on register `TIFR2`
pub const OCF2B: u8 = 0x4;

/// Bitfield on register `TIFR3`
pub const OCF3A: u8 = 0x2;

/// Bitfield on register `TIFR3`
pub const OCF3C: u8 = 0x8;

/// Bitfield on register `TIFR3`
pub const OCF3B: u8 = 0x4;

/// Bitfield on register `TIFR3`
pub const TOV3: u8 = 0x1;

/// Bitfield on register `TIFR3`
pub const ICF3: u8 = 0x20;

/// Bitfield on register `TIFR4`
pub const ICF4: u8 = 0x20;

/// Bitfield on register `TIFR4`
pub const TOV4: u8 = 0x1;

/// Bitfield on register `TIFR4`
pub const OCF4B: u8 = 0x4;

/// Bitfield on register `TIFR4`
pub const OCF4A: u8 = 0x2;

/// Bitfield on register `TIFR4`
pub const OCF4C: u8 = 0x8;

/// Bitfield on register `TIFR5`
pub const OCF5B: u8 = 0x4;

/// Bitfield on register `TIFR5`
pub const OCF5A: u8 = 0x2;

/// Bitfield on register `TIFR5`
pub const OCF5C: u8 = 0x8;

/// Bitfield on register `TIFR5`
pub const ICF5: u8 = 0x20;

/// Bitfield on register `TIFR5`
pub const TOV5: u8 = 0x1;

/// Bitfield on register `TIMSK0`
pub const OCIE0A: u8 = 0x2;

/// Bitfield on register `TIMSK0`
pub const OCIE0B: u8 = 0x4;

/// Bitfield on register `TIMSK0`
pub const TOIE0: u8 = 0x1;

/// Bitfield on register `TIMSK1`
pub const TOIE1: u8 = 0x1;

/// Bitfield on register `TIMSK1`
pub const OCIE1C: u8 = 0x8;

/// Bitfield on register `TIMSK1`
pub const ICIE1: u8 = 0x20;

/// Bitfield on register `TIMSK1`
pub const OCIE1A: u8 = 0x2;

/// Bitfield on register `TIMSK1`
pub const OCIE1B: u8 = 0x4;

/// Bitfield on register `TIMSK2`
pub const OCIE2A: u8 = 0x2;

/// Bitfield on register `TIMSK2`
pub const OCIE2B: u8 = 0x4;

/// Bitfield on register `TIMSK2`
pub const TOIE2: u8 = 0x1;

/// Bitfield on register `TIMSK3`
pub const OCIE3C: u8 = 0x8;

/// Bitfield on register `TIMSK3`
pub const OCIE3A: u8 = 0x2;

/// Bitfield on register `TIMSK3`
pub const OCIE3B: u8 = 0x4;

/// Bitfield on register `TIMSK3`
pub const TOIE3: u8 = 0x1;

/// Bitfield on register `TIMSK3`
pub const ICIE3: u8 = 0x20;

/// Bitfield on register `TIMSK4`
pub const OCIE4A: u8 = 0x2;

/// Bitfield on register `TIMSK4`
pub const OCIE4B: u8 = 0x4;

/// Bitfield on register `TIMSK4`
pub const TOIE4: u8 = 0x1;

/// Bitfield on register `TIMSK4`
pub const ICIE4: u8 = 0x20;

/// Bitfield on register `TIMSK4`
pub const OCIE4C: u8 = 0x8;

/// Bitfield on register `TIMSK5`
pub const ICIE5: u8 = 0x20;

/// Bitfield on register `TIMSK5`
pub const OCIE5B: u8 = 0x4;

/// Bitfield on register `TIMSK5`
pub const OCIE5A: u8 = 0x2;

/// Bitfield on register `TIMSK5`
pub const OCIE5C: u8 = 0x8;

/// Bitfield on register `TIMSK5`
pub const TOIE5: u8 = 0x1;

/// Bitfield on register `TWAMR`
pub const TWAM: u8 = 0xFE;

/// Bitfield on register `TWAR`
pub const TWGCE: u8 = 0x1;

/// Bitfield on register `TWAR`
pub const TWA: u8 = 0xFE;

/// Bitfield on register `TWCR`
pub const TWEN: u8 = 0x4;

/// Bitfield on register `TWCR`
pub const TWINT: u8 = 0x80;

/// Bitfield on register `TWCR`
pub const TWSTA: u8 = 0x20;

/// Bitfield on register `TWCR`
pub const TWSTO: u8 = 0x10;

/// Bitfield on register `TWCR`
pub const TWWC: u8 = 0x8;

/// Bitfield on register `TWCR`
pub const TWEA: u8 = 0x40;

/// Bitfield on register `TWCR`
pub const TWIE: u8 = 0x1;

/// Bitfield on register `TWSR`
pub const TWPS: u8 = 0x3;

/// Bitfield on register `TWSR`
pub const TWS: u8 = 0xF8;

/// Bitfield on register `UCSR0A`
pub const RXC0: u8 = 0x80;

/// Bitfield on register `UCSR0A`
pub const UPE0: u8 = 0x4;

/// Bitfield on register `UCSR0A`
pub const FE0: u8 = 0x10;

/// Bitfield on register `UCSR0A`
pub const UDRE0: u8 = 0x20;

/// Bitfield on register `UCSR0A`
pub const TXC0: u8 = 0x40;

/// Bitfield on register `UCSR0A`
pub const DOR0: u8 = 0x8;

/// Bitfield on register `UCSR0A`
pub const MPCM0: u8 = 0x1;

/// Bitfield on register `UCSR0A`
pub const U2X0: u8 = 0x2;

/// Bitfield on register `UCSR0B`
pub const RXCIE0: u8 = 0x80;

/// Bitfield on register `UCSR0B`
pub const TXEN0: u8 = 0x8;

/// Bitfield on register `UCSR0B`
pub const UDRIE0: u8 = 0x20;

/// Bitfield on register `UCSR0B`
pub const TXCIE0: u8 = 0x40;

/// Bitfield on register `UCSR0B`
pub const UCSZ02: u8 = 0x4;

/// Bitfield on register `UCSR0B`
pub const RXEN0: u8 = 0x10;

/// Bitfield on register `UCSR0B`
pub const RXB80: u8 = 0x2;

/// Bitfield on register `UCSR0B`
pub const TXB80: u8 = 0x1;

/// Bitfield on register `UCSR0C`
pub const UPM0: u8 = 0x30;

/// Bitfield on register `UCSR0C`
pub const UMSEL0: u8 = 0xC0;

/// Bitfield on register `UCSR0C`
pub const UCPOL0: u8 = 0x1;

/// Bitfield on register `UCSR0C`
pub const UCSZ0: u8 = 0x6;

/// Bitfield on register `UCSR0C`
pub const USBS0: u8 = 0x8;

/// Bitfield on register `UCSR1A`
pub const DOR1: u8 = 0x8;

/// Bitfield on register `UCSR1A`
pub const UPE1: u8 = 0x4;

/// Bitfield on register `UCSR1A`
pub const FE1: u8 = 0x10;

/// Bitfield on register `UCSR1A`
pub const UDRE1: u8 = 0x20;

/// Bitfield on register `UCSR1A`
pub const U2X1: u8 = 0x2;

/// Bitfield on register `UCSR1A`
pub const TXC1: u8 = 0x40;

/// Bitfield on register `UCSR1A`
pub const RXC1: u8 = 0x80;

/// Bitfield on register `UCSR1A`
pub const MPCM1: u8 = 0x1;

/// Bitfield on register `UCSR1B`
pub const RXB81: u8 = 0x2;

/// Bitfield on register `UCSR1B`
pub const UDRIE1: u8 = 0x20;

/// Bitfield on register `UCSR1B`
pub const UCSZ12: u8 = 0x4;

/// Bitfield on register `UCSR1B`
pub const TXEN1: u8 = 0x8;

/// Bitfield on register `UCSR1B`
pub const RXEN1: u8 = 0x10;

/// Bitfield on register `UCSR1B`
pub const TXB81: u8 = 0x1;

/// Bitfield on register `UCSR1B`
pub const TXCIE1: u8 = 0x40;

/// Bitfield on register `UCSR1B`
pub const RXCIE1: u8 = 0x80;

/// Bitfield on register `UCSR1C`
pub const UMSEL1: u8 = 0xC0;

/// Bitfield on register `UCSR1C`
pub const UCPOL1: u8 = 0x1;

/// Bitfield on register `UCSR1C`
pub const UPM1: u8 = 0x30;

/// Bitfield on register `UCSR1C`
pub const USBS1: u8 = 0x8;

/// Bitfield on register `UCSR1C`
pub const UCSZ1: u8 = 0x6;

/// Bitfield on register `UCSR2A`
pub const RXC2: u8 = 0x80;

/// Bitfield on register `UCSR2A`
pub const TXC2: u8 = 0x40;

/// Bitfield on register `UCSR2A`
pub const UPE2: u8 = 0x4;

/// Bitfield on register `UCSR2A`
pub const DOR2: u8 = 0x8;

/// Bitfield on register `UCSR2A`
pub const U2X2: u8 = 0x2;

/// Bitfield on register `UCSR2A`
pub const FE2: u8 = 0x10;

/// Bitfield on register `UCSR2A`
pub const UDRE2: u8 = 0x20;

/// Bitfield on register `UCSR2A`
pub const MPCM2: u8 = 0x1;

/// Bitfield on register `UCSR2B`
pub const RXEN2: u8 = 0x10;

/// Bitfield on register `UCSR2B`
pub const RXCIE2: u8 = 0x80;

/// Bitfield on register `UCSR2B`
pub const UDRIE2: u8 = 0x20;

/// Bitfield on register `UCSR2B`
pub const RXB82: u8 = 0x2;

/// Bitfield on register `UCSR2B`
pub const TXEN2: u8 = 0x8;

/// Bitfield on register `UCSR2B`
pub const TXCIE2: u8 = 0x40;

/// Bitfield on register `UCSR2B`
pub const TXB82: u8 = 0x1;

/// Bitfield on register `UCSR2B`
pub const UCSZ22: u8 = 0x4;

/// Bitfield on register `UCSR2C`
pub const USBS2: u8 = 0x8;

/// Bitfield on register `UCSR2C`
pub const UCPOL2: u8 = 0x1;

/// Bitfield on register `UCSR2C`
pub const UMSEL2: u8 = 0xC0;

/// Bitfield on register `UCSR2C`
pub const UPM2: u8 = 0x30;

/// Bitfield on register `UCSR2C`
pub const UCSZ2: u8 = 0x6;

/// Bitfield on register `UCSR3A`
pub const U2X3: u8 = 0x2;

/// Bitfield on register `UCSR3A`
pub const FE3: u8 = 0x10;

/// Bitfield on register `UCSR3A`
pub const MPCM3: u8 = 0x1;

/// Bitfield on register `UCSR3A`
pub const TXC3: u8 = 0x40;

/// Bitfield on register `UCSR3A`
pub const UPE3: u8 = 0x4;

/// Bitfield on register `UCSR3A`
pub const UDRE3: u8 = 0x20;

/// Bitfield on register `UCSR3A`
pub const DOR3: u8 = 0x8;

/// Bitfield on register `UCSR3A`
pub const RXC3: u8 = 0x80;

/// Bitfield on register `UCSR3B`
pub const RXB83: u8 = 0x2;

/// Bitfield on register `UCSR3B`
pub const UDRIE3: u8 = 0x20;

/// Bitfield on register `UCSR3B`
pub const TXCIE3: u8 = 0x40;

/// Bitfield on register `UCSR3B`
pub const RXEN3: u8 = 0x10;

/// Bitfield on register `UCSR3B`
pub const UCSZ32: u8 = 0x4;

/// Bitfield on register `UCSR3B`
pub const TXB83: u8 = 0x1;

/// Bitfield on register `UCSR3B`
pub const TXEN3: u8 = 0x8;

/// Bitfield on register `UCSR3B`
pub const RXCIE3: u8 = 0x80;

/// Bitfield on register `UCSR3C`
pub const UPM3: u8 = 0x30;

/// Bitfield on register `UCSR3C`
pub const UCSZ3: u8 = 0x6;

/// Bitfield on register `UCSR3C`
pub const UMSEL3: u8 = 0xC0;

/// Bitfield on register `UCSR3C`
pub const USBS3: u8 = 0x8;

/// Bitfield on register `UCSR3C`
pub const UCPOL3: u8 = 0x1;

/// Bitfield on register `WDTCSR`
pub const WDP: u8 = 0x27;

/// Bitfield on register `WDTCSR`
pub const WDE: u8 = 0x8;

/// Bitfield on register `WDTCSR`
pub const WDCE: u8 = 0x10;

/// Bitfield on register `WDTCSR`
pub const WDIF: u8 = 0x80;

/// Bitfield on register `WDTCSR`
pub const WDIE: u8 = 0x40;

/// Bitfield on register `XMCRA`
pub const SRL: u8 = 0x70;

/// Bitfield on register `XMCRA`
pub const SRW0: u8 = 0x3;

/// Bitfield on register `XMCRA`
pub const SRW1: u8 = 0xC;

/// Bitfield on register `XMCRA`
pub const SRE: u8 = 0x80;

/// Bitfield on register `XMCRB`
pub const XMBK: u8 = 0x80;

/// Bitfield on register `XMCRB`
pub const XMM: u8 = 0x7;

/// `ADC_MUX_DIFF6` value group
#[allow(non_upper_case_globals)]
pub mod adc_mux_diff6 {
   /// ADC Single Ended Input pin 0.
   pub const ADC0: u32 = 0x0;
   /// ADC Single Ended Input pin 1.
   pub const ADC1: u32 = 0x1;
   /// ADC Single Ended Input pin 2.
   pub const ADC2: u32 = 0x2;
   /// ADC Single Ended Input pin 3.
   pub const ADC3: u32 = 0x3;
   /// ADC Single Ended Input pin 4.
   pub const ADC4: u32 = 0x4;
   /// ADC Single Ended Input pin 5.
   pub const ADC5: u32 = 0x5;
   /// ADC Single Ended Input pin 6.
   pub const ADC6: u32 = 0x6;
   /// ADC Single Ended Input pin 7.
   pub const ADC7: u32 = 0x7;
   /// ADC Differential Inputs Postive pin 0 Negative pin 0 10x Gain.
   pub const ADC0_ADC0_10X: u32 = 0x8;
   /// ADC Differential Inputs Postive pin 1 Negative pin 0 10x Gain.
   pub const ADC1_ADC0_10X: u32 = 0x9;
   /// ADC Differential Inputs Postive pin 0 Negative pin 0 200x Gain.
   pub const ADC0_ADC0_200x: u32 = 0xA;
   /// ADC Differential Inputs Postive pin 1 Negative pin 0 200x Gain.
   pub const ADC1_ADC0_200X: u32 = 0xB;
   /// ADC Differential Inputs Postive pin 2 Negative pin 2 10x Gain.
   pub const ADC2_ADC2_10X: u32 = 0xC;
   /// ADC Differential Inputs Postive pin 3 Negative pin 2 10x Gain.
   pub const ADC3_ADC2_10X: u32 = 0xD;
   /// ADC Differential Inputs Postive pin 2 Negative pin 2 200x Gain.
   pub const ADC2_ADC2_200X: u32 = 0xE;
   /// ADC Differential Inputs Postive pin 3 Negative pin 2 200x Gain.
   pub const ADC3_ADC2_200X: u32 = 0xF;
   /// ADC Differential Inputs Postive pin 0 Negative pin 1 1x Gain.
   pub const ADC0_ADC1_1X: u32 = 0x10;
   /// ADC Differential Inputs Postive pin 1 Negative pin 1 1x Gain.
   pub const ADC1_ADC1_1X: u32 = 0x11;
   /// ADC Differential Inputs Postive pin 2 Negative pin 1 1x Gain.
   pub const ADC2_ADC1_1X: u32 = 0x12;
   /// ADC Differential Inputs Postive pin 3 Negative pin 1 1x Gain.
   pub const ADC3_ADC1_1X: u32 = 0x13;
   /// ADC Differential Inputs Postive pin 4 Negative pin 1 1x Gain.
   pub const ADC4_ADC1_1X: u32 = 0x14;
   /// ADC Differential Inputs Postive pin 5 Negative pin 1 1x Gain.
   pub const ADC5_ADC1_1X: u32 = 0x15;
   /// ADC Differential Inputs Postive pin 6 Negative pin 1 1x Gain.
   pub const ADC6_ADC1_1X: u32 = 0x16;
   /// ADC Differential Inputs Postive pin 7 Negative pin 1 1x Gain.
   pub const ADC7_ADC1_1X: u32 = 0x17;
   /// ADC Differential Inputs Postive pin 0 Negative pin 2 1x Gain.
   pub const ADC0_ADC2_1X: u32 = 0x18;
   /// ADC Differential Inputs Postive pin 1 Negative pin 2 1x Gain.
   pub const ADC1_ADC2_1X: u32 = 0x19;
   /// ADC Differential Inputs Postive pin 2 Negative pin 2 1x Gain.
   pub const ADC2_ADC2_1X: u32 = 0x1A;
   /// ADC Differential Inputs Postive pin 3 Negative pin 2 1x Gain.
   pub const ADC3_ADC2_1X: u32 = 0x1B;
   /// ADC Differential Inputs Postive pin 4 Negative pin 2 1x Gain.
   pub const ADC4_ADC2_1X: u32 = 0x1C;
   /// ADC Differential Inputs Postive pin 5 Negative pin 2 1x Gain.
   pub const ADC5_ADC2_1X: u32 = 0x1D;
   /// Internal Reference (VBG).
   pub const ADC_VBG: u32 = 0x1E;
   /// 0V (GND).
   pub const ADC_GND: u32 = 0x1F;
   /// ADC Single Ended Input pin 8.
   pub const ADC8: u32 = 0x20;
   /// ADC Single Ended Input pin 9.
   pub const ADC9: u32 = 0x21;
   /// ADC Single Ended Input pin 10.
   pub const ADC10: u32 = 0x22;
   /// ADC Single Ended Input pin 11.
   pub const ADC11: u32 = 0x23;
   /// ADC Single Ended Input pin 12.
   pub const ADC12: u32 = 0x24;
   /// ADC Single Ended Input pin 13.
   pub const ADC13: u32 = 0x25;
   /// ADC Single Ended Input pin 14.
   pub const ADC14: u32 = 0x26;
   /// ADC Single Ended Input pin 15.
   pub const ADC15: u32 = 0x27;
   /// ADC Differential Inputs Postive pin 8 Negative pin 8 10x Gain.
   pub const ADC8_ADC8_10X: u32 = 0x28;
   /// ADC Differential Inputs Postive pin 9 Negative pin 8 10x Gain.
   pub const ADC9_ADC8_10X: u32 = 0x29;
   /// ADC Differential Inputs Postive pin 8 Negative pin 8 200x Gain.
   pub const ADC8_ADC8_200x: u32 = 0x2A;
   /// ADC Differential Inputs Postive pin 9 Negative pin 8 200x Gain.
   pub const ADC9_ADC8_200X: u32 = 0x2B;
   /// ADC Differential Inputs Postive pin 10 Negative pin 10 10x Gain.
   pub const ADC10_ADC10_10X: u32 = 0x2C;
   /// ADC Differential Inputs Postive pin 11 Negative pin 10 10x Gain.
   pub const ADC11_ADC10_10X: u32 = 0x2D;
   /// ADC Differential Inputs Postive pin 10 Negative pin 10 200x Gain.
   pub const ADC10_ADC10_200X: u32 = 0x2E;
   /// ADC Differential Inputs Postive pin 11 Negative pin 10 200x Gain.
   pub const ADC11_ADC10_200X: u32 = 0x2F;
   /// ADC Differential Inputs Postive pin 8 Negative pin 9 1x Gain.
   pub const ADC8_ADC9_1X: u32 = 0x30;
   /// ADC Differential Inputs Postive pin 9 Negative pin 9 1x Gain.
   pub const ADC9_ADC9_1X: u32 = 0x31;
   /// ADC Differential Inputs Postive pin 10 Negative pin 9 1x Gain.
   pub const ADC10_ADC9_1X: u32 = 0x32;
   /// ADC Differential Inputs Postive pin 11 Negative pin 9 1x Gain.
   pub const ADC11_ADC9_1X: u32 = 0x33;
   /// ADC Differential Inputs Postive pin 12 Negative pin 9 1x Gain.
   pub const ADC12_ADC9_1X: u32 = 0x34;
   /// ADC Differential Inputs Postive pin 13 Negative pin 9 1x Gain.
   pub const ADC13_ADC9_1X: u32 = 0x35;
   /// ADC Differential Inputs Postive pin 14 Negative pin 9 1x Gain.
   pub const ADC14_ADC9_1X: u32 = 0x36;
   /// ADC Differential Inputs Postive pin 15 Negative pin 9 1x Gain.
   pub const ADC15_ADC9_1X: u32 = 0x37;
   /// ADC Differential Inputs Postive pin 8 Negative pin 10 1x Gain.
   pub const ADC8_ADC10_1X: u32 = 0x38;
   /// ADC Differential Inputs Postive pin 9 Negative pin 10 1x Gain.
   pub const ADC9_ADC10_1X: u32 = 0x39;
   /// ADC Differential Inputs Postive pin 10 Negative pin 10 1x Gain.
   pub const ADC10_ADC10_1X: u32 = 0x3A;
   /// ADC Differential Inputs Postive pin 11 Negative pin 10 1x Gain.
   pub const ADC11_ADC10_1X: u32 = 0x3B;
   /// ADC Differential Inputs Postive pin 12 Negative pin 10 1x Gain.
   pub const ADC12_ADC10_1X: u32 = 0x3C;
   /// ADC Differential Inputs Postive pin 13 Negative pin 10 1x Gain.
   pub const ADC13_ADC10_1X: u32 = 0x3D;
}

/// `ANALOG_ADC_AUTO_TRIGGER` value group
#[allow(non_upper_case_globals)]
pub mod analog_adc_auto_trigger {
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

/// `ANALOG_ADC_V_REF6` value group
#[allow(non_upper_case_globals)]
pub mod analog_adc_v_ref6 {
   /// AREF, Internal Vref turned off.
   pub const VAL_0x00: u32 = 0x0;
   /// AVCC with external capacitor at AREF pin.
   pub const VAL_0x01: u32 = 0x1;
   /// Internal 1.1V Voltage Reference with external capacitor at AREF pin.
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

/// `COMM_USART_MODE_2BIT` value group
#[allow(non_upper_case_globals)]
pub mod comm_usart_mode_2bit {
   /// Asynchronous USART.
   pub const VAL_0x00: u32 = 0x0;
   /// Synchronous USART.
   pub const VAL_0x01: u32 = 0x1;
   /// Master SPI.
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

/// `CPU_SECTOR_LIMITS2` value group
#[allow(non_upper_case_globals)]
pub mod cpu_sector_limits2 {
   /// LS = N/A, US = 0x1100 - 0xFFFF.
   pub const VAL_0x00: u32 = 0x0;
   /// LS = 0x2200 - 0x1FFF, US = 0x2000 - 0xFFFF.
   pub const VAL_0x01: u32 = 0x1;
   /// LS = 0x2200 - 0x3FFF, US = 0x4000 - 0xFFFF.
   pub const VAL_0x02: u32 = 0x2;
   /// LS = 0x2200 - 0x5FFF, US = 0x6000 - 0xFFFF.
   pub const VAL_0x03: u32 = 0x3;
   /// LS = 0x2200 - 0x7FFF, US = 0x8000 - 0xFFFF.
   pub const VAL_0x04: u32 = 0x4;
   /// LS = 0x2200 - 0x9FFF, US = 0xA000 - 0xFFFF.
   pub const VAL_0x05: u32 = 0x5;
   /// LS = 0x2200 - 0xBFFF, US = 0xC000 - 0xFFFF.
   pub const VAL_0x06: u32 = 0x6;
   /// LS = 0x2200 - 0xDFFF, US = 0xE000 - 0xFFFF.
   pub const VAL_0x07: u32 = 0x7;
}

/// `CPU_SLEEP_MODE_3BITS` value group
#[allow(non_upper_case_globals)]
pub mod cpu_sleep_mode_3bits {
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
   /// Extended Standby.
   pub const ESTDBY: u32 = 0x7;
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

/// `ENUM_BODLEVEL` value group
#[allow(non_upper_case_globals)]
pub mod enum_bodlevel {
   /// Brown-out detection disabled.
   pub const DISABLED: u32 = 0x7;
   /// Brown-out detection at VCC=1.8 V.
   pub const _1V8: u32 = 0x6;
   /// Brown-out detection at VCC=2.7 V.
   pub const _2V7: u32 = 0x5;
   /// Brown-out detection at VCC=4.3 V.
   pub const _4V3: u32 = 0x4;
}

/// `ENUM_BOOTSZ` value group
#[allow(non_upper_case_globals)]
pub mod enum_bootsz {
   /// Boot Flash size=512 words start address=$FE00.
   pub const _512W_FE00: u32 = 0x3;
   /// Boot Flash size=1024 words start address=$FC00.
   pub const _1024W_FC00: u32 = 0x2;
   /// Boot Flash size=2048 words start address=$F800.
   pub const _2048W_F800: u32 = 0x1;
   /// Boot Flash size=4096 words start address=$F000.
   pub const _4096W_F000: u32 = 0x0;
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
   /// Int. 128kHz RC Osc.; Start-up time: 6 CK + 0 ms.
   pub const INTRCOSC_128KHZ_6CK_0MS: u32 = 0x3;
   /// Int. 128kHz RC Osc.; Start-up time: 6 CK + 4 ms.
   pub const INTRCOSC_128KHZ_6CK_4MS: u32 = 0x13;
   /// Int. 128kHz RC Osc.; Start-up time: 6 CK + 64 ms.
   pub const INTRCOSC_128KHZ_6CK_64MS: u32 = 0x23;
   /// Ext. Low-Freq. Crystal; Start-up time: 1K CK + 0 ms.
   pub const EXTLOFXTAL_1KCK_0MS: u32 = 0x4;
   /// Ext. Low-Freq. Crystal; Start-up time: 1K CK + 4.1 ms.
   pub const EXTLOFXTAL_1KCK_4MS1: u32 = 0x14;
   /// Ext. Low-Freq. Crystal; Start-up time: 1K CK + 65 ms.
   pub const EXTLOFXTAL_1KCK_65MS: u32 = 0x24;
   /// Ext. Low-Freq. Crystal; Start-up time: 32K CK + 0 ms.
   pub const EXTLOFXTAL_32KCK_0MS: u32 = 0x5;
   /// Ext. Low-Freq. Crystal; Start-up time: 32K CK + 4.1 ms.
   pub const EXTLOFXTAL_32KCK_4MS1: u32 = 0x15;
   /// Ext. Low-Freq. Crystal; Start-up time: 32K CK + 65 ms.
   pub const EXTLOFXTAL_32KCK_65MS: u32 = 0x25;
   /// Full Swing Oscillator; Start-up time: 258 CK + 4.1 ms; Ceramic res.; fast rising power.
   pub const FSOSC_258CK_4MS1_CRES_FASTPWR: u32 = 0x6;
   /// Full Swing Oscillator; Start-up time: 258 CK + 65 ms; Ceramic res.; slowly rising power.
   pub const FSOSC_258CK_65MS_CRES_SLOWPWR: u32 = 0x16;
   /// Full Swing Oscillator; Start-up time: 1K CK + 0 ms; Ceramic res.; BOD enable.
   pub const FSOSC_1KCK_0MS_CRES_BODEN: u32 = 0x26;
   /// Full Swing Oscillator; Start-up time: 1K CK + 4.1 ms; Ceramic res.; fast rising power.
   pub const FSOSC_1KCK_4MS1_CRES_FASTPWR: u32 = 0x36;
   /// Full Swing Oscillator; Start-up time: 1K CK + 65 ms; Ceramic res.; slowly rising power.
   pub const FSOSC_1KCK_65MS_CRES_SLOWPWR: u32 = 0x7;
   /// Full Swing Oscillator; Start-up time: 16K CK + 0 ms; Crystal Osc.; BOD enabled.
   pub const FSOSC_16KCK_0MS_XOSC_BODEN: u32 = 0x17;
   /// Full Swing Oscillator; Start-up time: 16K CK + 4.1 ms; Crystal Osc.; fast rising power.
   pub const FSOSC_16KCK_4MS1_XOSC_FASTPWR: u32 = 0x27;
   /// Full Swing Oscillator; Start-up time: 16K CK + 65 ms; Crystal Osc.; slowly rising power.
   pub const FSOSC_16KCK_65MS_XOSC_SLOWPWR: u32 = 0x37;
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
   /// Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time: 16K CK + 0 ms.
   pub const EXTXOSC_0MHZ4_0MHZ9_16KCK_0MS: u32 = 0x19;
   /// Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time: 16K CK + 4.1 ms.
   pub const EXTXOSC_0MHZ4_0MHZ9_16KCK_4MS1: u32 = 0x29;
   /// Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time: 16K CK + 65 ms.
   pub const EXTXOSC_0MHZ4_0MHZ9_16KCK_65MS: u32 = 0x39;
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
   /// Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time: 16K CK + 0 ms.
   pub const EXTXOSC_0MHZ9_3MHZ_16KCK_0MS: u32 = 0x1B;
   /// Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time: 16K CK + 4.1 ms.
   pub const EXTXOSC_0MHZ9_3MHZ_16KCK_4MS1: u32 = 0x2B;
   /// Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time: 16K CK + 65 ms.
   pub const EXTXOSC_0MHZ9_3MHZ_16KCK_65MS: u32 = 0x3B;
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
   /// Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time: 16K CK + 0 ms.
   pub const EXTXOSC_3MHZ_8MHZ_16KCK_0MS: u32 = 0x1D;
   /// Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time: 16K CK + 4.1 ms.
   pub const EXTXOSC_3MHZ_8MHZ_16KCK_4MS1: u32 = 0x2D;
   /// Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time: 16K CK + 65 ms.
   pub const EXTXOSC_3MHZ_8MHZ_16KCK_65MS: u32 = 0x3D;
   /// Ext. Crystal Osc. 8.0-    MHz; Start-up time: 258 CK + 4.1 ms.
   pub const EXTXOSC_8MHZ_XX_258CK_4MS1: u32 = 0xE;
   /// Ext. Crystal Osc. 8.0-    MHz; Start-up time: 258 CK + 65 ms.
   pub const EXTXOSC_8MHZ_XX_258CK_65MS: u32 = 0x1E;
   /// Ext. Crystal Osc. 8.0-    MHz; Start-up time: 1K CK + 0 ms.
   pub const EXTXOSC_8MHZ_XX_1KCK_0MS: u32 = 0x2E;
   /// Ext. Crystal Osc. 8.0-    MHz; Start-up time: 1K CK + 4.1 ms.
   pub const EXTXOSC_8MHZ_XX_1KCK_4MS1: u32 = 0x3E;
   /// Ext. Crystal Osc. 8.0-    MHz; Start-up time: 1K CK + 65 ms.
   pub const EXTXOSC_8MHZ_XX_1KCK_65MS: u32 = 0xF;
   /// Ext. Crystal Osc. 8.0-    MHz; Start-up time: 16K CK + 0 ms.
   pub const EXTXOSC_8MHZ_XX_16KCK_0MS: u32 = 0x1F;
   /// Ext. Crystal Osc. 8.0-    MHz; Start-up time: 16K CK + 4.1 ms.
   pub const EXTXOSC_8MHZ_XX_16KCK_4MS1: u32 = 0x2F;
   /// Ext. Crystal Osc. 8.0-    MHz; Start-up time: 16K CK + 65 ms.
   pub const EXTXOSC_8MHZ_XX_16KCK_65MS: u32 = 0x3F;
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

