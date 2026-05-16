//! The AVR ATmega32U2 microcontroller
//!
//! # Variants
//! |        | Pinout | Mcu age | Operating temperature | Operating voltage | Max speed |
//! |--------|--------|---------|-----------------------|-------------------|-----------|
//! | ATmega32U2-AU | QUAD | TQFP32 | -40°C - 85°C | 2.7V - 5.5V | 16 MHz |
//! | ATmega32U2-MU | QUAD | QFN32 | -40°C - 85°C | 2.7V - 5.5V | 16 MHz |
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
/// | SUT_CKSEL | 111111 |
pub const LOW: *mut u8 = 0x0 as *mut u8;

/// `LOCKBIT` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BLB0 | 1100 |
/// | BLB1 | 110000 |
/// | LB | 11 |
pub const LOCKBIT: *mut u8 = 0x0 as *mut u8;

/// `HIGH` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DWEN | 10000000 |
/// | BOOTRST | 1 |
/// | SPIEN | 100000 |
/// | EESAVE | 1000 |
/// | WDTON | 10000 |
/// | RSTDISBL | 1000000 |
/// | BOOTSZ | 110 |
pub const HIGH: *mut u8 = 0x1 as *mut u8;

/// `EXTENDED` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BODLEVEL | 111 |
/// | HWBE | 1000 |
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
/// | OCF1A | 10 |
/// | TOV1 | 1 |
/// | OCF1B | 100 |
/// | ICF1 | 100000 |
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
/// | GPIOR03 | 1000 |
/// | GPIOR06 | 1000000 |
/// | GPIOR01 | 10 |
/// | GPIOR05 | 100000 |
/// | GPIOR00 | 1 |
/// | GPIOR02 | 100 |
pub const GPIOR0: *mut u8 = 0x3E as *mut u8;

/// EEPROM Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EEPM | 110000 |
/// | EEPE | 10 |
/// | EERE | 1 |
/// | EEMPE | 100 |
/// | EERIE | 1000 |
pub const EECR: *mut u8 = 0x3F as *mut u8;

/// EEPROM Data Register.
pub const EEDR: *mut u8 = 0x40 as *mut u8;

/// EEPROM Address Register Low Bytes low byte.
pub const EEARL: *mut u8 = 0x41 as *mut u8;

/// EEPROM Address Register Low Bytes.
pub const EEAR: *mut u16 = 0x41 as *mut u16;

/// EEPROM Address Register Low Bytes high byte.
pub const EEARH: *mut u8 = 0x42 as *mut u8;

/// General Timer/Counter Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TSM | 10000000 |
/// | PSRSYNC | 1 |
pub const GTCCR: *mut u8 = 0x43 as *mut u8;

/// Timer/Counter  Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | COM0A | 11000000 |
/// | COM0B | 110000 |
/// | WGM0 | 11 |
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

/// Timer/Counter0 Output Compare Register.
pub const OCR0A: *mut u8 = 0x47 as *mut u8;

/// Timer/Counter0 Output Compare Register.
pub const OCR0B: *mut u8 = 0x48 as *mut u8;

/// PLL Status and Control register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PLLE | 10 |
/// | PLLP | 11100 |
/// | PLOCK | 1 |
pub const PLLCSR: *mut u8 = 0x49 as *mut u8;

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
/// | DORD | 100000 |
/// | SPIE | 10000000 |
/// | CPOL | 1000 |
/// | SPR | 11 |
/// | SPE | 1000000 |
/// | MSTR | 10000 |
/// | CPHA | 100 |
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
/// | ACIC | 100 |
/// | ACIS | 11 |
/// | ACBG | 1000000 |
/// | ACO | 100000 |
/// | ACI | 10000 |
/// | ACIE | 1000 |
/// | ACD | 10000000 |
pub const ACSR: *mut u8 = 0x50 as *mut u8;

/// debugWire communication register.
pub const DWDR: *mut u8 = 0x51 as *mut u8;

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
/// | WDRF | 1000 |
/// | PORF | 1 |
/// | USBRF | 100000 |
/// | EXTRF | 10 |
/// | BORF | 100 |
pub const MCUSR: *mut u8 = 0x54 as *mut u8;

/// MCU Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | IVSEL | 10 |
/// | IVCE | 1 |
/// | PUD | 10000 |
pub const MCUCR: *mut u8 = 0x55 as *mut u8;

/// Store Program Memory Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PGERS | 10 |
/// | SIGRD | 100000 |
/// | PGWRT | 100 |
/// | SPMIE | 10000000 |
/// | SPMEN | 1 |
/// | RWWSB | 1000000 |
/// | RWWSRE | 10000 |
/// | BLBSET | 1000 |
pub const SPMCSR: *mut u8 = 0x57 as *mut u8;

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
/// | V | 1000 |
/// | T | 1000000 |
/// | S | 10000 |
/// | N | 100 |
/// | Z | 10 |
/// | C | 1 |
/// | H | 100000 |
/// | I | 10000000 |
pub const SREG: *mut u8 = 0x5F as *mut u8;

