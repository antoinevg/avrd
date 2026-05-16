//! The AVR ATtiny87 microcontroller
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
/// | LB | 11 |
pub const LOCKBIT: *mut u8 = 0x0 as *mut u8;

/// `HIGH` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BODLEVEL | 111 |
/// | SPIEN | 100000 |
/// | EESAVE | 1000 |
/// | RSTDISBL | 10000000 |
/// | DWEN | 1000000 |
/// | WDTON | 10000 |
pub const HIGH: *mut u8 = 0x1 as *mut u8;

/// `EXTENDED` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SELFPRGEN | 1 |
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

/// Port Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PUDB | 10 |
/// | BBMB | 100000 |
/// | PUDA | 1 |
/// | BBMA | 10000 |
pub const PORTCR: *mut u8 = 0x32 as *mut u8;

/// Timer/Counter0 Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCF0A | 10 |
/// | TOV0 | 1 |
pub const TIFR0: *mut u8 = 0x35 as *mut u8;

/// Timer/Counter1 Interrupt Flag register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCF1A | 10 |
/// | OCF1B | 100 |
/// | ICF1 | 100000 |
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
/// | INTF | 11 |
pub const EIFR: *mut u8 = 0x3C as *mut u8;

/// External Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | INT | 11 |
pub const EIMSK: *mut u8 = 0x3D as *mut u8;

/// General purpose register 0.
pub const GPIOR0: *mut u8 = 0x3E as *mut u8;

/// EEPROM Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EEMPE | 100 |
/// | EEPM | 110000 |
/// | EERE | 1 |
/// | EEPE | 10 |
/// | EERIE | 1000 |
pub const EECR: *mut u8 = 0x3F as *mut u8;

/// EEPROM Data Register.
pub const EEDR: *mut u8 = 0x40 as *mut u8;

/// EEPROM Address Register  Bytes.
pub const EEAR: *mut u16 = 0x41 as *mut u16;

/// EEPROM Address Register  Bytes low byte.
pub const EEARL: *mut u8 = 0x41 as *mut u8;

/// EEPROM Address Register  Bytes high byte.
pub const EEARH: *mut u8 = 0x42 as *mut u8;

/// General Timer Counter Control register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PSR1 | 1 |
/// | TSM | 10000000 |
/// | PSR0 | 10 |
pub const GTCCR: *mut u8 = 0x43 as *mut u8;

/// Timer/Counter0 Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | COM0A | 11000000 |
/// | WGM0 | 11 |
pub const TCCR0A: *mut u8 = 0x45 as *mut u8;

/// Timer/Counter0 Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CS0 | 111 |
/// | FOC0A | 10000000 |
pub const TCCR0B: *mut u8 = 0x46 as *mut u8;

/// Timer/Counter0.
pub const TCNT0: *mut u8 = 0x47 as *mut u8;

/// Timer/Counter0 Output Compare Register A.
pub const OCR0A: *mut u8 = 0x48 as *mut u8;

/// General Purpose register 1.
pub const GPIOR1: *mut u8 = 0x4A as *mut u8;

/// General Purpose IO register 2.
pub const GPIOR2: *mut u8 = 0x4B as *mut u8;

/// SPI Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SPE | 1000000 |
/// | DORD | 100000 |
/// | CPHA | 100 |
/// | SPR | 11 |
/// | SPIE | 10000000 |
/// | MSTR | 10000 |
/// | CPOL | 1000 |
pub const SPCR: *mut u8 = 0x4C as *mut u8;

/// SPI Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SPI2X | 1 |
/// | SPIF | 10000000 |
/// | WCOL | 1000000 |
pub const SPSR: *mut u8 = 0x4D as *mut u8;

/// SPI Data Register.
pub const SPDR: *mut u8 = 0x4E as *mut u8;

/// Analog Comparator Control And Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ACIS | 11 |
/// | ACI | 10000 |
/// | ACIC | 100 |
/// | ACO | 100000 |
/// | ACD | 10000000 |
/// | ACIRS | 1000000 |
/// | ACIE | 1000 |
pub const ACSR: *mut u8 = 0x50 as *mut u8;

/// DebugWire data register.
pub const DWDR: *mut u8 = 0x51 as *mut u8;

/// Sleep Mode Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SE | 1 |
/// | SM | 110 |
pub const SMCR: *mut u8 = 0x53 as *mut u8;

/// MCU Status register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDRF | 1000 |
/// | PORF | 1 |
/// | EXTRF | 10 |
/// | BORF | 100 |
pub const MCUSR: *mut u8 = 0x54 as *mut u8;

/// MCU Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BODSE | 100000 |
/// | BODS | 1000000 |
/// | PUD | 10000 |
pub const MCUCR: *mut u8 = 0x55 as *mut u8;

/// Store Program Memory Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PGWRT | 100 |
/// | SIGRD | 100000 |
/// | CTPB | 10000 |
/// | PGERS | 10 |
/// | SPMEN | 1 |
/// | RWWSB | 1000000 |
/// | RFLB | 1000 |
pub const SPMCSR: *mut u8 = 0x57 as *mut u8;

/// Stack Pointer  Bytes low byte.
pub const SPL: *mut u8 = 0x5D as *mut u8;

/// Stack Pointer  Bytes.
pub const SP: *mut u16 = 0x5D as *mut u16;

