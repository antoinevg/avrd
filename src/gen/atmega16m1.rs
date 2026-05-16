//! The AVR ATmega16M1 microcontroller
//!
//! # Variants
//! |        | Pinout | Mcu age | Operating temperature | Operating voltage | Max speed |
//! |--------|--------|---------|-----------------------|-------------------|-----------|
//! | ATmega16M1-AU | TQFPQFN32 | TQFP32 | -40°C - 85°C | 2.7V - 5.5V | 16 MHz |
//! | ATmega16M1-MU | TQFPQFN32 | QFN32 | -40°C - 85°C | 2.7V - 5.5V | 16 MHz |
//!

#![allow(non_upper_case_globals)]

/// `LOW` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CKOUT | 1000000 |
/// | SUT_CKSEL | 111111 |
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
/// | SPIEN | 100000 |
/// | WDTON | 10000 |
/// | BOOTRST | 1 |
/// | RSTDISBL | 10000000 |
/// | BOOTSZ | 110 |
/// | DWEN | 1000000 |
/// | EESAVE | 1000 |
pub const HIGH: *mut u8 = 0x1 as *mut u8;

/// `EXTENDED` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BODLEVEL | 111 |
/// | PSCRVB | 1000 |
/// | PSCRVA | 10000 |
/// | PSCRB | 100000 |
pub const EXTENDED: *mut u8 = 0x2 as *mut u8;

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

/// Port E Input Pins.
pub const PINE: *mut u8 = 0x2C as *mut u8;

/// Port E Data Direction Register.
pub const DDRE: *mut u8 = 0x2D as *mut u8;

/// Port E Data Register.
pub const PORTE: *mut u8 = 0x2E as *mut u8;

/// Timer/Counter0 Interrupt Flag register.
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
/// | ICF1 | 100000 |
/// | TOV1 | 1 |
/// | OCF1B | 100 |
pub const TIFR1: *mut u8 = 0x36 as *mut u8;

/// General Purpose IO Register 1.
pub const GPIOR1: *mut u8 = 0x39 as *mut u8;

/// General Purpose IO Register 2.
pub const GPIOR2: *mut u8 = 0x3A as *mut u8;

/// Pin Change Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCIF | 1111 |
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
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | GPIOR01 | 10 |
/// | GPIOR03 | 1000 |
/// | GPIOR05 | 100000 |
/// | GPIOR00 | 1 |
/// | GPIOR02 | 100 |
/// | GPIOR06 | 1000000 |
/// | GPIOR04 | 10000 |
/// | GPIOR07 | 10000000 |
pub const GPIOR0: *mut u8 = 0x3E as *mut u8;

/// EEPROM Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EEMWE | 100 |
/// | EEWE | 10 |
/// | EERE | 1 |
/// | EERIE | 1000 |
/// | EEPM | 110000 |
pub const EECR: *mut u8 = 0x3F as *mut u8;

/// EEPROM Data Register.
pub const EEDR: *mut u8 = 0x40 as *mut u8;

/// EEPROM Read/Write Access low byte.
pub const EEARL: *mut u8 = 0x41 as *mut u8;

/// EEPROM Read/Write Access.
pub const EEAR: *mut u16 = 0x41 as *mut u16;

/// EEPROM Read/Write Access high byte.
pub const EEARH: *mut u8 = 0x42 as *mut u8;

/// General Timer/Counter Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TSM | 10000000 |
/// | ICPSEL1 | 1000000 |
/// | PSRSYNC | 1 |
pub const GTCCR: *mut u8 = 0x43 as *mut u8;

/// Timer/Counter  Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WGM0 | 11 |
/// | COM0B | 110000 |
/// | COM0A | 11000000 |
pub const TCCR0A: *mut u8 = 0x44 as *mut u8;

/// Timer/Counter Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CS0 | 111 |
/// | FOC0B | 1000000 |
/// | WGM02 | 1000 |
/// | FOC0A | 10000000 |
pub const TCCR0B: *mut u8 = 0x45 as *mut u8;

/// Timer/Counter0.
pub const TCNT0: *mut u8 = 0x46 as *mut u8;

/// Timer/Counter0 Output Compare Register A.
pub const OCR0A: *mut u8 = 0x47 as *mut u8;

/// Timer/Counter0 Output Compare Register B.
pub const OCR0B: *mut u8 = 0x48 as *mut u8;

/// PLL Control And Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PLOCK | 1 |
/// | PLLF | 100 |
/// | PLLE | 10 |
pub const PLLCSR: *mut u8 = 0x49 as *mut u8;

/// SPI Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SPE | 1000000 |
/// | MSTR | 10000 |
/// | CPOL | 1000 |
/// | SPIE | 10000000 |
/// | CPHA | 100 |
/// | SPR | 11 |
/// | DORD | 100000 |
pub const SPCR: *mut u8 = 0x4C as *mut u8;

/// SPI Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SPI2X | 1 |
/// | WCOL | 1000000 |
/// | SPIF | 10000000 |
pub const SPSR: *mut u8 = 0x4D as *mut u8;

/// SPI Data Register.
pub const SPDR: *mut u8 = 0x4E as *mut u8;

/// Analog Comparator Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AC1O | 10 |
/// | AC0O | 1 |
/// | AC2O | 100 |
/// | AC2IF | 1000000 |
/// | AC3IF | 10000000 |
/// | AC3O | 1000 |
/// | AC1IF | 100000 |
/// | AC0IF | 10000 |
pub const ACSR: *mut u8 = 0x50 as *mut u8;

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
/// | IVCE | 1 |
/// | PUD | 10000 |
/// | SPIPS | 10000000 |
/// | IVSEL | 10 |
pub const MCUCR: *mut u8 = 0x55 as *mut u8;

/// Store Program Memory Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PGERS | 10 |
/// | RWWSB | 1000000 |
/// | SPMEN | 1 |
/// | RWWSRE | 10000 |
/// | BLBSET | 1000 |
/// | SPMIE | 10000000 |
/// | SIGRD | 100000 |
/// | PGWRT | 100 |
pub const SPMCSR: *mut u8 = 0x57 as *mut u8;

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
/// | T | 1000000 |
/// | Z | 10 |
/// | H | 100000 |
/// | S | 10000 |
/// | I | 10000000 |
/// | N | 100 |
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

/// `CLKPR` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CLKPCE | 10000000 |
/// | CLKPS | 1111 |
pub const CLKPR: *mut u8 = 0x61 as *mut u8;

/// Power Reduction Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRCAN | 1000000 |
/// | PRADC | 1 |
/// | PRPSC | 100000 |
/// | PRSPI | 100 |
/// | PRTIM0 | 1000 |
/// | PRLIN | 10 |
/// | PRTIM1 | 10000 |
pub const PRR: *mut u8 = 0x64 as *mut u8;

/// Oscillator Calibration Value.
pub const OSCCAL: *mut u8 = 0x66 as *mut u8;

/// Pin Change Interrupt Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCIE | 1111 |
pub const PCICR: *mut u8 = 0x68 as *mut u8;

/// External Interrupt Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ISC0 | 11 |
/// | ISC2 | 110000 |
/// | ISC3 | 11000000 |
/// | ISC1 | 1100 |
pub const EICRA: *mut u8 = 0x69 as *mut u8;

/// Pin Change Mask Register 0.
pub const PCMSK0: *mut u8 = 0x6A as *mut u8;

/// Pin Change Mask Register 1.
pub const PCMSK1: *mut u8 = 0x6B as *mut u8;

/// Pin Change Mask Register 2.
pub const PCMSK2: *mut u8 = 0x6C as *mut u8;

/// Pin Change Mask Register 3.
pub const PCMSK3: *mut u8 = 0x6D as *mut u8;

/// Timer/Counter0 Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCIE0A | 10 |
/// | TOIE0 | 1 |
/// | OCIE0B | 100 |
pub const TIMSK0: *mut u8 = 0x6E as *mut u8;

/// Timer/Counter Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ICIE1 | 100000 |
/// | OCIE1A | 10 |
/// | OCIE1B | 100 |
/// | TOIE1 | 1 |
pub const TIMSK1: *mut u8 = 0x6F as *mut u8;

/// Amplifier 0 Control and Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AMP0TS | 111 |
/// | AMP0G | 110000 |
/// | AMP0EN | 10000000 |
/// | AMPCMP0 | 1000 |
/// | AMP0IS | 1000000 |
pub const AMP0CSR: *mut u8 = 0x75 as *mut u8;

/// Amplifier 1 Control and Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AMPCMP1 | 1000 |
/// | AMP1TS | 111 |
/// | AMP1IS | 1000000 |
/// | AMP1EN | 10000000 |
/// | AMP1G | 110000 |
pub const AMP1CSR: *mut u8 = 0x76 as *mut u8;

/// Amplifier 2 Control and Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AMP2EN | 10000000 |
/// | AMP2TS | 111 |
/// | AMP2G | 110000 |
/// | AMPCMP2 | 1000 |
/// | AMP2IS | 1000000 |
pub const AMP2CSR: *mut u8 = 0x77 as *mut u8;

