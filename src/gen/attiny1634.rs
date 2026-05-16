//! The AVR ATtiny1634 microcontroller
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
/// | SUT_CKSEL | 11111 |
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
/// | SPIEN | 100000 |
/// | RSTDISBL | 10000000 |
/// | DWEN | 1000000 |
/// | BODLEVEL | 111 |
/// | WDTON | 10000 |
/// | EESAVE | 1000 |
pub const HIGH: *mut u8 = 0x1 as *mut u8;

/// `EXTENDED` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BODACT | 110 |
/// | SELFPRGEN | 1 |
/// | BODPD | 11000 |
pub const EXTENDED: *mut u8 = 0x2 as *mut u8;

/// ADC Data Register  Bytes.
pub const ADC: *mut u16 = 0x20 as *mut u16;

/// ADC Data Register  Bytes low byte.
pub const ADCL: *mut u8 = 0x20 as *mut u8;

/// ADC Data Register  Bytes high byte.
pub const ADCH: *mut u8 = 0x21 as *mut u8;

/// ADC Control and Status Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADLAR | 1000 |
/// | ADTS | 111 |
/// | VDEN | 10000000 |
/// | VDPD | 1000000 |
pub const ADCSRB: *mut u8 = 0x22 as *mut u8;

/// The ADC Control and Status register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADSC | 1000000 |
/// | ADEN | 10000000 |
/// | ADIF | 10000 |
/// | ADIE | 1000 |
/// | ADPS | 111 |
/// | ADATE | 100000 |
pub const ADCSRA: *mut u8 = 0x23 as *mut u8;

/// The ADC multiplexer Selection Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | MUX | 1111 |
/// | REFEN | 100000 |
/// | ADC0EN | 10000 |
/// | REFS | 11000000 |
pub const ADMUX: *mut u8 = 0x24 as *mut u8;

/// Analog Comparator Control And Status Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ACME | 100 |
/// | HSEL | 10000000 |
/// | ACLP | 100000 |
/// | ACIRS | 11 |
/// | HLEV | 1000000 |
/// | ACCE | 1000 |
pub const ACSRB: *mut u8 = 0x25 as *mut u8;

/// Analog Comparator Control And Status Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ACI | 10000 |
/// | ACIS | 11 |
/// | ACBG | 1000000 |
/// | ACIE | 1000 |
/// | ACIC | 100 |
/// | ACD | 10000000 |
/// | ACO | 100000 |
pub const ACSRA: *mut u8 = 0x26 as *mut u8;

/// Port C Input Pins.
pub const PINC: *mut u8 = 0x27 as *mut u8;

/// Data Direction Register, Port C.
pub const DDRC: *mut u8 = 0x28 as *mut u8;

/// Port C Data Register.
pub const PORTC: *mut u8 = 0x29 as *mut u8;

/// Pull-up Enable Control Register.
pub const PUEC: *mut u8 = 0x2A as *mut u8;

/// Port B Data register.
pub const PINB: *mut u8 = 0x2B as *mut u8;

/// Data Direction Register, Port B.
pub const DDRB: *mut u8 = 0x2C as *mut u8;

/// Input Pins, Port B.
pub const PORTB: *mut u8 = 0x2D as *mut u8;

/// Pull-up Enable Control Register.
pub const PUEB: *mut u8 = 0x2E as *mut u8;

/// Port A Input Pins.
pub const PINA: *mut u8 = 0x2F as *mut u8;

/// Data Direction Register, Port A.
pub const DDRA: *mut u8 = 0x30 as *mut u8;

/// Port A Data Register.
pub const PORTA: *mut u8 = 0x31 as *mut u8;

/// Pull-up Enable Control Register.
pub const PUEA: *mut u8 = 0x32 as *mut u8;

/// Port Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BBMA | 1 |
pub const PORTCR: *mut u8 = 0x33 as *mut u8;

/// General Purpose I/O Register 0.
pub const GPIOR0: *mut u8 = 0x34 as *mut u8;

/// General Purpose I/O Register 1.
pub const GPIOR1: *mut u8 = 0x35 as *mut u8;

/// General Purpose I/O Register 2.
pub const GPIOR2: *mut u8 = 0x36 as *mut u8;

/// Timer/Counter0 Output Compare Register.
pub const OCR0B: *mut u8 = 0x37 as *mut u8;

/// Timer/Counter0 Output Compare Register.
pub const OCR0A: *mut u8 = 0x38 as *mut u8;

/// Timer/Counter0.
pub const TCNT0: *mut u8 = 0x39 as *mut u8;

/// Timer/Counter Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FOC0A | 10000000 |
/// | WGM02 | 1000 |
/// | CS0 | 111 |
/// | FOC0B | 1000000 |
pub const TCCR0B: *mut u8 = 0x3A as *mut u8;