/// Stack Pointer  Bytes high byte.
pub const SPH: *mut u8 = 0x5E as *mut u8;

/// Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | S | 10000 |
/// | H | 100000 |
/// | Z | 10 |
/// | C | 1 |
/// | V | 1000 |
/// | N | 100 |
/// | I | 10000000 |
/// | T | 1000000 |
pub const SREG: *mut u8 = 0x5F as *mut u8;

/// Watchdog Timer Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDIE | 1000000 |
/// | WDE | 1000 |
/// | WDP | 100111 |
/// | WDCE | 10000 |
/// | WDIF | 10000000 |
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

/// Clock Control & Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CLKC | 1111 |
/// | CLKCCE | 10000000 |
/// | CLKRDY | 10000 |
pub const CLKCSR: *mut u8 = 0x62 as *mut u8;

/// Clock Selection Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | COUT | 1000000 |
/// | CSUT | 110000 |
/// | CSEL | 1111 |
pub const CLKSELR: *mut u8 = 0x63 as *mut u8;

/// Power Reduction Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRUSI | 10 |
/// | PRLIN | 100000 |
/// | PRADC | 1 |
/// | PRSPI | 10000 |
/// | PRTIM1 | 1000 |
/// | PRTIM0 | 100 |
pub const PRR: *mut u8 = 0x64 as *mut u8;

/// Oscillator Calibration Register.
pub const OSCCAL: *mut u8 = 0x66 as *mut u8;

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
/// | ISC0 | 11 |
/// | ISC1 | 1100 |
pub const EICRA: *mut u8 = 0x69 as *mut u8;

/// Pin Change Mask Register 0.
pub const PCMSK0: *mut u8 = 0x6B as *mut u8;

/// Pin Change Mask Register 1.
pub const PCMSK1: *mut u8 = 0x6C as *mut u8;

/// Timer/Counter0 Interrupt Mask register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCIE0A | 10 |
/// | TOIE0 | 1 |
pub const TIMSK0: *mut u8 = 0x6E as *mut u8;

/// Timer/Counter1 Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TOIE1 | 1 |
/// | OCIE1A | 10 |
/// | OCIE1B | 100 |
/// | ICIE1 | 100000 |
pub const TIMSK1: *mut u8 = 0x6F as *mut u8;

/// Analog Miscellaneous Control Register (Shared with AD_CONVERTER IO_MODULE).
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ISRCEN | 1 |
pub const AMISCR: *mut u8 = 0x77 as *mut u8;

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
/// | ADATE | 100000 |
/// | ADPS | 111 |
/// | ADSC | 1000000 |
/// | ADIE | 1000 |
/// | ADIF | 10000 |
/// | ADEN | 10000000 |
pub const ADCSRA: *mut u8 = 0x7A as *mut u8;

/// Analog Comparator & ADC Control and Status Register B (Shared with AD_CONVERTER IO_MODULE).
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ACME | 1000000 |
/// | ACIR | 110000 |
pub const ADCSRB: *mut u8 = 0x7B as *mut u8;

/// The ADC multiplexer Selection Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | REFS | 11000000 |
/// | MUX | 11111 |
/// | ADLAR | 100000 |
pub const ADMUX: *mut u8 = 0x7C as *mut u8;

/// Digital Input Disable Register 0.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADC2D | 100 |
/// | ADC6D | 1000000 |
/// | ADC3D | 1000 |
/// | ADC5D | 100000 |
/// | ADC1D | 10 |
/// | ADC4D | 10000 |
/// | ADC7D | 10000000 |
/// | ADC0D | 1 |
pub const DIDR0: *mut u8 = 0x7E as *mut u8;

/// Digital Input Disable Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADC8D | 10000 |
/// | ADC10D | 1000000 |
/// | ADC9D | 100000 |
pub const DIDR1: *mut u8 = 0x7F as *mut u8;

/// Timer/Counter1 Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | COM1A | 11000000 |
/// | COM1B | 110000 |
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
/// | FOC1B | 1000000 |
/// | FOC1A | 10000000 |
pub const TCCR1C: *mut u8 = 0x82 as *mut u8;

/// Timer/Counter1 Control Register D.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OC1BU | 10000 |
/// | OC1BW | 1000000 |
/// | OC1AX | 1000 |
/// | OC1BX | 10000000 |
/// | OC1BV | 100000 |
/// | OC1AW | 100 |
/// | OC1AU | 1 |
/// | OC1AV | 10 |
pub const TCCR1D: *mut u8 = 0x83 as *mut u8;

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

/// Timer/Counter1 Output Compare Register A  Bytes.
pub const OCR1A: *mut u16 = 0x88 as *mut u16;

/// Timer/Counter1 Output Compare Register A  Bytes low byte.
pub const OCR1AL: *mut u8 = 0x88 as *mut u8;

/// Timer/Counter1 Output Compare Register A  Bytes high byte.
pub const OCR1AH: *mut u8 = 0x89 as *mut u8;

/// Timer/Counter1 Output Compare Register B  Bytes low byte.
pub const OCR1BL: *mut u8 = 0x8A as *mut u8;

/// Timer/Counter1 Output Compare Register B  Bytes.
pub const OCR1B: *mut u16 = 0x8A as *mut u16;

/// Timer/Counter1 Output Compare Register B  Bytes high byte.
pub const OCR1BH: *mut u8 = 0x8B as *mut u8;