/// ADC Data Register Bytes.
pub const ADC: *mut u16 = 0x78 as *mut u16;

/// ADC Data Register Bytes low byte.
pub const ADCL: *mut u8 = 0x78 as *mut u8;

/// ADC Data Register Bytes high byte.
pub const ADCH: *mut u8 = 0x79 as *mut u8;

/// The ADC Control and Status register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADPS | 111 |
/// | ADSC | 1000000 |
/// | ADIE | 1000 |
/// | ADATE | 100000 |
/// | ADEN | 10000000 |
/// | ADIF | 10000 |
pub const ADCSRA: *mut u8 = 0x7A as *mut u8;

/// ADC Control and Status Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADTS | 1111 |
/// | AREFEN | 100000 |
/// | ADHSM | 10000000 |
/// | ISRCEN | 1000000 |
pub const ADCSRB: *mut u8 = 0x7B as *mut u8;

/// The ADC multiplexer Selection Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | REFS | 11000000 |
/// | ADLAR | 100000 |
/// | MUX | 11111 |
pub const ADMUX: *mut u8 = 0x7C as *mut u8;

/// Digital Input Disable Register 0.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADC6D | 1000000 |
/// | ADC7D | 10000000 |
/// | ADC1D | 10 |
/// | ADC4D | 10000 |
/// | ADC3D | 1000 |
/// | ADC0D | 1 |
/// | ADC5D | 100000 |
/// | ADC2D | 100 |
pub const DIDR0: *mut u8 = 0x7E as *mut u8;

/// Digital Input Disable Register 0.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADC10D | 100 |
/// | AMP0PD | 10000 |
/// | AMP0ND | 1000 |
/// | ADC8D | 1 |
/// | ACMP0D | 100000 |
/// | AMP2PD | 1000000 |
/// | ADC9D | 10 |
pub const DIDR1: *mut u8 = 0x7F as *mut u8;

/// Timer/Counter1 Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | COM1B | 110000 |
/// | COM1A | 11000000 |
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

/// Timer/Counter1 Control Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FOC1A | 10000000 |
/// | FOC1B | 1000000 |
pub const TCCR1C: *mut u8 = 0x82 as *mut u8;

/// Timer/Counter1 Bytes.
pub const TCNT1: *mut u16 = 0x84 as *mut u16;

/// Timer/Counter1 Bytes low byte.
pub const TCNT1L: *mut u8 = 0x84 as *mut u8;

/// Timer/Counter1 Bytes high byte.
pub const TCNT1H: *mut u8 = 0x85 as *mut u8;

/// Timer/Counter1 Input Capture Register low byte.
pub const ICR1L: *mut u8 = 0x86 as *mut u8;

/// Timer/Counter1 Input Capture Register.
pub const ICR1: *mut u16 = 0x86 as *mut u16;

/// Timer/Counter1 Input Capture Register high byte.
pub const ICR1H: *mut u8 = 0x87 as *mut u8;

/// Timer/Counter1 Output Compare Register A low byte.
pub const OCR1AL: *mut u8 = 0x88 as *mut u8;

/// Timer/Counter1 Output Compare Register A.
pub const OCR1A: *mut u16 = 0x88 as *mut u16;

/// Timer/Counter1 Output Compare Register A high byte.
pub const OCR1AH: *mut u8 = 0x89 as *mut u8;

/// Timer/Counter1 Output Compare Register B low byte.
pub const OCR1BL: *mut u8 = 0x8A as *mut u8;

/// Timer/Counter1 Output Compare Register B.
pub const OCR1B: *mut u16 = 0x8A as *mut u16;

/// Timer/Counter1 Output Compare Register B high byte.
pub const OCR1BH: *mut u8 = 0x8B as *mut u8;

/// DAC Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DATS | 1110000 |
/// | DALA | 100 |
/// | DAOE | 10 |
/// | DAEN | 1 |
/// | DAATE | 10000000 |
pub const DACON: *mut u8 = 0x90 as *mut u8;

/// DAC Data Register.
pub const DAC: *mut u16 = 0x91 as *mut u16;

/// DAC Data Register low byte.
pub const DACL: *mut u8 = 0x91 as *mut u8;

/// DAC Data Register high byte.
pub const DACH: *mut u8 = 0x92 as *mut u8;

/// Analog Comparator 0 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ACCKSEL | 1000 |
/// | AC0M | 111 |
/// | AC0IS | 110000 |
/// | AC0IE | 1000000 |
/// | AC0EN | 10000000 |
pub const AC0CON: *mut u8 = 0x94 as *mut u8;

/// Analog Comparator 1 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AC1EN | 10000000 |
/// | AC1IE | 1000000 |
/// | AC1IS | 110000 |
/// | AC1M | 111 |
/// | AC1ICE | 1000 |
pub const AC1CON: *mut u8 = 0x95 as *mut u8;

/// Analog Comparator 2 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AC2EN | 10000000 |
/// | AC2M | 111 |
/// | AC2IS | 110000 |
/// | AC2IE | 1000000 |
pub const AC2CON: *mut u8 = 0x96 as *mut u8;

/// Analog Comparator 3 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AC3IE | 1000000 |
/// | AC3M | 111 |
/// | AC3EN | 10000000 |
/// | AC3IS | 110000 |
pub const AC3CON: *mut u8 = 0x97 as *mut u8;

/// PSC Module 0 Output Compare SA Register.
pub const POCR0SA: *mut u16 = 0xA0 as *mut u16;

/// PSC Module 0 Output Compare SA Register low byte.
pub const POCR0SAL: *mut u8 = 0xA0 as *mut u8;

/// PSC Module 0 Output Compare SA Register high byte.
pub const POCR0SAH: *mut u8 = 0xA1 as *mut u8;

/// PSC Module 0 Output Compare RA Register.
pub const POCR0RA: *mut u16 = 0xA2 as *mut u16;

/// PSC Module 0 Output Compare RA Register low byte.
pub const POCR0RAL: *mut u8 = 0xA2 as *mut u8;

/// PSC Module 0 Output Compare RA Register high byte.
pub const POCR0RAH: *mut u8 = 0xA3 as *mut u8;

/// PSC Module 0 Output Compare SB Register low byte.
pub const POCR0SBL: *mut u8 = 0xA4 as *mut u8;

/// PSC Module 0 Output Compare SB Register.
pub const POCR0SB: *mut u16 = 0xA4 as *mut u16;

/// PSC Module 0 Output Compare SB Register high byte.
pub const POCR0SBH: *mut u8 = 0xA5 as *mut u8;

/// PSC Module 1 Output Compare SA Register.
pub const POCR1SA: *mut u16 = 0xA6 as *mut u16;

/// PSC Module 1 Output Compare SA Register low byte.
pub const POCR1SAL: *mut u8 = 0xA6 as *mut u8;

/// PSC Module 1 Output Compare SA Register high byte.
pub const POCR1SAH: *mut u8 = 0xA7 as *mut u8;

/// PSC Module 1 Output Compare RA Register.
pub const POCR1RA: *mut u16 = 0xA8 as *mut u16;

/// PSC Module 1 Output Compare RA Register low byte.
pub const POCR1RAL: *mut u8 = 0xA8 as *mut u8;

/// PSC Module 1 Output Compare RA Register high byte.
pub const POCR1RAH: *mut u8 = 0xA9 as *mut u8;

/// PSC Module 1 Output Compare SB Register low byte.
pub const POCR1SBL: *mut u8 = 0xAA as *mut u8;

/// PSC Module 1 Output Compare SB Register.
pub const POCR1SB: *mut u16 = 0xAA as *mut u16;

/// PSC Module 1 Output Compare SB Register high byte.
pub const POCR1SBH: *mut u8 = 0xAB as *mut u8;

/// PSC Module 2 Output Compare SA Register low byte.
pub const POCR2SAL: *mut u8 = 0xAC as *mut u8;

/// PSC Module 2 Output Compare SA Register.
pub const POCR2SA: *mut u16 = 0xAC as *mut u16;

/// PSC Module 2 Output Compare SA Register high byte.
pub const POCR2SAH: *mut u8 = 0xAD as *mut u8;

/// PSC Module 2 Output Compare RA Register low byte.
pub const POCR2RAL: *mut u8 = 0xAE as *mut u8;

/// PSC Module 2 Output Compare RA Register.
pub const POCR2RA: *mut u16 = 0xAE as *mut u16;

/// PSC Module 2 Output Compare RA Register high byte.
pub const POCR2RAH: *mut u8 = 0xAF as *mut u8;

/// PSC Module 2 Output Compare SB Register low byte.
pub const POCR2SBL: *mut u8 = 0xB0 as *mut u8;

/// PSC Module 2 Output Compare SB Register.
pub const POCR2SB: *mut u16 = 0xB0 as *mut u16;

/// PSC Module 2 Output Compare SB Register high byte.
pub const POCR2SBH: *mut u8 = 0xB1 as *mut u8;