/// Watchdog Timer Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDCE | 10000 |
/// | WDP | 100111 |
/// | WDE | 1000 |
/// | WDIE | 1000000 |
/// | WDIF | 10000000 |
pub const WDTCSR: *mut u8 = 0x60 as *mut u8;

/// `CLKPR` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CLKPS | 1111 |
/// | CLKPCE | 10000000 |
pub const CLKPR: *mut u8 = 0x61 as *mut u8;

/// Watchdog Timer Clock Divider.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDEWIE | 100 |
/// | WCLKD | 11 |
/// | WDEWIF | 1000 |
pub const WDTCKD: *mut u8 = 0x62 as *mut u8;

/// Regulator Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | REGDIS | 1 |
pub const REGCR: *mut u8 = 0x63 as *mut u8;

/// Power Reduction Register0.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRTIM0 | 100000 |
/// | PRSPI | 100 |
/// | PRTIM1 | 1000 |
pub const PRR0: *mut u8 = 0x64 as *mut u8;

/// Power Reduction Register1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRUSB | 10000000 |
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
/// | PCIE | 11 |
pub const PCICR: *mut u8 = 0x68 as *mut u8;

/// External Interrupt Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ISC2 | 110000 |
/// | ISC1 | 1100 |
/// | ISC0 | 11 |
/// | ISC3 | 11000000 |
pub const EICRA: *mut u8 = 0x69 as *mut u8;

/// External Interrupt Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ISC6 | 110000 |
/// | ISC5 | 1100 |
/// | ISC4 | 11 |
/// | ISC7 | 11000000 |
pub const EICRB: *mut u8 = 0x6A as *mut u8;

/// Pin Change Mask Register 0.
pub const PCMSK0: *mut u8 = 0x6B as *mut u8;

/// Pin Change Mask Register 1.
pub const PCMSK1: *mut u8 = 0x6C as *mut u8;

/// Timer/Counter0 Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCIE0B | 100 |
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
/// | OCIE1B | 100 |
/// | ICIE1 | 100000 |
/// | OCIE1A | 10 |
/// | OCIE1C | 1000 |
pub const TIMSK1: *mut u8 = 0x6F as *mut u8;

/// Analog Comparator Input Multiplexer.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CMUX | 111 |
pub const ACMUX: *mut u8 = 0x7D as *mut u8;

/// `DIDR1` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AIN3D | 1000 |
/// | AIN5D | 100000 |
/// | AIN0D | 1 |
/// | AIN6D | 1000000 |
/// | AIN4D | 10000 |
/// | AIN7D | 10000000 |
/// | AIN2D | 100 |
/// | AIN1D | 10 |
pub const DIDR1: *mut u8 = 0x7F as *mut u8;

/// Timer/Counter1 Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | COM1C | 1100 |
/// | COM1A | 11000000 |
/// | COM1B | 110000 |
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

/// Timer/Counter1 Output Compare Register A  Bytes.
pub const OCR1A: *mut u16 = 0x88 as *mut u16;

/// Timer/Counter1 Output Compare Register A  Bytes low byte.
pub const OCR1AL: *mut u8 = 0x88 as *mut u8;

/// Timer/Counter1 Output Compare Register A  Bytes high byte.
pub const OCR1AH: *mut u8 = 0x89 as *mut u8;

/// Timer/Counter1 Output Compare Register B  Bytes.
pub const OCR1B: *mut u16 = 0x8A as *mut u16;

/// Timer/Counter1 Output Compare Register B  Bytes low byte.
pub const OCR1BL: *mut u8 = 0x8A as *mut u8;

/// Timer/Counter1 Output Compare Register B  Bytes high byte.
pub const OCR1BH: *mut u8 = 0x8B as *mut u8;

/// Timer/Counter1 Output Compare Register C  Bytes low byte.
pub const OCR1CL: *mut u8 = 0x8C as *mut u8;

/// Timer/Counter1 Output Compare Register C  Bytes.
pub const OCR1C: *mut u16 = 0x8C as *mut u16;

/// Timer/Counter1 Output Compare Register C  Bytes high byte.
pub const OCR1CH: *mut u8 = 0x8D as *mut u8;

/// USART Control and Status Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FE1 | 10000 |
/// | RXC1 | 10000000 |
/// | UPE1 | 100 |
/// | MPCM1 | 1 |
/// | DOR1 | 1000 |
/// | U2X1 | 10 |
/// | UDRE1 | 100000 |
/// | TXC1 | 1000000 |
pub const UCSR1A: *mut u8 = 0xC8 as *mut u8;

/// USART Control and Status Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TXEN1 | 1000 |
/// | RXB81 | 10 |
/// | TXCIE1 | 1000000 |
/// | TXB81 | 1 |
/// | RXEN1 | 10000 |
/// | UCSZ12 | 100 |
/// | RXCIE1 | 10000000 |
/// | UDRIE1 | 100000 |
pub const UCSR1B: *mut u8 = 0xC9 as *mut u8;

