//! The AVR ATtiny828 microcontroller
//!
//! # Variants
//! |        | Pinout | Mcu age | Operating temperature | Operating voltage | Max speed |
//! |--------|--------|---------|-----------------------|-------------------|-----------|
//! | standard |  |  | 0°C - 0°C | 1.62V - 5.5V | 0 MHz |
//!

#![allow(non_upper_case_globals)]

/// `LOW` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CKDIV8 | 10000000 |
/// | CKOUT | 1000000 |
/// | SUT_CKSEL | 110011 |
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
/// | SPIEN | 100000 |
/// | DWEN | 1000000 |
/// | RSTDISBL | 10000000 |
/// | BODLEVEL | 111 |
/// | WDTON | 10000 |
pub const HIGH: *mut u8 = 0x1 as *mut u8;

/// `EXTENDED` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BODPD | 11000000 |
/// | BOOTRST | 1 |
/// | BOOTSZ | 110 |
/// | BODACT | 110000 |
pub const EXTENDED: *mut u8 = 0x2 as *mut u8;

/// Port A Input Pins.
pub const PINA: *mut u8 = 0x20 as *mut u8;

/// Data Direction Register, Port A.
pub const DDRA: *mut u8 = 0x21 as *mut u8;

/// Port A Data Register.
pub const PORTA: *mut u8 = 0x22 as *mut u8;

/// Pull-up Enable Control Register.
pub const PUEA: *mut u8 = 0x23 as *mut u8;

/// Port B Input Pins.
pub const PINB: *mut u8 = 0x24 as *mut u8;

/// Data Direction Register, Port B.
pub const DDRB: *mut u8 = 0x25 as *mut u8;

/// Port B Data Register.
pub const PORTB: *mut u8 = 0x26 as *mut u8;

/// Pull-up Enable Control Register.
pub const PUEB: *mut u8 = 0x27 as *mut u8;

/// Port C Input Pins.
pub const PINC: *mut u8 = 0x28 as *mut u8;

/// Data Direction Register, Port C.
pub const DDRC: *mut u8 = 0x29 as *mut u8;

/// Port C Data Register.
pub const PORTC: *mut u8 = 0x2A as *mut u8;

/// Pull-up Enable Control Register.
pub const PUEC: *mut u8 = 0x2B as *mut u8;

/// Port D Input Pins.
pub const PIND: *mut u8 = 0x2C as *mut u8;

/// Data Direction Register, Port D.
pub const DDRD: *mut u8 = 0x2D as *mut u8;

/// Port D Data Register.
pub const PORTD: *mut u8 = 0x2E as *mut u8;

/// Pull-up Enable Control Register.
pub const PUED: *mut u8 = 0x2F as *mut u8;

/// Port High Drive Enable Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PHDEC | 100 |
pub const PHDE: *mut u8 = 0x34 as *mut u8;

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
/// | ICF1 | 100000 |
/// | OCF1B | 100 |
/// | TOV1 | 1 |
/// | OCF1A | 10 |
pub const TIFR1: *mut u8 = 0x36 as *mut u8;

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

/// General Purpose I/O Register 0.
pub const GPIOR0: *mut u8 = 0x3E as *mut u8;

/// EEPROM Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EEPE | 10 |
/// | EEMPE | 100 |
/// | EEPM | 110000 |
/// | EERIE | 1000 |
/// | EERE | 1 |
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

/// Timer/Counter  Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | COM0B | 110000 |
/// | COM0A | 11000000 |
/// | WGM0 | 11 |
pub const TCCR0A: *mut u8 = 0x44 as *mut u8;

/// Timer/Counter Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WGM02 | 1000 |
/// | FOC0A | 10000000 |
/// | CS0 | 111 |
/// | FOC0B | 1000000 |
pub const TCCR0B: *mut u8 = 0x45 as *mut u8;

/// Timer/Counter0.
pub const TCNT0: *mut u8 = 0x46 as *mut u8;

/// Timer/Counter0 Output Compare Register.
pub const OCR0A: *mut u8 = 0x47 as *mut u8;

/// Timer/Counter0 Output Compare Register.
pub const OCR0B: *mut u8 = 0x48 as *mut u8;

/// General Purpose I/O Register 1.
pub const GPIOR1: *mut u8 = 0x4A as *mut u8;

/// General Purpose I/O Register 2.
pub const GPIOR2: *mut u8 = 0x4B as *mut u8;

/// SPI Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CPOL | 1000 |
/// | DORD | 100000 |
/// | SPR | 11 |
/// | SPE | 1000000 |
/// | CPHA | 100 |
/// | MSTR | 10000 |
/// | SPIE | 10000000 |
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