/// PSC Output Compare RB Register low byte.
pub const POCR_RBL: *mut u8 = 0xB2 as *mut u8;

/// PSC Output Compare RB Register.
pub const POCR_RB: *mut u16 = 0xB2 as *mut u16;

/// PSC Output Compare RB Register high byte.
pub const POCR_RBH: *mut u8 = 0xB3 as *mut u8;

/// PSC Synchro Configuration.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PSYNC2 | 110000 |
/// | PSYNC0 | 11 |
/// | PSYNC1 | 1100 |
pub const PSYNC: *mut u8 = 0xB4 as *mut u8;

/// PSC Configuration Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | POPA | 100 |
/// | POPB | 1000 |
/// | PULOCK | 100000 |
/// | PMODE | 10000 |
pub const PCNF: *mut u8 = 0xB5 as *mut u8;

/// PSC Output Configuration.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | POEN2B | 100000 |
/// | POEN2A | 10000 |
/// | POEN1A | 100 |
/// | POEN0B | 10 |
/// | POEN1B | 1000 |
/// | POEN0A | 1 |
pub const POC: *mut u8 = 0xB6 as *mut u8;

/// PSC Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCLKSEL | 100000 |
/// | PPRE | 11000000 |
/// | PRUN | 1 |
/// | PCCYC | 10 |
pub const PCTL: *mut u8 = 0xB7 as *mut u8;

/// PSC Module 0 Input Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PFLTE0 | 10000 |
/// | PELEV0 | 100000 |
/// | PRFM0 | 111 |
/// | PAOC0 | 1000 |
/// | POVEN0 | 10000000 |
/// | PISEL0 | 1000000 |
pub const PMIC0: *mut u8 = 0xB8 as *mut u8;

/// PSC Module 1 Input Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PFLTE1 | 10000 |
/// | PRFM1 | 111 |
/// | POVEN1 | 10000000 |
/// | PAOC1 | 1000 |
/// | PISEL1 | 1000000 |
/// | PELEV1 | 100000 |
pub const PMIC1: *mut u8 = 0xB9 as *mut u8;

/// PSC Module 2 Input Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PISEL2 | 1000000 |
/// | PAOC2 | 1000 |
/// | PRFM2 | 111 |
/// | PELEV2 | 100000 |
/// | POVEN2 | 10000000 |
/// | PFLTE2 | 10000 |
pub const PMIC2: *mut u8 = 0xBA as *mut u8;

/// PSC Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PEVE | 1110 |
/// | PEOPE | 1 |
pub const PIM: *mut u8 = 0xBB as *mut u8;

/// PSC Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PEOP | 1 |
/// | PEV | 1110 |
pub const PIFR: *mut u8 = 0xBC as *mut u8;

/// LIN Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LSWRES | 10000000 |
/// | LIN13 | 1000000 |
/// | LCMD | 111 |
/// | LCONF | 110000 |
/// | LENA | 1000 |
pub const LINCR: *mut u8 = 0xC8 as *mut u8;

/// LIN Status and Interrupt Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LBUSY | 10000 |
/// | LIDST | 11100000 |
/// | LIDOK | 100 |
/// | LTXOK | 10 |
/// | LRXOK | 1 |
/// | LERR | 1000 |
pub const LINSIR: *mut u8 = 0xC9 as *mut u8;

/// LIN Enable Interrupt Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LENTXOK | 10 |
/// | LENIDOK | 100 |
/// | LENRXOK | 1 |
/// | LENERR | 1000 |
pub const LINENIR: *mut u8 = 0xCA as *mut u8;

/// LIN Error Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LPERR | 100 |
/// | LBERR | 1 |
/// | LABORT | 10000000 |
/// | LTOERR | 1000000 |
/// | LCERR | 10 |
/// | LOVERR | 100000 |
/// | LSERR | 1000 |
/// | LFERR | 10000 |
pub const LINERR: *mut u8 = 0xCB as *mut u8;

/// LIN Bit Timing Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LDISR | 10000000 |
/// | LBT | 111111 |
pub const LINBTR: *mut u8 = 0xCC as *mut u8;

/// LIN Baud Rate Register low byte.
pub const LINBRRL: *mut u8 = 0xCD as *mut u8;

/// LIN Baud Rate Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LDIV | 111111111111 |
pub const LINBRR: *mut u16 = 0xCD as *mut u16;

/// LIN Baud Rate Register high byte.
pub const LINBRRH: *mut u8 = 0xCE as *mut u8;

/// LIN Data Length Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LRXDL | 1111 |
/// | LTXDL | 11110000 |
pub const LINDLR: *mut u8 = 0xCF as *mut u8;

/// LIN Identifier Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LID | 111111 |
/// | LP | 11000000 |
pub const LINIDR: *mut u8 = 0xD0 as *mut u8;

/// LIN Data Buffer Selection Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LAINC | 1000 |
/// | LINDX | 111 |
pub const LINSEL: *mut u8 = 0xD1 as *mut u8;

/// LIN Data Register.
pub const LINDAT: *mut u8 = 0xD2 as *mut u8;

/// CAN General Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ABRQ | 10000000 |
/// | TTC | 100000 |
/// | TEST | 100 |
/// | SYNTTC | 10000 |
/// | SWRES | 1 |
/// | LISTEN | 1000 |
/// | OVRQ | 1000000 |
/// | ENASTB | 10 |
pub const CANGCON: *mut u8 = 0xD8 as *mut u8;

/// CAN General Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BOFF | 10 |
/// | RXBSY | 1000 |
/// | TXBSY | 10000 |
/// | OVFG | 1000000 |
/// | ENFG | 100 |
/// | ERRP | 1 |
pub const CANGSTA: *mut u8 = 0xD9 as *mut u8;

/// CAN General Interrupt Register Flags.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FERG | 10 |
/// | BXOK | 10000 |
/// | CERG | 100 |
/// | OVRTIM | 100000 |
/// | SERG | 1000 |
/// | CANIT | 10000000 |
/// | AERG | 1 |
/// | BOFFIT | 1000000 |
pub const CANGIT: *mut u8 = 0xDA as *mut u8;

/// CAN General Interrupt Enable Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ENERR | 1000 |
/// | ENTX | 10000 |
/// | ENERG | 10 |
/// | ENOVRT | 1 |
/// | ENBOFF | 1000000 |
/// | ENRX | 100000 |
/// | ENBX | 100 |
/// | ENIT | 10000000 |
pub const CANGIE: *mut u8 = 0xDB as *mut u8;

/// Enable MOb Register 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ENMOB | 111111 |
pub const CANEN2: *mut u8 = 0xDC as *mut u8;

/// Enable MOb Register 1(empty).
pub const CANEN1: *mut u8 = 0xDD as *mut u8;

/// Enable Interrupt MOb Register 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | IEMOB | 111111 |
pub const CANIE2: *mut u8 = 0xDE as *mut u8;

/// Enable Interrupt MOb Register 1 (empty).
pub const CANIE1: *mut u8 = 0xDF as *mut u8;

/// CAN Status Interrupt MOb Register 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SIT | 111111 |
pub const CANSIT2: *mut u8 = 0xE0 as *mut u8;

/// CAN Status Interrupt MOb Register 1 (empty).
pub const CANSIT1: *mut u8 = 0xE1 as *mut u8;

/// CAN Bit Timing Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BRP | 1111110 |
pub const CANBT1: *mut u8 = 0xE2 as *mut u8;

/// CAN Bit Timing Register 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRS | 1110 |
/// | SJW | 1100000 |
pub const CANBT2: *mut u8 = 0xE3 as *mut u8;

/// CAN Bit Timing Register 3.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PHS1 | 1110 |
/// | SMP | 1 |
/// | PHS2 | 1110000 |
pub const CANBT3: *mut u8 = 0xE4 as *mut u8;

/// Timer Control Register.
pub const CANTCON: *mut u8 = 0xE5 as *mut u8;

/// Timer Register.
pub const CANTIM: *mut u16 = 0xE6 as *mut u16;

/// Timer Register low byte.
pub const CANTIML: *mut u8 = 0xE6 as *mut u8;

/// Timer Register high byte.
pub const CANTIMH: *mut u8 = 0xE7 as *mut u8;

/// TTC Timer Register.
pub const CANTTC: *mut u16 = 0xE8 as *mut u16;

/// TTC Timer Register low byte.
pub const CANTTCL: *mut u8 = 0xE8 as *mut u8;

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
/// | INDX | 111 |
/// | MOBNB | 11110000 |
/// | AINC | 1000 |
pub const CANPAGE: *mut u8 = 0xED as *mut u8;

/// MOb Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DLCW | 10000000 |
/// | FERR | 10 |
/// | AERR | 1 |
/// | TXOK | 1000000 |
/// | BERR | 10000 |
/// | SERR | 1000 |
/// | CERR | 100 |
/// | RXOK | 100000 |
pub const CANSTMOB: *mut u8 = 0xEE as *mut u8;