/// USART Control and Status Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | UPM1 | 110000 |
/// | UCPOL1 | 1 |
/// | USBS1 | 1000 |
/// | UCSZ1 | 110 |
/// | UMSEL1 | 11000000 |
pub const UCSR1C: *mut u8 = 0xCA as *mut u8;

/// USART Control and Status Register D.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RTSEN | 1 |
/// | CTSEN | 10 |
pub const UCSR1D: *mut u8 = 0xCB as *mut u8;

/// USART Baud Rate Register  Bytes low byte.
pub const UBRR1L: *mut u8 = 0xCC as *mut u8;

/// USART Baud Rate Register  Bytes.
pub const UBRR1: *mut u16 = 0xCC as *mut u16;

/// USART Baud Rate Register  Bytes high byte.
pub const UBRR1H: *mut u8 = 0xCD as *mut u8;

/// USART I/O Data Register.
pub const UDR1: *mut u8 = 0xCE as *mut u8;

/// `CLKSEL0` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EXTE | 100 |
/// | RCE | 1000 |
/// | CLKS | 1 |
/// | RCSUT | 11000000 |
/// | EXSUT | 110000 |
pub const CLKSEL0: *mut u8 = 0xD0 as *mut u8;

/// `CLKSEL1` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EXCKSEL | 1111 |
/// | RCCKSEL | 11110000 |
pub const CLKSEL1: *mut u8 = 0xD1 as *mut u8;

/// `CLKSTA` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RCON | 10 |
/// | EXTON | 1 |
pub const CLKSTA: *mut u8 = 0xD2 as *mut u8;

/// USB General Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | USBE | 10000000 |
/// | FRZCLK | 100000 |
pub const USBCON: *mut u8 = 0xD8 as *mut u8;

/// USB Device Control Registers.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DETACH | 1 |
/// | RMWKUP | 10 |
/// | RSTCPU | 100 |
pub const UDCON: *mut u8 = 0xE0 as *mut u8;

/// USB Device Interrupt Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | UPRSMI | 1000000 |
/// | SOFI | 100 |
/// | WAKEUPI | 10000 |
/// | EORSMI | 100000 |
/// | EORSTI | 1000 |
/// | SUSPI | 1 |
pub const UDINT: *mut u8 = 0xE1 as *mut u8;

/// USB Device Interrupt Enable Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EORSME | 100000 |
/// | SUSPE | 1 |
/// | SOFE | 100 |
/// | EORSTE | 1000 |
/// | WAKEUPE | 10000 |
/// | UPRSME | 1000000 |
pub const UDIEN: *mut u8 = 0xE2 as *mut u8;

/// USB Device Address Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | UADD | 1111111 |
/// | ADDEN | 10000000 |
pub const UDADDR: *mut u8 = 0xE3 as *mut u8;

/// USB Device Frame Number High Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FNUM | 11111111111 |
pub const UDFNUM: *mut u16 = 0xE4 as *mut u16;

/// USB Device Frame Number High Register low byte.
pub const UDFNUML: *mut u8 = 0xE4 as *mut u8;

/// USB Device Frame Number High Register high byte.
pub const UDFNUMH: *mut u8 = 0xE5 as *mut u8;

/// USB Device Micro Frame Number.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FNCERR | 10000 |
pub const UDMFN: *mut u8 = 0xE6 as *mut u8;

/// USB Endpoint Interrupt Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | NAKOUTI | 10000 |
/// | RXOUTI | 100 |
/// | FIFOCON | 10000000 |
/// | RXSTPI | 1000 |
/// | RWAL | 100000 |
/// | STALLEDI | 10 |
/// | TXINI | 1 |
/// | NAKINI | 1000000 |
pub const UEINTX: *mut u8 = 0xE8 as *mut u8;

/// USB Endpoint Number.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EPNUM | 111 |
pub const UENUM: *mut u8 = 0xE9 as *mut u8;

/// USB Endpoint Reset Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EPRST | 11111 |
pub const UERST: *mut u8 = 0xEA as *mut u8;

/// USB Endpoint Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | STALLRQ | 100000 |
/// | RSTDT | 1000 |
/// | STALLRQC | 10000 |
/// | EPEN | 1 |
pub const UECONX: *mut u8 = 0xEB as *mut u8;

/// USB Endpoint Configuration 0 Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EPTYPE | 11000000 |
/// | EPDIR | 1 |
pub const UECFG0X: *mut u8 = 0xEC as *mut u8;

/// USB Endpoint Configuration 1 Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EPSIZE | 1110000 |
/// | ALLOC | 10 |
/// | EPBK | 1100 |
pub const UECFG1X: *mut u8 = 0xED as *mut u8;

/// USB Endpoint Status 0 Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | UNDERFI | 100000 |
/// | OVERFI | 1000000 |
/// | NBUSYBK | 11 |
/// | CFGOK | 10000000 |
/// | DTSEQ | 1100 |
pub const UESTA0X: *mut u8 = 0xEE as *mut u8;