/// Asynchronous Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TCR0BUB | 1 |
/// | TCN0UB | 10000 |
/// | TCR0AUB | 10 |
/// | AS0 | 100000 |
/// | EXCLK | 1000000 |
/// | OCR0AUB | 1000 |
pub const ASSR: *mut u8 = 0xB6 as *mut u8;

/// USI Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | USISIE | 10000000 |
/// | USIOIE | 1000000 |
/// | USICLK | 10 |
/// | USICS | 1100 |
/// | USIWM | 110000 |
/// | USITC | 1 |
pub const USICR: *mut u8 = 0xB8 as *mut u8;

/// USI Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | USISIF | 10000000 |
/// | USICNT | 1111 |
/// | USIPF | 100000 |
/// | USIOIF | 1000000 |
/// | USIDC | 10000 |
pub const USISR: *mut u8 = 0xB9 as *mut u8;

/// USI Data Register.
pub const USIDR: *mut u8 = 0xBA as *mut u8;

/// USI Buffer Register.
pub const USIBR: *mut u8 = 0xBB as *mut u8;

/// USI Pin Position.
pub const USIPP: *mut u8 = 0xBC as *mut u8;

/// LIN Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LIN13 | 1000000 |
/// | LCONF | 110000 |
/// | LSWRES | 10000000 |
/// | LENA | 1000 |
/// | LCMD | 111 |
pub const LINCR: *mut u8 = 0xC8 as *mut u8;

/// LIN Status and Interrupt Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LBUSY | 10000 |
/// | LIDST | 11100000 |
/// | LTXOK | 10 |
/// | LRXOK | 1 |
/// | LIDOK | 100 |
/// | LERR | 1000 |
pub const LINSIR: *mut u8 = 0xC9 as *mut u8;

/// LIN Enable Interrupt Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LENERR | 1000 |
/// | LENIDOK | 100 |
/// | LENTXOK | 10 |
/// | LENRXOK | 1 |
pub const LINENIR: *mut u8 = 0xCA as *mut u8;

/// LIN Error Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LTOERR | 1000000 |
/// | LOVERR | 100000 |
/// | LFERR | 10000 |
/// | LABORT | 10000000 |
/// | LSERR | 1000 |
/// | LBERR | 1 |
/// | LPERR | 100 |
/// | LCERR | 10 |
pub const LINERR: *mut u8 = 0xCB as *mut u8;

/// LIN Bit Timing Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LBT | 111111 |
/// | LDISR | 10000000 |
pub const LINBTR: *mut u8 = 0xCC as *mut u8;

/// LIN Baud Rate Low Register.
pub const LINBRRL: *mut u8 = 0xCD as *mut u8;

/// LIN Baud Rate High Register.
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

/// Bitfield on register `ACSR`
pub const ACIS: u8 = 0x3;

/// Bitfield on register `ACSR`
pub const ACI: u8 = 0x10;

/// Bitfield on register `ACSR`
pub const ACIC: u8 = 0x4;

/// Bitfield on register `ACSR`
pub const ACO: u8 = 0x20;

/// Bitfield on register `ACSR`
pub const ACD: u8 = 0x80;

/// Bitfield on register `ACSR`
pub const ACIRS: u8 = 0x40;

/// Bitfield on register `ACSR`
pub const ACIE: u8 = 0x8;

/// Bitfield on register `ADCSRA`
pub const ADATE: u8 = 0x20;

/// Bitfield on register `ADCSRA`
pub const ADPS: u8 = 0x7;

/// Bitfield on register `ADCSRA`
pub const ADSC: u8 = 0x40;

/// Bitfield on register `ADCSRA`
pub const ADIE: u8 = 0x8;

/// Bitfield on register `ADCSRA`
pub const ADIF: u8 = 0x10;

/// Bitfield on register `ADCSRA`
pub const ADEN: u8 = 0x80;

/// Bitfield on register `ADCSRB`
pub const ACME: u8 = 0x40;

/// Bitfield on register `ADCSRB`
pub const ACIR: u8 = 0x30;

/// Bitfield on register `ADMUX`
pub const REFS: u8 = 0xC0;

/// Bitfield on register `ADMUX`
pub const MUX: u8 = 0x1F;

/// Bitfield on register `ADMUX`
pub const ADLAR: u8 = 0x20;

/// Bitfield on register `AMISCR`
pub const ISRCEN: u8 = 0x1;

/// Bitfield on register `ASSR`
pub const TCR0BUB: u8 = 0x1;

/// Bitfield on register `ASSR`
pub const TCN0UB: u8 = 0x10;

/// Bitfield on register `ASSR`
pub const TCR0AUB: u8 = 0x2;

/// Bitfield on register `ASSR`
pub const AS0: u8 = 0x20;

/// Bitfield on register `ASSR`
pub const EXCLK: u8 = 0x40;

/// Bitfield on register `ASSR`
pub const OCR0AUB: u8 = 0x8;

/// Bitfield on register `CLKCSR`
pub const CLKC: u8 = 0xF;

/// Bitfield on register `CLKCSR`
pub const CLKCCE: u8 = 0x80;

/// Bitfield on register `CLKCSR`
pub const CLKRDY: u8 = 0x10;

/// Bitfield on register `CLKPR`
pub const CLKPS: u8 = 0xF;