/// Analog Comparator Control And Status Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ACNMUX | 1100 |
/// | ACPMUX | 11 |
/// | HSEL | 10000000 |
/// | HLEV | 1000000 |
pub const ACSRB: *mut u8 = 0x4F as *mut u8;

/// Analog Comparator Control And Status Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ACO | 100000 |
/// | ACIC | 100 |
/// | ACPMUX2 | 1000000 |
/// | ACD | 10000000 |
/// | ACI | 10000 |
/// | ACIE | 1000 |
/// | ACIS | 11 |
pub const ACSRA: *mut u8 = 0x50 as *mut u8;

/// Sleep Mode Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SM | 110 |
/// | SE | 1 |
pub const SMCR: *mut u8 = 0x53 as *mut u8;

/// MCU Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDRF | 1000 |
/// | BORF | 100 |
/// | EXTRF | 10 |
/// | PORF | 1 |
pub const MCUSR: *mut u8 = 0x54 as *mut u8;

/// MCU Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | IVSEL | 10 |
pub const MCUCR: *mut u8 = 0x55 as *mut u8;

/// Configuration Change Protection.
pub const CCP: *mut u8 = 0x56 as *mut u8;

/// Store Program Memory Control and Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SPMIE | 10000000 |
/// | RWWSRE | 10000 |
/// | PGWRT | 100 |
/// | PGERS | 10 |
/// | RSIG | 100000 |
/// | RWFLB | 1000 |
/// | SPMEN | 1 |
/// | RWWSB | 1000000 |
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
/// | N | 100 |
/// | I | 10000000 |
/// | V | 1000 |
/// | T | 1000000 |
/// | Z | 10 |
/// | H | 100000 |
/// | S | 10000 |
/// | C | 1 |
pub const SREG: *mut u8 = 0x5F as *mut u8;

/// Watchdog Timer Control and Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDP | 100111 |
/// | WDIF | 10000000 |
/// | WDE | 1000 |
/// | WDIE | 1000000 |
pub const WDTCSR: *mut u8 = 0x60 as *mut u8;

/// Clock Prescale Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CLKPS | 1111 |
pub const CLKPR: *mut u8 = 0x61 as *mut u8;

/// Power Reduction Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRTIM1 | 1000 |
/// | PRTWI | 10000000 |
/// | PRTIM0 | 100000 |
/// | PRUSART0 | 10 |
/// | PRSPI | 100 |
/// | PRADC | 1 |
pub const PRR: *mut u8 = 0x64 as *mut u8;

/// Oscillator Calibration Register 8MHz.
pub const OSCCAL0: *mut u8 = 0x66 as *mut u8;

/// Oscillator Calibration Register 32kHz.
pub const OSCCAL1: *mut u8 = 0x67 as *mut u8;

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
/// | ISC1 | 1100 |
pub const EICRA: *mut u8 = 0x69 as *mut u8;

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
/// | TOIE0 | 1 |
/// | OCIE0A | 10 |
/// | OCIE0B | 100 |
pub const TIMSK0: *mut u8 = 0x6E as *mut u8;

/// Timer/Counter Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCIE1A | 10 |
/// | ICIE1 | 100000 |
/// | OCIE1B | 100 |
/// | TOIE1 | 1 |
pub const TIMSK1: *mut u8 = 0x6F as *mut u8;

/// Pin Change Mask Register 3.
pub const PCMSK3: *mut u8 = 0x73 as *mut u8;

/// ADC Data Register  Bytes low byte.
pub const ADCL: *mut u8 = 0x78 as *mut u8;

/// ADC Data Register  Bytes.
pub const ADC: *mut u16 = 0x78 as *mut u16;

/// ADC Data Register  Bytes high byte.
pub const ADCH: *mut u8 = 0x79 as *mut u8;

/// The ADC Control and Status register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADSC | 1000000 |
/// | ADPS | 111 |
/// | ADIF | 10000 |
/// | ADATE | 100000 |
/// | ADIE | 1000 |
/// | ADEN | 10000000 |
pub const ADCSRA: *mut u8 = 0x7A as *mut u8;

/// ADC Control and Status Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADTS | 111 |
/// | ADLAR | 1000 |
pub const ADCSRB: *mut u8 = 0x7B as *mut u8;

/// The ADC multiplexer Selection Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | MUX | 11111 |
pub const ADMUXA: *mut u8 = 0x7C as *mut u8;

/// The ADC multiplexer Selection Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | MUX5 | 1 |
/// | REFS | 100000 |
pub const ADMUXB: *mut u8 = 0x7D as *mut u8;