/// MOb Control and DLC Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | IDE | 10000 |
/// | DLC | 1111 |
/// | RPLV | 100000 |
/// | CONMOB | 11000000 |
pub const CANCDMOB: *mut u8 = 0xEF as *mut u8;

/// Identifier Tag Register 4.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RB0TAG | 1 |
/// | RB1TAG | 10 |
/// | RTRTAG | 100 |
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
/// | RTRMSK | 100 |
/// | IDEMSK | 1 |
pub const CANIDM4: *mut u8 = 0xF4 as *mut u8;

/// Identifier Mask Register 3.
pub const CANIDM3: *mut u8 = 0xF5 as *mut u8;

/// Identifier Mask Register 2.
pub const CANIDM2: *mut u8 = 0xF6 as *mut u8;

/// Identifier Mask Register 1.
pub const CANIDM1: *mut u8 = 0xF7 as *mut u8;

/// Time Stamp Register low byte.
pub const CANSTML: *mut u8 = 0xF8 as *mut u8;

/// Time Stamp Register.
pub const CANSTM: *mut u16 = 0xF8 as *mut u16;

/// Time Stamp Register high byte.
pub const CANSTMH: *mut u8 = 0xF9 as *mut u8;

/// Message Data Register.
pub const CANMSG: *mut u8 = 0xFA as *mut u8;

/// Bitfield on register `AC0CON`
pub const ACCKSEL: u8 = 0x8;

/// Bitfield on register `AC0CON`
pub const AC0M: u8 = 0x7;

/// Bitfield on register `AC0CON`
pub const AC0IS: u8 = 0x30;

/// Bitfield on register `AC0CON`
pub const AC0IE: u8 = 0x40;

/// Bitfield on register `AC0CON`
pub const AC0EN: u8 = 0x80;

/// Bitfield on register `AC1CON`
pub const AC1EN: u8 = 0x80;

/// Bitfield on register `AC1CON`
pub const AC1IE: u8 = 0x40;

/// Bitfield on register `AC1CON`
pub const AC1IS: u8 = 0x30;

/// Bitfield on register `AC1CON`
pub const AC1M: u8 = 0x7;

/// Bitfield on register `AC1CON`
pub const AC1ICE: u8 = 0x8;

/// Bitfield on register `AC2CON`
pub const AC2EN: u8 = 0x80;

/// Bitfield on register `AC2CON`
pub const AC2M: u8 = 0x7;

/// Bitfield on register `AC2CON`
pub const AC2IS: u8 = 0x30;

/// Bitfield on register `AC2CON`
pub const AC2IE: u8 = 0x40;

/// Bitfield on register `AC3CON`
pub const AC3IE: u8 = 0x40;

/// Bitfield on register `AC3CON`
pub const AC3M: u8 = 0x7;

/// Bitfield on register `AC3CON`
pub const AC3EN: u8 = 0x80;

/// Bitfield on register `AC3CON`
pub const AC3IS: u8 = 0x30;

/// Bitfield on register `ACSR`
pub const AC1O: u8 = 0x2;

/// Bitfield on register `ACSR`
pub const AC0O: u8 = 0x1;

/// Bitfield on register `ACSR`
pub const AC2O: u8 = 0x4;

/// Bitfield on register `ACSR`
pub const AC2IF: u8 = 0x40;

/// Bitfield on register `ACSR`
pub const AC3IF: u8 = 0x80;

/// Bitfield on register `ACSR`
pub const AC3O: u8 = 0x8;

/// Bitfield on register `ACSR`
pub const AC1IF: u8 = 0x20;

/// Bitfield on register `ACSR`
pub const AC0IF: u8 = 0x10;

/// Bitfield on register `ADCSRA`
pub const ADPS: u8 = 0x7;

/// Bitfield on register `ADCSRA`
pub const ADSC: u8 = 0x40;

/// Bitfield on register `ADCSRA`
pub const ADIE: u8 = 0x8;

/// Bitfield on register `ADCSRA`
pub const ADATE: u8 = 0x20;

/// Bitfield on register `ADCSRA`
pub const ADEN: u8 = 0x80;

/// Bitfield on register `ADCSRA`
pub const ADIF: u8 = 0x10;

/// Bitfield on register `ADCSRB`
pub const ADTS: u8 = 0xF;

/// Bitfield on register `ADCSRB`
pub const AREFEN: u8 = 0x20;

/// Bitfield on register `ADCSRB`
pub const ADHSM: u8 = 0x80;

/// Bitfield on register `ADCSRB`
pub const ISRCEN: u8 = 0x40;

/// Bitfield on register `ADMUX`
pub const REFS: u8 = 0xC0;

/// Bitfield on register `ADMUX`
pub const ADLAR: u8 = 0x20;

/// Bitfield on register `ADMUX`
pub const MUX: u8 = 0x1F;

/// Bitfield on register `AMP0CSR`
pub const AMP0TS: u8 = 0x7;

/// Bitfield on register `AMP0CSR`
pub const AMP0G: u8 = 0x30;

/// Bitfield on register `AMP0CSR`
pub const AMP0EN: u8 = 0x80;

/// Bitfield on register `AMP0CSR`
pub const AMPCMP0: u8 = 0x8;

/// Bitfield on register `AMP0CSR`
pub const AMP0IS: u8 = 0x40;

/// Bitfield on register `AMP1CSR`
pub const AMPCMP1: u8 = 0x8;

/// Bitfield on register `AMP1CSR`
pub const AMP1TS: u8 = 0x7;

/// Bitfield on register `AMP1CSR`
pub const AMP1IS: u8 = 0x40;

/// Bitfield on register `AMP1CSR`
pub const AMP1EN: u8 = 0x80;

/// Bitfield on register `AMP1CSR`
pub const AMP1G: u8 = 0x30;

/// Bitfield on register `AMP2CSR`
pub const AMP2EN: u8 = 0x80;

/// Bitfield on register `AMP2CSR`
pub const AMP2TS: u8 = 0x7;

/// Bitfield on register `AMP2CSR`
pub const AMP2G: u8 = 0x30;

/// Bitfield on register `AMP2CSR`
pub const AMPCMP2: u8 = 0x8;

/// Bitfield on register `AMP2CSR`
pub const AMP2IS: u8 = 0x40;

/// Bitfield on register `CANBT1`
pub const BRP: u8 = 0x7E;

/// Bitfield on register `CANBT2`
pub const PRS: u8 = 0xE;

/// Bitfield on register `CANBT2`
pub const SJW: u8 = 0x60;

/// Bitfield on register `CANBT3`
pub const PHS1: u8 = 0xE;

/// Bitfield on register `CANBT3`
pub const SMP: u8 = 0x1;

/// Bitfield on register `CANBT3`
pub const PHS2: u8 = 0x70;

/// Bitfield on register `CANCDMOB`
pub const IDE: u8 = 0x10;

/// Bitfield on register `CANCDMOB`
pub const DLC: u8 = 0xF;

/// Bitfield on register `CANCDMOB`
pub const RPLV: u8 = 0x20;

/// Bitfield on register `CANCDMOB`
pub const CONMOB: u8 = 0xC0;

/// Bitfield on register `CANEN2`
pub const ENMOB: u8 = 0x3F;

/// Bitfield on register `CANGCON`
pub const ABRQ: u8 = 0x80;

/// Bitfield on register `CANGCON`
pub const TTC: u8 = 0x20;

/// Bitfield on register `CANGCON`
pub const TEST: u8 = 0x4;

/// Bitfield on register `CANGCON`
pub const SYNTTC: u8 = 0x10;

/// Bitfield on register `CANGCON`
pub const SWRES: u8 = 0x1;

/// Bitfield on register `CANGCON`
pub const LISTEN: u8 = 0x8;

/// Bitfield on register `CANGCON`
pub const OVRQ: u8 = 0x40;

/// Bitfield on register `CANGCON`
pub const ENASTB: u8 = 0x2;

/// Bitfield on register `CANGIE`
pub const ENERR: u8 = 0x8;

/// Bitfield on register `CANGIE`
pub const ENTX: u8 = 0x10;

/// Bitfield on register `CANGIE`
pub const ENERG: u8 = 0x2;

/// Bitfield on register `CANGIE`
pub const ENOVRT: u8 = 0x1;

/// Bitfield on register `CANGIE`
pub const ENBOFF: u8 = 0x40;

/// Bitfield on register `CANGIE`
pub const ENRX: u8 = 0x20;

/// Bitfield on register `CANGIE`
pub const ENBX: u8 = 0x4;

/// Bitfield on register `CANGIE`
pub const ENIT: u8 = 0x80;

/// Bitfield on register `CANGIT`
pub const FERG: u8 = 0x2;

/// Bitfield on register `CANGIT`
pub const BXOK: u8 = 0x10;

/// Bitfield on register `CANGIT`
pub const CERG: u8 = 0x4;

/// Bitfield on register `CANGIT`
pub const OVRTIM: u8 = 0x20;