/// USB Endpoint Status 1 Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CTRLDIR | 100 |
/// | CURRBK | 11 |
pub const UESTA1X: *mut u8 = 0xEF as *mut u8;

/// USB Endpoint Interrupt Enable Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TXINE | 1 |
/// | RXSTPE | 1000 |
/// | RXOUTE | 100 |
/// | NAKINE | 1000000 |
/// | FLERRE | 10000000 |
/// | NAKOUTE | 10000 |
/// | STALLEDE | 10 |
pub const UEIENX: *mut u8 = 0xF0 as *mut u8;

/// USB Data Endpoint.
pub const UEDATX: *mut u8 = 0xF1 as *mut u8;

/// USB Endpoint Byte Count Register.
pub const UEBCLX: *mut u8 = 0xF2 as *mut u8;

/// USB Endpoint Number Interrupt Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EPINT | 11111 |
pub const UEINT: *mut u8 = 0xF4 as *mut u8;

/// USB Software Output Enable register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | UPDRV | 110000 |
/// | DMI | 1 |
/// | DPI | 10 |
/// | UPWE | 11000000 |
pub const UPOE: *mut u8 = 0xFB as *mut u8;

/// Bitfield on register `ACMUX`
pub const CMUX: u8 = 0x7;

/// Bitfield on register `ACSR`
pub const ACIC: u8 = 0x4;

/// Bitfield on register `ACSR`
pub const ACIS: u8 = 0x3;

/// Bitfield on register `ACSR`
pub const ACBG: u8 = 0x40;

/// Bitfield on register `ACSR`
pub const ACO: u8 = 0x20;

/// Bitfield on register `ACSR`
pub const ACI: u8 = 0x10;

/// Bitfield on register `ACSR`
pub const ACIE: u8 = 0x8;

/// Bitfield on register `ACSR`
pub const ACD: u8 = 0x80;

/// Bitfield on register `CLKPR`
pub const CLKPS: u8 = 0xF;

/// Bitfield on register `CLKPR`
pub const CLKPCE: u8 = 0x80;

/// Bitfield on register `CLKSEL0`
pub const EXTE: u8 = 0x4;

/// Bitfield on register `CLKSEL0`
pub const RCE: u8 = 0x8;

/// Bitfield on register `CLKSEL0`
pub const CLKS: u8 = 0x1;

/// Bitfield on register `CLKSEL0`
pub const RCSUT: u8 = 0xC0;

/// Bitfield on register `CLKSEL0`
pub const EXSUT: u8 = 0x30;

/// Bitfield on register `CLKSEL1`
pub const EXCKSEL: u8 = 0xF;

/// Bitfield on register `CLKSEL1`
pub const RCCKSEL: u8 = 0xF0;

/// Bitfield on register `CLKSTA`
pub const RCON: u8 = 0x2;

/// Bitfield on register `CLKSTA`
pub const EXTON: u8 = 0x1;

/// Bitfield on register `DIDR1`
pub const AIN3D: u8 = 0x8;

/// Bitfield on register `DIDR1`
pub const AIN5D: u8 = 0x20;

/// Bitfield on register `DIDR1`
pub const AIN0D: u8 = 0x1;

/// Bitfield on register `DIDR1`
pub const AIN6D: u8 = 0x40;

/// Bitfield on register `DIDR1`
pub const AIN4D: u8 = 0x10;

/// Bitfield on register `DIDR1`
pub const AIN7D: u8 = 0x80;

/// Bitfield on register `DIDR1`
pub const AIN2D: u8 = 0x4;

/// Bitfield on register `DIDR1`
pub const AIN1D: u8 = 0x2;

/// Bitfield on register `EECR`
pub const EEPM: u8 = 0x30;

/// Bitfield on register `EECR`
pub const EEPE: u8 = 0x2;

/// Bitfield on register `EECR`
pub const EERE: u8 = 0x1;

/// Bitfield on register `EECR`
pub const EEMPE: u8 = 0x4;

/// Bitfield on register `EECR`
pub const EERIE: u8 = 0x8;

/// Bitfield on register `EICRA`
pub const ISC2: u8 = 0x30;

/// Bitfield on register `EICRA`
pub const ISC1: u8 = 0xC;

/// Bitfield on register `EICRA`
pub const ISC0: u8 = 0x3;

/// Bitfield on register `EICRA`
pub const ISC3: u8 = 0xC0;

/// Bitfield on register `EICRB`
pub const ISC6: u8 = 0x30;

/// Bitfield on register `EICRB`
pub const ISC5: u8 = 0xC;

/// Bitfield on register `EICRB`
pub const ISC4: u8 = 0x3;

/// Bitfield on register `EICRB`
pub const ISC7: u8 = 0xC0;

/// Bitfield on register `EXTENDED`
pub const BODLEVEL: u8 = 0x7;

/// Bitfield on register `EXTENDED`
pub const HWBE: u8 = 0x8;

/// Bitfield on register `GPIOR0`
pub const GPIOR04: u8 = 0x10;