/// Digital Input Disable Register 0.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADC6D | 1000000 |
/// | ADC3D | 1000 |
/// | ADC5D | 100000 |
/// | ADC4D | 10000 |
/// | ADC2D | 100 |
/// | ADC0D | 1 |
/// | ADC7D | 10000000 |
/// | ADC1D | 10 |
pub const DIDR0: *mut u8 = 0x7E as *mut u8;

/// Digital Input Disable Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADC12D | 10000 |
/// | ADC11D | 1000 |
/// | ADC10D | 100 |
/// | ADC15D | 10000000 |
/// | ADC14D | 1000000 |
/// | ADC13D | 100000 |
/// | ADC9D | 10 |
/// | ADC8D | 1 |
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
/// | ICNC1 | 10000000 |
/// | CS1 | 111 |
/// | ICES1 | 1000000 |
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

/// Timer/Counter1  Bytes.
pub const TCNT1: *mut u16 = 0x84 as *mut u16;

/// Timer/Counter1  Bytes low byte.
pub const TCNT1L: *mut u8 = 0x84 as *mut u8;

/// Timer/Counter1  Bytes high byte.
pub const TCNT1H: *mut u8 = 0x85 as *mut u8;

/// Timer/Counter1 Input Capture Register  Bytes low byte.
pub const ICR1L: *mut u8 = 0x86 as *mut u8;

/// Timer/Counter1 Input Capture Register  Bytes.
pub const ICR1: *mut u16 = 0x86 as *mut u16;

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

/// TWI Slave Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TWEN | 1000 |
/// | TWPME | 10 |
/// | TWSME | 1 |
/// | TWSHE | 10000000 |
/// | TWASIE | 10000 |
/// | TWDIE | 100000 |
/// | TWSIE | 100 |
pub const TWSCRA: *mut u8 = 0xB8 as *mut u8;

/// TWI Slave Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TWCMD | 11 |
/// | TWHNM | 1000 |
/// | TWAA | 100 |
pub const TWSCRB: *mut u8 = 0xB9 as *mut u8;

/// TWI Slave Status Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TWBE | 100 |
/// | TWDIR | 10 |
/// | TWCH | 100000 |
/// | TWDIF | 10000000 |
/// | TWC | 1000 |
/// | TWASIF | 1000000 |
/// | TWRA | 10000 |
/// | TWAS | 1 |
pub const TWSSRA: *mut u8 = 0xBA as *mut u8;

/// TWI Slave Address Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TWAE | 1 |
pub const TWSAM: *mut u8 = 0xBB as *mut u8;

/// TWI Slave Address Register.
pub const TWSA: *mut u8 = 0xBC as *mut u8;

/// TWI Slave Data Register.
pub const TWSD: *mut u8 = 0xBD as *mut u8;

/// USART Control and Status Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TXC | 1000000 |
/// | UDRE | 100000 |
/// | UPE | 100 |
/// | RXC | 10000000 |
/// | DOR | 1000 |
/// | MPCM | 1 |
/// | FE | 10000 |
/// | U2X | 10 |
pub const UCSRA: *mut u8 = 0xC0 as *mut u8;

/// USART Control and Status Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RXB8 | 10 |
/// | RXEN | 10000 |
/// | UCSZ2 | 100 |
/// | TXCIE | 1000000 |
/// | RXCIE | 10000000 |
/// | TXB8 | 1 |
/// | UDRIE | 100000 |
/// | TXEN | 1000 |
pub const UCSRB: *mut u8 = 0xC1 as *mut u8;

/// USART Control and Status Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | UMSEL | 11000000 |
/// | UCSZ | 110 |
/// | USBS | 1000 |
/// | UCPOL | 1 |
/// | UPM | 110000 |
pub const UCSRC: *mut u8 = 0xC2 as *mut u8;

/// USART Control and Status Register D.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RXSIE | 10000000 |
/// | SFDE | 100000 |
/// | RXS | 1000000 |
pub const UCSRD: *mut u8 = 0xC3 as *mut u8;

/// USART Baud Rate Register  Bytes.
pub const UBRR: *mut u16 = 0xC4 as *mut u16;

/// USART Baud Rate Register  Bytes low byte.
pub const UBRRL: *mut u8 = 0xC4 as *mut u8;

/// USART Baud Rate Register  Bytes high byte.
pub const UBRRH: *mut u8 = 0xC5 as *mut u8;

/// USART I/O Data Register.
pub const UDR: *mut u8 = 0xC6 as *mut u8;

/// Digital Input Disable Register 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADC17D | 10 |
/// | ADC19D | 1000 |
/// | ADC22D | 1000000 |
/// | ADC16D | 1 |
/// | ADC21D | 100000 |
/// | ADC23D | 10000000 |
/// | ADC20D | 10000 |
/// | ADC18D | 100 |
pub const DIDR2: *mut u8 = 0xDE as *mut u8;