/// Timer/Counter  Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | COM0A | 11000000 |
/// | WGM0 | 11 |
/// | COM0B | 110000 |
pub const TCCR0A: *mut u8 = 0x3B as *mut u8;

/// EEPROM Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EEPE | 10 |
/// | EERE | 1 |
/// | EEPM | 110000 |
/// | EERIE | 1000 |
/// | EEMPE | 100 |
pub const EECR: *mut u8 = 0x3C as *mut u8;

/// EEPROM Data Register.
pub const EEDR: *mut u8 = 0x3D as *mut u8;

/// EEPROM Read/Write Access.
pub const EEAR: *mut u16 = 0x3E as *mut u16;

/// EEPROM Read/Write Access low byte.
pub const EEARL: *mut u8 = 0x3E as *mut u8;

/// EEPROM Read/Write Access high byte.
pub const EEARH: *mut u8 = 0x3F as *mut u8;

/// USART I/O Data Register.
pub const UDR0: *mut u8 = 0x40 as *mut u8;

/// USART Baud Rate Register  Bytes.
pub const UBRR0: *mut u16 = 0x41 as *mut u16;

/// USART Baud Rate Register  Bytes low byte.
pub const UBRR0L: *mut u8 = 0x41 as *mut u8;

/// USART Baud Rate Register  Bytes high byte.
pub const UBRR0H: *mut u8 = 0x42 as *mut u8;

/// USART Control and Status Register D.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SFDE0 | 100000 |
/// | RXSIE0 | 10000000 |
/// | RXS0 | 1000000 |
pub const UCSR0D: *mut u8 = 0x43 as *mut u8;

/// USART Control and Status Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | UCPOL0 | 1 |
/// | USBS0 | 1000 |
/// | UCSZ0 | 110 |
/// | UPM0 | 110000 |
/// | UMSEL0 | 11000000 |
pub const UCSR0C: *mut u8 = 0x44 as *mut u8;

/// USART Control and Status Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RXB80 | 10 |
/// | TXCIE0 | 1000000 |
/// | RXEN0 | 10000 |
/// | TXEN0 | 1000 |
/// | RXCIE0 | 10000000 |
/// | UCSZ02 | 100 |
/// | UDRIE0 | 100000 |
/// | TXB80 | 1 |
pub const UCSR0B: *mut u8 = 0x45 as *mut u8;

/// USART Control and Status Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DOR0 | 1000 |
/// | UPE0 | 100 |
/// | FE0 | 10000 |
/// | TXC0 | 1000000 |
/// | MPCM0 | 1 |
/// | UDRE0 | 100000 |
/// | U2X0 | 10 |
/// | RXC0 | 10000000 |
pub const UCSR0A: *mut u8 = 0x46 as *mut u8;

/// Pin Change Mask Register 0.
pub const PCMSK0: *mut u8 = 0x47 as *mut u8;

/// Pin Change Mask Register 1.
pub const PCMSK1: *mut u8 = 0x48 as *mut u8;

/// Pin Change Mask Register 2.
pub const PCMSK2: *mut u8 = 0x49 as *mut u8;

/// USI Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | USITC | 1 |
/// | USIWM | 110000 |
/// | USIOIE | 1000000 |
/// | USICLK | 10 |
/// | USICS | 1100 |
/// | USISIE | 10000000 |
pub const USICR: *mut u8 = 0x4A as *mut u8;

/// USI Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | USICNT | 1111 |
/// | USIPF | 100000 |
/// | USISIF | 10000000 |
/// | USIOIF | 1000000 |
/// | USIDC | 10000 |
pub const USISR: *mut u8 = 0x4B as *mut u8;

/// USI Data Register.
pub const USIDR: *mut u8 = 0x4C as *mut u8;

/// USI Buffer Register.
pub const USIBR: *mut u8 = 0x4D as *mut u8;

/// Configuration Change Protection.
pub const CCP: *mut u8 = 0x4F as *mut u8;

/// Watchdog Timer Control and Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDIF | 10000000 |
/// | WDP | 100111 |
/// | WDIE | 1000000 |
/// | WDE | 1000 |
pub const WDTCSR: *mut u8 = 0x50 as *mut u8;

/// Clock Setting Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OSCRDY | 10000000 |
/// | CSTR | 1000000 |
/// | CKSEL | 1111 |
/// | SUT | 10000 |
/// | CKOUT_IO | 100000 |
pub const CLKSR: *mut u8 = 0x52 as *mut u8;

/// Clock Prescale Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CLKPS | 1111 |
pub const CLKPR: *mut u8 = 0x53 as *mut u8;

/// Power Reduction Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRUSART1 | 100 |
/// | PRADC | 1 |
/// | PRTIM1 | 100000 |
/// | PRUSART0 | 10 |
/// | PRTIM0 | 10000 |
/// | PRTWI | 1000000 |
/// | PRUSI | 1000 |
pub const PRR: *mut u8 = 0x54 as *mut u8;