/// Bitfield on register `GPIOR0`
pub const GPIOR07: u8 = 0x80;

/// Bitfield on register `GPIOR0`
pub const GPIOR03: u8 = 0x8;

/// Bitfield on register `GPIOR0`
pub const GPIOR06: u8 = 0x40;

/// Bitfield on register `GPIOR0`
pub const GPIOR01: u8 = 0x2;

/// Bitfield on register `GPIOR0`
pub const GPIOR05: u8 = 0x20;

/// Bitfield on register `GPIOR0`
pub const GPIOR00: u8 = 0x1;

/// Bitfield on register `GPIOR0`
pub const GPIOR02: u8 = 0x4;

/// Bitfield on register `GTCCR`
pub const TSM: u8 = 0x80;

/// Bitfield on register `GTCCR`
pub const PSRSYNC: u8 = 0x1;

/// Bitfield on register `HIGH`
pub const DWEN: u8 = 0x80;

/// Bitfield on register `HIGH`
pub const BOOTRST: u8 = 0x1;

/// Bitfield on register `HIGH`
pub const SPIEN: u8 = 0x20;

/// Bitfield on register `HIGH`
pub const EESAVE: u8 = 0x8;

/// Bitfield on register `HIGH`
pub const WDTON: u8 = 0x10;

/// Bitfield on register `HIGH`
pub const RSTDISBL: u8 = 0x40;

/// Bitfield on register `HIGH`
pub const BOOTSZ: u8 = 0x6;

/// Bitfield on register `LOCKBIT`
pub const BLB0: u8 = 0xC;

/// Bitfield on register `LOCKBIT`
pub const BLB1: u8 = 0x30;

/// Bitfield on register `LOCKBIT`
pub const LB: u8 = 0x3;

/// Bitfield on register `LOW`
pub const CKDIV8: u8 = 0x80;

/// Bitfield on register `LOW`
pub const CKOUT: u8 = 0x40;

/// Bitfield on register `LOW`
pub const SUT_CKSEL: u8 = 0x3F;

/// Bitfield on register `MCUCR`
pub const IVSEL: u8 = 0x2;

/// Bitfield on register `MCUCR`
pub const IVCE: u8 = 0x1;

/// Bitfield on register `MCUCR`
pub const PUD: u8 = 0x10;

/// Bitfield on register `MCUSR`
pub const WDRF: u8 = 0x8;

/// Bitfield on register `MCUSR`
pub const PORF: u8 = 0x1;

/// Bitfield on register `MCUSR`
pub const USBRF: u8 = 0x20;

/// Bitfield on register `MCUSR`
pub const EXTRF: u8 = 0x2;

/// Bitfield on register `MCUSR`
pub const BORF: u8 = 0x4;

/// Bitfield on register `PCICR`
pub const PCIE: u8 = 0x3;

/// Bitfield on register `PCIFR`
pub const PCIF: u8 = 0x3;

/// Bitfield on register `PLLCSR`
pub const PLLE: u8 = 0x2;

/// Bitfield on register `PLLCSR`
pub const PLLP: u8 = 0x1C;

/// Bitfield on register `PLLCSR`
pub const PLOCK: u8 = 0x1;

/// Bitfield on register `PRR0`
pub const PRTIM0: u8 = 0x20;

/// Bitfield on register `PRR0`
pub const PRSPI: u8 = 0x4;

/// Bitfield on register `PRR0`
pub const PRTIM1: u8 = 0x8;

/// Bitfield on register `PRR1`
pub const PRUSB: u8 = 0x80;

/// Bitfield on register `PRR1`
pub const PRUSART1: u8 = 0x1;

/// Bitfield on register `REGCR`
pub const REGDIS: u8 = 0x1;

/// Bitfield on register `SMCR`
pub const SE: u8 = 0x1;

/// Bitfield on register `SMCR`
pub const SM: u8 = 0xE;

/// Bitfield on register `SPCR`
pub const DORD: u8 = 0x20;

/// Bitfield on register `SPCR`
pub const SPIE: u8 = 0x80;

/// Bitfield on register `SPCR`
pub const CPOL: u8 = 0x8;

/// Bitfield on register `SPCR`
pub const SPR: u8 = 0x3;

/// Bitfield on register `SPCR`
pub const SPE: u8 = 0x40;

/// Bitfield on register `SPCR`
pub const MSTR: u8 = 0x10;

/// Bitfield on register `SPCR`
pub const CPHA: u8 = 0x4;

/// Bitfield on register `SPMCSR`
pub const PGERS: u8 = 0x2;

/// Bitfield on register `SPMCSR`
pub const SIGRD: u8 = 0x20;

/// Bitfield on register `SPMCSR`
pub const PGWRT: u8 = 0x4;

/// Bitfield on register `SPMCSR`
pub const SPMIE: u8 = 0x80;

/// Bitfield on register `SPMCSR`
pub const SPMEN: u8 = 0x1;

/// Bitfield on register `SPMCSR`
pub const RWWSB: u8 = 0x40;