/// Bitfield on register `CLKPR`
pub const CLKPCE: u8 = 0x80;

/// Bitfield on register `CLKSELR`
pub const COUT: u8 = 0x40;

/// Bitfield on register `CLKSELR`
pub const CSUT: u8 = 0x30;

/// Bitfield on register `CLKSELR`
pub const CSEL: u8 = 0xF;

/// Bitfield on register `DIDR0`
pub const ADC2D: u8 = 0x4;

/// Bitfield on register `DIDR0`
pub const ADC6D: u8 = 0x40;

/// Bitfield on register `DIDR0`
pub const ADC3D: u8 = 0x8;

/// Bitfield on register `DIDR0`
pub const ADC5D: u8 = 0x20;

/// Bitfield on register `DIDR0`
pub const ADC1D: u8 = 0x2;

/// Bitfield on register `DIDR0`
pub const ADC4D: u8 = 0x10;

/// Bitfield on register `DIDR0`
pub const ADC7D: u8 = 0x80;

/// Bitfield on register `DIDR0`
pub const ADC0D: u8 = 0x1;

/// Bitfield on register `DIDR1`
pub const ADC8D: u8 = 0x10;

/// Bitfield on register `DIDR1`
pub const ADC10D: u8 = 0x40;

/// Bitfield on register `DIDR1`
pub const ADC9D: u8 = 0x20;

/// Bitfield on register `EECR`
pub const EEMPE: u8 = 0x4;

/// Bitfield on register `EECR`
pub const EEPM: u8 = 0x30;

/// Bitfield on register `EECR`
pub const EERE: u8 = 0x1;

/// Bitfield on register `EECR`
pub const EEPE: u8 = 0x2;

/// Bitfield on register `EECR`
pub const EERIE: u8 = 0x8;

/// Bitfield on register `EICRA`
pub const ISC0: u8 = 0x3;

/// Bitfield on register `EICRA`
pub const ISC1: u8 = 0xC;

/// Bitfield on register `EIFR`
pub const INTF: u8 = 0x3;

/// Bitfield on register `EIMSK`
pub const INT: u8 = 0x3;

/// Bitfield on register `EXTENDED`
pub const SELFPRGEN: u8 = 0x1;

/// Bitfield on register `GTCCR`
pub const PSR1: u8 = 0x1;

/// Bitfield on register `GTCCR`
pub const TSM: u8 = 0x80;

/// Bitfield on register `GTCCR`
pub const PSR0: u8 = 0x2;

/// Bitfield on register `HIGH`
pub const BODLEVEL: u8 = 0x7;

/// Bitfield on register `HIGH`
pub const SPIEN: u8 = 0x20;

/// Bitfield on register `HIGH`
pub const EESAVE: u8 = 0x8;

/// Bitfield on register `HIGH`
pub const RSTDISBL: u8 = 0x80;

/// Bitfield on register `HIGH`
pub const DWEN: u8 = 0x40;

/// Bitfield on register `HIGH`
pub const WDTON: u8 = 0x10;

/// Bitfield on register `LINBTR`
pub const LBT: u8 = 0x3F;

/// Bitfield on register `LINBTR`
pub const LDISR: u8 = 0x80;

/// Bitfield on register `LINCR`
pub const LIN13: u8 = 0x40;

/// Bitfield on register `LINCR`
pub const LCONF: u8 = 0x30;

/// Bitfield on register `LINCR`
pub const LSWRES: u8 = 0x80;

/// Bitfield on register `LINCR`
pub const LENA: u8 = 0x8;

/// Bitfield on register `LINCR`
pub const LCMD: u8 = 0x7;

/// Bitfield on register `LINDLR`
pub const LRXDL: u8 = 0xF;

/// Bitfield on register `LINDLR`
pub const LTXDL: u8 = 0xF0;

/// Bitfield on register `LINENIR`
pub const LENERR: u8 = 0x8;

/// Bitfield on register `LINENIR`
pub const LENIDOK: u8 = 0x4;

/// Bitfield on register `LINENIR`
pub const LENTXOK: u8 = 0x2;

/// Bitfield on register `LINENIR`
pub const LENRXOK: u8 = 0x1;

/// Bitfield on register `LINERR`
pub const LTOERR: u8 = 0x40;

/// Bitfield on register `LINERR`
pub const LOVERR: u8 = 0x20;

/// Bitfield on register `LINERR`
pub const LFERR: u8 = 0x10;

/// Bitfield on register `LINERR`
pub const LABORT: u8 = 0x80;

/// Bitfield on register `LINERR`
pub const LSERR: u8 = 0x8;

/// Bitfield on register `LINERR`
pub const LBERR: u8 = 0x1;

/// Bitfield on register `LINERR`
pub const LPERR: u8 = 0x4;

/// Bitfield on register `LINERR`
pub const LCERR: u8 = 0x2;

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
pub const LTXOK: u8 = 0x2;

/// Bitfield on register `LINSIR`
pub const LRXOK: u8 = 0x1;

/// Bitfield on register `LINSIR`
pub const LIDOK: u8 = 0x4;

/// Bitfield on register `LINSIR`
pub const LERR: u8 = 0x8;

/// Bitfield on register `LOCKBIT`
pub const LB: u8 = 0x3;

/// Bitfield on register `LOW`
pub const CKOUT: u8 = 0x40;