/// MCU Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDRF | 1000 |
/// | PORF | 1 |
/// | EXTRF | 10 |
/// | BORF | 100 |
pub const MCUSR: *mut u8 = 0x55 as *mut u8;

/// MCU Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ISC0 | 11 |
/// | SM | 1100000 |
/// | SE | 10000 |
pub const MCUCR: *mut u8 = 0x56 as *mut u8;

/// Store Program Memory Control and Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RFLB | 1000 |
/// | PGERS | 10 |
/// | PGWRT | 100 |
/// | RSIG | 100000 |
/// | SPMEN | 1 |
/// | CTPB | 10000 |
pub const SPMCSR: *mut u8 = 0x57 as *mut u8;

/// Timer/Counter Interrupt Flag register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TOV0 | 10 |
/// | OCF0A | 1 |
/// | OCF0B | 100 |
pub const TIFR: *mut u8 = 0x59 as *mut u8;

/// Timer/Counter Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TOIE0 | 10 |
/// | OCIE0B | 100 |
/// | OCIE0A | 1 |
pub const TIMSK: *mut u8 = 0x5A as *mut u8;

/// General Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | INTF0 | 1000000 |
/// | PCIF | 111000 |
pub const GIFR: *mut u8 = 0x5B as *mut u8;

/// General Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | INT0 | 1000000 |
/// | PCIE | 111000 |
pub const GIMSK: *mut u8 = 0x5C as *mut u8;

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
/// | H | 100000 |
/// | V | 1000 |
/// | Z | 10 |
/// | I | 10000000 |
/// | S | 10000 |
/// | C | 1 |
/// | T | 1000000 |
/// | N | 100 |
pub const SREG: *mut u8 = 0x5F as *mut u8;

/// Digital Input Disable Register 0.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AIN1D | 100 |
/// | ADC3D | 1000000 |
/// | ADC1D | 10000 |
/// | ADC4D | 10000000 |
/// | AREFD | 1 |
/// | ADC2D | 100000 |
/// | AIN0D | 10 |
/// | ADC0D | 1000 |
pub const DIDR0: *mut u8 = 0x60 as *mut u8;

/// Digital Input Disable Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADC8D | 1000 |
/// | ADC5D | 1 |
/// | ADC6D | 10 |
/// | ADC7D | 100 |
pub const DIDR1: *mut u8 = 0x61 as *mut u8;

/// Digital Input Disable Register 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADC9D | 1 |
/// | ADC11D | 100 |
/// | ADC10D | 10 |
pub const DIDR2: *mut u8 = 0x62 as *mut u8;

/// Oscillator Calibration Value.
pub const OSCCAL0: *mut u8 = 0x63 as *mut u8;

/// `OSCTCAL0A` register
pub const OSCTCAL0A: *mut u8 = 0x64 as *mut u8;

/// `OSCTCAL0B` register
pub const OSCTCAL0B: *mut u8 = 0x65 as *mut u8;

/// `OSCCAL1` register
pub const OSCCAL1: *mut u8 = 0x66 as *mut u8;

/// General Timer/Counter Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TSM | 10000000 |
/// | PSR10 | 1 |
pub const GTCCR: *mut u8 = 0x67 as *mut u8;

/// Timer/Counter1 Input Capture Register  Bytes low byte.
pub const ICR1L: *mut u8 = 0x68 as *mut u8;

/// Timer/Counter1 Input Capture Register  Bytes.
pub const ICR1: *mut u16 = 0x68 as *mut u16;

/// Timer/Counter1 Input Capture Register  Bytes high byte.
pub const ICR1H: *mut u8 = 0x69 as *mut u8;

/// Timer/Counter1 Output Compare Register  Bytes low byte.
pub const OCR1BL: *mut u8 = 0x6A as *mut u8;

/// Timer/Counter1 Output Compare Register  Bytes.
pub const OCR1B: *mut u16 = 0x6A as *mut u16;

/// Timer/Counter1 Output Compare Register  Bytes high byte.
pub const OCR1BH: *mut u8 = 0x6B as *mut u8;

/// Timer/Counter1 Output Compare Register  Bytes low byte.
pub const OCR1AL: *mut u8 = 0x6C as *mut u8;

/// Timer/Counter1 Output Compare Register  Bytes.
pub const OCR1A: *mut u16 = 0x6C as *mut u16;

/// Timer/Counter1 Output Compare Register  Bytes high byte.
pub const OCR1AH: *mut u8 = 0x6D as *mut u8;

/// Timer/Counter1  Bytes.
pub const TCNT1: *mut u16 = 0x6E as *mut u16;

/// Timer/Counter1  Bytes low byte.
pub const TCNT1L: *mut u8 = 0x6E as *mut u8;

/// Timer/Counter1  Bytes high byte.
pub const TCNT1H: *mut u8 = 0x6F as *mut u8;

/// Timer/Counter1 Control Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FOC1A | 10000000 |
/// | FOC1B | 1000000 |
pub const TCCR1C: *mut u8 = 0x70 as *mut u8;