/// Bitfield on register `CANGIT`
pub const SERG: u8 = 0x8;

/// Bitfield on register `CANGIT`
pub const CANIT: u8 = 0x80;

/// Bitfield on register `CANGIT`
pub const AERG: u8 = 0x1;

/// Bitfield on register `CANGIT`
pub const BOFFIT: u8 = 0x40;

/// Bitfield on register `CANGSTA`
pub const BOFF: u8 = 0x2;

/// Bitfield on register `CANGSTA`
pub const RXBSY: u8 = 0x8;

/// Bitfield on register `CANGSTA`
pub const TXBSY: u8 = 0x10;

/// Bitfield on register `CANGSTA`
pub const OVFG: u8 = 0x40;

/// Bitfield on register `CANGSTA`
pub const ENFG: u8 = 0x4;

/// Bitfield on register `CANGSTA`
pub const ERRP: u8 = 0x1;

/// Bitfield on register `CANHPMOB`
pub const CGP: u8 = 0xF;

/// Bitfield on register `CANHPMOB`
pub const HPMOB: u8 = 0xF0;

/// Bitfield on register `CANIDM4`
pub const RTRMSK: u8 = 0x4;

/// Bitfield on register `CANIDM4`
pub const IDEMSK: u8 = 0x1;

/// Bitfield on register `CANIDT4`
pub const RB0TAG: u8 = 0x1;

/// Bitfield on register `CANIDT4`
pub const RB1TAG: u8 = 0x2;

/// Bitfield on register `CANIDT4`
pub const RTRTAG: u8 = 0x4;

/// Bitfield on register `CANIE2`
pub const IEMOB: u8 = 0x3F;

/// Bitfield on register `CANPAGE`
pub const INDX: u8 = 0x7;

/// Bitfield on register `CANPAGE`
pub const MOBNB: u8 = 0xF0;

/// Bitfield on register `CANPAGE`
pub const AINC: u8 = 0x8;

/// Bitfield on register `CANSIT2`
pub const SIT: u8 = 0x3F;

/// Bitfield on register `CANSTMOB`
pub const DLCW: u8 = 0x80;

/// Bitfield on register `CANSTMOB`
pub const FERR: u8 = 0x2;

/// Bitfield on register `CANSTMOB`
pub const AERR: u8 = 0x1;

/// Bitfield on register `CANSTMOB`
pub const TXOK: u8 = 0x40;

/// Bitfield on register `CANSTMOB`
pub const BERR: u8 = 0x10;

/// Bitfield on register `CANSTMOB`
pub const SERR: u8 = 0x8;

/// Bitfield on register `CANSTMOB`
pub const CERR: u8 = 0x4;

/// Bitfield on register `CANSTMOB`
pub const RXOK: u8 = 0x20;

/// Bitfield on register `CLKPR`
pub const CLKPCE: u8 = 0x80;

/// Bitfield on register `CLKPR`
pub const CLKPS: u8 = 0xF;

/// Bitfield on register `DACON`
pub const DATS: u8 = 0x70;

/// Bitfield on register `DACON`
pub const DALA: u8 = 0x4;

/// Bitfield on register `DACON`
pub const DAOE: u8 = 0x2;

/// Bitfield on register `DACON`
pub const DAEN: u8 = 0x1;

/// Bitfield on register `DACON`
pub const DAATE: u8 = 0x80;

/// Bitfield on register `DIDR0`
pub const ADC6D: u8 = 0x40;

/// Bitfield on register `DIDR0`
pub const ADC7D: u8 = 0x80;

/// Bitfield on register `DIDR0`
pub const ADC1D: u8 = 0x2;

/// Bitfield on register `DIDR0`
pub const ADC4D: u8 = 0x10;

/// Bitfield on register `DIDR0`
pub const ADC3D: u8 = 0x8;

/// Bitfield on register `DIDR0`
pub const ADC0D: u8 = 0x1;

/// Bitfield on register `DIDR0`
pub const ADC5D: u8 = 0x20;

/// Bitfield on register `DIDR0`
pub const ADC2D: u8 = 0x4;

/// Bitfield on register `DIDR1`
pub const ADC10D: u8 = 0x4;

/// Bitfield on register `DIDR1`
pub const AMP0PD: u8 = 0x10;

/// Bitfield on register `DIDR1`
pub const AMP0ND: u8 = 0x8;

/// Bitfield on register `DIDR1`
pub const ADC8D: u8 = 0x1;

/// Bitfield on register `DIDR1`
pub const ACMP0D: u8 = 0x20;

/// Bitfield on register `DIDR1`
pub const AMP2PD: u8 = 0x40;

/// Bitfield on register `DIDR1`
pub const ADC9D: u8 = 0x2;

/// Bitfield on register `EECR`
pub const EEMWE: u8 = 0x4;

/// Bitfield on register `EECR`
pub const EEWE: u8 = 0x2;

/// Bitfield on register `EECR`
pub const EERE: u8 = 0x1;

/// Bitfield on register `EECR`
pub const EERIE: u8 = 0x8;

/// Bitfield on register `EECR`
pub const EEPM: u8 = 0x30;

/// Bitfield on register `EICRA`
pub const ISC0: u8 = 0x3;

/// Bitfield on register `EICRA`
pub const ISC2: u8 = 0x30;

/// Bitfield on register `EICRA`
pub const ISC3: u8 = 0xC0;

/// Bitfield on register `EICRA`
pub const ISC1: u8 = 0xC;

/// Bitfield on register `EIFR`
pub const INTF: u8 = 0xF;

/// Bitfield on register `EIMSK`
pub const INT: u8 = 0xF;

/// Bitfield on register `EXTENDED`
pub const BODLEVEL: u8 = 0x7;

/// Bitfield on register `EXTENDED`
pub const PSCRVB: u8 = 0x8;

/// Bitfield on register `EXTENDED`
pub const PSCRVA: u8 = 0x10;

/// Bitfield on register `EXTENDED`
pub const PSCRB: u8 = 0x20;

/// Bitfield on register `GPIOR0`
pub const GPIOR01: u8 = 0x2;

/// Bitfield on register `GPIOR0`
pub const GPIOR03: u8 = 0x8;

/// Bitfield on register `GPIOR0`
pub const GPIOR05: u8 = 0x20;

/// Bitfield on register `GPIOR0`
pub const GPIOR00: u8 = 0x1;

/// Bitfield on register `GPIOR0`
pub const GPIOR02: u8 = 0x4;

/// Bitfield on register `GPIOR0`
pub const GPIOR06: u8 = 0x40;

/// Bitfield on register `GPIOR0`
pub const GPIOR04: u8 = 0x10;

/// Bitfield on register `GPIOR0`
pub const GPIOR07: u8 = 0x80;

/// Bitfield on register `GTCCR`
pub const TSM: u8 = 0x80;

/// Bitfield on register `GTCCR`
pub const ICPSEL1: u8 = 0x40;

/// Bitfield on register `GTCCR`
pub const PSRSYNC: u8 = 0x1;

/// Bitfield on register `HIGH`
pub const SPIEN: u8 = 0x20;

/// Bitfield on register `HIGH`
pub const WDTON: u8 = 0x10;

/// Bitfield on register `HIGH`
pub const BOOTRST: u8 = 0x1;

/// Bitfield on register `HIGH`
pub const RSTDISBL: u8 = 0x80;

/// Bitfield on register `HIGH`
pub const BOOTSZ: u8 = 0x6;

/// Bitfield on register `HIGH`
pub const DWEN: u8 = 0x40;

/// Bitfield on register `HIGH`
pub const EESAVE: u8 = 0x8;

/// Bitfield on register `LINBRR`
pub const LDIV: u16 = 0xFFF;

/// Bitfield on register `LINBTR`
pub const LDISR: u8 = 0x80;

/// Bitfield on register `LINBTR`
pub const LBT: u8 = 0x3F;

/// Bitfield on register `LINCR`
pub const LSWRES: u8 = 0x80;

/// Bitfield on register `LINCR`
pub const LIN13: u8 = 0x40;

/// Bitfield on register `LINCR`
pub const LCMD: u8 = 0x7;

/// Bitfield on register `LINCR`
pub const LCONF: u8 = 0x30;

/// Bitfield on register `LINCR`
pub const LENA: u8 = 0x8;

/// Bitfield on register `LINDLR`
pub const LRXDL: u8 = 0xF;

/// Bitfield on register `LINDLR`
pub const LTXDL: u8 = 0xF0;

/// Bitfield on register `LINENIR`
pub const LENTXOK: u8 = 0x2;

/// Bitfield on register `LINENIR`
pub const LENIDOK: u8 = 0x4;

/// Bitfield on register `LINENIR`
pub const LENRXOK: u8 = 0x1;

/// Bitfield on register `LINENIR`
pub const LENERR: u8 = 0x8;

/// Bitfield on register `LINERR`
pub const LPERR: u8 = 0x4;

/// Bitfield on register `LINERR`
pub const LBERR: u8 = 0x1;