/// Bitfield on register `LOW`
pub const SUT_CKSEL: u8 = 0x3F;

/// Bitfield on register `LOW`
pub const CKDIV8: u8 = 0x80;

/// Bitfield on register `MCUCR`
pub const BODSE: u8 = 0x20;

/// Bitfield on register `MCUCR`
pub const BODS: u8 = 0x40;

/// Bitfield on register `MCUCR`
pub const PUD: u8 = 0x10;

/// Bitfield on register `MCUSR`
pub const WDRF: u8 = 0x8;

/// Bitfield on register `MCUSR`
pub const PORF: u8 = 0x1;

/// Bitfield on register `MCUSR`
pub const EXTRF: u8 = 0x2;

/// Bitfield on register `MCUSR`
pub const BORF: u8 = 0x4;

/// Bitfield on register `PCICR`
pub const PCIE: u8 = 0x3;

/// Bitfield on register `PCIFR`
pub const PCIF: u8 = 0x3;

/// Bitfield on register `PORTCR`
pub const PUDB: u8 = 0x2;

/// Bitfield on register `PORTCR`
pub const BBMB: u8 = 0x20;

/// Bitfield on register `PORTCR`
pub const PUDA: u8 = 0x1;

/// Bitfield on register `PORTCR`
pub const BBMA: u8 = 0x10;

/// Bitfield on register `PRR`
pub const PRUSI: u8 = 0x2;

/// Bitfield on register `PRR`
pub const PRLIN: u8 = 0x20;

/// Bitfield on register `PRR`
pub const PRADC: u8 = 0x1;

/// Bitfield on register `PRR`
pub const PRSPI: u8 = 0x10;

/// Bitfield on register `PRR`
pub const PRTIM1: u8 = 0x8;

/// Bitfield on register `PRR`
pub const PRTIM0: u8 = 0x4;

/// Bitfield on register `SMCR`
pub const SE: u8 = 0x1;

/// Bitfield on register `SMCR`
pub const SM: u8 = 0x6;

/// Bitfield on register `SPCR`
pub const SPE: u8 = 0x40;

/// Bitfield on register `SPCR`
pub const DORD: u8 = 0x20;

/// Bitfield on register `SPCR`
pub const CPHA: u8 = 0x4;

/// Bitfield on register `SPCR`
pub const SPR: u8 = 0x3;

/// Bitfield on register `SPCR`
pub const SPIE: u8 = 0x80;

/// Bitfield on register `SPCR`
pub const MSTR: u8 = 0x10;

/// Bitfield on register `SPCR`
pub const CPOL: u8 = 0x8;

/// Bitfield on register `SPMCSR`
pub const PGWRT: u8 = 0x4;

/// Bitfield on register `SPMCSR`
pub const SIGRD: u8 = 0x20;

/// Bitfield on register `SPMCSR`
pub const CTPB: u8 = 0x10;

/// Bitfield on register `SPMCSR`
pub const PGERS: u8 = 0x2;

/// Bitfield on register `SPMCSR`
pub const SPMEN: u8 = 0x1;

/// Bitfield on register `SPMCSR`
pub const RWWSB: u8 = 0x40;

/// Bitfield on register `SPMCSR`
pub const RFLB: u8 = 0x8;

/// Bitfield on register `SPSR`
pub const SPI2X: u8 = 0x1;

/// Bitfield on register `SPSR`
pub const SPIF: u8 = 0x80;

/// Bitfield on register `SPSR`
pub const WCOL: u8 = 0x40;

/// Bitfield on register `SREG`
pub const S: u8 = 0x10;

/// Bitfield on register `SREG`
pub const H: u8 = 0x20;

/// Bitfield on register `SREG`
pub const Z: u8 = 0x2;

/// Bitfield on register `SREG`
pub const C: u8 = 0x1;

/// Bitfield on register `SREG`
pub const V: u8 = 0x8;

/// Bitfield on register `SREG`
pub const N: u8 = 0x4;

/// Bitfield on register `SREG`
pub const I: u8 = 0x80;

/// Bitfield on register `SREG`
pub const T: u8 = 0x40;

/// Bitfield on register `TCCR0A`
pub const COM0A: u8 = 0xC0;

/// Bitfield on register `TCCR0A`
pub const WGM0: u8 = 0x3;

/// Bitfield on register `TCCR0B`
pub const CS0: u8 = 0x7;

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
pub const FOC1B: u8 = 0x40;

/// Bitfield on register `TCCR1C`
pub const FOC1A: u8 = 0x80;

/// Bitfield on register `TCCR1D`
pub const OC1BU: u8 = 0x10;

/// Bitfield on register `TCCR1D`
pub const OC1BW: u8 = 0x40;

/// Bitfield on register `TCCR1D`
pub const OC1AX: u8 = 0x8;

/// Bitfield on register `TCCR1D`
pub const OC1BX: u8 = 0x80;

/// Bitfield on register `TCCR1D`
pub const OC1BV: u8 = 0x20;

/// Bitfield on register `TCCR1D`
pub const OC1AW: u8 = 0x4;

/// Bitfield on register `TCCR1D`
pub const OC1AU: u8 = 0x1;

/// Bitfield on register `TCCR1D`
pub const OC1AV: u8 = 0x2;

/// Bitfield on register `TIFR0`
pub const OCF0A: u8 = 0x2;