/// Digital Input Disable Register 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADC25D | 10 |
/// | ADC27D | 1000 |
/// | ADC24D | 1 |
/// | ADC26D | 100 |
pub const DIDR3: *mut u8 = 0xDF as *mut u8;

/// Timer Output Compare Pin Mux Channel Output Enable.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TOCC5OE | 100000 |
/// | TOCC0OE | 1 |
/// | TOCC1OE | 10 |
/// | TOCC6OE | 1000000 |
/// | TOCC4OE | 10000 |
/// | TOCC3OE | 1000 |
/// | TOCC2OE | 100 |
/// | TOCC7OE | 10000000 |
pub const TOCPMCOE: *mut u8 = 0xE2 as *mut u8;

/// Timer Output Compare Pin Mux Selection 0.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TOCC2S | 110000 |
/// | TOCC1S | 1100 |
/// | TOCC0S | 11 |
/// | TOCC3S | 11000000 |
pub const TOCPMSA0: *mut u8 = 0xE8 as *mut u8;

/// Timer Output Compare Pin Mux Selection 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TOCC6S | 110000 |
/// | TOCC5S | 1100 |
/// | TOCC4S | 11 |
/// | TOCC7S | 11000000 |
pub const TOCPMSA1: *mut u8 = 0xE9 as *mut u8;

/// Oscillator Temperature Calibration Register A.
pub const OSCTCAL0A: *mut u8 = 0xF0 as *mut u8;

/// Oscillator Temperature Calibration Register B.
pub const OSCTCAL0B: *mut u8 = 0xF1 as *mut u8;

/// Bitfield on register `ACSRA`
pub const ACO: u8 = 0x20;

/// Bitfield on register `ACSRA`
pub const ACIC: u8 = 0x4;

/// Bitfield on register `ACSRA`
pub const ACPMUX2: u8 = 0x40;

/// Bitfield on register `ACSRA`
pub const ACD: u8 = 0x80;

/// Bitfield on register `ACSRA`
pub const ACI: u8 = 0x10;

/// Bitfield on register `ACSRA`
pub const ACIE: u8 = 0x8;

/// Bitfield on register `ACSRA`
pub const ACIS: u8 = 0x3;

/// Bitfield on register `ACSRB`
pub const ACNMUX: u8 = 0xC;

/// Bitfield on register `ACSRB`
pub const ACPMUX: u8 = 0x3;

/// Bitfield on register `ACSRB`
pub const HSEL: u8 = 0x80;

/// Bitfield on register `ACSRB`
pub const HLEV: u8 = 0x40;

/// Bitfield on register `ADCSRA`
pub const ADSC: u8 = 0x40;

/// Bitfield on register `ADCSRA`
pub const ADPS: u8 = 0x7;

/// Bitfield on register `ADCSRA`
pub const ADIF: u8 = 0x10;

/// Bitfield on register `ADCSRA`
pub const ADATE: u8 = 0x20;

/// Bitfield on register `ADCSRA`
pub const ADIE: u8 = 0x8;

/// Bitfield on register `ADCSRA`
pub const ADEN: u8 = 0x80;

/// Bitfield on register `ADCSRB`
pub const ADTS: u8 = 0x7;

/// Bitfield on register `ADCSRB`
pub const ADLAR: u8 = 0x8;

/// Bitfield on register `ADMUXA`
pub const MUX: u8 = 0x1F;

/// Bitfield on register `ADMUXB`
pub const MUX5: u8 = 0x1;

/// Bitfield on register `ADMUXB`
pub const REFS: u8 = 0x20;

/// Bitfield on register `CLKPR`
pub const CLKPS: u8 = 0xF;

/// Bitfield on register `DIDR0`
pub const ADC6D: u8 = 0x40;

/// Bitfield on register `DIDR0`
pub const ADC3D: u8 = 0x8;

/// Bitfield on register `DIDR0`
pub const ADC5D: u8 = 0x20;

/// Bitfield on register `DIDR0`
pub const ADC4D: u8 = 0x10;

/// Bitfield on register `DIDR0`
pub const ADC2D: u8 = 0x4;

/// Bitfield on register `DIDR0`
pub const ADC0D: u8 = 0x1;

/// Bitfield on register `DIDR0`
pub const ADC7D: u8 = 0x80;

/// Bitfield on register `DIDR0`
pub const ADC1D: u8 = 0x2;

/// Bitfield on register `DIDR1`
pub const ADC12D: u8 = 0x10;