/// Bitfield on register `SPMCSR`
pub const RWWSRE: u8 = 0x10;

/// Bitfield on register `SPMCSR`
pub const BLBSET: u8 = 0x8;

/// Bitfield on register `SPSR`
pub const WCOL: u8 = 0x40;

/// Bitfield on register `SPSR`
pub const SPIF: u8 = 0x80;

/// Bitfield on register `SPSR`
pub const SPI2X: u8 = 0x1;

/// Bitfield on register `SREG`
pub const V: u8 = 0x8;

/// Bitfield on register `SREG`
pub const T: u8 = 0x40;

/// Bitfield on register `SREG`
pub const S: u8 = 0x10;

/// Bitfield on register `SREG`
pub const N: u8 = 0x4;

/// Bitfield on register `SREG`
pub const Z: u8 = 0x2;

/// Bitfield on register `SREG`
pub const C: u8 = 0x1;

/// Bitfield on register `SREG`
pub const H: u8 = 0x20;

/// Bitfield on register `SREG`
pub const I: u8 = 0x80;

/// Bitfield on register `TCCR0A`
pub const COM0A: u8 = 0xC0;

/// Bitfield on register `TCCR0A`
pub const COM0B: u8 = 0x30;

/// Bitfield on register `TCCR0A`
pub const WGM0: u8 = 0x3;

/// Bitfield on register `TCCR0B`
pub const CS0: u8 = 0x7;

/// Bitfield on register `TCCR0B`
pub const FOC0B: u8 = 0x40;

/// Bitfield on register `TCCR0B`
pub const WGM02: u8 = 0x8;

/// Bitfield on register `TCCR0B`
pub const FOC0A: u8 = 0x80;

/// Bitfield on register `TCCR1A`
pub const COM1C: u8 = 0xC;

/// Bitfield on register `TCCR1A`
pub const COM1A: u8 = 0xC0;

/// Bitfield on register `TCCR1A`
pub const COM1B: u8 = 0x30;

/// Bitfield on register `TCCR1B`
pub const ICNC1: u8 = 0x80;

/// Bitfield on register `TCCR1B`
pub const CS1: u8 = 0x7;

/// Bitfield on register `TCCR1B`
pub const ICES1: u8 = 0x40;

/// Bitfield on register `TCCR1C`
pub const FOC1A: u8 = 0x80;

/// Bitfield on register `TCCR1C`
pub const FOC1C: u8 = 0x20;

/// Bitfield on register `TCCR1C`
pub const FOC1B: u8 = 0x40;

/// Bitfield on register `TIFR0`
pub const OCF0A: u8 = 0x2;

/// Bitfield on register `TIFR0`
pub const OCF0B: u8 = 0x4;

/// Bitfield on register `TIFR0`
pub const TOV0: u8 = 0x1;

/// Bitfield on register `TIFR1`
pub const OCF1C: u8 = 0x8;

/// Bitfield on register `TIFR1`
pub const OCF1A: u8 = 0x2;

/// Bitfield on register `TIFR1`
pub const TOV1: u8 = 0x1;

/// Bitfield on register `TIFR1`
pub const OCF1B: u8 = 0x4;

/// Bitfield on register `TIFR1`
pub const ICF1: u8 = 0x20;

/// Bitfield on register `TIMSK0`
pub const OCIE0B: u8 = 0x4;

/// Bitfield on register `TIMSK0`
pub const OCIE0A: u8 = 0x2;

/// Bitfield on register `TIMSK0`
pub const TOIE0: u8 = 0x1;

/// Bitfield on register `TIMSK1`
pub const TOIE1: u8 = 0x1;

/// Bitfield on register `TIMSK1`
pub const OCIE1B: u8 = 0x4;

/// Bitfield on register `TIMSK1`
pub const ICIE1: u8 = 0x20;

/// Bitfield on register `TIMSK1`
pub const OCIE1A: u8 = 0x2;

/// Bitfield on register `TIMSK1`
pub const OCIE1C: u8 = 0x8;

/// Bitfield on register `UCSR1A`
pub const FE1: u8 = 0x10;

/// Bitfield on register `UCSR1A`
pub const RXC1: u8 = 0x80;

/// Bitfield on register `UCSR1A`
pub const UPE1: u8 = 0x4;

/// Bitfield on register `UCSR1A`
pub const MPCM1: u8 = 0x1;

/// Bitfield on register `UCSR1A`
pub const DOR1: u8 = 0x8;

/// Bitfield on register `UCSR1A`
pub const U2X1: u8 = 0x2;

/// Bitfield on register `UCSR1A`
pub const UDRE1: u8 = 0x20;

/// Bitfield on register `UCSR1A`
pub const TXC1: u8 = 0x40;

/// Bitfield on register `UCSR1B`
pub const TXEN1: u8 = 0x8;

/// Bitfield on register `UCSR1B`
pub const RXB81: u8 = 0x2;

/// Bitfield on register `UCSR1B`
pub const TXCIE1: u8 = 0x40;