/// Bitfield on register `TIFR0`
pub const TOV0: u8 = 0x1;

/// Bitfield on register `TIFR1`
pub const OCF1A: u8 = 0x2;

/// Bitfield on register `TIFR1`
pub const OCF1B: u8 = 0x4;

/// Bitfield on register `TIFR1`
pub const ICF1: u8 = 0x20;

/// Bitfield on register `TIFR1`
pub const TOV1: u8 = 0x1;

/// Bitfield on register `TIMSK0`
pub const OCIE0A: u8 = 0x2;

/// Bitfield on register `TIMSK0`
pub const TOIE0: u8 = 0x1;

/// Bitfield on register `TIMSK1`
pub const TOIE1: u8 = 0x1;

/// Bitfield on register `TIMSK1`
pub const OCIE1A: u8 = 0x2;

/// Bitfield on register `TIMSK1`
pub const OCIE1B: u8 = 0x4;

/// Bitfield on register `TIMSK1`
pub const ICIE1: u8 = 0x20;

/// Bitfield on register `USICR`
pub const USISIE: u8 = 0x80;

/// Bitfield on register `USICR`
pub const USIOIE: u8 = 0x40;

/// Bitfield on register `USICR`
pub const USICLK: u8 = 0x2;

/// Bitfield on register `USICR`
pub const USICS: u8 = 0xC;

/// Bitfield on register `USICR`
pub const USIWM: u8 = 0x30;

/// Bitfield on register `USICR`
pub const USITC: u8 = 0x1;

/// Bitfield on register `USISR`
pub const USISIF: u8 = 0x80;

/// Bitfield on register `USISR`
pub const USICNT: u8 = 0xF;

/// Bitfield on register `USISR`
pub const USIPF: u8 = 0x20;

/// Bitfield on register `USISR`
pub const USIOIF: u8 = 0x40;

/// Bitfield on register `USISR`
pub const USIDC: u8 = 0x10;

/// Bitfield on register `WDTCR`
pub const WDIE: u8 = 0x40;

/// Bitfield on register `WDTCR`
pub const WDE: u8 = 0x8;

/// Bitfield on register `WDTCR`
pub const WDP: u8 = 0x27;

/// Bitfield on register `WDTCR`
pub const WDCE: u8 = 0x10;

/// Bitfield on register `WDTCR`
pub const WDIF: u8 = 0x80;