/// Bitfield on register `DIDR1`
pub const ADC11D: u8 = 0x8;

/// Bitfield on register `DIDR1`
pub const ADC10D: u8 = 0x4;

/// Bitfield on register `DIDR1`
pub const ADC15D: u8 = 0x80;

/// Bitfield on register `DIDR1`
pub const ADC14D: u8 = 0x40;

/// Bitfield on register `DIDR1`
pub const ADC13D: u8 = 0x20;

/// Bitfield on register `DIDR1`
pub const ADC9D: u8 = 0x2;

/// Bitfield on register `DIDR1`
pub const ADC8D: u8 = 0x1;

/// Bitfield on register `DIDR2`
pub const ADC17D: u8 = 0x2;

/// Bitfield on register `DIDR2`
pub const ADC19D: u8 = 0x8;

/// Bitfield on register `DIDR2`
pub const ADC22D: u8 = 0x40;

/// Bitfield on register `DIDR2`
pub const ADC16D: u8 = 0x1;

/// Bitfield on register `DIDR2`
pub const ADC21D: u8 = 0x20;

/// Bitfield on register `DIDR2`
pub const ADC23D: u8 = 0x80;

/// Bitfield on register `DIDR2`
pub const ADC20D: u8 = 0x10;

/// Bitfield on register `DIDR2`
pub const ADC18D: u8 = 0x4;

/// Bitfield on register `DIDR3`
pub const ADC25D: u8 = 0x2;

/// Bitfield on register `DIDR3`
pub const ADC27D: u8 = 0x8;

/// Bitfield on register `DIDR3`
pub const ADC24D: u8 = 0x1;

/// Bitfield on register `DIDR3`
pub const ADC26D: u8 = 0x4;

/// Bitfield on register `EECR`
pub const EEPE: u8 = 0x2;

/// Bitfield on register `EECR`
pub const EEMPE: u8 = 0x4;

/// Bitfield on register `EECR`
pub const EEPM: u8 = 0x30;

/// Bitfield on register `EECR`
pub const EERIE: u8 = 0x8;

/// Bitfield on register `EECR`
pub const EERE: u8 = 0x1;

/// Bitfield on register `EICRA`
pub const ISC0: u8 = 0x3;

/// Bitfield on register `EICRA`
pub const ISC1: u8 = 0xC;

/// Bitfield on register `EIFR`
pub const INTF: u8 = 0x3;

/// Bitfield on register `EIMSK`
pub const INT: u8 = 0x3;

/// Bitfield on register `EXTENDED`
pub const BODPD: u8 = 0xC0;

/// Bitfield on register `EXTENDED`
pub const BOOTRST: u8 = 0x1;

/// Bitfield on register `EXTENDED`
pub const BOOTSZ: u8 = 0x6;

/// Bitfield on register `EXTENDED`
pub const BODACT: u8 = 0x30;

/// Bitfield on register `GTCCR`
pub const PSRSYNC: u8 = 0x1;

/// Bitfield on register `GTCCR`
pub const TSM: u8 = 0x80;

/// Bitfield on register `HIGH`
pub const EESAVE: u8 = 0x8;

/// Bitfield on register `HIGH`
pub const SPIEN: u8 = 0x20;

/// Bitfield on register `HIGH`
pub const DWEN: u8 = 0x40;

/// Bitfield on register `HIGH`
pub const RSTDISBL: u8 = 0x80;

/// Bitfield on register `HIGH`
pub const BODLEVEL: u8 = 0x7;

/// Bitfield on register `HIGH`
pub const WDTON: u8 = 0x10;

/// Bitfield on register `LOCKBIT`
pub const BLB1: u8 = 0x30;

/// Bitfield on register `LOCKBIT`
pub const BLB0: u8 = 0xC;

/// Bitfield on register `LOCKBIT`
pub const LB: u8 = 0x3;

/// Bitfield on register `LOW`
pub const CKDIV8: u8 = 0x80;

/// Bitfield on register `LOW`
pub const CKOUT: u8 = 0x40;

/// Bitfield on register `LOW`
pub const SUT_CKSEL: u8 = 0x33;

/// Bitfield on register `MCUCR`
pub const IVSEL: u8 = 0x2;

/// Bitfield on register `MCUSR`
pub const WDRF: u8 = 0x8;

/// Bitfield on register `MCUSR`
pub const BORF: u8 = 0x4;

/// Bitfield on register `MCUSR`
pub const EXTRF: u8 = 0x2;

/// Bitfield on register `MCUSR`
pub const PORF: u8 = 0x1;

/// Bitfield on register `PCICR`
pub const PCIE: u8 = 0xF;