/// Bitfield on register `UCSR1B`
pub const TXB81: u8 = 0x1;

/// Bitfield on register `UCSR1B`
pub const RXEN1: u8 = 0x10;

/// Bitfield on register `UCSR1B`
pub const UCSZ12: u8 = 0x4;

/// Bitfield on register `UCSR1B`
pub const RXCIE1: u8 = 0x80;

/// Bitfield on register `UCSR1B`
pub const UDRIE1: u8 = 0x20;

/// Bitfield on register `UCSR1C`
pub const UPM1: u8 = 0x30;

/// Bitfield on register `UCSR1C`
pub const UCPOL1: u8 = 0x1;

/// Bitfield on register `UCSR1C`
pub const USBS1: u8 = 0x8;

/// Bitfield on register `UCSR1C`
pub const UCSZ1: u8 = 0x6;

/// Bitfield on register `UCSR1C`
pub const UMSEL1: u8 = 0xC0;

/// Bitfield on register `UCSR1D`
pub const RTSEN: u8 = 0x1;

/// Bitfield on register `UCSR1D`
pub const CTSEN: u8 = 0x2;

/// Bitfield on register `UDADDR`
pub const UADD: u8 = 0x7F;

/// Bitfield on register `UDADDR`
pub const ADDEN: u8 = 0x80;

/// Bitfield on register `UDCON`
pub const DETACH: u8 = 0x1;

/// Bitfield on register `UDCON`
pub const RMWKUP: u8 = 0x2;

/// Bitfield on register `UDCON`
pub const RSTCPU: u8 = 0x4;

/// Bitfield on register `UDFNUM`
pub const FNUM: u16 = 0x7FF;

/// Bitfield on register `UDIEN`
pub const EORSME: u8 = 0x20;

/// Bitfield on register `UDIEN`
pub const SUSPE: u8 = 0x1;

/// Bitfield on register `UDIEN`
pub const SOFE: u8 = 0x4;

/// Bitfield on register `UDIEN`
pub const EORSTE: u8 = 0x8;

/// Bitfield on register `UDIEN`
pub const WAKEUPE: u8 = 0x10;

/// Bitfield on register `UDIEN`
pub const UPRSME: u8 = 0x40;

/// Bitfield on register `UDINT`
pub const UPRSMI: u8 = 0x40;

/// Bitfield on register `UDINT`
pub const SOFI: u8 = 0x4;

/// Bitfield on register `UDINT`
pub const WAKEUPI: u8 = 0x10;

/// Bitfield on register `UDINT`
pub const EORSMI: u8 = 0x20;

/// Bitfield on register `UDINT`
pub const EORSTI: u8 = 0x8;

/// Bitfield on register `UDINT`
pub const SUSPI: u8 = 0x1;

/// Bitfield on register `UDMFN`
pub const FNCERR: u8 = 0x10;

/// Bitfield on register `UECFG0X`
pub const EPTYPE: u8 = 0xC0;

/// Bitfield on register `UECFG0X`
pub const EPDIR: u8 = 0x1;

/// Bitfield on register `UECFG1X`
pub const EPSIZE: u8 = 0x70;

/// Bitfield on register `UECFG1X`
pub const ALLOC: u8 = 0x2;

/// Bitfield on register `UECFG1X`
pub const EPBK: u8 = 0xC;

/// Bitfield on register `UECONX`
pub const STALLRQ: u8 = 0x20;

/// Bitfield on register `UECONX`
pub const RSTDT: u8 = 0x8;

/// Bitfield on register `UECONX`
pub const STALLRQC: u8 = 0x10;

/// Bitfield on register `UECONX`
pub const EPEN: u8 = 0x1;

/// Bitfield on register `UEIENX`
pub const TXINE: u8 = 0x1;

/// Bitfield on register `UEIENX`
pub const RXSTPE: u8 = 0x8;

/// Bitfield on register `UEIENX`
pub const RXOUTE: u8 = 0x4;

/// Bitfield on register `UEIENX`
pub const NAKINE: u8 = 0x40;

/// Bitfield on register `UEIENX`
pub const FLERRE: u8 = 0x80;

/// Bitfield on register `UEIENX`
pub const NAKOUTE: u8 = 0x10;

/// Bitfield on register `UEIENX`
pub const STALLEDE: u8 = 0x2;

/// Bitfield on register `UEINT`
pub const EPINT: u8 = 0x1F;

/// Bitfield on register `UEINTX`
pub const NAKOUTI: u8 = 0x10;

/// Bitfield on register `UEINTX`
pub const RXOUTI: u8 = 0x4;

/// Bitfield on register `UEINTX`
pub const FIFOCON: u8 = 0x80;

/// Bitfield on register `UEINTX`
pub const RXSTPI: u8 = 0x8;

/// Bitfield on register `UEINTX`
pub const RWAL: u8 = 0x20;

/// Bitfield on register `UEINTX`
pub const STALLEDI: u8 = 0x2;