/// Timer/Counter1 Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ICES1 | 1000000 |
/// | ICNC1 | 10000000 |
/// | CS1 | 111 |
pub const TCCR1B: *mut u8 = 0x71 as *mut u8;

/// Timer/Counter1 Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | COM1A | 11000000 |
/// | COM1B | 110000 |
pub const TCCR1A: *mut u8 = 0x72 as *mut u8;

/// USART I/O Data Register.
pub const UDR1: *mut u8 = 0x73 as *mut u8;

/// USART Baud Rate Register  Bytes low byte.
pub const UBRR1L: *mut u8 = 0x74 as *mut u8;

/// USART Baud Rate Register  Bytes.
pub const UBRR1: *mut u16 = 0x74 as *mut u16;

/// USART Baud Rate Register  Bytes high byte.
pub const UBRR1H: *mut u8 = 0x75 as *mut u8;

/// USART Control and Status Register D.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SFDE1 | 100000 |
/// | RXS1 | 1000000 |
/// | RXSIE1 | 10000000 |
pub const UCSR1D: *mut u8 = 0x76 as *mut u8;

/// USART Control and Status Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | UCSZ1 | 110 |
/// | UCPOL1 | 1 |
/// | UMSEL1 | 11000000 |
/// | UPM1 | 110000 |
/// | USBS1 | 1000 |
pub const UCSR1C: *mut u8 = 0x77 as *mut u8;

/// USART Control and Status Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RXEN1 | 10000 |
/// | TXEN1 | 1000 |
/// | UCSZ12 | 100 |
/// | UDRIE1 | 100000 |
/// | TXCIE1 | 1000000 |
/// | RXB81 | 10 |
/// | RXCIE1 | 10000000 |
/// | TXB81 | 1 |
pub const UCSR1B: *mut u8 = 0x78 as *mut u8;

/// USART Control and Status Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | U2X1 | 10 |
/// | UPE1 | 100 |
/// | UDRE1 | 100000 |
/// | TXC1 | 1000000 |
/// | RXC1 | 10000000 |
/// | DOR1 | 1000 |
/// | MPCM1 | 1 |
/// | FE1 | 10000 |
pub const UCSR1A: *mut u8 = 0x79 as *mut u8;

/// TWI Slave Data Register.
pub const TWSD: *mut u8 = 0x7A as *mut u8;

/// TWI Slave Address Mask Register.
pub const TWSAM: *mut u8 = 0x7B as *mut u8;

/// TWI Slave Address Register.
pub const TWSA: *mut u8 = 0x7C as *mut u8;

/// TWI Slave Status Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TWDIF | 10000000 |
/// | TWRA | 10000 |
/// | TWCH | 100000 |
/// | TWASIF | 1000000 |
/// | TWBE | 100 |
/// | TWC | 1000 |
/// | TWDIR | 10 |
/// | TWAS | 1 |
pub const TWSSRA: *mut u8 = 0x7D as *mut u8;

/// TWI Slave Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TWCMD | 11 |
/// | TWAA | 100 |
pub const TWSCRB: *mut u8 = 0x7E as *mut u8;

/// TWI Slave Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TWASIE | 10000 |
/// | TWSIE | 100 |
/// | TWPME | 10 |
/// | TWEN | 1000 |
/// | TWSME | 1 |
/// | TWSHE | 10000000 |
/// | TWDIE | 100000 |
pub const TWSCRA: *mut u8 = 0x7F as *mut u8;

/// Bitfield on register `ACSRA`
pub const ACI: u8 = 0x10;

/// Bitfield on register `ACSRA`
pub const ACIS: u8 = 0x3;

/// Bitfield on register `ACSRA`
pub const ACBG: u8 = 0x40;

/// Bitfield on register `ACSRA`
pub const ACIE: u8 = 0x8;

/// Bitfield on register `ACSRA`
pub const ACIC: u8 = 0x4;

/// Bitfield on register `ACSRA`
pub const ACD: u8 = 0x80;

/// Bitfield on register `ACSRA`
pub const ACO: u8 = 0x20;

/// Bitfield on register `ACSRB`
pub const ACME: u8 = 0x4;

/// Bitfield on register `ACSRB`
pub const HSEL: u8 = 0x80;

/// Bitfield on register `ACSRB`
pub const ACLP: u8 = 0x20;

/// Bitfield on register `ACSRB`
pub const ACIRS: u8 = 0x3;

/// Bitfield on register `ACSRB`
pub const HLEV: u8 = 0x40;

/// Bitfield on register `ACSRB`
pub const ACCE: u8 = 0x8;

/// Bitfield on register `ADCSRA`
pub const ADSC: u8 = 0x40;

/// Bitfield on register `ADCSRA`
pub const ADEN: u8 = 0x80;

/// Bitfield on register `ADCSRA`
pub const ADIF: u8 = 0x10;