/// Bitfield on register `PCIFR`
pub const PCIF: u8 = 0xF;

/// Bitfield on register `PHDE`
pub const PHDEC: u8 = 0x4;

/// Bitfield on register `PRR`
pub const PRTIM1: u8 = 0x8;

/// Bitfield on register `PRR`
pub const PRTWI: u8 = 0x80;

/// Bitfield on register `PRR`
pub const PRTIM0: u8 = 0x20;

/// Bitfield on register `PRR`
pub const PRUSART0: u8 = 0x2;

/// Bitfield on register `PRR`
pub const PRSPI: u8 = 0x4;

/// Bitfield on register `PRR`
pub const PRADC: u8 = 0x1;

/// Bitfield on register `SMCR`
pub const SM: u8 = 0x6;

/// Bitfield on register `SMCR`
pub const SE: u8 = 0x1;

/// Bitfield on register `SPCR`
pub const CPOL: u8 = 0x8;

/// Bitfield on register `SPCR`
pub const DORD: u8 = 0x20;

/// Bitfield on register `SPCR`
pub const SPR: u8 = 0x3;

/// Bitfield on register `SPCR`
pub const SPE: u8 = 0x40;

/// Bitfield on register `SPCR`
pub const CPHA: u8 = 0x4;

/// Bitfield on register `SPCR`
pub const MSTR: u8 = 0x10;

/// Bitfield on register `SPCR`
pub const SPIE: u8 = 0x80;

/// Bitfield on register `SPMCSR`
pub const SPMIE: u8 = 0x80;

/// Bitfield on register `SPMCSR`
pub const RWWSRE: u8 = 0x10;

/// Bitfield on register `SPMCSR`
pub const PGWRT: u8 = 0x4;

/// Bitfield on register `SPMCSR`
pub const PGERS: u8 = 0x2;

/// Bitfield on register `SPMCSR`
pub const RSIG: u8 = 0x20;

/// Bitfield on register `SPMCSR`
pub const RWFLB: u8 = 0x8;

/// Bitfield on register `SPMCSR`
pub const SPMEN: u8 = 0x1;

/// Bitfield on register `SPMCSR`
pub const RWWSB: u8 = 0x40;

/// Bitfield on register `SPSR`
pub const WCOL: u8 = 0x40;

/// Bitfield on register `SPSR`
pub const SPI2X: u8 = 0x1;

/// Bitfield on register `SPSR`
pub const SPIF: u8 = 0x80;

/// Bitfield on register `SREG`
pub const N: u8 = 0x4;

/// Bitfield on register `SREG`
pub const I: u8 = 0x80;

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
pub const C: u8 = 0x1;

/// Bitfield on register `TCCR0A`
pub const COM0B: u8 = 0x30;

/// Bitfield on register `TCCR0A`
pub const COM0A: u8 = 0xC0;

/// Bitfield on register `TCCR0A`
pub const WGM0: u8 = 0x3;

/// Bitfield on register `TCCR0B`
pub const WGM02: u8 = 0x8;

/// Bitfield on register `TCCR0B`
pub const FOC0A: u8 = 0x80;

/// Bitfield on register `TCCR0B`
pub const CS0: u8 = 0x7;

/// Bitfield on register `TCCR0B`
pub const FOC0B: u8 = 0x40;

/// Bitfield on register `TCCR1A`
pub const COM1B: u8 = 0x30;

/// Bitfield on register `TCCR1A`
pub const COM1A: u8 = 0xC0;

/// Bitfield on register `TCCR1B`
pub const ICNC1: u8 = 0x80;

/// Bitfield on register `TCCR1B`
pub const CS1: u8 = 0x7;

/// Bitfield on register `TCCR1B`
pub const ICES1: u8 = 0x40;

/// Bitfield on register `TCCR1C`
pub const FOC1B: u8 = 0x40;

/// Bitfield on register `TCCR1C`
pub const FOC1A: u8 = 0x80;

/// Bitfield on register `TIFR0`
pub const TOV0: u8 = 0x1;

/// Bitfield on register `TIFR0`
pub const OCF0B: u8 = 0x4;

/// Bitfield on register `TIFR0`
pub const OCF0A: u8 = 0x2;

/// Bitfield on register `TIFR1`
pub const ICF1: u8 = 0x20;

/// Bitfield on register `TIFR1`
pub const OCF1B: u8 = 0x4;

/// Bitfield on register `TIFR1`
pub const TOV1: u8 = 0x1;

/// Bitfield on register `TIFR1`
pub const OCF1A: u8 = 0x2;