/// `ANALOG_ADC_AUTO_TRIGGER4` value group
#[allow(non_upper_case_globals)]
pub mod analog_adc_auto_trigger4 {
   /// Free Running mode.
   pub const VAL_0x00: u32 = 0x0;
   /// Analog Comparator.
   pub const VAL_0x01: u32 = 0x1;
   /// External Interrupt Request 0.
   pub const VAL_0x02: u32 = 0x2;
   /// Timer/Counter1 Compare Match A.
   pub const VAL_0x03: u32 = 0x3;
   /// Timer/Counter1 Overflow.
   pub const VAL_0x04: u32 = 0x4;
   /// Timer/Counter1 Compare Match B.
   pub const VAL_0x05: u32 = 0x5;
   /// Timer/Counter1 Capture Event.
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

/// `ANALOG_ADC_V_REF8` value group
#[allow(non_upper_case_globals)]
pub mod analog_adc_v_ref8 {
   /// If AREFEN==0 then Internal AVCC as Voltage Reference. If AREFEN==1 then AREF pin as Voltage Reference.
   pub const VAL_0x00: u32 = 0x0;
   /// If AREFEN==0 then Internal 1.1V as Voltage Reference without external capacitor. If AREFEN==1 then Internal 1.1V as Voltage Reference with external capacitor at AREF pin.
   pub const VAL_0x01: u32 = 0x1;
   /// If AREFEN==0 then Internal AVCC as Voltage Reference. If AREFEN==1 then AREF pin as Voltage Reference.
   pub const VAL_0x02: u32 = 0x2;
   /// If AREFEN==0 then Internal 2.56V as Voltage Reference without external capacitor. If AREFEN==1 then Internal 2.56V as Voltage Reference with external capacitor at AREF pin.
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

/// `CPU_CLK_COMMAND_LIST_4_BITS` value group
#[allow(non_upper_case_globals)]
pub mod cpu_clk_command_list_4_bits {
   /// No Command.
   pub const VAL_0x00: u32 = 0x0;
   /// Disable Clock Source.
   pub const VAL_0x01: u32 = 0x1;
   /// Enable Clock Source.
   pub const VAL_0x02: u32 = 0x2;
   /// Request for Clock Availability.
   pub const VAL_0x03: u32 = 0x3;
   /// Clock Source Switch.
   pub const VAL_0x04: u32 = 0x4;
   /// Recovery System Clock Source Code.
   pub const VAL_0x05: u32 = 0x5;
   /// Enable Watchdog in Automatic Reload Mode.
   pub const VAL_0x06: u32 = 0x6;
   /// CKOUT Command.
   pub const VAL_0x07: u32 = 0x7;
   /// From 0x08 up to 0x0F: No command.
   pub const VAL_0x08: u32 = 0x8;
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

/// `CPU_SLEEP_MODE2` value group
#[allow(non_upper_case_globals)]
pub mod cpu_sleep_mode2 {
   /// Idle.
   pub const IDLE: u32 = 0x0;
   /// ADC Noise Reduction (If Available).
   pub const ADC: u32 = 0x1;
   /// Power Down.
   pub const PDOWN: u32 = 0x2;
   /// Reserved.
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

/// `ENUM_BODLEVEL` value group
#[allow(non_upper_case_globals)]
pub mod enum_bodlevel {
   /// Brown-out detection at VCC=4.3 V.
   pub const _4V3: u32 = 0x4;
   /// Brown-out detection at VCC=2.7 V.
   pub const _2V7: u32 = 0x5;
   /// Brown-out detection at VCC=1.8 V.
   pub const _1V8: u32 = 0x6;
   /// Brown-out detection at VCC=2.3 V.
   pub const _2V3: u32 = 0x3;
   /// Brown-out detection at VCC=2.2 V.
   pub const _2V2: u32 = 0x2;
   /// Brown-out detection at VCC=1.9 V.
   pub const _1V9: u32 = 0x1;
   /// Brown-out detection at VCC=2.0 V.
   pub const _2V0: u32 = 0x0;
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
   /// Ext. Clock; Start-up time PWRDWN/RESET: 6 CK/14 CK + 0 ms.
   pub const EXTCLK_6CK_14CK_0MS: u32 = 0x0;
   /// Ext. Clock; Start-up time PWRDWN/RESET: 6 CK/14 CK + 4 ms.
   pub const EXTCLK_6CK_14CK_4MS: u32 = 0x10;
   /// Ext. Clock; Start-up time PWRDWN/RESET: 6 CK/14 CK + 64 ms.
   pub const EXTCLK_6CK_14CK_64MS: u32 = 0x20;
   /// Int. RC Osc. 8 MHz; Start-up time PWRDWN/RESET: 6 CK/14 CK + 0 ms.
   pub const INTRCOSC_8MHZ_6CK_14CK_0MS: u32 = 0x2;
   /// Int. RC Osc. 8 MHz; Start-up time PWRDWN/RESET: 6 CK/14 CK + 4 ms.
   pub const INTRCOSC_8MHZ_6CK_14CK_4MS: u32 = 0x12;
   /// Int. RC Osc. 8 MHz; Start-up time PWRDWN/RESET: 6 CK/14 CK + 64 ms.
   pub const INTRCOSC_8MHZ_6CK_14CK_64MS: u32 = 0x22;
   /// WD. Osc. 128 kHz; Start-up time PWRDWN/RESET: 6 CK/14 CK + 0 ms.
   pub const WDOSC_128KHZ_6CK_14CK_0MS: u32 = 0x3;
   /// WD. Osc. 128 kHz; Start-up time PWRDWN/RESET: 6 CK/14 CK + 4 ms.
   pub const WDOSC_128KHZ_6CK_14CK_4MS: u32 = 0x13;
   /// WD. Osc. 128 kHz; Start-up time PWRDWN/RESET: 6 CK/14 CK + 64 ms.
   pub const WDOSC_128KHZ_6CK_14CK_64MS: u32 = 0x23;
   /// Ext. Low-Freq. Crystal; Start-up time PWRDWN/RESET: 1024 CK 4 ms.
   pub const EXTLOFXTAL_1KCK_4MS: u32 = 0x4;
   /// Ext. Low-Freq. Crystal; Start-up time PWRDWN/RESET: 1024 CK + 64 ms.
   pub const EXTLOFXTAL_1KCK_64MS: u32 = 0x14;
   /// Ext. Low-Freq. Crystal; Start-up time PWRDWN/RESET: 32768 CK + 64 ms.
   pub const EXTLOFXTAL_32KCK_64MS: u32 = 0x24;
   /// Ext. Ceramic Res. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 258 CK/14 CK + 4.1 ms.
   pub const EXTCRES_0MHZ4_0MHZ9_258CK_14CK_4MS1: u32 = 0x8;
   /// Ext. Ceramic Res. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 258 CK/14 CK + 65 ms.
   pub const EXTCRES_0MHZ4_0MHZ9_258CK_14CK_65MS: u32 = 0x18;
   /// Ext. Ceramic Res. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 1024 CK /14 CK + 0 ms.
   pub const EXTCRES_0MHZ4_0MHZ9_1KCK_14CK_0MS: u32 = 0x28;
   /// Ext. Ceramic Res. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 1024 CK /14 CK + 4.1 ms.
   pub const EXTCRES_0MHZ4_0MHZ9_1KCK_14CK_4MS1: u32 = 0x38;
   /// Ext. Ceramic Res. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 1024 CK /14 CK + 65 ms.
   pub const EXTCRES_0MHZ4_0MHZ9_1KCK_14CK_65MS: u32 = 0x9;
   /// Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 16384 CK/14 CK + 0 ms.
   pub const EXTXOSC_0MHZ4_0MHZ9_16KCK_14CK_0MS: u32 = 0x19;
   /// Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 16384 CK/14 CK + 4.1 ms.
   pub const EXTXOSC_0MHZ4_0MHZ9_16KCK_14CK_4MS1: u32 = 0x29;
   /// Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 16384 CK/14 CK + 65 ms.
   pub const EXTXOSC_0MHZ4_0MHZ9_16KCK_14CK_65MS: u32 = 0x39;
   /// Ext. Ceramic Res. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 258 CK/14 CK + 4.1 ms.
   pub const EXTCRES_0MHZ9_3MHZ_258CK_14CK_4MS1: u32 = 0xA;
   /// Ext. Ceramic Res. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 258 CK/14 CK + 65 ms.
   pub const EXTCRES_0MHZ9_3MHZ_258CK_14CK_65MS: u32 = 0x1A;
   /// Ext. Ceramic Res. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 1024 CK /14 CK + 0 ms.
   pub const EXTCRES_0MHZ9_3MHZ_1KCK_14CK_0MS: u32 = 0x2A;
   /// Ext. Ceramic Res. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 1024 CK /14 CK + 4.1 ms.
   pub const EXTCRES_0MHZ9_3MHZ_1KCK_14CK_4MS1: u32 = 0x3A;
   /// Ext. Ceramic Res. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 1024 CK /14 CK + 65 ms.
   pub const EXTCRES_0MHZ9_3MHZ_1KCK_14CK_65MS: u32 = 0xB;
   /// Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 16384 CK/14 CK + 0 ms.
   pub const EXTXOSC_0MHZ9_3MHZ_16KCK_14CK_0MS: u32 = 0x1B;
   /// Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 16384 CK/14 CK + 4.1 ms.
   pub const EXTXOSC_0MHZ9_3MHZ_16KCK_14CK_4MS1: u32 = 0x2B;
   /// Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 16384 CK/14 CK + 65 ms.
   pub const EXTXOSC_0MHZ9_3MHZ_16KCK_14CK_65MS: u32 = 0x3B;
   /// Ext. Ceramic Res. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 258 CK/14 CK + 4.1 ms.
   pub const EXTCRES_3MHZ_8MHZ_258CK_14CK_4MS1: u32 = 0xC;
   /// Ext. Ceramic Res. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 258 CK/14 CK + 65 ms.
   pub const EXTCRES_3MHZ_8MHZ_258CK_14CK_65MS: u32 = 0x1C;
   /// Ext. Ceramic Res. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 1024 CK /14 CK + 0 ms.
   pub const EXTCRES_3MHZ_8MHZ_1KCK_14CK_0MS: u32 = 0x2C;
   /// Ext. Ceramic Res. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 1024 CK /14 CK + 4.1 ms.
   pub const EXTCRES_3MHZ_8MHZ_1KCK_14CK_4MS1: u32 = 0x3C;
   /// Ext. Ceramic Res. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 1024 CK /14 CK + 65 ms.
   pub const EXTCRES_3MHZ_8MHZ_1KCK_14CK_65MS: u32 = 0xD;
   /// Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 16384 CK/14 CK + 0 ms.
   pub const EXTXOSC_3MHZ_8MHZ_16KCK_14CK_0MS: u32 = 0x1D;
   /// Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 16384 CK/14 CK + 4.1 ms.
   pub const EXTXOSC_3MHZ_8MHZ_16KCK_14CK_4MS1: u32 = 0x2D;
   /// Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 16384 CK/14 CK + 65 ms.
   pub const EXTXOSC_3MHZ_8MHZ_16KCK_14CK_65MS: u32 = 0x3D;
   /// Ext. Ceramic Res. 8.0-16.0 MHz; Start-up time PWRDWN/RESET: 258 CK/14 CK + 4.1 ms.
   pub const EXTCRES_8MHZ_16MHZ_258CK_14CK_4MS1: u32 = 0xE;
   /// Ext. Ceramic Res. 8.0-16.0 MHz; Start-up time PWRDWN/RESET: 258 CK/14 CK + 65 ms.
   pub const EXTCRES_8MHZ_16MHZ_258CK_14CK_65MS: u32 = 0x1E;
   /// Ext. Ceramic Res. 8.0-16.0 MHz; Start-up time PWRDWN/RESET: 1024 CK /14 CK + 0 ms.
   pub const EXTCRES_8MHZ_16MHZ_1KCK_14CK_0MS: u32 = 0x2E;
   /// Ext. Ceramic Res. 8.0-16.0 MHz; Start-up time PWRDWN/RESET: 1024 CK /14 CK + 4.1 ms.
   pub const EXTCRES_8MHZ_16MHZ_1KCK_14CK_4MS1: u32 = 0x3E;
   /// Ext. Ceramic Res. 8.0-16.0 MHz; Start-up time PWRDWN/RESET: 1024 CK /14 CK + 65 ms.
   pub const EXTCRES_8MHZ_16MHZ_1KCK_14CK_65MS: u32 = 0xF;
   /// Ext. Crystal Osc. 8.0-16.0 MHz; Start-up time PWRDWN/RESET: 16384 CK/14 CK + 0 ms.
   pub const EXTXOSC_8MHZ_16MHZ_16KCK_14CK_0MS: u32 = 0x1F;
   /// Ext. Crystal Osc. 8.0-16.0 MHz; Start-up time PWRDWN/RESET: 16384 CK/14 CK + 4.1 ms.
   pub const EXTXOSC_8MHZ_16MHZ_16KCK_14CK_4MS1: u32 = 0x2F;
   /// Ext. Crystal Osc. 8.0-16.0 MHz; Start-up time PWRDWN/RESET: 16384 CK/14 CK + 65 ms.
   pub const EXTXOSC_8MHZ_16MHZ_16KCK_14CK_65MS: u32 = 0x3F;
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