/// Bitfield on register `ADCSRA`
pub const ADIE: u8 = 0x8;

/// Bitfield on register `ADCSRA`
pub const ADPS: u8 = 0x7;

/// Bitfield on register `ADCSRA`
pub const ADATE: u8 = 0x20;

/// Bitfield on register `ADCSRB`
pub const ADLAR: u8 = 0x8;

/// Bitfield on register `ADCSRB`
pub const ADTS: u8 = 0x7;

/// Bitfield on register `ADCSRB`
pub const VDEN: u8 = 0x80;

/// Bitfield on register `ADCSRB`
pub const VDPD: u8 = 0x40;

/// Bitfield on register `ADMUX`
pub const MUX: u8 = 0xF;

/// Bitfield on register `ADMUX`
pub const REFEN: u8 = 0x20;

/// Bitfield on register `ADMUX`
pub const ADC0EN: u8 = 0x10;

/// Bitfield on register `ADMUX`
pub const REFS: u8 = 0xC0;

/// Bitfield on register `CLKPR`
pub const CLKPS: u8 = 0xF;

/// Bitfield on register `CLKSR`
pub const OSCRDY: u8 = 0x80;

/// Bitfield on register `CLKSR`
pub const CSTR: u8 = 0x40;

/// Bitfield on register `CLKSR`
pub const CKSEL: u8 = 0xF;

/// Bitfield on register `CLKSR`
pub const SUT: u8 = 0x10;

/// Bitfield on register `CLKSR`
pub const CKOUT_IO: u8 = 0x20;

/// Bitfield on register `DIDR0`
pub const AIN1D: u8 = 0x4;

/// Bitfield on register `DIDR0`
pub const ADC3D: u8 = 0x40;

/// Bitfield on register `DIDR0`
pub const ADC1D: u8 = 0x10;

/// Bitfield on register `DIDR0`
pub const ADC4D: u8 = 0x80;

/// Bitfield on register `DIDR0`
pub const AREFD: u8 = 0x1;

/// Bitfield on register `DIDR0`
pub const ADC2D: u8 = 0x20;

/// Bitfield on register `DIDR0`
pub const AIN0D: u8 = 0x2;

/// Bitfield on register `DIDR0`
pub const ADC0D: u8 = 0x8;

/// Bitfield on register `DIDR1`
pub const ADC8D: u8 = 0x8;

/// Bitfield on register `DIDR1`
pub const ADC5D: u8 = 0x1;

/// Bitfield on register `DIDR1`
pub const ADC6D: u8 = 0x2;

/// Bitfield on register `DIDR1`
pub const ADC7D: u8 = 0x4;

/// Bitfield on register `DIDR2`
pub const ADC9D: u8 = 0x1;

/// Bitfield on register `DIDR2`
pub const ADC11D: u8 = 0x4;

/// Bitfield on register `DIDR2`
pub const ADC10D: u8 = 0x2;

/// Bitfield on register `EECR`
pub const EEPE: u8 = 0x2;

/// Bitfield on register `EECR`
pub const EERE: u8 = 0x1;

/// Bitfield on register `EECR`
pub const EEPM: u8 = 0x30;

/// Bitfield on register `EECR`
pub const EERIE: u8 = 0x8;

/// Bitfield on register `EECR`
pub const EEMPE: u8 = 0x4;

/// Bitfield on register `EXTENDED`
pub const BODACT: u8 = 0x6;

/// Bitfield on register `EXTENDED`
pub const SELFPRGEN: u8 = 0x1;

/// Bitfield on register `EXTENDED`
pub const BODPD: u8 = 0x18;

/// Bitfield on register `GIFR`
pub const INTF0: u8 = 0x40;

/// Bitfield on register `GIFR`
pub const PCIF: u8 = 0x38;

/// Bitfield on register `GIMSK`
pub const INT0: u8 = 0x40;

/// Bitfield on register `GIMSK`
pub const PCIE: u8 = 0x38;

/// Bitfield on register `GTCCR`
pub const TSM: u8 = 0x80;

/// Bitfield on register `GTCCR`
pub const PSR10: u8 = 0x1;

/// Bitfield on register `HIGH`
pub const SPIEN: u8 = 0x20;

/// Bitfield on register `HIGH`
pub const RSTDISBL: u8 = 0x80;

/// Bitfield on register `HIGH`
pub const DWEN: u8 = 0x40;

/// Bitfield on register `HIGH`
pub const BODLEVEL: u8 = 0x7;

/// Bitfield on register `HIGH`
pub const WDTON: u8 = 0x10;

/// Bitfield on register `HIGH`
pub const EESAVE: u8 = 0x8;

/// Bitfield on register `LOCKBIT`
pub const LB: u8 = 0x3;

/// Bitfield on register `LOW`
pub const CKDIV8: u8 = 0x80;