/// Bitfield on register `TIMSK0`
pub const TOIE0: u8 = 0x1;

/// Bitfield on register `TIMSK0`
pub const OCIE0A: u8 = 0x2;

/// Bitfield on register `TIMSK0`
pub const OCIE0B: u8 = 0x4;

/// Bitfield on register `TIMSK1`
pub const OCIE1A: u8 = 0x2;

/// Bitfield on register `TIMSK1`
pub const ICIE1: u8 = 0x20;

/// Bitfield on register `TIMSK1`
pub const OCIE1B: u8 = 0x4;

/// Bitfield on register `TIMSK1`
pub const TOIE1: u8 = 0x1;

/// Bitfield on register `TOCPMCOE`
pub const TOCC5OE: u8 = 0x20;

/// Bitfield on register `TOCPMCOE`
pub const TOCC0OE: u8 = 0x1;

/// Bitfield on register `TOCPMCOE`
pub const TOCC1OE: u8 = 0x2;

/// Bitfield on register `TOCPMCOE`
pub const TOCC6OE: u8 = 0x40;

/// Bitfield on register `TOCPMCOE`
pub const TOCC4OE: u8 = 0x10;

/// Bitfield on register `TOCPMCOE`
pub const TOCC3OE: u8 = 0x8;

/// Bitfield on register `TOCPMCOE`
pub const TOCC2OE: u8 = 0x4;

/// Bitfield on register `TOCPMCOE`
pub const TOCC7OE: u8 = 0x80;

/// Bitfield on register `TOCPMSA0`
pub const TOCC2S: u8 = 0x30;

/// Bitfield on register `TOCPMSA0`
pub const TOCC1S: u8 = 0xC;

/// Bitfield on register `TOCPMSA0`
pub const TOCC0S: u8 = 0x3;

/// Bitfield on register `TOCPMSA0`
pub const TOCC3S: u8 = 0xC0;

/// Bitfield on register `TOCPMSA1`
pub const TOCC6S: u8 = 0x30;

/// Bitfield on register `TOCPMSA1`
pub const TOCC5S: u8 = 0xC;

/// Bitfield on register `TOCPMSA1`
pub const TOCC4S: u8 = 0x3;

/// Bitfield on register `TOCPMSA1`
pub const TOCC7S: u8 = 0xC0;

/// Bitfield on register `TWSAM`
pub const TWAE: u8 = 0x1;

/// Bitfield on register `TWSCRA`
pub const TWEN: u8 = 0x8;

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

/// Bitfield on register `TWSCRA`
pub const TWSIE: u8 = 0x4;

/// Bitfield on register `TWSCRB`
pub const TWCMD: u8 = 0x3;

/// Bitfield on register `TWSCRB`
pub const TWHNM: u8 = 0x8;

/// Bitfield on register `TWSCRB`
pub const TWAA: u8 = 0x4;

/// Bitfield on register `TWSSRA`
pub const TWBE: u8 = 0x4;

/// Bitfield on register `TWSSRA`
pub const TWDIR: u8 = 0x2;

/// Bitfield on register `TWSSRA`
pub const TWCH: u8 = 0x20;

/// Bitfield on register `TWSSRA`
pub const TWDIF: u8 = 0x80;

/// Bitfield on register `TWSSRA`
pub const TWC: u8 = 0x8;

/// Bitfield on register `TWSSRA`
pub const TWASIF: u8 = 0x40;

/// Bitfield on register `TWSSRA`
pub const TWRA: u8 = 0x10;

/// Bitfield on register `TWSSRA`
pub const TWAS: u8 = 0x1;

/// Bitfield on register `UCSRA`
pub const TXC: u8 = 0x40;

/// Bitfield on register `UCSRA`
pub const UDRE: u8 = 0x20;

/// Bitfield on register `UCSRA`
pub const UPE: u8 = 0x4;

/// Bitfield on register `UCSRA`
pub const RXC: u8 = 0x80;

/// Bitfield on register `UCSRA`
pub const DOR: u8 = 0x8;

/// Bitfield on register `UCSRA`
pub const MPCM: u8 = 0x1;

/// Bitfield on register `UCSRA`
pub const FE: u8 = 0x10;

/// Bitfield on register `UCSRA`
pub const U2X: u8 = 0x2;

/// Bitfield on register `UCSRB`
pub const RXB8: u8 = 0x2;

/// Bitfield on register `UCSRB`
pub const RXEN: u8 = 0x10;

/// Bitfield on register `UCSRB`
pub const UCSZ2: u8 = 0x4;

/// Bitfield on register `UCSRB`
pub const TXCIE: u8 = 0x40;