/// Bitfield on register `LINERR`
pub const LABORT: u8 = 0x80;

/// Bitfield on register `LINERR`
pub const LTOERR: u8 = 0x40;

/// Bitfield on register `LINERR`
pub const LCERR: u8 = 0x2;

/// Bitfield on register `LINERR`
pub const LOVERR: u8 = 0x20;

/// Bitfield on register `LINERR`
pub const LSERR: u8 = 0x8;

/// Bitfield on register `LINERR`
pub const LFERR: u8 = 0x10;

/// Bitfield on register `LINIDR`
pub const LID: u8 = 0x3F;

/// Bitfield on register `LINIDR`
pub const LP: u8 = 0xC0;

/// Bitfield on register `LINSEL`
pub const LAINC: u8 = 0x8;

/// Bitfield on register `LINSEL`
pub const LINDX: u8 = 0x7;

/// Bitfield on register `LINSIR`
pub const LBUSY: u8 = 0x10;

/// Bitfield on register `LINSIR`
pub const LIDST: u8 = 0xE0;

/// Bitfield on register `LINSIR`
pub const LIDOK: u8 = 0x4;

/// Bitfield on register `LINSIR`
pub const LTXOK: u8 = 0x2;

/// Bitfield on register `LINSIR`
pub const LRXOK: u8 = 0x1;

/// Bitfield on register `LINSIR`
pub const LERR: u8 = 0x8;

/// Bitfield on register `LOCKBIT`
pub const BLB1: u8 = 0x30;

/// Bitfield on register `LOCKBIT`
pub const BLB0: u8 = 0xC;

/// Bitfield on register `LOCKBIT`
pub const LB: u8 = 0x3;

/// Bitfield on register `LOW`
pub const CKOUT: u8 = 0x40;

/// Bitfield on register `LOW`
pub const SUT_CKSEL: u8 = 0x3F;

/// Bitfield on register `LOW`
pub const CKDIV8: u8 = 0x80;

/// Bitfield on register `MCUCR`
pub const IVCE: u8 = 0x1;

/// Bitfield on register `MCUCR`
pub const PUD: u8 = 0x10;

/// Bitfield on register `MCUCR`
pub const SPIPS: u8 = 0x80;

/// Bitfield on register `MCUCR`
pub const IVSEL: u8 = 0x2;

/// Bitfield on register `MCUSR`
pub const EXTRF: u8 = 0x2;

/// Bitfield on register `MCUSR`
pub const WDRF: u8 = 0x8;

/// Bitfield on register `MCUSR`
pub const PORF: u8 = 0x1;

/// Bitfield on register `MCUSR`
pub const BORF: u8 = 0x4;

/// Bitfield on register `PCICR`
pub const PCIE: u8 = 0xF;

/// Bitfield on register `PCIFR`
pub const PCIF: u8 = 0xF;

/// Bitfield on register `PCNF`
pub const POPA: u8 = 0x4;

/// Bitfield on register `PCNF`
pub const POPB: u8 = 0x8;

/// Bitfield on register `PCNF`
pub const PULOCK: u8 = 0x20;

/// Bitfield on register `PCNF`
pub const PMODE: u8 = 0x10;

/// Bitfield on register `PCTL`
pub const PCLKSEL: u8 = 0x20;

/// Bitfield on register `PCTL`
pub const PPRE: u8 = 0xC0;

/// Bitfield on register `PCTL`
pub const PRUN: u8 = 0x1;

/// Bitfield on register `PCTL`
pub const PCCYC: u8 = 0x2;

/// Bitfield on register `PIFR`
pub const PEOP: u8 = 0x1;

/// Bitfield on register `PIFR`
pub const PEV: u8 = 0xE;

/// Bitfield on register `PIM`
pub const PEVE: u8 = 0xE;

/// Bitfield on register `PIM`
pub const PEOPE: u8 = 0x1;

/// Bitfield on register `PLLCSR`
pub const PLOCK: u8 = 0x1;

/// Bitfield on register `PLLCSR`
pub const PLLF: u8 = 0x4;

/// Bitfield on register `PLLCSR`
pub const PLLE: u8 = 0x2;

/// Bitfield on register `PMIC0`
pub const PFLTE0: u8 = 0x10;

/// Bitfield on register `PMIC0`
pub const PELEV0: u8 = 0x20;

/// Bitfield on register `PMIC0`
pub const PRFM0: u8 = 0x7;

/// Bitfield on register `PMIC0`
pub const PAOC0: u8 = 0x8;

/// Bitfield on register `PMIC0`
pub const POVEN0: u8 = 0x80;

/// Bitfield on register `PMIC0`
pub const PISEL0: u8 = 0x40;

/// Bitfield on register `PMIC1`
pub const PFLTE1: u8 = 0x10;

/// Bitfield on register `PMIC1`
pub const PRFM1: u8 = 0x7;

/// Bitfield on register `PMIC1`
pub const POVEN1: u8 = 0x80;

/// Bitfield on register `PMIC1`
pub const PAOC1: u8 = 0x8;

/// Bitfield on register `PMIC1`
pub const PISEL1: u8 = 0x40;

/// Bitfield on register `PMIC1`
pub const PELEV1: u8 = 0x20;

/// Bitfield on register `PMIC2`
pub const PISEL2: u8 = 0x40;

/// Bitfield on register `PMIC2`
pub const PAOC2: u8 = 0x8;

/// Bitfield on register `PMIC2`
pub const PRFM2: u8 = 0x7;

/// Bitfield on register `PMIC2`
pub const PELEV2: u8 = 0x20;

/// Bitfield on register `PMIC2`
pub const POVEN2: u8 = 0x80;

/// Bitfield on register `PMIC2`
pub const PFLTE2: u8 = 0x10;

/// Bitfield on register `POC`
pub const POEN2B: u8 = 0x20;

/// Bitfield on register `POC`
pub const POEN2A: u8 = 0x10;

/// Bitfield on register `POC`
pub const POEN1A: u8 = 0x4;

/// Bitfield on register `POC`
pub const POEN0B: u8 = 0x2;

/// Bitfield on register `POC`
pub const POEN1B: u8 = 0x8;

/// Bitfield on register `POC`
pub const POEN0A: u8 = 0x1;

/// Bitfield on register `PRR`
pub const PRCAN: u8 = 0x40;

/// Bitfield on register `PRR`
pub const PRADC: u8 = 0x1;

/// Bitfield on register `PRR`
pub const PRPSC: u8 = 0x20;

/// Bitfield on register `PRR`
pub const PRSPI: u8 = 0x4;

/// Bitfield on register `PRR`
pub const PRTIM0: u8 = 0x8;

/// Bitfield on register `PRR`
pub const PRLIN: u8 = 0x2;

/// Bitfield on register `PRR`
pub const PRTIM1: u8 = 0x10;

/// Bitfield on register `PSYNC`
pub const PSYNC2: u8 = 0x30;

/// Bitfield on register `PSYNC`
pub const PSYNC0: u8 = 0x3;

/// Bitfield on register `PSYNC`
pub const PSYNC1: u8 = 0xC;

/// Bitfield on register `SMCR`
pub const SE: u8 = 0x1;

/// Bitfield on register `SMCR`
pub const SM: u8 = 0xE;

/// Bitfield on register `SPCR`
pub const SPE: u8 = 0x40;

/// Bitfield on register `SPCR`
pub const MSTR: u8 = 0x10;

/// Bitfield on register `SPCR`
pub const CPOL: u8 = 0x8;

/// Bitfield on register `SPCR`
pub const SPIE: u8 = 0x80;

/// Bitfield on register `SPCR`
pub const CPHA: u8 = 0x4;

/// Bitfield on register `SPCR`
pub const SPR: u8 = 0x3;

/// Bitfield on register `SPCR`
pub const DORD: u8 = 0x20;

/// Bitfield on register `SPMCSR`
pub const PGERS: u8 = 0x2;

/// Bitfield on register `SPMCSR`
pub const RWWSB: u8 = 0x40;

/// Bitfield on register `SPMCSR`
pub const SPMEN: u8 = 0x1;

/// Bitfield on register `SPMCSR`
pub const RWWSRE: u8 = 0x10;

/// Bitfield on register `SPMCSR`
pub const BLBSET: u8 = 0x8;

/// Bitfield on register `SPMCSR`
pub const SPMIE: u8 = 0x80;

/// Bitfield on register `SPMCSR`
pub const SIGRD: u8 = 0x20;

/// Bitfield on register `SPMCSR`
pub const PGWRT: u8 = 0x4;

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
pub const T: u8 = 0x40;

/// Bitfield on register `SREG`
pub const Z: u8 = 0x2;

/// Bitfield on register `SREG`
pub const H: u8 = 0x20;

/// Bitfield on register `SREG`
pub const S: u8 = 0x10;

/// Bitfield on register `SREG`
pub const I: u8 = 0x80;

/// Bitfield on register `SREG`
pub const N: u8 = 0x4;