/// Bitfield on register `LOW`
pub const SUT_CKSEL: u8 = 0x1F;

/// Bitfield on register `LOW`
pub const CKOUT: u8 = 0x40;

/// Bitfield on register `MCUCR`
pub const ISC0: u8 = 0x3;

/// Bitfield on register `MCUCR`
pub const SM: u8 = 0x60;

/// Bitfield on register `MCUCR`
pub const SE: u8 = 0x10;

/// Bitfield on register `MCUSR`
pub const WDRF: u8 = 0x8;

/// Bitfield on register `MCUSR`
pub const PORF: u8 = 0x1;

/// Bitfield on register `MCUSR`
pub const EXTRF: u8 = 0x2;

/// Bitfield on register `MCUSR`
pub const BORF: u8 = 0x4;

/// Bitfield on register `PORTCR`
pub const BBMA: u8 = 0x1;

/// Bitfield on register `PRR`
pub const PRUSART1: u8 = 0x4;

/// Bitfield on register `PRR`
pub const PRADC: u8 = 0x1;

/// Bitfield on register `PRR`
pub const PRTIM1: u8 = 0x20;

/// Bitfield on register `PRR`
pub const PRUSART0: u8 = 0x2;

/// Bitfield on register `PRR`
pub const PRTIM0: u8 = 0x10;

/// Bitfield on register `PRR`
pub const PRTWI: u8 = 0x40;

/// Bitfield on register `PRR`
pub const PRUSI: u8 = 0x8;

/// Bitfield on register `SPMCSR`
pub const RFLB: u8 = 0x8;

/// Bitfield on register `SPMCSR`
pub const PGERS: u8 = 0x2;

/// Bitfield on register `SPMCSR`
pub const PGWRT: u8 = 0x4;

/// Bitfield on register `SPMCSR`
pub const RSIG: u8 = 0x20;

/// Bitfield on register `SPMCSR`
pub const SPMEN: u8 = 0x1;

/// Bitfield on register `SPMCSR`
pub const CTPB: u8 = 0x10;

/// Bitfield on register `SREG`
pub const H: u8 = 0x20;

/// Bitfield on register `SREG`
pub const V: u8 = 0x8;

/// Bitfield on register `SREG`
pub const Z: u8 = 0x2;

/// Bitfield on register `SREG`
pub const I: u8 = 0x80;

/// Bitfield on register `SREG`
pub const S: u8 = 0x10;

/// Bitfield on register `SREG`
pub const C: u8 = 0x1;

/// Bitfield on register `SREG`
pub const T: u8 = 0x40;

/// Bitfield on register `SREG`
pub const N: u8 = 0x4;

/// Bitfield on register `TCCR0A`
pub const COM0A: u8 = 0xC0;

/// Bitfield on register `TCCR0A`
pub const WGM0: u8 = 0x3;

/// Bitfield on register `TCCR0A`
pub const COM0B: u8 = 0x30;

/// Bitfield on register `TCCR0B`
pub const FOC0A: u8 = 0x80;

/// Bitfield on register `TCCR0B`
pub const WGM02: u8 = 0x8;

/// Bitfield on register `TCCR0B`
pub const CS0: u8 = 0x7;

/// Bitfield on register `TCCR0B`
pub const FOC0B: u8 = 0x40;

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
pub const TOV0: u8 = 0x2;

/// Bitfield on register `TIFR`
pub const OCF0A: u8 = 0x1;

/// Bitfield on register `TIFR`
pub const OCF0B: u8 = 0x4;

/// Bitfield on register `TIMSK`
pub const TOIE0: u8 = 0x2;

/// Bitfield on register `TIMSK`
pub const OCIE0B: u8 = 0x4;

/// Bitfield on register `TIMSK`
pub const OCIE0A: u8 = 0x1;

/// Bitfield on register `TWSCRA`
pub const TWASIE: u8 = 0x10;

/// Bitfield on register `TWSCRA`
pub const TWSIE: u8 = 0x4;

/// Bitfield on register `TWSCRA`
pub const TWPME: u8 = 0x2;

/// Bitfield on register `TWSCRA`
pub const TWEN: u8 = 0x8;

/// Bitfield on register `TWSCRA`
pub const TWSME: u8 = 0x1;

/// Bitfield on register `TWSCRA`
pub const TWSHE: u8 = 0x80;

/// Bitfield on register `TWSCRA`
pub const TWDIE: u8 = 0x20;

/// Bitfield on register `TWSCRB`
pub const TWCMD: u8 = 0x3;

/// Bitfield on register `TWSCRB`
pub const TWAA: u8 = 0x4;

/// Bitfield on register `TWSSRA`
pub const TWDIF: u8 = 0x80;

/// Bitfield on register `TWSSRA`
pub const TWRA: u8 = 0x10;

/// Bitfield on register `TWSSRA`
pub const TWCH: u8 = 0x20;

/// Bitfield on register `TWSSRA`
pub const TWASIF: u8 = 0x40;