/// Bitfield on register `UCSRB`
pub const RXCIE: u8 = 0x80;

/// Bitfield on register `UCSRB`
pub const TXB8: u8 = 0x1;

/// Bitfield on register `UCSRB`
pub const UDRIE: u8 = 0x20;

/// Bitfield on register `UCSRB`
pub const TXEN: u8 = 0x8;

/// Bitfield on register `UCSRC`
pub const UMSEL: u8 = 0xC0;

/// Bitfield on register `UCSRC`
pub const UCSZ: u8 = 0x6;

/// Bitfield on register `UCSRC`
pub const USBS: u8 = 0x8;

/// Bitfield on register `UCSRC`
pub const UCPOL: u8 = 0x1;

/// Bitfield on register `UCSRC`
pub const UPM: u8 = 0x30;

/// Bitfield on register `UCSRD`
pub const RXSIE: u8 = 0x80;

/// Bitfield on register `UCSRD`
pub const SFDE: u8 = 0x20;

/// Bitfield on register `UCSRD`
pub const RXS: u8 = 0x40;

/// Bitfield on register `WDTCSR`
pub const WDP: u8 = 0x27;

/// Bitfield on register `WDTCSR`
pub const WDIF: u8 = 0x80;

/// Bitfield on register `WDTCSR`
pub const WDE: u8 = 0x8;

/// Bitfield on register `WDTCSR`
pub const WDIE: u8 = 0x40;

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
   /// Brown-out detection at VCC=4.3 V.
   pub const _4V3: u32 = 0x4;
   /// Brown-out detection at VCC=2.7 V.
   pub const _2V7: u32 = 0x5;
   /// Brown-out detection at VCC=1.8 V.
   pub const _1V8: u32 = 0x6;
}

/// `ENUM_BODMODE` value group
#[allow(non_upper_case_globals)]
pub mod enum_bodmode {
   /// Sampled.
   pub const BOD_SAMPLED: u32 = 0x1;
   /// Enabled.
   pub const BOD_ENABLED: u32 = 0x2;
   /// Disabled.
   pub const BOD_DISABLED: u32 = 0x3;
}

/// `ENUM_BOOTSZ` value group
#[allow(non_upper_case_globals)]
pub mod enum_bootsz {
   /// Boot Flash size=128 words Boot address=$0F80.
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
   /// Int. RC Osc. 8 MHz; Start-up time PWRDWN/RESET: 6 + 14 CK + 0 ms.
   pub const INTRCOSC_8MHZ_6CK_14CK_0MS: u32 = 0x2;
   /// Int. RC Osc. 8 MHz; Start-up time PWRDWN/RESET: 6 + 14 CK + 4.1 ms.
   pub const INTRCOSC_8MHZ_6CK_14CK_4MS1: u32 = 0x12;
   /// Int. ULP Osc. 32kHz; Start-up time PWRDWN/RESET: 6 + 14 CK + 0 ms.
   pub const INTULPOSC_32KHZ_6CK_14CK_0MS: u32 = 0x3;
   /// Int. ULP Osc. 32kHz; Start-up time PWRDWN/RESET: 6 + 14 CK + 4.1 ms.
   pub const INTULPOSC_32KHZ_6CK_14CK_4MS1: u32 = 0x13;
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
   /// 32 kHz.
   pub const _32_kHz: u32 = 0x3;
}

/// `WDOG_TIMER_PRESCALE_4BITS_32KHZ` value group
#[allow(non_upper_case_globals)]
pub mod wdog_timer_prescale_4bits_32khz {
   /// Oscillator Cycles 512 (16 ms).
   pub const VAL_0x00: u32 = 0x0;
   /// Oscillator Cycles 1K (32 ms).
   pub const VAL_0x01: u32 = 0x1;
   /// Oscillator Cycles 2K (64 ms).
   pub const VAL_0x02: u32 = 0x2;
   /// Oscillator Cycles 4K (0.125 s).
   pub const VAL_0x03: u32 = 0x3;
   /// Oscillator Cycles 8K (0.25 s).
   pub const VAL_0x04: u32 = 0x4;
   /// Oscillator Cycles 16K (0.5 s).
   pub const VAL_0x05: u32 = 0x5;
   /// Oscillator Cycles 32K (1.0 s).
   pub const VAL_0x06: u32 = 0x6;
   /// Oscillator Cycles 64K (2.0 s).
   pub const VAL_0x07: u32 = 0x7;
   /// Oscillator Cycles 128K (4.0 s).
   pub const VAL_0x08: u32 = 0x8;
   /// Oscillator Cycles 256K (8.0 s).
   pub const VAL_0x09: u32 = 0x9;
}