/// Bitfield on register `TCCR0A`
pub const WGM0: u8 = 0x3;

/// Bitfield on register `TCCR0A`
pub const COM0B: u8 = 0x30;

/// Bitfield on register `TCCR0A`
pub const COM0A: u8 = 0xC0;

/// Bitfield on register `TCCR0B`
pub const CS0: u8 = 0x7;

/// Bitfield on register `TCCR0B`
pub const FOC0B: u8 = 0x40;

/// Bitfield on register `TCCR0B`
pub const WGM02: u8 = 0x8;

/// Bitfield on register `TCCR0B`
pub const FOC0A: u8 = 0x80;

/// Bitfield on register `TCCR1A`
pub const COM1B: u8 = 0x30;

/// Bitfield on register `TCCR1A`
pub const COM1A: u8 = 0xC0;

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

/// Bitfield on register `TIFR0`
pub const TOV0: u8 = 0x1;

/// Bitfield on register `TIFR0`
pub const OCF0B: u8 = 0x4;

/// Bitfield on register `TIFR0`
pub const OCF0A: u8 = 0x2;

/// Bitfield on register `TIFR1`
pub const OCF1A: u8 = 0x2;

/// Bitfield on register `TIFR1`
pub const ICF1: u8 = 0x20;

/// Bitfield on register `TIFR1`
pub const TOV1: u8 = 0x1;

/// Bitfield on register `TIFR1`
pub const OCF1B: u8 = 0x4;

/// Bitfield on register `TIMSK0`
pub const OCIE0A: u8 = 0x2;

/// Bitfield on register `TIMSK0`
pub const TOIE0: u8 = 0x1;

/// Bitfield on register `TIMSK0`
pub const OCIE0B: u8 = 0x4;

/// Bitfield on register `TIMSK1`
pub const ICIE1: u8 = 0x20;

/// Bitfield on register `TIMSK1`
pub const OCIE1A: u8 = 0x2;

/// Bitfield on register `TIMSK1`
pub const OCIE1B: u8 = 0x4;

/// Bitfield on register `TIMSK1`
pub const TOIE1: u8 = 0x1;

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

/// `ANALOG_ADC_AUTO_TRIGGER_4BITS` value group
#[allow(non_upper_case_globals)]
pub mod analog_adc_auto_trigger_4bits {
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
   /// PSC0ASY Event.
   pub const VAL_0x08: u32 = 0x8;
   /// PSC1ASY Event.
   pub const VAL_0x09: u32 = 0x9;
   /// PSC2ASY Event.
   pub const VAL_0x0A: u32 = 0xA;
   /// Analog comparator 1.
   pub const VAL_0x0B: u32 = 0xB;
   /// Analog comparator 2.
   pub const VAL_0x0C: u32 = 0xC;
}

/// Analog Channel Selection Bits select
#[allow(non_upper_case_globals)]
pub mod analog_adc_muxpos {
   /// ADC input pin 0.
   pub const ADC0: u32 = 0x0;
   /// ADC input pin 1.
   pub const ADC1: u32 = 0x1;
   /// ADC input pin 2.
   pub const ADC2: u32 = 0x2;
   /// ADC input pin 3.
   pub const ADC3: u32 = 0x3;
   /// ADC input pin 4.
   pub const ADC4: u32 = 0x4;
   /// ADC input pin 5.
   pub const ADC5: u32 = 0x5;
   /// ADC input pin 6.
   pub const ADC6: u32 = 0x6;
   /// ADC input pin 7.
   pub const ADC7: u32 = 0x7;
   /// ADC input pin 8.
   pub const ADC8: u32 = 0x8;
   /// ADC input pin 9.
   pub const ADC9: u32 = 0x9;
   /// ADC input pin 10.
   pub const ADC10: u32 = 0xA;
   /// Temperature sensor.
   pub const TEMPSENSE: u32 = 0xB;
   /// Internal VCC / 4.
   pub const VCC4: u32 = 0xC;
   /// Current source.
   pub const ISRC: u32 = 0xD;
   /// Analog Differential Amplifier 0.
   pub const AMP0: u32 = 0xE;
   /// Analog Differential Amplifier 1.
   pub const AMP1: u32 = 0xF;
   /// Analog Differential Amplifier 2.
   pub const AMP2: u32 = 0x10;
   /// Bandgap.
   pub const BNDGAP: u32 = 0x11;
   /// 0V (GND).
   pub const GND: u32 = 0x12;
}