/// Bitfield on register `TWSSRA`
pub const TWBE: u8 = 0x4;

/// Bitfield on register `TWSSRA`
pub const TWC: u8 = 0x8;

/// Bitfield on register `TWSSRA`
pub const TWDIR: u8 = 0x2;

/// Bitfield on register `TWSSRA`
pub const TWAS: u8 = 0x1;

/// Bitfield on register `UCSR0A`
pub const DOR0: u8 = 0x8;

/// Bitfield on register `UCSR0A`
pub const UPE0: u8 = 0x4;

/// Bitfield on register `UCSR0A`
pub const FE0: u8 = 0x10;

/// Bitfield on register `UCSR0A`
pub const TXC0: u8 = 0x40;

/// Bitfield on register `UCSR0A`
pub const MPCM0: u8 = 0x1;

/// Bitfield on register `UCSR0A`
pub const UDRE0: u8 = 0x20;

/// Bitfield on register `UCSR0A`
pub const U2X0: u8 = 0x2;

/// Bitfield on register `UCSR0A`
pub const RXC0: u8 = 0x80;

/// Bitfield on register `UCSR0B`
pub const RXB80: u8 = 0x2;

/// Bitfield on register `UCSR0B`
pub const TXCIE0: u8 = 0x40;

/// Bitfield on register `UCSR0B`
pub const RXEN0: u8 = 0x10;

/// Bitfield on register `UCSR0B`
pub const TXEN0: u8 = 0x8;

/// Bitfield on register `UCSR0B`
pub const RXCIE0: u8 = 0x80;

/// Bitfield on register `UCSR0B`
pub const UCSZ02: u8 = 0x4;

/// Bitfield on register `UCSR0B`
pub const UDRIE0: u8 = 0x20;

/// Bitfield on register `UCSR0B`
pub const TXB80: u8 = 0x1;

/// Bitfield on register `UCSR0C`
pub const UCPOL0: u8 = 0x1;

/// Bitfield on register `UCSR0C`
pub const USBS0: u8 = 0x8;

/// Bitfield on register `UCSR0C`
pub const UCSZ0: u8 = 0x6;

/// Bitfield on register `UCSR0C`
pub const UPM0: u8 = 0x30;

/// Bitfield on register `UCSR0C`
pub const UMSEL0: u8 = 0xC0;

/// Bitfield on register `UCSR0D`
pub const SFDE0: u8 = 0x20;

/// Bitfield on register `UCSR0D`
pub const RXSIE0: u8 = 0x80;

/// Bitfield on register `UCSR0D`
pub const RXS0: u8 = 0x40;

/// Bitfield on register `UCSR1A`
pub const U2X1: u8 = 0x2;

/// Bitfield on register `UCSR1A`
pub const UPE1: u8 = 0x4;

/// Bitfield on register `UCSR1A`
pub const UDRE1: u8 = 0x20;

/// Bitfield on register `UCSR1A`
pub const TXC1: u8 = 0x40;

/// Bitfield on register `UCSR1A`
pub const RXC1: u8 = 0x80;

/// Bitfield on register `UCSR1A`
pub const DOR1: u8 = 0x8;

/// Bitfield on register `UCSR1A`
pub const MPCM1: u8 = 0x1;

/// Bitfield on register `UCSR1A`
pub const FE1: u8 = 0x10;

/// Bitfield on register `UCSR1B`
pub const RXEN1: u8 = 0x10;

/// Bitfield on register `UCSR1B`
pub const TXEN1: u8 = 0x8;

/// Bitfield on register `UCSR1B`
pub const UCSZ12: u8 = 0x4;

/// Bitfield on register `UCSR1B`
pub const UDRIE1: u8 = 0x20;

/// Bitfield on register `UCSR1B`
pub const TXCIE1: u8 = 0x40;

/// Bitfield on register `UCSR1B`
pub const RXB81: u8 = 0x2;

/// Bitfield on register `UCSR1B`
pub const RXCIE1: u8 = 0x80;

/// Bitfield on register `UCSR1B`
pub const TXB81: u8 = 0x1;

/// Bitfield on register `UCSR1C`
pub const UCSZ1: u8 = 0x6;

/// Bitfield on register `UCSR1C`
pub const UCPOL1: u8 = 0x1;

/// Bitfield on register `UCSR1C`
pub const UMSEL1: u8 = 0xC0;

/// Bitfield on register `UCSR1C`
pub const UPM1: u8 = 0x30;

/// Bitfield on register `UCSR1C`
pub const USBS1: u8 = 0x8;

/// Bitfield on register `UCSR1D`
pub const SFDE1: u8 = 0x20;

/// Bitfield on register `UCSR1D`
pub const RXS1: u8 = 0x40;

/// Bitfield on register `UCSR1D`
pub const RXSIE1: u8 = 0x80;