/// Bitfield on register `UEINTX`
pub const TXINI: u8 = 0x1;

/// Bitfield on register `UEINTX`
pub const NAKINI: u8 = 0x40;

/// Bitfield on register `UENUM`
pub const EPNUM: u8 = 0x7;

/// Bitfield on register `UERST`
pub const EPRST: u8 = 0x1F;

/// Bitfield on register `UESTA0X`
pub const UNDERFI: u8 = 0x20;

/// Bitfield on register `UESTA0X`
pub const OVERFI: u8 = 0x40;

/// Bitfield on register `UESTA0X`
pub const NBUSYBK: u8 = 0x3;

/// Bitfield on register `UESTA0X`
pub const CFGOK: u8 = 0x80;

/// Bitfield on register `UESTA0X`
pub const DTSEQ: u8 = 0xC;

/// Bitfield on register `UESTA1X`
pub const CTRLDIR: u8 = 0x4;

/// Bitfield on register `UESTA1X`
pub const CURRBK: u8 = 0x3;

/// Bitfield on register `UPOE`
pub const UPDRV: u8 = 0x30;

/// Bitfield on register `UPOE`
pub const DMI: u8 = 0x1;

/// Bitfield on register `UPOE`
pub const DPI: u8 = 0x2;

/// Bitfield on register `UPOE`
pub const UPWE: u8 = 0xC0;

/// Bitfield on register `USBCON`
pub const USBE: u8 = 0x80;

/// Bitfield on register `USBCON`
pub const FRZCLK: u8 = 0x20;

/// Bitfield on register `WDTCKD`
pub const WDEWIE: u8 = 0x4;

/// Bitfield on register `WDTCKD`
pub const WCLKD: u8 = 0x3;

/// Bitfield on register `WDTCKD`
pub const WDEWIF: u8 = 0x8;

/// Bitfield on register `WDTCSR`
pub const WDCE: u8 = 0x10;

/// Bitfield on register `WDTCSR`
pub const WDP: u8 = 0x27;

/// Bitfield on register `WDTCSR`
pub const WDE: u8 = 0x8;

/// Bitfield on register `WDTCSR`
pub const WDIE: u8 = 0x40;

/// Bitfield on register `WDTCSR`
pub const WDIF: u8 = 0x80;

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

/// `ANALOG_COMP_SELECTION_BITS` value group
#[allow(non_upper_case_globals)]
pub mod analog_comp_selection_bits {
   /// AIN1.
   pub const AIN1: u32 = 0x0;
   /// AIN2.
   pub const AIN2: u32 = 0x1;
   /// AIN3.
   pub const AIN3: u32 = 0x2;
   /// AIN4.
   pub const AIN4: u32 = 0x3;
   /// AIN5.
   pub const AIN5: u32 = 0x4;
   /// AIN6.
   pub const AIN6: u32 = 0x5;
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

/// `CPU_SLEEP_MODE_3BITS` value group
#[allow(non_upper_case_globals)]
pub mod cpu_sleep_mode_3bits {
   /// Idle.
   pub const IDLE: u32 = 0x0;
   /// Reserved.
   pub const VAL_0x01: u32 = 0x1;
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
   /// Brown-out detection level at VCC=2.7 V.
   pub const _2V7: u32 = 0x6;
   /// Brown-out detection level at VCC=2.9 V.
   pub const _2V9: u32 = 0x5;
   /// Brown-out detection level at VCC=3.0 V.
   pub const _3V0: u32 = 0x4;
   /// Brown-out detection level at VCC=3.5 V.
   pub const _3V5: u32 = 0x3;
   /// Brown-out detection level at VCC=3.6 V.
   pub const _3V6: u32 = 0x2;
   /// Brown-out detection level at VCC=4.0 V.
   pub const _4V0: u32 = 0x1;
   /// Brown-out detection level at VCC=4.3 V.
   pub const _4V3: u32 = 0x0;
}

/// `ENUM_BOOTSZ` value group
#[allow(non_upper_case_globals)]
pub mod enum_bootsz {
   /// Boot Flash size=256 words start address=$3F00.
   pub const _256W_3F00: u32 = 0x3;
   /// Boot Flash size=512 words start address=$3E00.
   pub const _512W_3E00: u32 = 0x2;
   /// Boot Flash size=1024 words start address=$3C00.
   pub const _1024W_3C00: u32 = 0x1;
   /// Boot Flash size=2048 words start address=$3800.
   pub const _2048W_3800: u32 = 0x0;
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

/// Oscillator Calibration Values
#[allow(non_upper_case_globals)]
pub mod osccal_value_addresses {
   /// 8.0 MHz.
   pub const _8_0_MHz: u32 = 0x0;
}

/// `PLL_INPUT_PRESCALER` value group
#[allow(non_upper_case_globals)]
pub mod pll_input_prescaler {
   /// Clock/4.
   pub const VAL_0x03: u32 = 0x3;
   /// Clock/8.
   pub const VAL_0x05: u32 = 0x5;
}