/// `ANALOG_ADC_V_REF2` value group
#[allow(non_upper_case_globals)]
pub mod analog_adc_v_ref2 {
   /// AREF, Internal Vref turned off.
   pub const VAL_0x00: u32 = 0x0;
   /// AVCC reference.
   pub const VAL_0x01: u32 = 0x1;
   /// Reserved.
   pub const VAL_0x02: u32 = 0x2;
   /// Internal 2.56V Voltage Reference.
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

/// `ANALOG_DAC_AUTO_TRIGGER` value group
#[allow(non_upper_case_globals)]
pub mod analog_dac_auto_trigger {
   /// Analog Comparator 0.
   pub const VAL_0x00: u32 = 0x0;
   /// Analog Comparator 1.
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

/// `CPU_SLEEP_MODE_3BITS4` value group
#[allow(non_upper_case_globals)]
pub mod cpu_sleep_mode_3bits4 {
   /// Idle.
   pub const IDLE: u32 = 0x0;
   /// ADC Noise Reduction (If Available).
   pub const ADC: u32 = 0x1;
   /// Power Down.
   pub const PDOWN: u32 = 0x2;
   /// Reserved.
   pub const VAL_0x03: u32 = 0x3;
   /// Reserved.
   pub const VAL_0x04: u32 = 0x4;
   /// Reserved.
   pub const VAL_0x05: u32 = 0x5;
   /// Standby.
   pub const STDBY: u32 = 0x6;
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

/// `ENUM_BODLEVEL` value group
#[allow(non_upper_case_globals)]
pub mod enum_bodlevel {
   /// Brown-out detection disabled.
   pub const DISABLED: u32 = 0x7;
   /// Brown-out detection at VCC=4.5 V.
   pub const _4V5: u32 = 0x6;
   /// Brown-out detection at VCC=2.7 V.
   pub const _2V7: u32 = 0x5;
   /// Brown-out detection at VCC=4.3 V.
   pub const _4V3: u32 = 0x4;
   /// Brown-out detection at VCC=4.4 V.
   pub const _4V4: u32 = 0x3;
   /// Brown-out detection at VCC=4.2 V.
   pub const _4V2: u32 = 0x2;
   /// Brown-out detection at VCC=2.8 V.
   pub const _2V8: u32 = 0x1;
   /// Brown-out detection at VCC=2.6 V.
   pub const _2V6: u32 = 0x0;
}

/// `ENUM_BOOTSZ` value group
#[allow(non_upper_case_globals)]
pub mod enum_bootsz {
   /// Boot Flash size=256 words Boot address=$1F00.
   pub const _256W_1F00: u32 = 0x3;
   /// Boot Flash size=512 words Boot address=$1E00.
   pub const _512W_1E00: u32 = 0x2;
   /// Boot Flash size=1024 words Boot address=$1C00.
   pub const _1024W_1C00: u32 = 0x1;
   /// Boot Flash size=2048 words Boot address=$1800.
   pub const _2048W_1800: u32 = 0x0;
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
   /// Ext. Clock; Start-up time PWRDWN/RESET: 6 CK/14 CK + 0 ms.
   pub const EXTCLK_6CK_14CK_0MS: u32 = 0x0;
   /// Ext. Clock; Start-up time PWRDWN/RESET: 6 CK/14 CK + 4.1 ms.
   pub const EXTCLK_6CK_14CK_4MS1: u32 = 0x10;
   /// Ext. Clock; Start-up time PWRDWN/RESET: 6 CK/14 CK + 65 ms.
   pub const EXTCLK_6CK_14CK_65MS: u32 = 0x20;
   /// Int. RC Osc. 8 MHz; Start-up time PWRDWN/RESET: 6 CK/14 CK + 0 ms.
   pub const INTRCOSC_8MHZ_6CK_14CK_0MS: u32 = 0x2;
   /// Int. RC Osc. 8 MHz; Start-up time PWRDWN/RESET: 6 CK/14 CK + 4.1 ms.
   pub const INTRCOSC_8MHZ_6CK_14CK_4MS1: u32 = 0x12;
   /// Int. RC Osc. 8 MHz; Start-up time PWRDWN/RESET: 6 CK/14 CK + 65 ms.
   pub const INTRCOSC_8MHZ_6CK_14CK_65MS: u32 = 0x22;
   /// Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 258 CK/14 CK + 4.1 ms.
   pub const EXTXOSC_0MHZ4_0MHZ9_258CK_14CK_4MS1: u32 = 0x8;
   /// Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 258 CK/14 CK + 65 ms.
   pub const EXTXOSC_0MHZ4_0MHZ9_258CK_14CK_65MS: u32 = 0x18;
   /// Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 0 ms.
   pub const EXTXOSC_0MHZ4_0MHZ9_1KCK_14CK_0MS: u32 = 0x28;
   /// Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 4.1 ms.
   pub const EXTXOSC_0MHZ4_0MHZ9_1KCK_14CK_4MS1: u32 = 0x38;
   /// Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 65 ms.
   pub const EXTXOSC_0MHZ4_0MHZ9_1KCK_14CK_65MS: u32 = 0x9;
   /// Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 0 ms.
   pub const EXTXOSC_0MHZ4_0MHZ9_16KCK_14CK_0MS: u32 = 0x19;
   /// Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 4.1 ms.
   pub const EXTXOSC_0MHZ4_0MHZ9_16KCK_14CK_4MS1: u32 = 0x29;
   /// Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 65 ms.
   pub const EXTXOSC_0MHZ4_0MHZ9_16KCK_14CK_65MS: u32 = 0x39;
   /// Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 258 CK/14 CK + 4.1 ms.
   pub const EXTXOSC_0MHZ9_3MHZ_258CK_14CK_4MS1: u32 = 0xA;
   /// Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 258 CK/14 CK + 65 ms.
   pub const EXTXOSC_0MHZ9_3MHZ_258CK_14CK_65MS: u32 = 0x1A;
   /// Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 0 ms.
   pub const EXTXOSC_0MHZ9_3MHZ_1KCK_14CK_0MS: u32 = 0x2A;
   /// Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 4.1 ms.
   pub const EXTXOSC_0MHZ9_3MHZ_1KCK_14CK_4MS1: u32 = 0x3A;
   /// Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 65 ms.
   pub const EXTXOSC_0MHZ9_3MHZ_1KCK_14CK_65MS: u32 = 0xB;
   /// Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 0 ms.
   pub const EXTXOSC_0MHZ9_3MHZ_16KCK_14CK_0MS: u32 = 0x1B;
   /// Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 4.1 ms.
   pub const EXTXOSC_0MHZ9_3MHZ_16KCK_14CK_4MS1: u32 = 0x2B;
   /// Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 65 ms.
   pub const EXTXOSC_0MHZ9_3MHZ_16KCK_14CK_65MS: u32 = 0x3B;
   /// Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 258 CK/14 CK + 4.1 ms.
   pub const EXTXOSC_3MHZ_8MHZ_258CK_14CK_4MS1: u32 = 0xC;
   /// Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 258 CK/14 CK + 65 ms.
   pub const EXTXOSC_3MHZ_8MHZ_258CK_14CK_65MS: u32 = 0x1C;
   /// Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 0 ms.
   pub const EXTXOSC_3MHZ_8MHZ_1KCK_14CK_0MS: u32 = 0x2C;
   /// Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 4.1 ms.
   pub const EXTXOSC_3MHZ_8MHZ_1KCK_14CK_4MS1: u32 = 0x3C;
   /// Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 65 ms.
   pub const EXTXOSC_3MHZ_8MHZ_1KCK_14CK_65MS: u32 = 0xD;
   /// Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 0 ms.
   pub const EXTXOSC_3MHZ_8MHZ_16KCK_14CK_0MS: u32 = 0x1D;
   /// Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 4.1 ms.
   pub const EXTXOSC_3MHZ_8MHZ_16KCK_14CK_4MS1: u32 = 0x2D;
   /// Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 65 ms.
   pub const EXTXOSC_3MHZ_8MHZ_16KCK_14CK_65MS: u32 = 0x3D;
   /// Ext. Crystal Osc. 8.0-    MHz; Start-up time PWRDWN/RESET: 258 CK/14 CK + 4.1 ms.
   pub const EXTXOSC_8MHZ_XX_258CK_14CK_4MS1: u32 = 0xE;
   /// Ext. Crystal Osc. 8.0-    MHz; Start-up time PWRDWN/RESET: 258 CK/14 CK + 65 ms.
   pub const EXTXOSC_8MHZ_XX_258CK_14CK_65MS: u32 = 0x1E;
   /// Ext. Crystal Osc. 8.0-    MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 0 ms.
   pub const EXTXOSC_8MHZ_XX_1KCK_14CK_0MS: u32 = 0x2E;
   /// Ext. Crystal Osc. 8.0-    MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 4.1 ms.
   pub const EXTXOSC_8MHZ_XX_1KCK_14CK_4MS1: u32 = 0x3E;
   /// Ext. Crystal Osc. 8.0-    MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 65 ms.
   pub const EXTXOSC_8MHZ_XX_1KCK_14CK_65MS: u32 = 0xF;
   /// Ext. Crystal Osc. 8.0-    MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 0 ms.
   pub const EXTXOSC_8MHZ_XX_16KCK_14CK_0MS: u32 = 0x1F;
   /// Ext. Crystal Osc. 8.0-    MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 4.1 ms.
   pub const EXTXOSC_8MHZ_XX_16KCK_14CK_4MS1: u32 = 0x2F;
   /// Ext. Crystal Osc. 8.0-    MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 65 ms.
   pub const EXTXOSC_8MHZ_XX_16KCK_14CK_65MS: u32 = 0x3F;
   /// PLL clock 16 MHz; Start-up time PWRDWN/RESET: 1K CK/14 CK + 0 ms.
   pub const PLLCLK_16MHZ_1KCK_14CK_0MS: u32 = 0x3;
   /// PLL clock 16 MHz; Start-up time PWRDWN/RESET: 1K CK/14 CK + 4.1 ms.
   pub const PLLCLK_16MHZ_1KCK_14CK_4MS1: u32 = 0x13;
   /// PLL clock 16 MHz; Start-up time PWRDWN/RESET: 1K CK/14 CK + 65 ms.
   pub const PLLCLK_16MHZ_1KCK_14CK_65MS: u32 = 0x23;
   /// PLL clock 16 MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 0 ms.
   pub const PLLCLK_16MHZ_16KCK_14CK_0MS: u32 = 0x33;
   /// PLL clock /4; PLL input: Ext. Clock; Start-up time PWRDWN/RESET: 6K CK/14 CK + 0 ms.
   pub const PLLCLK_PLLIN_EXTCLK_6KCK_14CK_0MS: u32 = 0x1;
   /// PLL clock /4; PLL input: Ext. Clock; Start-up time PWRDWN/RESET: 6K CK/14 CK + 4 ms.
   pub const PLLCLK_PLLIN_EXTCLK_6KCK_14CK_4MS: u32 = 0x11;
   /// PLL clock /4; PLL input: Ext. Clock; Start-up time PWRDWN/RESET: 6K CK/14 CK + 64 ms.
   pub const PLLCLK_PLLIN_EXTCLK_6KCK_14CK_64MS: u32 = 0x21;
   /// PLL clock /4; PLL input: Ext. Crystal Osc.; Start-up time PWRDWN/RESET: 1K CK/14 CK + 0 ms.
   pub const PLLCLK_PLLIN_EXTXOSC_1KCK_14CK_0MS: u32 = 0x5;
   /// PLL clock /4; PLL input: Ext. Crystal Osc.; Start-up time PWRDWN/RESET: 1K CK/14 CK + 4 ms.
   pub const PLLCLK_PLLIN_EXTXOSC_1KCK_14CK_4MS: u32 = 0x15;
   /// PLL clock /4; PLL input: Ext. Crystal Osc.; Start-up time PWRDWN/RESET: 16K CK/14 CK + 4 ms.
   pub const PLLCLK_PLLIN_EXTXOSC_16KCK_14CK_4MS: u32 = 0x25;
   /// PLL clock /4; PLL input: Ext. Crystal Osc.; Start-up time PWRDWN/RESET: 16K CK/14 CK + 64 ms.
   pub const PLLCLK_PLLIN_EXTXOSC_16KCK_14CK_64MS: u32 = 0x35;
   /// Ext. Crystal Osc.; PLL input: Ext. Crystal Osc.; Start-up time PWRDWN/RESET: 1K CK/14 CK + 0 ms.
   pub const EXTXOSC_PLLIN_EXTXOSC_1KCK_14CK_0MS: u32 = 0x4;
   /// Ext. Crystal Osc.; PLL input: Ext. Crystal Osc.; Start-up time PWRDWN/RESET: 1K CK/14 CK + 4 ms.
   pub const EXTXOSC_PLLIN_EXTXOSC_1KCK_14CK_4MS: u32 = 0x14;
   /// Ext. Crystal Osc.; PLL input: Ext. Crystal Osc.; Start-up time PWRDWN/RESET: 16K CK/14 CK + 4 ms.
   pub const EXTXOSC_PLLIN_EXTXOSC_16KCK_14CK_4MS: u32 = 0x24;
   /// Ext. Crystal Osc.; PLL input: Ext. Crystal Osc.; Start-up time PWRDWN/RESET: 16K CK/14 CK + 64 ms.
   pub const EXTXOSC_PLLIN_EXTXOSC_16KCK_14CK_64MS: u32 = 0x34;
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