/// Bitfield on register `USICR`
pub const USITC: u8 = 0x1;

/// Bitfield on register `USICR`
pub const USIWM: u8 = 0x30;

/// Bitfield on register `USICR`
pub const USIOIE: u8 = 0x40;

/// Bitfield on register `USICR`
pub const USICLK: u8 = 0x2;

/// Bitfield on register `USICR`
pub const USICS: u8 = 0xC;

/// Bitfield on register `USICR`
pub const USISIE: u8 = 0x80;

/// Bitfield on register `USISR`
pub const USICNT: u8 = 0xF;

/// Bitfield on register `USISR`
pub const USIPF: u8 = 0x20;

/// Bitfield on register `USISR`
pub const USISIF: u8 = 0x80;

/// Bitfield on register `USISR`
pub const USIOIF: u8 = 0x40;

/// Bitfield on register `USISR`
pub const USIDC: u8 = 0x10;

/// Bitfield on register `WDTCSR`
pub const WDIF: u8 = 0x80;

/// Bitfield on register `WDTCSR`
pub const WDP: u8 = 0x27;

/// Bitfield on register `WDTCSR`
pub const WDIE: u8 = 0x40;

/// Bitfield on register `WDTCSR`
pub const WDE: u8 = 0x8;

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

/// `CPU_SLEEP_MODE` value group
#[allow(non_upper_case_globals)]
pub mod cpu_sleep_mode {
   /// Idle.
   pub const IDLE: u32 = 0x0;
   /// ADC Noise Reduction (If Available).
   pub const ADC: u32 = 0x1;
   /// Power Down.
   pub const PDOWN: u32 = 0x2;
   /// Standby.
   pub const STDBY: u32 = 0x3;
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
   /// Ext. Clock; Start-up time PWRDWN/RESET: 6 CK/16 CK + 16 ms.
   pub const EXTCLK_6CK_16CK_16MS: u32 = 0x0;
   /// Int. RC Osc. 8 MHz; Start-up time PWRDWN/RESET: 6 CK/16 CK + 16 ms.
   pub const INTRCOSC_8MHZ_6CK_16CK_16MS: u32 = 0x2;
   /// Int. ULP Osc. 32 kHz; Start-up time PWRDWN/RESET: 6 CK/16 CK + 16 ms.
   pub const INTULPOSC_32KHZ_6CK_16CK_16MS: u32 = 0x4;
   /// Ext. Ceramic Res. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 258 CK/16 CK + 16 ms.
   pub const EXTCRES_0MHZ4_0MHZ9_258CK_16CK_16MS: u32 = 0x8;
   /// Ext. Ceramic Res. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 1K CK/16 CK + 16 ms.
   pub const EXTCRES_0MHZ4_0MHZ9_1KCK_16CK_16MS: u32 = 0x18;
   /// Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 16 K CK/16 CK + 16 ms.
   pub const EXTXOSC_0MHZ4_0MHZ9_16KCK_16CK_16MS: u32 = 0x9;
   /// Ext. Ceramic Res. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 258 CK/16 CK + 16 ms.
   pub const EXTCRES_0MHZ9_3MHZ_258CK_16CK_16MS: u32 = 0xA;
   /// Ext. Ceramic Res. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 1K CK/16 CK + 16 ms.
   pub const EXTCRES_0MHZ9_3MHZ_1KCK_16CK_16MS: u32 = 0x1A;
   /// Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 16 K CK/16 CK + 16 ms.
   pub const EXTXOSC_0MHZ9_3MHZ_16KCK_16CK_16MS: u32 = 0xB;
   /// Ext. Ceramic Res. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 258 CK/16 CK + 16 ms.
   pub const EXTCRES_3MHZ_8MHZ_258CK_16CK_16MS: u32 = 0xC;
   /// Ext. Ceramic Res. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 1K CK/16 CK + 16 ms.
   pub const EXTCRES_3MHZ_8MHZ_1KCK_16CK_16MS: u32 = 0x1C;
   /// Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 16 K CK/16 CK + 16 ms.
   pub const EXTXOSC_3MHZ_8MHZ_16KCK_16CK_16MS: u32 = 0xD;
   /// Ext. Ceramic Res. 8.0- MHz; Start-up time PWRDWN/RESET: 258 CK/16 CK + 16 ms.
   pub const EXTCRES_8MHZ_XX_258CK_16CK_16MS: u32 = 0xE;
   /// Ext. Ceramic Res. 8.0- MHz; Start-up time PWRDWN/RESET: 1K CK/16 CK + 16 ms.
   pub const EXTCRES_8MHZ_XX_1KCK_16CK_16MS: u32 = 0x1E;
   /// Ext. Crystal Osc. 8.0- MHz; Start-up time PWRDWN/RESET: 16 K CK/16 CK + 16 ms.
   pub const EXTXOSC_8MHZ_XX_16KCK_16CK_16MS: u32 = 0xF;
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

