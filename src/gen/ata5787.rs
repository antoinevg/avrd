//! The AVR ATA5787 microcontroller
//!
//! # Variants
//! |        | Pinout | Mcu age | Operating temperature | Operating voltage | Max speed |
//! |--------|--------|---------|-----------------------|-------------------|-----------|
//! | standard |  |  | 0°C - 0°C | 2.4V - 5.5V | 0 MHz |
//!

#![allow(non_upper_case_globals)]

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

/// `LOW` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RSTDISBL | 10 |
/// | BOOTRST | 100 |
/// | EXTCLKEN | 1 |
/// | DWEN | 1000000 |
/// | EESAVE | 1000 |
/// | SPIEN | 100000 |
/// | WDTON | 10000 |
/// | CKDIV8 | 10000000 |
pub const LOW: *mut u8 = 0x0 as *mut u8;

/// Power Reduction Register 0.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRSPI | 1 |
/// | PRTXDC | 100 |
/// | PRUART | 1000000 |
/// | PRRXDC | 10 |
/// | PRCRC | 1000 |
/// | PRCO | 100000 |
/// | PRTRC | 10000000 |
/// | PRVM | 10000 |
pub const PRR0: *mut u8 = 0x21 as *mut u8;

/// Power Reduction Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRT2 | 10 |
/// | PRT4 | 1000 |
/// | PRT3 | 100 |
/// | PRT5 | 10000 |
/// | PRT1 | 1 |
pub const PRR1: *mut u8 = 0x22 as *mut u8;

/// Power Reduction Register 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRSF | 100 |
/// | PRXB | 1 |
/// | PRIDS | 10000 |
/// | PRSSM | 10000000 |
/// | PRDF | 1000 |
/// | PRRS | 100000 |
/// | PRXA | 10 |
pub const PRR2: *mut u8 = 0x23 as *mut u8;

/// Rx DSP Power Reduction.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | APRPTB | 10000 |
/// | RDPRF | 10000000 |
/// | PRTMP | 1000 |
/// | PRFLT | 100 |
/// | PRPTA | 10 |
/// | ARDPRF | 1000000 |
/// | PRPTB | 1 |
/// | APRPTA | 100000 |
pub const RDPR: *mut u8 = 0x24 as *mut u8;

/// Port B Input Pins.
pub const PINB: *mut u8 = 0x25 as *mut u8;

/// Port B Data Direction.
pub const DDRB: *mut u8 = 0x26 as *mut u8;

/// Port B Data Register.
pub const PORTB: *mut u8 = 0x27 as *mut u8;

/// Port C Input Pins.
pub const PINC: *mut u8 = 0x28 as *mut u8;

/// Port C Data Direction.
pub const DDRC: *mut u8 = 0x29 as *mut u8;

/// Port C Data Register.
pub const PORTC: *mut u8 = 0x2A as *mut u8;

/// Rx DSP Status Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SOTB | 100000 |
/// | WCOA | 1000000 |
/// | NBITA | 1 |
/// | WCOB | 10000000 |
/// | EOTA | 100 |
/// | SOTA | 10000 |
/// | EOTB | 1000 |
/// | NBITB | 10 |
pub const RDSIFR: *mut u8 = 0x2D as *mut u8;

/// MCU Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SPIIO | 100 |
/// | IVCE | 1 |
/// | ENPS | 1000 |
/// | PB7HS | 10000000 |
/// | PUD | 10000 |
/// | PB4HS | 100000 |
/// | PB7LS | 1000000 |
/// | IVSEL | 10 |
pub const MCUCR: *mut u8 = 0x2E as *mut u8;

/// Pin Change Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCIF0 | 1 |
/// | PCIF1 | 10 |
pub const PCIFR: *mut u8 = 0x2F as *mut u8;

/// Timer0 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T0PR | 10000 |
/// | T0IE | 1000 |
/// | T0PS | 111 |
pub const T0CR: *mut u8 = 0x30 as *mut u8;

/// Timer1 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T1OTM | 1 |
/// | T1RES | 100000 |
/// | T1CTM | 10 |
/// | T1TOP | 10000 |
/// | T1ENA | 10000000 |
/// | T1TOS | 1000000 |
/// | T1CRM | 100 |
pub const T1CR: *mut u8 = 0x31 as *mut u8;

/// Timer2 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T2CTM | 10 |
/// | T2TOP | 10000 |
/// | T2TOS | 1000000 |
/// | T2RES | 100000 |
/// | T2CRM | 100 |
/// | T2OTM | 1 |
/// | T2ENA | 10000000 |
pub const T2CR: *mut u8 = 0x32 as *mut u8;

/// Timer3 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T3TOS | 1000000 |
/// | T3ENA | 10000000 |
/// | T3CRM | 100 |
/// | T3RES | 100000 |
/// | T3TOP | 10000 |
/// | T3OTM | 1 |
/// | T3CTM | 10 |
/// | T3CPRM | 1000 |
pub const T3CR: *mut u8 = 0x33 as *mut u8;

/// Timer4 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T4CRM | 100 |
/// | T4ENA | 10000000 |
/// | T4CPRM | 1000 |
/// | T4TOS | 1000000 |
/// | T4CTM | 10 |
/// | T4TOP | 10000 |
/// | T4RES | 100000 |
/// | T4OTM | 1 |
pub const T4CR: *mut u8 = 0x34 as *mut u8;

/// Timer1 Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T1COF | 10 |
/// | T1OFF | 1 |
pub const T1IFR: *mut u8 = 0x35 as *mut u8;

/// Timer2 Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T2OFF | 1 |
/// | T2COF | 10 |
pub const T2IFR: *mut u8 = 0x36 as *mut u8;

/// Timer3 Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T3COF | 10 |
/// | T3ICF | 100 |
/// | T3OFF | 1 |
pub const T3IFR: *mut u8 = 0x37 as *mut u8;

/// Timer4 Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T4OFF | 1 |
/// | T4ICF | 100 |
/// | T4COF | 10 |
pub const T4IFR: *mut u8 = 0x38 as *mut u8;

/// Timer5 Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T5OFF | 1 |
/// | T5COF | 10 |
pub const T5IFR: *mut u8 = 0x39 as *mut u8;

/// General Purpose I/O Register 0.
pub const GPIOR0: *mut u8 = 0x3A as *mut u8;

/// General Purpose I/O Register 3.
pub const GPIOR3: *mut u8 = 0x3B as *mut u8;

/// General Purpose I/O Register 4.
pub const GPIOR4: *mut u8 = 0x3C as *mut u8;

/// General Purpose I/O Register 5.
pub const GPIOR5: *mut u8 = 0x3D as *mut u8;

/// General Purpose I/O Register 6.
pub const GPIOR6: *mut u8 = 0x3E as *mut u8;

/// EEPROM Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EERIE | 1000 |
/// | EEWE | 10 |
/// | EEMWE | 100 |
/// | EERE | 1 |
/// | EEPM | 110000 |
/// | NVMBSY | 10000000 |
/// | EEPAGE | 1000000 |
pub const EECR: *mut u8 = 0x3F as *mut u8;

/// EEPROM Data Register.
pub const EEDR: *mut u8 = 0x40 as *mut u8;

/// EEPROM Address Register.
pub const EEAR: *mut u16 = 0x41 as *mut u16;

/// EEPROM Address Register low byte.
pub const EEARL: *mut u8 = 0x41 as *mut u8;

/// EEPROM Address Register high byte.
pub const EEARH: *mut u8 = 0x42 as *mut u8;

/// EEPROM Protect Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EEAP | 1111 |
pub const EEPR: *mut u8 = 0x43 as *mut u8;

/// General Purpose I/O Register 1.
pub const GPIOR1: *mut u8 = 0x44 as *mut u8;

/// General Purpose I/O Register 2.
pub const GPIOR2: *mut u8 = 0x45 as *mut u8;

/// Pin Change Interrupt Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCIE1 | 10 |
/// | PCIE0 | 1 |
pub const PCICR: *mut u8 = 0x46 as *mut u8;

/// External Interrupt Mask.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | INT0 | 1 |
/// | INT1 | 10 |
pub const EIMSK: *mut u8 = 0x47 as *mut u8;

/// External Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | INTF1 | 10 |
/// | INTF0 | 1 |
pub const EIFR: *mut u8 = 0x48 as *mut u8;

/// CRC Data Input Register.
pub const CRCDIR: *mut u8 = 0x49 as *mut u8;

/// Voltage Monitor Control and Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | VMF | 100000 |
/// | VMLS | 1111 |
/// | VMIM | 10000 |
pub const VMCSR: *mut u8 = 0x4A as *mut u8;

/// MCU Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDRF | 1000 |
/// | DWRF | 10000 |
/// | PORF | 1 |
/// | EXTRF | 10 |
pub const MCUSR: *mut u8 = 0x4B as *mut u8;

/// SPI Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SPIE | 10000000 |
/// | DORD | 100000 |
/// | CPOL | 1000 |
/// | MSTR | 10000 |
/// | SPE | 1000000 |
/// | SPR | 11 |
/// | CPHA | 100 |
pub const SPCR: *mut u8 = 0x4C as *mut u8;

/// SPI Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TXIF | 100000 |
/// | SPIF | 10000000 |
/// | SPI2X | 1 |
/// | RXIF | 10000 |
pub const SPSR: *mut u8 = 0x4D as *mut u8;

/// SPI Data Register.
pub const SPDR: *mut u8 = 0x4E as *mut u8;

/// Timer0 Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T0F | 1 |
pub const T0IFR: *mut u8 = 0x4F as *mut u8;

/// Debug Wire Data Register.
pub const DWDR: *mut u8 = 0x51 as *mut u8;

/// Rx DSP Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RDPU | 1 |
/// | ADIVEN | 10 |
/// | RDEN | 100 |
pub const RDCR: *mut u8 = 0x52 as *mut u8;

/// End Of Telegram Status Path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RRFA | 1000000 |
/// | EOTBF | 10000000 |
/// | TMOFA | 10000 |
/// | CARFA | 1 |
/// | AMPFA | 10 |
/// | MANFA | 1000 |
/// | TELRA | 100000 |
/// | SYTFA | 100 |
pub const EOTSA: *mut u8 = 0x53 as *mut u8;

/// End Of Telegram Conditions Path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | MANFEA | 1000 |
/// | AMPFEA | 10 |
/// | RRFEA | 1000000 |
/// | CARFEA | 1 |
/// | SYTFEA | 100 |
/// | TELREA | 100000 |
/// | TMOFEA | 10000 |
/// | EOTBFE | 10000000 |
pub const EOTCA: *mut u8 = 0x54 as *mut u8;

/// End Of Telegram Status Path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EOTAF | 10000000 |
/// | TMOFB | 10000 |
/// | MANFB | 1000 |
/// | RRFB | 1000000 |
/// | AMPFB | 10 |
/// | TELRB | 100000 |
/// | SYTFB | 100 |
/// | CARFB | 1 |
pub const EOTSB: *mut u8 = 0x55 as *mut u8;

/// End Of Telegram Conditions Path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AMPFEB | 10 |
/// | TELREB | 100000 |
/// | MANFEB | 1000 |
/// | RRFEB | 1000000 |
/// | SYTFEB | 100 |
/// | CARFEB | 1 |
/// | TMOFEB | 10000 |
/// | EOTAFE | 10000000 |
pub const EOTCB: *mut u8 = 0x56 as *mut u8;

/// Store Program Memory Control and Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PGWRT | 100 |
/// | PGERS | 10 |
/// | SELFPRGEN | 1 |
/// | SPMIE | 10000000 |
/// | BLBSET | 1000 |
pub const SPMCSR: *mut u8 = 0x57 as *mut u8;

/// Sleep Mode Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SM | 110 |
/// | SE | 1 |
pub const SMCR: *mut u8 = 0x59 as *mut u8;

/// Clock Management Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CCS | 1000 |
/// | CMONEN | 1000000 |
/// | CMM | 111 |
/// | CMCCE | 10000000 |
pub const CMCR: *mut u8 = 0x5A as *mut u8;

/// Clock Management Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ECIE | 1 |
pub const CMIMR: *mut u8 = 0x5B as *mut u8;

/// Clock Prescaler.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CLTPS | 111000 |
/// | CLKPS | 111 |
/// | CLPCE | 10000000 |
pub const CLPR: *mut u8 = 0x5C as *mut u8;

/// Stack Pointer.
pub const SP: *mut u16 = 0x5D as *mut u16;

/// Stack Pointer low byte.
pub const SPL: *mut u8 = 0x5D as *mut u8;

/// Stack Pointer high byte.
pub const SPH: *mut u8 = 0x5E as *mut u8;

/// Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | H | 100000 |
/// | V | 1000 |
/// | S | 10000 |
/// | Z | 10 |
/// | T | 1000000 |
/// | I | 10000000 |
/// | C | 1 |
/// | N | 100 |
pub const SREG: *mut u8 = 0x5F as *mut u8;

/// Frequency Synthesizer Enable.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SDEN | 10 |
/// | SDPU | 1 |
pub const FSEN: *mut u8 = 0x60 as *mut u8;

/// Fractional Frequency 1 Low Byte.
pub const FFREQ1L: *mut u8 = 0x64 as *mut u8;

/// Fractional Frequency 1 Middle Byte.
pub const FFREQ1M: *mut u8 = 0x65 as *mut u8;

/// Fractional Frequency 1 High Byte.
pub const FFREQ1H: *mut u8 = 0x66 as *mut u8;

/// Fractional Frequency 2 Low Byte.
pub const FFREQ2L: *mut u8 = 0x67 as *mut u8;

/// Fractional Frequency 2 Middle Byte.
pub const FFREQ2M: *mut u8 = 0x68 as *mut u8;

/// Fractional Frequency 2 High Byte.
pub const FFREQ2H: *mut u8 = 0x69 as *mut u8;

/// External Interrupt Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ISC0 | 11 |
/// | ISC1 | 1100 |
pub const EICRA: *mut u8 = 0x6B as *mut u8;

/// Pin Change Mask Register 0.
pub const PCMSK0: *mut u8 = 0x6C as *mut u8;

/// Pin Change Mask Register 1.
pub const PCMSK1: *mut u8 = 0x6D as *mut u8;

/// Watchdog Timer0 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDE | 1000 |
/// | WDCE | 10000 |
/// | WDPS | 111 |
pub const WDTCR: *mut u8 = 0x6E as *mut u8;

/// Timer1 Counter Register.
pub const T1CNT: *mut u8 = 0x6F as *mut u8;

/// Timer1 Compare Register.
pub const T1COR: *mut u8 = 0x70 as *mut u8;

/// Timer1 Mode Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T1PS | 111100 |
/// | T1CS | 11 |
/// | T1DC | 11000000 |
pub const T1MR: *mut u8 = 0x71 as *mut u8;

/// Timer1 Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T1CIM | 10 |
/// | T1OIM | 1 |
pub const T1IMR: *mut u8 = 0x72 as *mut u8;

/// Timer2 Counter Register.
pub const T2CNT: *mut u8 = 0x73 as *mut u8;

/// Timer2 Compare Register.
pub const T2COR: *mut u8 = 0x74 as *mut u8;

/// Timer2 Mode Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T2DC | 11000000 |
/// | T2PS | 111100 |
/// | T2CS | 11 |
pub const T2MR: *mut u8 = 0x75 as *mut u8;

/// Timer2 Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T2OIM | 1 |
/// | T2CIM | 10 |
pub const T2IMR: *mut u8 = 0x76 as *mut u8;

/// Timer3 Counter low byte.
pub const T3CNTL: *mut u8 = 0x77 as *mut u8;

/// Timer3 Counter.
pub const T3CNT: *mut u16 = 0x77 as *mut u16;

/// Timer3 Counter high byte.
pub const T3CNTH: *mut u8 = 0x78 as *mut u8;

/// Timer3 Compare low byte.
pub const T3CORL: *mut u8 = 0x79 as *mut u8;

/// Timer3 Compare.
pub const T3COR: *mut u16 = 0x79 as *mut u16;

/// Timer3 Compare high byte.
pub const T3CORH: *mut u8 = 0x7A as *mut u8;

/// Timer3 Input Capture low byte.
pub const T3ICRL: *mut u8 = 0x7B as *mut u8;

/// Timer3 Input Capture.
pub const T3ICR: *mut u16 = 0x7B as *mut u16;

/// Timer3 Input Capture high byte.
pub const T3ICRH: *mut u8 = 0x7C as *mut u8;

/// Timer3 Mode Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T3CS | 11 |
/// | T3PS | 11100 |
pub const T3MRA: *mut u8 = 0x7D as *mut u8;

/// Timer3 Mode Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T3CE | 11000 |
/// | T3CNC | 100 |
/// | T3ICS | 11100000 |
/// | T3SCE | 10 |
pub const T3MRB: *mut u8 = 0x7E as *mut u8;

/// Timer3 Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T3CPIM | 100 |
/// | T3CIM | 10 |
/// | T3OIM | 1 |
pub const T3IMR: *mut u8 = 0x7F as *mut u8;

/// Timer4 Counter.
pub const T4CNT: *mut u16 = 0x80 as *mut u16;

/// Timer4 Counter low byte.
pub const T4CNTL: *mut u8 = 0x80 as *mut u8;

/// Timer4 Counter high byte.
pub const T4CNTH: *mut u8 = 0x81 as *mut u8;

/// Timer4 Compare.
pub const T4COR: *mut u16 = 0x82 as *mut u16;

/// Timer4 Compare low byte.
pub const T4CORL: *mut u8 = 0x82 as *mut u8;

/// Timer4 Compare high byte.
pub const T4CORH: *mut u8 = 0x83 as *mut u8;

/// Timer4 Input Capture.
pub const T4ICR: *mut u16 = 0x84 as *mut u16;

/// Timer4 Input Capture low byte.
pub const T4ICRL: *mut u8 = 0x84 as *mut u8;

/// Timer4 Input Capture high byte.
pub const T4ICRH: *mut u8 = 0x85 as *mut u8;

/// Timer4 Mode Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T4CS | 11 |
/// | T4PS | 11100 |
pub const T4MRA: *mut u8 = 0x86 as *mut u8;

/// Timer4 Mode Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T4ICS | 11100000 |
/// | T4CNC | 100 |
/// | T4CE | 11000 |
/// | T4SCE | 10 |
pub const T4MRB: *mut u8 = 0x87 as *mut u8;

/// Timer4 Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T4OIM | 1 |
/// | T4CIM | 10 |
/// | T4CPIM | 100 |
pub const T4IMR: *mut u8 = 0x88 as *mut u8;

/// Timer5 Output Compare.
pub const T5OCR: *mut u16 = 0x8A as *mut u16;

/// Timer5 Output Compare low byte.
pub const T5OCRL: *mut u8 = 0x8A as *mut u8;

/// Timer5 Output Compare high byte.
pub const T5OCRH: *mut u8 = 0x8B as *mut u8;

/// Timer5 Configuration and Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T5CTC | 1000 |
/// | T5CS | 111 |
pub const T5CCR: *mut u8 = 0x8C as *mut u8;

/// Timer5 Counter.
pub const T5CNT: *mut u16 = 0x8D as *mut u16;

/// Timer5 Counter low byte.
pub const T5CNTL: *mut u8 = 0x8D as *mut u8;

/// Timer5 Counter high byte.
pub const T5CNTH: *mut u8 = 0x8E as *mut u8;

/// Timer5 Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T5OIM | 1 |
/// | T5CIM | 10 |
pub const T5IMR: *mut u8 = 0x8F as *mut u8;

/// General Timer/Counter Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PSR10 | 1 |
/// | TSM | 10000000 |
pub const GTCCR: *mut u8 = 0x90 as *mut u8;

/// Start Of Telegram Status Path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RROB | 1000000 |
/// | SYTOB | 100 |
/// | WCOAO | 10000000 |
/// | WUPOB | 10000 |
/// | SFIDOB | 100000 |
/// | CAROB | 1 |
/// | AMPOB | 10 |
/// | MANOB | 1000 |
pub const SOTSB: *mut u8 = 0x91 as *mut u8;

/// Start Of Telegram Status Path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RROA | 1000000 |
/// | MANOA | 1000 |
/// | AMPOA | 10 |
/// | SYTOA | 100 |
/// | WUPOA | 10000 |
/// | SFIDOA | 100000 |
/// | CAROA | 1 |
/// | WCOBO | 10000000 |
pub const SOTSA: *mut u8 = 0x92 as *mut u8;

/// Start Of Telegram Conditions Path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CAROEB | 1 |
/// | AMPOEB | 10 |
/// | RROEB | 1000000 |
/// | WCOAOE | 10000000 |
/// | SYTOEB | 100 |
/// | MANOEB | 1000 |
/// | WUPEB | 10000 |
/// | SFIDEB | 100000 |
pub const SOTCB: *mut u8 = 0x93 as *mut u8;

/// Start Of Telegram Conditions Path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WUPEA | 10000 |
/// | RROEA | 1000000 |
/// | WCOBOE | 10000000 |
/// | CAROEA | 1 |
/// | MANOEA | 1000 |
/// | SYTOEA | 100 |
/// | SFIDEA | 100000 |
/// | AMPOEA | 10 |
pub const SOTCA: *mut u8 = 0x94 as *mut u8;

/// Telegram Status Register Path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EOTLB | 110 |
/// | CRCOB | 1 |
pub const TESRB: *mut u8 = 0x95 as *mut u8;

/// Telegram Status Register Path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EOTLA | 110 |
/// | CRCOA | 1 |
pub const TESRA: *mut u8 = 0x96 as *mut u8;

/// Rx DSP Status Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SOTAM | 10000 |
/// | EOTBM | 1000 |
/// | EOTAM | 100 |
/// | NBITBM | 10 |
/// | WCOBM | 10000000 |
/// | SOTBM | 100000 |
/// | NBITAM | 1 |
/// | WCOAM | 1000000 |
pub const RDSIMR: *mut u8 = 0x98 as *mut u8;

/// Rx DSP Output Control.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RDSIDB | 1000000 |
/// | TMDS | 110 |
/// | ETRPA | 1000 |
/// | ETRPB | 10000 |
/// | RDSIDA | 100000 |
pub const RDOCR: *mut u8 = 0x99 as *mut u8;

/// Temperature.
pub const TEMP: *mut u16 = 0x9B as *mut u16;

/// Temperature low byte.
pub const TEMPL: *mut u8 = 0x9B as *mut u8;

/// Temperature high byte.
pub const TEMPH: *mut u8 = 0x9C as *mut u8;

/// Symbol Check Configuration Path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SYCSB | 1111 |
/// | SYTLB | 11110000 |
pub const SYCB: *mut u8 = 0x9D as *mut u8;

/// Symbol Check Configuration Path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SYCSA | 1111 |
/// | SYTLA | 11110000 |
pub const SYCA: *mut u8 = 0x9E as *mut u8;

/// Received Frequency Offset vs Intermediate Frequency Path B.
pub const RXFOB: *mut u8 = 0x9F as *mut u8;

/// Received Frequency Offset vs Intermediate Frequency Path A.
pub const RXFOA: *mut u8 = 0xA0 as *mut u8;

/// Demodulator Signal Check Pattern Path B.
pub const DMPATB: *mut u8 = 0xA1 as *mut u8;

/// Demodulator Signal Check Pattern Path A.
pub const DMPATA: *mut u8 = 0xA2 as *mut u8;

/// Demodulator Pattern Check Control.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCENA | 1000 |
/// | PCSIGB | 1000000 |
/// | PCENB | 10000000 |
/// | PCIALA | 10 |
/// | PCIALB | 100000 |
/// | PCFTDA | 1 |
/// | PCSIGA | 100 |
/// | PCFTDB | 10000 |
pub const DMPC: *mut u8 = 0xA3 as *mut u8;

/// Demodulator Pattern Check Control Path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCSEVB | 111 |
/// | PSELB | 11000000 |
/// | PCLENB | 111000 |
pub const DMPCB: *mut u8 = 0xA4 as *mut u8;

/// Demodulator Pattern Check Control Path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCLENA | 111000 |
/// | PSELA | 11000000 |
/// | PCSEVA | 111 |
pub const DMPCA: *mut u8 = 0xA5 as *mut u8;

/// Demodulator Symbol Rate Path B.
pub const DMSRB: *mut u8 = 0xA6 as *mut u8;

/// Demodulator Symbol Rate Path A.
pub const DMSRA: *mut u8 = 0xA7 as *mut u8;

/// Demodulator Mode Path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DMNEB | 10000000 |
/// | DMPB | 100000 |
/// | DMHB | 1000000 |
/// | DMATB | 11111 |
pub const DMMB: *mut u8 = 0xA8 as *mut u8;

/// Demodulator Mode Path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DMATA | 11111 |
/// | DMHA | 1000000 |
/// | DMPA | 100000 |
/// | DMNEA | 10000000 |
pub const DMMA: *mut u8 = 0xA9 as *mut u8;

/// Demodulator Carrier Detect Path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DMCLB | 11111 |
/// | DMCTB | 11100000 |
pub const DMCDB: *mut u8 = 0xAA as *mut u8;

/// Demodulator Carrier Detect Path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DMCTA | 11100000 |
/// | DMCLA | 11111 |
pub const DMCDA: *mut u8 = 0xAB as *mut u8;

/// Demodulator Control Path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SASKB | 100000 |
/// | SY1TB | 1000000 |
/// | DMPGB | 11111 |
/// | DMARB | 10000000 |
pub const DMCRB: *mut u8 = 0xAC as *mut u8;

/// Demodulator Control Path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SASKA | 100000 |
/// | DMARA | 10000000 |
/// | SY1TA | 1000000 |
/// | DMPGA | 11111 |
pub const DMCRA: *mut u8 = 0xAD as *mut u8;

/// Demodulator Down Sampling.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DMDNB | 11110000 |
/// | DMDNA | 1111 |
pub const DMDN: *mut u8 = 0xAE as *mut u8;

/// Channel Filter Configuration Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BWM | 1111 |
pub const CHCR: *mut u8 = 0xAF as *mut u8;

/// Channel Filter Down Sampling Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADCDN | 100000 |
/// | BBDN | 11111 |
pub const CHDN: *mut u8 = 0xB0 as *mut u8;

/// Start Frame ID Configuration Path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SEMEB | 10000000 |
/// | SFIDTB | 11111 |
pub const SFIDCB: *mut u8 = 0xB1 as *mut u8;

/// Start Frame ID Length Path B.
pub const SFIDLB: *mut u8 = 0xB2 as *mut u8;

/// Wake-Up Pattern Threshold Path B.
pub const WUPTB: *mut u8 = 0xB3 as *mut u8;

/// Wake-Up Pattern Length Path B.
pub const WUPLB: *mut u8 = 0xB4 as *mut u8;

/// Start Frame ID Byte 1 Path B.
pub const SFID1B: *mut u8 = 0xB5 as *mut u8;

/// Start Frame ID Byte 2 Path B.
pub const SFID2B: *mut u8 = 0xB6 as *mut u8;

/// Start Frame ID Byte 3 Path B.
pub const SFID3B: *mut u8 = 0xB7 as *mut u8;

/// Start Frame ID Byte 4 Path B.
pub const SFID4B: *mut u8 = 0xB8 as *mut u8;

/// Wake-Up Pattern Byte 1 Path B.
pub const WUP1B: *mut u8 = 0xB9 as *mut u8;

/// Wake-Up Pattern Byte 2 Path B.
pub const WUP2B: *mut u8 = 0xBA as *mut u8;

/// Wake-Up Pattern Byte 3 Path B.
pub const WUP3B: *mut u8 = 0xBB as *mut u8;

/// Wake-Up Pattern Byte 4 Path B.
pub const WUP4B: *mut u8 = 0xBC as *mut u8;

/// Start Frame ID Configuration Path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SEMEA | 10000000 |
/// | SFIDTA | 11111 |
pub const SFIDCA: *mut u8 = 0xBD as *mut u8;

/// Start Frame ID Length Path A.
pub const SFIDLA: *mut u8 = 0xBE as *mut u8;

/// Wake-Up Pattern Threshold Path A.
pub const WUPTA: *mut u8 = 0xBF as *mut u8;

/// Wake-Up Pattern Length Path A.
pub const WUPLA: *mut u8 = 0xC0 as *mut u8;

/// Start Frame ID Byte 1 Path A.
pub const SFID1A: *mut u8 = 0xC1 as *mut u8;

/// Start Frame ID Byte 2 Path A.
pub const SFID2A: *mut u8 = 0xC2 as *mut u8;

/// Start Frame ID Byte 3 Path A.
pub const SFID3A: *mut u8 = 0xC3 as *mut u8;

/// Start Frame ID Byte 4 Path A.
pub const SFID4A: *mut u8 = 0xC4 as *mut u8;

/// Wake-Up Pattern Byte 1 Path A.
pub const WUP1A: *mut u8 = 0xC5 as *mut u8;

/// Wake-Up Pattern Byte 2 Path A.
pub const WUP2A: *mut u8 = 0xC6 as *mut u8;

/// Wake-Up Pattern Byte 3 Path A.
pub const WUP3A: *mut u8 = 0xC7 as *mut u8;

/// Wake-Up Pattern Byte 4 Path A.
pub const WUP4A: *mut u8 = 0xC8 as *mut u8;

/// Clock Output Divider.
pub const CLKOD: *mut u8 = 0xC9 as *mut u8;

/// Clock output control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CLKOEN | 100 |
/// | CLKOS | 11 |
pub const CLKOCR: *mut u8 = 0xCA as *mut u8;

/// XROW Fuse.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | E2PT | 1100 |
/// | FLPT | 11 |
/// | CKOUT | 1000000 |
/// | NVPTE | 10000 |
pub const XFUSE: *mut u8 = 0xCB as *mut u8;

/// Slow RC Oscillator Calibration.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SRCTC | 11000000 |
pub const SRCCAL: *mut u8 = 0xCC as *mut u8;

/// Fast RC Oscillator Calibration.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FRCTC | 100000 |
pub const FRCCAL: *mut u8 = 0xCD as *mut u8;

/// Clock Management Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ECF | 1 |
pub const CMSR: *mut u8 = 0xCE as *mut u8;

/// Clock Management Override Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FRCAO | 1 |
/// | SRCACT | 1000 |
/// | FRCACT | 100 |
/// | SRCAO | 10 |
pub const CMOCR: *mut u8 = 0xCF as *mut u8;

/// Supply Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DCERF | 1000 |
/// | AVCCLF | 10 |
/// | AVCCRF | 1 |
/// | DCRDYF | 100 |
pub const SUPFR: *mut u8 = 0xD0 as *mut u8;

/// Supply Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AVEN | 100000 |
/// | AVCCRM | 1 |
/// | AVDIC | 1000000 |
/// | AVCCLM | 10 |
pub const SUPCR: *mut u8 = 0xD1 as *mut u8;

/// Supply Calibration 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | VVCAL | 1100 |
/// | AVCAL | 110000 |
/// | DVCAL | 11 |
pub const SUPCA1: *mut u8 = 0xD2 as *mut u8;

/// Supply Calibration 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BGCAL | 111111 |
pub const SUPCA2: *mut u8 = 0xD3 as *mut u8;

/// Supply Calibration 3.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DMCAL | 11 |
/// | VMOCAL | 11000000 |
/// | AMCAL | 110000 |
/// | VMCAL | 1100 |
pub const SUPCA3: *mut u8 = 0xD4 as *mut u8;

/// Supply Calibration 4.
pub const SUPCA4: *mut u8 = 0xD5 as *mut u8;

/// DCDC Converter Calibration 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ZCCAL | 11110000 |
/// | CCAL | 1111 |
pub const DCCAL1: *mut u8 = 0xD6 as *mut u8;

/// DCDC Converter Calibration 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OFCAL | 11110000 |
/// | DTCAL | 1111 |
pub const DCCAL2: *mut u8 = 0xD7 as *mut u8;

/// DCDC Converter Calibration 3.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SAWCAL | 11 |
pub const DCCAL3: *mut u8 = 0xD8 as *mut u8;

/// DCDC Converter Test Mode.
pub const DCTST: *mut u8 = 0xD9 as *mut u8;

/// Calibration Ready Signature.
pub const CALRDY: *mut u8 = 0xDA as *mut u8;

/// Resistor Capacitor 4 Bit Tuning.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RTUNE4 | 11110000 |
/// | CTUNE4 | 1111 |
pub const RCTUNE4: *mut u8 = 0xDC as *mut u8;

/// DCDC Converter Control 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DCHSSW | 10 |
/// | DCCDIV | 110000 |
/// | DCEN | 1 |
pub const DCC1: *mut u8 = 0xDD as *mut u8;

/// DCDC Converter Control 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DCCLIM | 1100 |
/// | DCVOUT | 11 |
/// | DCDRV | 110000 |
pub const DCC2: *mut u8 = 0xDE as *mut u8;

/// Data FIFO Status.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DFOFL | 100 |
/// | DFUFL | 10 |
/// | DFFLRF | 1 |
pub const DFS: *mut u8 = 0xDF as *mut u8;

/// Data FIFO Telegram Length.
pub const DFTL: *mut u16 = 0xE0 as *mut u16;

/// Data FIFO Telegram Length low byte.
pub const DFTLL: *mut u8 = 0xE0 as *mut u8;

/// Data FIFO Telegram Length high byte.
pub const DFTLH: *mut u8 = 0xE1 as *mut u8;

/// Data FIFO Fill Level.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DFFLS | 111111 |
/// | DFCLR | 10000000 |
pub const DFL: *mut u8 = 0xE2 as *mut u8;

/// Data FIFO Write Pointer.
pub const DFWP: *mut u8 = 0xE3 as *mut u8;

/// Data FIFO Read Pointer.
pub const DFRP: *mut u8 = 0xE4 as *mut u8;

/// Data FIFO Data.
pub const DFD: *mut u8 = 0xE5 as *mut u8;

/// Data FIFO Interrupt Mask.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DFERIM | 10 |
/// | DFFLIM | 1 |
pub const DFI: *mut u8 = 0xE6 as *mut u8;

/// Data FIFO Configuration.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DFFLC | 111111 |
/// | DFDRA | 10000000 |
pub const DFC: *mut u8 = 0xE7 as *mut u8;

/// Support FIFO Status.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SFOFL | 100 |
/// | SFFLRF | 1 |
/// | SFUFL | 10 |
pub const SFS: *mut u8 = 0xE8 as *mut u8;

/// Support FIFO Fill Level.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SFCLR | 10000000 |
/// | SFFLS | 11111 |
pub const SFL: *mut u8 = 0xE9 as *mut u8;

/// Support FIFO Write Pointer.
pub const SFWP: *mut u8 = 0xEA as *mut u8;

/// Support FIFO Read Pointer.
pub const SFRP: *mut u8 = 0xEB as *mut u8;

/// Support FIFO Data.
pub const SFD: *mut u8 = 0xEC as *mut u8;

/// Support FIFO Interrupt Mask.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SFFLIM | 1 |
/// | SFERIM | 10 |
pub const SFI: *mut u8 = 0xED as *mut u8;

/// Support FIFO Configuration.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SFDRA | 10000000 |
/// | SFFLC | 11111 |
pub const SFC: *mut u8 = 0xEE as *mut u8;

/// Sequencer State Machine Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SETRPB | 10000000 |
/// | SSMTM | 10 |
/// | SSMTX | 1 |
/// | SETRPA | 1000000 |
pub const SSMCR: *mut u8 = 0xEF as *mut u8;

/// Sequencer State Machine Rx Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SSMPA | 1 |
/// | SSMTMOE | 10000000 |
/// | SSMPVS | 10000 |
/// | SSMIFA | 100000 |
/// | SSMAD | 100 |
/// | SSMPB | 10 |
/// | SSMHIS | 1000 |
/// | SSMIDSE | 1000000 |
pub const SSMRCR: *mut u8 = 0xF0 as *mut u8;

/// Sequencer State Machine Filter Bandwidth Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SSMHADT | 10000 |
/// | SSMDFDT | 1000 |
/// | SSMFID | 111 |
/// | SSMPLDT | 100000 |
pub const SSMFBR: *mut u8 = 0xF1 as *mut u8;

/// Sequencer State Machine Run Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SSMST | 10 |
/// | SSMR | 1 |
pub const SSMRR: *mut u8 = 0xF2 as *mut u8;

/// Sequencer State Machine Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SSMESM | 1111 |
/// | SSMERR | 10000000 |
pub const SSMSR: *mut u8 = 0xF3 as *mut u8;

/// Sequencer State Machine Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SSMIF | 1 |
pub const SSMIFR: *mut u8 = 0xF4 as *mut u8;

/// Sequencer State Machine Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SSMIM | 1 |
pub const SSMIMR: *mut u8 = 0xF5 as *mut u8;

/// Master State Machine State Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SSMMST | 11111 |
pub const MSMSTR: *mut u8 = 0xF6 as *mut u8;

/// Sequencer State Machine State Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SSMSTA | 111111 |
pub const SSMSTR: *mut u8 = 0xF7 as *mut u8;

/// Sequencer State Machine Extended State Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SSMSTB | 111111 |
pub const SSMXSR: *mut u8 = 0xF8 as *mut u8;

/// Master State Machine Control Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | MSMSM0 | 1111 |
/// | MSMSM1 | 11110000 |
pub const MSMCR1: *mut u8 = 0xF9 as *mut u8;

/// Master State Machine Control Register 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | MSMSM3 | 11110000 |
/// | MSMSM2 | 1111 |
pub const MSMCR2: *mut u8 = 0xFA as *mut u8;

/// Master State Machine Control Register 3.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | MSMSM4 | 1111 |
/// | MSMSM5 | 11110000 |
pub const MSMCR3: *mut u8 = 0xFB as *mut u8;

/// Master State Machine Control Register 4.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | MSMSM6 | 1111 |
/// | MSMSM7 | 11110000 |
pub const MSMCR4: *mut u8 = 0xFC as *mut u8;

/// Get Telegram Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RXTEHA | 1 |
/// | GAPMA | 10 |
/// | DARA | 100 |
/// | RXTEHB | 10000 |
/// | GAPMB | 100000 |
/// | DARB | 1000000 |
/// | IWUPA | 1000 |
/// | IWUPB | 10000000 |
pub const GTCR: *mut u8 = 0xFD as *mut u8;

/// RF Front End Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PLCK | 1000 |
/// | XRDY | 100 |
/// | SAT | 1 |
pub const FESR: *mut u8 = 0x100 as *mut u8;

/// RF Front End Enable 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | XTOEN | 100 |
/// | PLCAL | 10 |
/// | ADEN | 10000 |
/// | PLSP1 | 1000000 |
/// | PLEN | 1 |
/// | ADCLK | 100000 |
/// | LNAEN | 1000 |
pub const FEEN1: *mut u8 = 0x101 as *mut u8;

/// RF Front End Enable 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TMPM | 1000 |
/// | PAEN | 100 |
/// | XTPEN | 100000 |
/// | PLPEN | 10000 |
/// | XTOEXT | 10000000 |
pub const FEEN2: *mut u8 = 0x102 as *mut u8;

/// RF Front End LNA Bias.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LNABH | 11110000 |
/// | LNABN | 1111 |
pub const FELNA: *mut u8 = 0x103 as *mut u8;

/// RF Front End Antenna Switch.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SDRX1 | 1 |
/// | SDTX1 | 10 |
/// | SDRX2 | 100 |
/// | SDTX3 | 100000 |
/// | SDTX2 | 1000 |
/// | SDRX3 | 10000 |
pub const FEAS: *mut u8 = 0x104 as *mut u8;

/// RF Front End VCO Tuning.
pub const FEVCT: *mut u8 = 0x106 as *mut u8;

/// RF Front End RC Tuning.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CTN2 | 11 |
/// | RTN2 | 1100 |
pub const FEBT: *mut u8 = 0x107 as *mut u8;

/// RF Front End Main and Swallow Counter.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PLLM | 11110000 |
/// | PLLS | 1111 |
pub const FEMS: *mut u8 = 0x108 as *mut u8;

/// RF Front End RC Tuning 4bit Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CTN4 | 1111 |
/// | RTN4 | 11110000 |
pub const FETN4: *mut u8 = 0x109 as *mut u8;

/// RF Front End Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LBNHB | 1 |
/// | ANPS | 100000 |
/// | ADHS | 1000 |
/// | ANDP | 100 |
/// | S4N3 | 10 |
/// | PLCKG | 10000 |
pub const FECR: *mut u8 = 0x10A as *mut u8;

/// RF Front End VCO and PLL Control.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | VCOB | 11110000 |
/// | CPCC | 1111 |
pub const FEVCO: *mut u8 = 0x10B as *mut u8;

/// RF Front End Amplifier Bias.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | IFAEN | 10000000 |
/// | HISEN | 1000000 |
pub const FEBIA: *mut u8 = 0x10C as *mut u8;

/// RF Front End Spare Register 1.
pub const SPARE1: *mut u8 = 0x10D as *mut u8;

/// Start Of Telegram Conditions 1 Path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RROEA1 | 1000000 |
/// | SFIDEA1 | 100000 |
/// | CAROEA1 | 1 |
/// | SYTOEA1 | 100 |
/// | MANOEA1 | 1000 |
/// | AMPOEA1 | 10 |
/// | WCOBOE1 | 10000000 |
/// | WUPEA1 | 10000 |
pub const SOTC1A: *mut u8 = 0x120 as *mut u8;

/// Start Of Telegram Conditions 2 Path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | MANOEA2 | 1000 |
/// | SYTOEA2 | 100 |
/// | SFIDEA2 | 100000 |
/// | WUPEA2 | 10000 |
/// | AMPOEA2 | 10 |
/// | RROEA2 | 1000000 |
/// | CAROEA2 | 1 |
/// | WCOBOE2 | 10000000 |
pub const SOTC2A: *mut u8 = 0x121 as *mut u8;

/// Start Of Telegram Conditions 1 Path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RROEB1 | 1000000 |
/// | WCOAOE1 | 10000000 |
/// | AMPOEB1 | 10 |
/// | CAROEB1 | 1 |
/// | MANOEB1 | 1000 |
/// | SFIDEB1 | 100000 |
/// | SYTOEB1 | 100 |
/// | WUPEB1 | 10000 |
pub const SOTC1B: *mut u8 = 0x122 as *mut u8;

/// Start Of Telegram Conditions 2 Path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SFIDEB2 | 100000 |
/// | WUPEB2 | 10000 |
/// | AMPOEB2 | 10 |
/// | WCOAOE2 | 10000000 |
/// | RROEB2 | 1000000 |
/// | CAROEB2 | 1 |
/// | SYTOEB2 | 100 |
/// | MANOEB2 | 1000 |
pub const SOTC2B: *mut u8 = 0x123 as *mut u8;

/// End Of Telegram Conditions 1 Path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AMPFEA1 | 10 |
/// | MANFEA1 | 1000 |
/// | CARFEA1 | 1 |
/// | SYTFEA1 | 100 |
/// | TMOFEA1 | 10000 |
/// | TELREA1 | 100000 |
/// | RRFEA1 | 1000000 |
/// | EOTBFE1 | 10000000 |
pub const EOTC1A: *mut u8 = 0x124 as *mut u8;

/// End Of Telegram Conditions 2 Path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CARFEA2 | 1 |
/// | TELREA2 | 100000 |
/// | MANFEA2 | 1000 |
/// | TMOFEA2 | 10000 |
/// | AMPFEA2 | 10 |
/// | EOTBFE2 | 10000000 |
/// | SYTFEA2 | 100 |
/// | RRFEA2 | 1000000 |
pub const EOTC2A: *mut u8 = 0x125 as *mut u8;

/// End Of Telegram Conditions 3 Path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SYTFEA3 | 100 |
/// | TMOFEA3 | 10000 |
/// | MANFEA3 | 1000 |
/// | AMPFEA3 | 10 |
/// | RRFEA3 | 1000000 |
/// | CARFEA3 | 1 |
/// | EOTBFE3 | 10000000 |
/// | TELREA3 | 100000 |
pub const EOTC3A: *mut u8 = 0x126 as *mut u8;

/// End Of Telegram Conditions 1 Path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EOTAFE1 | 10000000 |
/// | TELREB1 | 100000 |
/// | AMPFEB1 | 10 |
/// | MANFEB1 | 1000 |
/// | TMOFEB1 | 10000 |
/// | CARFEB1 | 1 |
/// | RRFEB1 | 1000000 |
/// | SYTFEB1 | 100 |
pub const EOTC1B: *mut u8 = 0x127 as *mut u8;

/// End Of Telegram Conditions 2 Path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AMPFEB2 | 10 |
/// | MANFEB2 | 1000 |
/// | RRFEB2 | 1000000 |
/// | CARFEB2 | 1 |
/// | EOTAFE2 | 10000000 |
/// | TMOFEB2 | 10000 |
/// | SYTFEB2 | 100 |
/// | TELREB2 | 100000 |
pub const EOTC2B: *mut u8 = 0x128 as *mut u8;

/// End Of Telegram Conditions 3 Path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | MANFEB3 | 1000 |
/// | EOTAFE3 | 10000000 |
/// | CARFEB3 | 1 |
/// | AMPFEB3 | 10 |
/// | TELREB3 | 100000 |
/// | SYTFEB3 | 100 |
/// | TMOFEB3 | 10000 |
/// | RRFEB3 | 1000000 |
pub const EOTC3B: *mut u8 = 0x129 as *mut u8;

/// Wake Check Ok Time-Out Path A.
pub const WCOTOA: *mut u8 = 0x12A as *mut u8;

/// Wake Check Ok Time-Out Path B.
pub const WCOTOB: *mut u8 = 0x12B as *mut u8;

/// Start Of Telegram Time Out Path A.
pub const SOTTOA: *mut u8 = 0x12C as *mut u8;

/// Start Of Telegram Time Out Path B.
pub const SOTTOB: *mut u8 = 0x12D as *mut u8;

/// Sequencer State Machine Flow Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SSMCEH | 1000 |
/// | SSMSEH | 100 |
/// | SSMIDSO | 1 |
/// | SSMIDSF | 10 |
pub const SSMFCR: *mut u8 = 0x12E as *mut u8;

/// Rx Buffer Configuration 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RXCBLB | 1100000 |
/// | RXCEA | 1 |
/// | RXCBLA | 110 |
/// | RXLSBA | 1000 |
/// | RXLSBB | 10000000 |
/// | RXCEB | 10000 |
pub const RXBC1: *mut u8 = 0x13E as *mut u8;

/// Rx Buffer Configuration 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RXBPB | 1 |
/// | RXBCLR | 100 |
/// | RXBF | 10 |
pub const RXBC2: *mut u8 = 0x13F as *mut u8;

/// Rx Telegram Length Low Byte Path B.
pub const RXTLLB: *mut u8 = 0x140 as *mut u8;

/// Rx Telegram Length High Byte Path B.
pub const RXTLHB: *mut u8 = 0x141 as *mut u8;

/// Rx CRC Result Low Byte Path B.
pub const RXCRLB: *mut u8 = 0x142 as *mut u8;

/// Rx CRC Result High Byte Path B.
pub const RXCRHB: *mut u8 = 0x143 as *mut u8;

/// Rx CRC Skip Bit Number Path B.
pub const RXCSBB: *mut u8 = 0x144 as *mut u8;

/// Rx CRC Init Value Low Byte Path B.
pub const RXCILB: *mut u8 = 0x145 as *mut u8;

/// Rx CRC Init Value High Byte Path B.
pub const RXCIHB: *mut u8 = 0x146 as *mut u8;

/// Rx CRC Polynomial Low Byte Path B.
pub const RXCPLB: *mut u8 = 0x147 as *mut u8;

/// Rx CRC Polynomial High Byte Path B.
pub const RXCPHB: *mut u8 = 0x148 as *mut u8;

/// Receive Data Shift Register Path B.
pub const RXDSB: *mut u8 = 0x149 as *mut u8;

/// Rx Telegram Length Low Byte Path A.
pub const RXTLLA: *mut u8 = 0x14A as *mut u8;

/// Rx Telegram Length High Byte Path A.
pub const RXTLHA: *mut u8 = 0x14B as *mut u8;

/// Rx CRC Result Low Byte Path A.
pub const RXCRLA: *mut u8 = 0x14C as *mut u8;

/// Rx CRC Result High Byte Path A.
pub const RXCRHA: *mut u8 = 0x14D as *mut u8;

/// Rx CRC Skip Bit Number Path A.
pub const RXCSBA: *mut u8 = 0x14E as *mut u8;

/// Rx CRC Init Value Low Byte Path A.
pub const RXCILA: *mut u8 = 0x14F as *mut u8;

/// Rx CRC Init Value High Byte Path A.
pub const RXCIHA: *mut u8 = 0x150 as *mut u8;

/// Rx CRC Polynomial Low Byte Path A.
pub const RXCPLA: *mut u8 = 0x151 as *mut u8;

/// Rx CRC Polynomial High Byte Path A.
pub const RXCPHA: *mut u8 = 0x152 as *mut u8;

/// Receive Data Shift Register Path A.
pub const RXDSA: *mut u8 = 0x153 as *mut u8;

/// CRC Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | REFLO | 100 |
/// | CRCRS | 1 |
/// | REFLI | 10 |
pub const CRCCR: *mut u8 = 0x154 as *mut u8;

/// CRC Data Output Register.
pub const CRCDOR: *mut u8 = 0x155 as *mut u8;

/// ID Check Byte 0.
pub const IDB0: *mut u8 = 0x156 as *mut u8;

/// ID Check Byte 1.
pub const IDB1: *mut u8 = 0x157 as *mut u8;

/// ID Check Byte 2.
pub const IDB2: *mut u8 = 0x158 as *mut u8;

/// ID Check Byte 3.
pub const IDB3: *mut u8 = 0x159 as *mut u8;

/// ID Check Configuration.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | IDL | 11 |
/// | IDBO | 1100 |
/// | IDCLR | 1000000 |
/// | IDFIM | 100000 |
/// | IDCE | 10000000 |
pub const IDC: *mut u8 = 0x15A as *mut u8;

/// ID Check Status.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | IDOK | 1 |
/// | IDFULL | 10 |
pub const IDS: *mut u8 = 0x15B as *mut u8;

/// RSSI Average Value.
pub const RSSAV: *mut u8 = 0x15C as *mut u8;

/// RSSI Peak Value.
pub const RSSPK: *mut u8 = 0x15D as *mut u8;

/// RSSI Low Threshold.
pub const RSSL: *mut u8 = 0x15E as *mut u8;

/// RSSI High Threshold.
pub const RSSH: *mut u8 = 0x15F as *mut u8;

/// RSSI Configuration Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RSPKF | 1000000 |
/// | RSUP | 1111 |
/// | RSWLH | 10000 |
/// | RSHRX | 100000 |
pub const RSSC: *mut u8 = 0x160 as *mut u8;

/// Debounce Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DBTMS | 100 |
/// | DBMD | 1 |
/// | DBCS | 10 |
pub const DBCR: *mut u8 = 0x161 as *mut u8;

/// Debounce Timer Compare.
pub const DBTC: *mut u8 = 0x162 as *mut u8;

/// Debounce Enable Port B.
pub const DBENB: *mut u8 = 0x163 as *mut u8;

/// Debounce Enable Port C.
pub const DBENC: *mut u8 = 0x164 as *mut u8;

/// Debug Support Switch.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CPBF | 1000000 |
/// | DBGGS | 1111 |
/// | CPBFOS | 110000 |
/// | DBGSE | 10000000 |
pub const DBGSW: *mut u8 = 0x165 as *mut u8;

/// SPI FIFO Fill Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RFC | 1000 |
/// | TFC | 10000000 |
/// | RFL | 111 |
/// | TFL | 1110000 |
pub const SFFR: *mut u8 = 0x166 as *mut u8;

/// SPI FIFO Interrupt Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RIL | 111 |
/// | SRIE | 1000 |
/// | STIE | 10000000 |
/// | TIL | 1110000 |
pub const SFIR: *mut u8 = 0x167 as *mut u8;

/// EEPROM Control Register 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EEFF | 1000000 |
/// | EECF | 10000000 |
/// | EEBRE | 1 |
pub const EECR2: *mut u8 = 0x168 as *mut u8;

/// Program Memory Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PGMSYN | 11111 |
pub const PGMST: *mut u8 = 0x169 as *mut u8;

/// EEPROM Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EESYN | 1111 |
pub const EEST: *mut u8 = 0x16A as *mut u8;

/// RSSI LNA High Sensitivity Gain.
pub const RSHSG: *mut u8 = 0x16B as *mut u8;

/// RSSI IF Amplifier Gain.
pub const RSIFG: *mut u8 = 0x16C as *mut u8;

/// RSSI Low Band Damping Value.
pub const RSLDV: *mut u8 = 0x16D as *mut u8;

/// RSSI High Band Damping Value.
pub const RSHDV: *mut u8 = 0x16E as *mut u8;

/// RSSI Compensation Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RSIFC | 10 |
/// | RSDC | 1 |
/// | RSHISC | 100 |
pub const RSCOM: *mut u8 = 0x16F as *mut u8;

/// Oscillator Calibration Counter Configuration Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCSEL | 10 |
/// | OCEN | 1 |
pub const OCCR: *mut u8 = 0x170 as *mut u8;

/// Oscillator Calibration Counter Value.
pub const OCCNT: *mut u8 = 0x171 as *mut u8;

/// Oscillator Calibration Counter Gate.
pub const OCGATE: *mut u8 = 0x172 as *mut u8;

/// LIN/UART Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LCMD | 111 |
/// | LSWRES | 10000000 |
/// | LCONF | 110000 |
/// | LENA | 1000 |
/// | LIN13 | 1000000 |
pub const LINCR: *mut u8 = 0x173 as *mut u8;

/// LIN Status and Interrupt Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LTXOK | 10 |
/// | LIDOK | 100 |
/// | LERR | 1000 |
/// | LIDST | 11100000 |
/// | LBUSY | 10000 |
/// | LRXOK | 1 |
pub const LINSIR: *mut u8 = 0x174 as *mut u8;

/// LIN/UART Enable Interrupt Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LENRXOK | 1 |
/// | LENERR | 1000 |
/// | LENIDOK | 100 |
/// | LENTXOK | 10 |
pub const LINENIR: *mut u8 = 0x175 as *mut u8;

/// LIN/UART Error Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LFERR | 10000 |
/// | LBERR | 1 |
/// | LCERR | 10 |
/// | LPERR | 100 |
/// | LOVERR | 100000 |
/// | LTOERR | 1000000 |
/// | LSERR | 1000 |
/// | LABORT | 10000000 |
pub const LINERR: *mut u8 = 0x176 as *mut u8;

/// LIN/UART Bit Timing Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LBT | 111111 |
/// | LDISR | 10000000 |
pub const LINBTR: *mut u8 = 0x177 as *mut u8;

/// LIN/UART Baud Rate Register Low Byte.
pub const LINBRRL: *mut u8 = 0x178 as *mut u8;

/// LIN/UART Baud Rate Register High Byte.
pub const LINBRRH: *mut u8 = 0x179 as *mut u8;

/// LIN/UART Data Length Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LRXDL | 1111 |
/// | LTXDL | 11110000 |
pub const LINDLR: *mut u8 = 0x17A as *mut u8;

/// LIN/UART Identifier Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LID | 111111 |
/// | LP | 11000000 |
pub const LINIDR: *mut u8 = 0x17B as *mut u8;

/// LIN/UART Data Buffer Selection.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LAINC | 1000 |
/// | LINDX | 111 |
pub const LINSEL: *mut u8 = 0x17C as *mut u8;

/// LIN/UART Data Register.
pub const LINDAT: *mut u8 = 0x17D as *mut u8;

/// Trace Unit Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TRCEN | 10 |
/// | TRCCE | 1 |
pub const TRCCR: *mut u8 = 0x17E as *mut u8;

/// Trace Unit Data Register.
pub const TRCDR: *mut u8 = 0x17F as *mut u8;

/// Trace Identifier.
pub const TRCID: *mut u16 = 0x180 as *mut u16;

/// Trace Identifier low byte.
pub const TRCIDL: *mut u8 = 0x180 as *mut u8;

/// Trace Identifier high byte.
pub const TRCIDH: *mut u8 = 0x181 as *mut u8;

/// Bitfield on register `CHCR`
pub const BWM: u8 = 0xF;

/// Bitfield on register `CHDN`
pub const ADCDN: u8 = 0x20;

/// Bitfield on register `CHDN`
pub const BBDN: u8 = 0x1F;

/// Bitfield on register `CLKOCR`
pub const CLKOEN: u8 = 0x4;

/// Bitfield on register `CLKOCR`
pub const CLKOS: u8 = 0x3;

/// Bitfield on register `CLPR`
pub const CLTPS: u8 = 0x38;

/// Bitfield on register `CLPR`
pub const CLKPS: u8 = 0x7;

/// Bitfield on register `CLPR`
pub const CLPCE: u8 = 0x80;

/// Bitfield on register `CMCR`
pub const CCS: u8 = 0x8;

/// Bitfield on register `CMCR`
pub const CMONEN: u8 = 0x40;

/// Bitfield on register `CMCR`
pub const CMM: u8 = 0x7;

/// Bitfield on register `CMCR`
pub const CMCCE: u8 = 0x80;

/// Bitfield on register `CMIMR`
pub const ECIE: u8 = 0x1;

/// Bitfield on register `CMOCR`
pub const FRCAO: u8 = 0x1;

/// Bitfield on register `CMOCR`
pub const SRCACT: u8 = 0x8;

/// Bitfield on register `CMOCR`
pub const FRCACT: u8 = 0x4;

/// Bitfield on register `CMOCR`
pub const SRCAO: u8 = 0x2;

/// Bitfield on register `CMSR`
pub const ECF: u8 = 0x1;

/// Bitfield on register `CRCCR`
pub const REFLO: u8 = 0x4;

/// Bitfield on register `CRCCR`
pub const CRCRS: u8 = 0x1;

/// Bitfield on register `CRCCR`
pub const REFLI: u8 = 0x2;

/// Bitfield on register `DBCR`
pub const DBTMS: u8 = 0x4;

/// Bitfield on register `DBCR`
pub const DBMD: u8 = 0x1;

/// Bitfield on register `DBCR`
pub const DBCS: u8 = 0x2;

/// Bitfield on register `DBGSW`
pub const CPBF: u8 = 0x40;

/// Bitfield on register `DBGSW`
pub const DBGGS: u8 = 0xF;

/// Bitfield on register `DBGSW`
pub const CPBFOS: u8 = 0x30;

/// Bitfield on register `DBGSW`
pub const DBGSE: u8 = 0x80;

/// Bitfield on register `DCC1`
pub const DCHSSW: u8 = 0x2;

/// Bitfield on register `DCC1`
pub const DCCDIV: u8 = 0x30;

/// Bitfield on register `DCC1`
pub const DCEN: u8 = 0x1;

/// Bitfield on register `DCC2`
pub const DCCLIM: u8 = 0xC;

/// Bitfield on register `DCC2`
pub const DCVOUT: u8 = 0x3;

/// Bitfield on register `DCC2`
pub const DCDRV: u8 = 0x30;

/// Bitfield on register `DCCAL1`
pub const ZCCAL: u8 = 0xF0;

/// Bitfield on register `DCCAL1`
pub const CCAL: u8 = 0xF;

/// Bitfield on register `DCCAL2`
pub const OFCAL: u8 = 0xF0;

/// Bitfield on register `DCCAL2`
pub const DTCAL: u8 = 0xF;

/// Bitfield on register `DCCAL3`
pub const SAWCAL: u8 = 0x3;

/// Bitfield on register `DFC`
pub const DFFLC: u8 = 0x3F;

/// Bitfield on register `DFC`
pub const DFDRA: u8 = 0x80;

/// Bitfield on register `DFI`
pub const DFERIM: u8 = 0x2;

/// Bitfield on register `DFI`
pub const DFFLIM: u8 = 0x1;

/// Bitfield on register `DFL`
pub const DFFLS: u8 = 0x3F;

/// Bitfield on register `DFL`
pub const DFCLR: u8 = 0x80;

/// Bitfield on register `DFS`
pub const DFOFL: u8 = 0x4;

/// Bitfield on register `DFS`
pub const DFUFL: u8 = 0x2;

/// Bitfield on register `DFS`
pub const DFFLRF: u8 = 0x1;

/// Bitfield on register `DMCDA`
pub const DMCTA: u8 = 0xE0;

/// Bitfield on register `DMCDA`
pub const DMCLA: u8 = 0x1F;

/// Bitfield on register `DMCDB`
pub const DMCLB: u8 = 0x1F;

/// Bitfield on register `DMCDB`
pub const DMCTB: u8 = 0xE0;

/// Bitfield on register `DMCRA`
pub const SASKA: u8 = 0x20;

/// Bitfield on register `DMCRA`
pub const DMARA: u8 = 0x80;

/// Bitfield on register `DMCRA`
pub const SY1TA: u8 = 0x40;

/// Bitfield on register `DMCRA`
pub const DMPGA: u8 = 0x1F;

/// Bitfield on register `DMCRB`
pub const SASKB: u8 = 0x20;

/// Bitfield on register `DMCRB`
pub const SY1TB: u8 = 0x40;

/// Bitfield on register `DMCRB`
pub const DMPGB: u8 = 0x1F;

/// Bitfield on register `DMCRB`
pub const DMARB: u8 = 0x80;

/// Bitfield on register `DMDN`
pub const DMDNB: u8 = 0xF0;

/// Bitfield on register `DMDN`
pub const DMDNA: u8 = 0xF;

/// Bitfield on register `DMMA`
pub const DMATA: u8 = 0x1F;

/// Bitfield on register `DMMA`
pub const DMHA: u8 = 0x40;

/// Bitfield on register `DMMA`
pub const DMPA: u8 = 0x20;

/// Bitfield on register `DMMA`
pub const DMNEA: u8 = 0x80;

/// Bitfield on register `DMMB`
pub const DMNEB: u8 = 0x80;

/// Bitfield on register `DMMB`
pub const DMPB: u8 = 0x20;

/// Bitfield on register `DMMB`
pub const DMHB: u8 = 0x40;

/// Bitfield on register `DMMB`
pub const DMATB: u8 = 0x1F;

/// Bitfield on register `DMPC`
pub const PCENA: u8 = 0x8;

/// Bitfield on register `DMPC`
pub const PCSIGB: u8 = 0x40;

/// Bitfield on register `DMPC`
pub const PCENB: u8 = 0x80;

/// Bitfield on register `DMPC`
pub const PCIALA: u8 = 0x2;

/// Bitfield on register `DMPC`
pub const PCIALB: u8 = 0x20;

/// Bitfield on register `DMPC`
pub const PCFTDA: u8 = 0x1;

/// Bitfield on register `DMPC`
pub const PCSIGA: u8 = 0x4;

/// Bitfield on register `DMPC`
pub const PCFTDB: u8 = 0x10;

/// Bitfield on register `DMPCA`
pub const PCLENA: u8 = 0x38;

/// Bitfield on register `DMPCA`
pub const PSELA: u8 = 0xC0;

/// Bitfield on register `DMPCA`
pub const PCSEVA: u8 = 0x7;

/// Bitfield on register `DMPCB`
pub const PCSEVB: u8 = 0x7;

/// Bitfield on register `DMPCB`
pub const PSELB: u8 = 0xC0;

/// Bitfield on register `DMPCB`
pub const PCLENB: u8 = 0x38;

/// Bitfield on register `EECR`
pub const EERIE: u8 = 0x8;

/// Bitfield on register `EECR`
pub const EEWE: u8 = 0x2;

/// Bitfield on register `EECR`
pub const EEMWE: u8 = 0x4;

/// Bitfield on register `EECR`
pub const EERE: u8 = 0x1;

/// Bitfield on register `EECR`
pub const EEPM: u8 = 0x30;

/// Bitfield on register `EECR`
pub const NVMBSY: u8 = 0x80;

/// Bitfield on register `EECR`
pub const EEPAGE: u8 = 0x40;

/// Bitfield on register `EECR2`
pub const EEFF: u8 = 0x40;

/// Bitfield on register `EECR2`
pub const EECF: u8 = 0x80;

/// Bitfield on register `EECR2`
pub const EEBRE: u8 = 0x1;

/// Bitfield on register `EEPR`
pub const EEAP: u8 = 0xF;

/// Bitfield on register `EEST`
pub const EESYN: u8 = 0xF;

/// Bitfield on register `EICRA`
pub const ISC0: u8 = 0x3;

/// Bitfield on register `EICRA`
pub const ISC1: u8 = 0xC;

/// Bitfield on register `EIFR`
pub const INTF1: u8 = 0x2;

/// Bitfield on register `EIFR`
pub const INTF0: u8 = 0x1;

/// Bitfield on register `EIMSK`
pub const INT0: u8 = 0x1;

/// Bitfield on register `EIMSK`
pub const INT1: u8 = 0x2;

/// Bitfield on register `EOTC1A`
pub const AMPFEA1: u8 = 0x2;

/// Bitfield on register `EOTC1A`
pub const MANFEA1: u8 = 0x8;

/// Bitfield on register `EOTC1A`
pub const CARFEA1: u8 = 0x1;

/// Bitfield on register `EOTC1A`
pub const SYTFEA1: u8 = 0x4;

/// Bitfield on register `EOTC1A`
pub const TMOFEA1: u8 = 0x10;

/// Bitfield on register `EOTC1A`
pub const TELREA1: u8 = 0x20;

/// Bitfield on register `EOTC1A`
pub const RRFEA1: u8 = 0x40;

/// Bitfield on register `EOTC1A`
pub const EOTBFE1: u8 = 0x80;

/// Bitfield on register `EOTC1B`
pub const EOTAFE1: u8 = 0x80;

/// Bitfield on register `EOTC1B`
pub const TELREB1: u8 = 0x20;

/// Bitfield on register `EOTC1B`
pub const AMPFEB1: u8 = 0x2;

/// Bitfield on register `EOTC1B`
pub const MANFEB1: u8 = 0x8;

/// Bitfield on register `EOTC1B`
pub const TMOFEB1: u8 = 0x10;

/// Bitfield on register `EOTC1B`
pub const CARFEB1: u8 = 0x1;

/// Bitfield on register `EOTC1B`
pub const RRFEB1: u8 = 0x40;

/// Bitfield on register `EOTC1B`
pub const SYTFEB1: u8 = 0x4;

/// Bitfield on register `EOTC2A`
pub const CARFEA2: u8 = 0x1;

/// Bitfield on register `EOTC2A`
pub const TELREA2: u8 = 0x20;

/// Bitfield on register `EOTC2A`
pub const MANFEA2: u8 = 0x8;

/// Bitfield on register `EOTC2A`
pub const TMOFEA2: u8 = 0x10;

/// Bitfield on register `EOTC2A`
pub const AMPFEA2: u8 = 0x2;

/// Bitfield on register `EOTC2A`
pub const EOTBFE2: u8 = 0x80;

/// Bitfield on register `EOTC2A`
pub const SYTFEA2: u8 = 0x4;

/// Bitfield on register `EOTC2A`
pub const RRFEA2: u8 = 0x40;

/// Bitfield on register `EOTC2B`
pub const AMPFEB2: u8 = 0x2;

/// Bitfield on register `EOTC2B`
pub const MANFEB2: u8 = 0x8;

/// Bitfield on register `EOTC2B`
pub const RRFEB2: u8 = 0x40;

/// Bitfield on register `EOTC2B`
pub const CARFEB2: u8 = 0x1;

/// Bitfield on register `EOTC2B`
pub const EOTAFE2: u8 = 0x80;

/// Bitfield on register `EOTC2B`
pub const TMOFEB2: u8 = 0x10;

/// Bitfield on register `EOTC2B`
pub const SYTFEB2: u8 = 0x4;

/// Bitfield on register `EOTC2B`
pub const TELREB2: u8 = 0x20;

/// Bitfield on register `EOTC3A`
pub const SYTFEA3: u8 = 0x4;

/// Bitfield on register `EOTC3A`
pub const TMOFEA3: u8 = 0x10;

/// Bitfield on register `EOTC3A`
pub const MANFEA3: u8 = 0x8;

/// Bitfield on register `EOTC3A`
pub const AMPFEA3: u8 = 0x2;

/// Bitfield on register `EOTC3A`
pub const RRFEA3: u8 = 0x40;

/// Bitfield on register `EOTC3A`
pub const CARFEA3: u8 = 0x1;

/// Bitfield on register `EOTC3A`
pub const EOTBFE3: u8 = 0x80;

/// Bitfield on register `EOTC3A`
pub const TELREA3: u8 = 0x20;

/// Bitfield on register `EOTC3B`
pub const MANFEB3: u8 = 0x8;

/// Bitfield on register `EOTC3B`
pub const EOTAFE3: u8 = 0x80;

/// Bitfield on register `EOTC3B`
pub const CARFEB3: u8 = 0x1;

/// Bitfield on register `EOTC3B`
pub const AMPFEB3: u8 = 0x2;

/// Bitfield on register `EOTC3B`
pub const TELREB3: u8 = 0x20;

/// Bitfield on register `EOTC3B`
pub const SYTFEB3: u8 = 0x4;

/// Bitfield on register `EOTC3B`
pub const TMOFEB3: u8 = 0x10;

/// Bitfield on register `EOTC3B`
pub const RRFEB3: u8 = 0x40;

/// Bitfield on register `EOTCA`
pub const MANFEA: u8 = 0x8;

/// Bitfield on register `EOTCA`
pub const AMPFEA: u8 = 0x2;

/// Bitfield on register `EOTCA`
pub const RRFEA: u8 = 0x40;

/// Bitfield on register `EOTCA`
pub const CARFEA: u8 = 0x1;

/// Bitfield on register `EOTCA`
pub const SYTFEA: u8 = 0x4;

/// Bitfield on register `EOTCA`
pub const TELREA: u8 = 0x20;

/// Bitfield on register `EOTCA`
pub const TMOFEA: u8 = 0x10;

/// Bitfield on register `EOTCA`
pub const EOTBFE: u8 = 0x80;

/// Bitfield on register `EOTCB`
pub const AMPFEB: u8 = 0x2;

/// Bitfield on register `EOTCB`
pub const TELREB: u8 = 0x20;

/// Bitfield on register `EOTCB`
pub const MANFEB: u8 = 0x8;

/// Bitfield on register `EOTCB`
pub const RRFEB: u8 = 0x40;

/// Bitfield on register `EOTCB`
pub const SYTFEB: u8 = 0x4;

/// Bitfield on register `EOTCB`
pub const CARFEB: u8 = 0x1;

/// Bitfield on register `EOTCB`
pub const TMOFEB: u8 = 0x10;

/// Bitfield on register `EOTCB`
pub const EOTAFE: u8 = 0x80;

/// Bitfield on register `EOTSA`
pub const RRFA: u8 = 0x40;

/// Bitfield on register `EOTSA`
pub const EOTBF: u8 = 0x80;

/// Bitfield on register `EOTSA`
pub const TMOFA: u8 = 0x10;

/// Bitfield on register `EOTSA`
pub const CARFA: u8 = 0x1;

/// Bitfield on register `EOTSA`
pub const AMPFA: u8 = 0x2;

/// Bitfield on register `EOTSA`
pub const MANFA: u8 = 0x8;

/// Bitfield on register `EOTSA`
pub const TELRA: u8 = 0x20;

/// Bitfield on register `EOTSA`
pub const SYTFA: u8 = 0x4;

/// Bitfield on register `EOTSB`
pub const EOTAF: u8 = 0x80;

/// Bitfield on register `EOTSB`
pub const TMOFB: u8 = 0x10;

/// Bitfield on register `EOTSB`
pub const MANFB: u8 = 0x8;

/// Bitfield on register `EOTSB`
pub const RRFB: u8 = 0x40;

/// Bitfield on register `EOTSB`
pub const AMPFB: u8 = 0x2;

/// Bitfield on register `EOTSB`
pub const TELRB: u8 = 0x20;

/// Bitfield on register `EOTSB`
pub const SYTFB: u8 = 0x4;

/// Bitfield on register `EOTSB`
pub const CARFB: u8 = 0x1;

/// Bitfield on register `FEAS`
pub const SDRX1: u8 = 0x1;

/// Bitfield on register `FEAS`
pub const SDTX1: u8 = 0x2;

/// Bitfield on register `FEAS`
pub const SDRX2: u8 = 0x4;

/// Bitfield on register `FEAS`
pub const SDTX3: u8 = 0x20;

/// Bitfield on register `FEAS`
pub const SDTX2: u8 = 0x8;

/// Bitfield on register `FEAS`
pub const SDRX3: u8 = 0x10;

/// Bitfield on register `FEBIA`
pub const IFAEN: u8 = 0x80;

/// Bitfield on register `FEBIA`
pub const HISEN: u8 = 0x40;

/// Bitfield on register `FEBT`
pub const CTN2: u8 = 0x3;

/// Bitfield on register `FEBT`
pub const RTN2: u8 = 0xC;

/// Bitfield on register `FECR`
pub const LBNHB: u8 = 0x1;

/// Bitfield on register `FECR`
pub const ANPS: u8 = 0x20;

/// Bitfield on register `FECR`
pub const ADHS: u8 = 0x8;

/// Bitfield on register `FECR`
pub const ANDP: u8 = 0x4;

/// Bitfield on register `FECR`
pub const S4N3: u8 = 0x2;

/// Bitfield on register `FECR`
pub const PLCKG: u8 = 0x10;

/// Bitfield on register `FEEN1`
pub const XTOEN: u8 = 0x4;

/// Bitfield on register `FEEN1`
pub const PLCAL: u8 = 0x2;

/// Bitfield on register `FEEN1`
pub const ADEN: u8 = 0x10;

/// Bitfield on register `FEEN1`
pub const PLSP1: u8 = 0x40;

/// Bitfield on register `FEEN1`
pub const PLEN: u8 = 0x1;

/// Bitfield on register `FEEN1`
pub const ADCLK: u8 = 0x20;

/// Bitfield on register `FEEN1`
pub const LNAEN: u8 = 0x8;

/// Bitfield on register `FEEN2`
pub const TMPM: u8 = 0x8;

/// Bitfield on register `FEEN2`
pub const PAEN: u8 = 0x4;

/// Bitfield on register `FEEN2`
pub const XTPEN: u8 = 0x20;

/// Bitfield on register `FEEN2`
pub const PLPEN: u8 = 0x10;

/// Bitfield on register `FEEN2`
pub const XTOEXT: u8 = 0x80;

/// Bitfield on register `FELNA`
pub const LNABH: u8 = 0xF0;

/// Bitfield on register `FELNA`
pub const LNABN: u8 = 0xF;

/// Bitfield on register `FEMS`
pub const PLLM: u8 = 0xF0;

/// Bitfield on register `FEMS`
pub const PLLS: u8 = 0xF;

/// Bitfield on register `FESR`
pub const PLCK: u8 = 0x8;

/// Bitfield on register `FESR`
pub const XRDY: u8 = 0x4;

/// Bitfield on register `FESR`
pub const SAT: u8 = 0x1;

/// Bitfield on register `FETN4`
pub const CTN4: u8 = 0xF;

/// Bitfield on register `FETN4`
pub const RTN4: u8 = 0xF0;

/// Bitfield on register `FEVCO`
pub const VCOB: u8 = 0xF0;

/// Bitfield on register `FEVCO`
pub const CPCC: u8 = 0xF;

/// Bitfield on register `FRCCAL`
pub const FRCTC: u8 = 0x20;

/// Bitfield on register `FSEN`
pub const SDEN: u8 = 0x2;

/// Bitfield on register `FSEN`
pub const SDPU: u8 = 0x1;

/// Bitfield on register `GTCCR`
pub const PSR10: u8 = 0x1;

/// Bitfield on register `GTCCR`
pub const TSM: u8 = 0x80;

/// Bitfield on register `GTCR`
pub const RXTEHA: u8 = 0x1;

/// Bitfield on register `GTCR`
pub const GAPMA: u8 = 0x2;

/// Bitfield on register `GTCR`
pub const DARA: u8 = 0x4;

/// Bitfield on register `GTCR`
pub const RXTEHB: u8 = 0x10;

/// Bitfield on register `GTCR`
pub const GAPMB: u8 = 0x20;

/// Bitfield on register `GTCR`
pub const DARB: u8 = 0x40;

/// Bitfield on register `GTCR`
pub const IWUPA: u8 = 0x8;

/// Bitfield on register `GTCR`
pub const IWUPB: u8 = 0x80;

/// Bitfield on register `IDC`
pub const IDL: u8 = 0x3;

/// Bitfield on register `IDC`
pub const IDBO: u8 = 0xC;

/// Bitfield on register `IDC`
pub const IDCLR: u8 = 0x40;

/// Bitfield on register `IDC`
pub const IDFIM: u8 = 0x20;

/// Bitfield on register `IDC`
pub const IDCE: u8 = 0x80;

/// Bitfield on register `IDS`
pub const IDOK: u8 = 0x1;

/// Bitfield on register `IDS`
pub const IDFULL: u8 = 0x2;

/// Bitfield on register `LINBTR`
pub const LBT: u8 = 0x3F;

/// Bitfield on register `LINBTR`
pub const LDISR: u8 = 0x80;

/// Bitfield on register `LINCR`
pub const LCMD: u8 = 0x7;

/// Bitfield on register `LINCR`
pub const LSWRES: u8 = 0x80;

/// Bitfield on register `LINCR`
pub const LCONF: u8 = 0x30;

/// Bitfield on register `LINCR`
pub const LENA: u8 = 0x8;

/// Bitfield on register `LINCR`
pub const LIN13: u8 = 0x40;

/// Bitfield on register `LINDLR`
pub const LRXDL: u8 = 0xF;

/// Bitfield on register `LINDLR`
pub const LTXDL: u8 = 0xF0;

/// Bitfield on register `LINENIR`
pub const LENRXOK: u8 = 0x1;

/// Bitfield on register `LINENIR`
pub const LENERR: u8 = 0x8;

/// Bitfield on register `LINENIR`
pub const LENIDOK: u8 = 0x4;

/// Bitfield on register `LINENIR`
pub const LENTXOK: u8 = 0x2;

/// Bitfield on register `LINERR`
pub const LFERR: u8 = 0x10;

/// Bitfield on register `LINERR`
pub const LBERR: u8 = 0x1;

/// Bitfield on register `LINERR`
pub const LCERR: u8 = 0x2;

/// Bitfield on register `LINERR`
pub const LPERR: u8 = 0x4;

/// Bitfield on register `LINERR`
pub const LOVERR: u8 = 0x20;

/// Bitfield on register `LINERR`
pub const LTOERR: u8 = 0x40;

/// Bitfield on register `LINERR`
pub const LSERR: u8 = 0x8;

/// Bitfield on register `LINERR`
pub const LABORT: u8 = 0x80;

/// Bitfield on register `LINIDR`
pub const LID: u8 = 0x3F;

/// Bitfield on register `LINIDR`
pub const LP: u8 = 0xC0;

/// Bitfield on register `LINSEL`
pub const LAINC: u8 = 0x8;

/// Bitfield on register `LINSEL`
pub const LINDX: u8 = 0x7;

/// Bitfield on register `LINSIR`
pub const LTXOK: u8 = 0x2;

/// Bitfield on register `LINSIR`
pub const LIDOK: u8 = 0x4;

/// Bitfield on register `LINSIR`
pub const LERR: u8 = 0x8;

/// Bitfield on register `LINSIR`
pub const LIDST: u8 = 0xE0;

/// Bitfield on register `LINSIR`
pub const LBUSY: u8 = 0x10;

/// Bitfield on register `LINSIR`
pub const LRXOK: u8 = 0x1;

/// Bitfield on register `LOCKBIT`
pub const BLB0: u8 = 0xC;

/// Bitfield on register `LOCKBIT`
pub const BLB1: u8 = 0x30;

/// Bitfield on register `LOCKBIT`
pub const LB: u8 = 0x3;

/// Bitfield on register `LOW`
pub const RSTDISBL: u8 = 0x2;

/// Bitfield on register `LOW`
pub const BOOTRST: u8 = 0x4;

/// Bitfield on register `LOW`
pub const EXTCLKEN: u8 = 0x1;

/// Bitfield on register `LOW`
pub const DWEN: u8 = 0x40;

/// Bitfield on register `LOW`
pub const EESAVE: u8 = 0x8;

/// Bitfield on register `LOW`
pub const SPIEN: u8 = 0x20;

/// Bitfield on register `LOW`
pub const WDTON: u8 = 0x10;

/// Bitfield on register `LOW`
pub const CKDIV8: u8 = 0x80;

/// Bitfield on register `MCUCR`
pub const SPIIO: u8 = 0x4;

/// Bitfield on register `MCUCR`
pub const IVCE: u8 = 0x1;

/// Bitfield on register `MCUCR`
pub const ENPS: u8 = 0x8;

/// Bitfield on register `MCUCR`
pub const PB7HS: u8 = 0x80;

/// Bitfield on register `MCUCR`
pub const PUD: u8 = 0x10;

/// Bitfield on register `MCUCR`
pub const PB4HS: u8 = 0x20;

/// Bitfield on register `MCUCR`
pub const PB7LS: u8 = 0x40;

/// Bitfield on register `MCUCR`
pub const IVSEL: u8 = 0x2;

/// Bitfield on register `MCUSR`
pub const WDRF: u8 = 0x8;

/// Bitfield on register `MCUSR`
pub const DWRF: u8 = 0x10;

/// Bitfield on register `MCUSR`
pub const PORF: u8 = 0x1;

/// Bitfield on register `MCUSR`
pub const EXTRF: u8 = 0x2;

/// Bitfield on register `MSMCR1`
pub const MSMSM0: u8 = 0xF;

/// Bitfield on register `MSMCR1`
pub const MSMSM1: u8 = 0xF0;

/// Bitfield on register `MSMCR2`
pub const MSMSM3: u8 = 0xF0;

/// Bitfield on register `MSMCR2`
pub const MSMSM2: u8 = 0xF;

/// Bitfield on register `MSMCR3`
pub const MSMSM4: u8 = 0xF;

/// Bitfield on register `MSMCR3`
pub const MSMSM5: u8 = 0xF0;

/// Bitfield on register `MSMCR4`
pub const MSMSM6: u8 = 0xF;

/// Bitfield on register `MSMCR4`
pub const MSMSM7: u8 = 0xF0;

/// Bitfield on register `MSMSTR`
pub const SSMMST: u8 = 0x1F;

/// Bitfield on register `OCCR`
pub const OCSEL: u8 = 0x2;

/// Bitfield on register `OCCR`
pub const OCEN: u8 = 0x1;

/// Bitfield on register `PCICR`
pub const PCIE1: u8 = 0x2;

/// Bitfield on register `PCICR`
pub const PCIE0: u8 = 0x1;

/// Bitfield on register `PCIFR`
pub const PCIF0: u8 = 0x1;

/// Bitfield on register `PCIFR`
pub const PCIF1: u8 = 0x2;

/// Bitfield on register `PGMST`
pub const PGMSYN: u8 = 0x1F;

/// Bitfield on register `PRR0`
pub const PRSPI: u8 = 0x1;

/// Bitfield on register `PRR0`
pub const PRTXDC: u8 = 0x4;

/// Bitfield on register `PRR0`
pub const PRUART: u8 = 0x40;

/// Bitfield on register `PRR0`
pub const PRRXDC: u8 = 0x2;

/// Bitfield on register `PRR0`
pub const PRCRC: u8 = 0x8;

/// Bitfield on register `PRR0`
pub const PRCO: u8 = 0x20;

/// Bitfield on register `PRR0`
pub const PRTRC: u8 = 0x80;

/// Bitfield on register `PRR0`
pub const PRVM: u8 = 0x10;

/// Bitfield on register `PRR1`
pub const PRT2: u8 = 0x2;

/// Bitfield on register `PRR1`
pub const PRT4: u8 = 0x8;

/// Bitfield on register `PRR1`
pub const PRT3: u8 = 0x4;

/// Bitfield on register `PRR1`
pub const PRT5: u8 = 0x10;

/// Bitfield on register `PRR1`
pub const PRT1: u8 = 0x1;

/// Bitfield on register `PRR2`
pub const PRSF: u8 = 0x4;

/// Bitfield on register `PRR2`
pub const PRXB: u8 = 0x1;

/// Bitfield on register `PRR2`
pub const PRIDS: u8 = 0x10;

/// Bitfield on register `PRR2`
pub const PRSSM: u8 = 0x80;

/// Bitfield on register `PRR2`
pub const PRDF: u8 = 0x8;

/// Bitfield on register `PRR2`
pub const PRRS: u8 = 0x20;

/// Bitfield on register `PRR2`
pub const PRXA: u8 = 0x2;

/// Bitfield on register `RCTUNE4`
pub const RTUNE4: u8 = 0xF0;

/// Bitfield on register `RCTUNE4`
pub const CTUNE4: u8 = 0xF;

/// Bitfield on register `RDCR`
pub const RDPU: u8 = 0x1;

/// Bitfield on register `RDCR`
pub const ADIVEN: u8 = 0x2;

/// Bitfield on register `RDCR`
pub const RDEN: u8 = 0x4;

/// Bitfield on register `RDOCR`
pub const RDSIDB: u8 = 0x40;

/// Bitfield on register `RDOCR`
pub const TMDS: u8 = 0x6;

/// Bitfield on register `RDOCR`
pub const ETRPA: u8 = 0x8;

/// Bitfield on register `RDOCR`
pub const ETRPB: u8 = 0x10;

/// Bitfield on register `RDOCR`
pub const RDSIDA: u8 = 0x20;

/// Bitfield on register `RDPR`
pub const APRPTB: u8 = 0x10;

/// Bitfield on register `RDPR`
pub const RDPRF: u8 = 0x80;

/// Bitfield on register `RDPR`
pub const PRTMP: u8 = 0x8;

/// Bitfield on register `RDPR`
pub const PRFLT: u8 = 0x4;

/// Bitfield on register `RDPR`
pub const PRPTA: u8 = 0x2;

/// Bitfield on register `RDPR`
pub const ARDPRF: u8 = 0x40;

/// Bitfield on register `RDPR`
pub const PRPTB: u8 = 0x1;

/// Bitfield on register `RDPR`
pub const APRPTA: u8 = 0x20;

/// Bitfield on register `RDSIFR`
pub const SOTB: u8 = 0x20;

/// Bitfield on register `RDSIFR`
pub const WCOA: u8 = 0x40;

/// Bitfield on register `RDSIFR`
pub const NBITA: u8 = 0x1;

/// Bitfield on register `RDSIFR`
pub const WCOB: u8 = 0x80;

/// Bitfield on register `RDSIFR`
pub const EOTA: u8 = 0x4;

/// Bitfield on register `RDSIFR`
pub const SOTA: u8 = 0x10;

/// Bitfield on register `RDSIFR`
pub const EOTB: u8 = 0x8;

/// Bitfield on register `RDSIFR`
pub const NBITB: u8 = 0x2;

/// Bitfield on register `RDSIMR`
pub const SOTAM: u8 = 0x10;

/// Bitfield on register `RDSIMR`
pub const EOTBM: u8 = 0x8;

/// Bitfield on register `RDSIMR`
pub const EOTAM: u8 = 0x4;

/// Bitfield on register `RDSIMR`
pub const NBITBM: u8 = 0x2;

/// Bitfield on register `RDSIMR`
pub const WCOBM: u8 = 0x80;

/// Bitfield on register `RDSIMR`
pub const SOTBM: u8 = 0x20;

/// Bitfield on register `RDSIMR`
pub const NBITAM: u8 = 0x1;

/// Bitfield on register `RDSIMR`
pub const WCOAM: u8 = 0x40;

/// Bitfield on register `RSCOM`
pub const RSIFC: u8 = 0x2;

/// Bitfield on register `RSCOM`
pub const RSDC: u8 = 0x1;

/// Bitfield on register `RSCOM`
pub const RSHISC: u8 = 0x4;

/// Bitfield on register `RSSC`
pub const RSPKF: u8 = 0x40;

/// Bitfield on register `RSSC`
pub const RSUP: u8 = 0xF;

/// Bitfield on register `RSSC`
pub const RSWLH: u8 = 0x10;

/// Bitfield on register `RSSC`
pub const RSHRX: u8 = 0x20;

/// Bitfield on register `RXBC1`
pub const RXCBLB: u8 = 0x60;

/// Bitfield on register `RXBC1`
pub const RXCEA: u8 = 0x1;

/// Bitfield on register `RXBC1`
pub const RXCBLA: u8 = 0x6;

/// Bitfield on register `RXBC1`
pub const RXLSBA: u8 = 0x8;

/// Bitfield on register `RXBC1`
pub const RXLSBB: u8 = 0x80;

/// Bitfield on register `RXBC1`
pub const RXCEB: u8 = 0x10;

/// Bitfield on register `RXBC2`
pub const RXBPB: u8 = 0x1;

/// Bitfield on register `RXBC2`
pub const RXBCLR: u8 = 0x4;

/// Bitfield on register `RXBC2`
pub const RXBF: u8 = 0x2;

/// Bitfield on register `SFC`
pub const SFDRA: u8 = 0x80;

/// Bitfield on register `SFC`
pub const SFFLC: u8 = 0x1F;

/// Bitfield on register `SFFR`
pub const RFC: u8 = 0x8;

/// Bitfield on register `SFFR`
pub const TFC: u8 = 0x80;

/// Bitfield on register `SFFR`
pub const RFL: u8 = 0x7;

/// Bitfield on register `SFFR`
pub const TFL: u8 = 0x70;

/// Bitfield on register `SFI`
pub const SFFLIM: u8 = 0x1;

/// Bitfield on register `SFI`
pub const SFERIM: u8 = 0x2;

/// Bitfield on register `SFIDCA`
pub const SEMEA: u8 = 0x80;

/// Bitfield on register `SFIDCA`
pub const SFIDTA: u8 = 0x1F;

/// Bitfield on register `SFIDCB`
pub const SEMEB: u8 = 0x80;

/// Bitfield on register `SFIDCB`
pub const SFIDTB: u8 = 0x1F;

/// Bitfield on register `SFIR`
pub const RIL: u8 = 0x7;

/// Bitfield on register `SFIR`
pub const SRIE: u8 = 0x8;

/// Bitfield on register `SFIR`
pub const STIE: u8 = 0x80;

/// Bitfield on register `SFIR`
pub const TIL: u8 = 0x70;

/// Bitfield on register `SFL`
pub const SFCLR: u8 = 0x80;

/// Bitfield on register `SFL`
pub const SFFLS: u8 = 0x1F;

/// Bitfield on register `SFS`
pub const SFOFL: u8 = 0x4;

/// Bitfield on register `SFS`
pub const SFFLRF: u8 = 0x1;

/// Bitfield on register `SFS`
pub const SFUFL: u8 = 0x2;

/// Bitfield on register `SMCR`
pub const SM: u8 = 0x6;

/// Bitfield on register `SMCR`
pub const SE: u8 = 0x1;

/// Bitfield on register `SOTC1A`
pub const RROEA1: u8 = 0x40;

/// Bitfield on register `SOTC1A`
pub const SFIDEA1: u8 = 0x20;

/// Bitfield on register `SOTC1A`
pub const CAROEA1: u8 = 0x1;

/// Bitfield on register `SOTC1A`
pub const SYTOEA1: u8 = 0x4;

/// Bitfield on register `SOTC1A`
pub const MANOEA1: u8 = 0x8;

/// Bitfield on register `SOTC1A`
pub const AMPOEA1: u8 = 0x2;

/// Bitfield on register `SOTC1A`
pub const WCOBOE1: u8 = 0x80;

/// Bitfield on register `SOTC1A`
pub const WUPEA1: u8 = 0x10;

/// Bitfield on register `SOTC1B`
pub const RROEB1: u8 = 0x40;

/// Bitfield on register `SOTC1B`
pub const WCOAOE1: u8 = 0x80;

/// Bitfield on register `SOTC1B`
pub const AMPOEB1: u8 = 0x2;

/// Bitfield on register `SOTC1B`
pub const CAROEB1: u8 = 0x1;

/// Bitfield on register `SOTC1B`
pub const MANOEB1: u8 = 0x8;

/// Bitfield on register `SOTC1B`
pub const SFIDEB1: u8 = 0x20;

/// Bitfield on register `SOTC1B`
pub const SYTOEB1: u8 = 0x4;

/// Bitfield on register `SOTC1B`
pub const WUPEB1: u8 = 0x10;

/// Bitfield on register `SOTC2A`
pub const MANOEA2: u8 = 0x8;

/// Bitfield on register `SOTC2A`
pub const SYTOEA2: u8 = 0x4;

/// Bitfield on register `SOTC2A`
pub const SFIDEA2: u8 = 0x20;

/// Bitfield on register `SOTC2A`
pub const WUPEA2: u8 = 0x10;

/// Bitfield on register `SOTC2A`
pub const AMPOEA2: u8 = 0x2;

/// Bitfield on register `SOTC2A`
pub const RROEA2: u8 = 0x40;

/// Bitfield on register `SOTC2A`
pub const CAROEA2: u8 = 0x1;

/// Bitfield on register `SOTC2A`
pub const WCOBOE2: u8 = 0x80;

/// Bitfield on register `SOTC2B`
pub const SFIDEB2: u8 = 0x20;

/// Bitfield on register `SOTC2B`
pub const WUPEB2: u8 = 0x10;

/// Bitfield on register `SOTC2B`
pub const AMPOEB2: u8 = 0x2;

/// Bitfield on register `SOTC2B`
pub const WCOAOE2: u8 = 0x80;

/// Bitfield on register `SOTC2B`
pub const RROEB2: u8 = 0x40;

/// Bitfield on register `SOTC2B`
pub const CAROEB2: u8 = 0x1;

/// Bitfield on register `SOTC2B`
pub const SYTOEB2: u8 = 0x4;

/// Bitfield on register `SOTC2B`
pub const MANOEB2: u8 = 0x8;

/// Bitfield on register `SOTCA`
pub const WUPEA: u8 = 0x10;

/// Bitfield on register `SOTCA`
pub const RROEA: u8 = 0x40;

/// Bitfield on register `SOTCA`
pub const WCOBOE: u8 = 0x80;

/// Bitfield on register `SOTCA`
pub const CAROEA: u8 = 0x1;

/// Bitfield on register `SOTCA`
pub const MANOEA: u8 = 0x8;

/// Bitfield on register `SOTCA`
pub const SYTOEA: u8 = 0x4;

/// Bitfield on register `SOTCA`
pub const SFIDEA: u8 = 0x20;

/// Bitfield on register `SOTCA`
pub const AMPOEA: u8 = 0x2;

/// Bitfield on register `SOTCB`
pub const CAROEB: u8 = 0x1;

/// Bitfield on register `SOTCB`
pub const AMPOEB: u8 = 0x2;

/// Bitfield on register `SOTCB`
pub const RROEB: u8 = 0x40;

/// Bitfield on register `SOTCB`
pub const WCOAOE: u8 = 0x80;

/// Bitfield on register `SOTCB`
pub const SYTOEB: u8 = 0x4;

/// Bitfield on register `SOTCB`
pub const MANOEB: u8 = 0x8;

/// Bitfield on register `SOTCB`
pub const WUPEB: u8 = 0x10;

/// Bitfield on register `SOTCB`
pub const SFIDEB: u8 = 0x20;

/// Bitfield on register `SOTSA`
pub const RROA: u8 = 0x40;

/// Bitfield on register `SOTSA`
pub const MANOA: u8 = 0x8;

/// Bitfield on register `SOTSA`
pub const AMPOA: u8 = 0x2;

/// Bitfield on register `SOTSA`
pub const SYTOA: u8 = 0x4;

/// Bitfield on register `SOTSA`
pub const WUPOA: u8 = 0x10;

/// Bitfield on register `SOTSA`
pub const SFIDOA: u8 = 0x20;

/// Bitfield on register `SOTSA`
pub const CAROA: u8 = 0x1;

/// Bitfield on register `SOTSA`
pub const WCOBO: u8 = 0x80;

/// Bitfield on register `SOTSB`
pub const RROB: u8 = 0x40;

/// Bitfield on register `SOTSB`
pub const SYTOB: u8 = 0x4;

/// Bitfield on register `SOTSB`
pub const WCOAO: u8 = 0x80;

/// Bitfield on register `SOTSB`
pub const WUPOB: u8 = 0x10;

/// Bitfield on register `SOTSB`
pub const SFIDOB: u8 = 0x20;

/// Bitfield on register `SOTSB`
pub const CAROB: u8 = 0x1;

/// Bitfield on register `SOTSB`
pub const AMPOB: u8 = 0x2;

/// Bitfield on register `SOTSB`
pub const MANOB: u8 = 0x8;

/// Bitfield on register `SPCR`
pub const SPIE: u8 = 0x80;

/// Bitfield on register `SPCR`
pub const DORD: u8 = 0x20;

/// Bitfield on register `SPCR`
pub const CPOL: u8 = 0x8;

/// Bitfield on register `SPCR`
pub const MSTR: u8 = 0x10;

/// Bitfield on register `SPCR`
pub const SPE: u8 = 0x40;

/// Bitfield on register `SPCR`
pub const SPR: u8 = 0x3;

/// Bitfield on register `SPCR`
pub const CPHA: u8 = 0x4;

/// Bitfield on register `SPMCSR`
pub const PGWRT: u8 = 0x4;

/// Bitfield on register `SPMCSR`
pub const PGERS: u8 = 0x2;

/// Bitfield on register `SPMCSR`
pub const SELFPRGEN: u8 = 0x1;

/// Bitfield on register `SPMCSR`
pub const SPMIE: u8 = 0x80;

/// Bitfield on register `SPMCSR`
pub const BLBSET: u8 = 0x8;

/// Bitfield on register `SPSR`
pub const TXIF: u8 = 0x20;

/// Bitfield on register `SPSR`
pub const SPIF: u8 = 0x80;

/// Bitfield on register `SPSR`
pub const SPI2X: u8 = 0x1;

/// Bitfield on register `SPSR`
pub const RXIF: u8 = 0x10;

/// Bitfield on register `SRCCAL`
pub const SRCTC: u8 = 0xC0;

/// Bitfield on register `SREG`
pub const H: u8 = 0x20;

/// Bitfield on register `SREG`
pub const V: u8 = 0x8;

/// Bitfield on register `SREG`
pub const S: u8 = 0x10;

/// Bitfield on register `SREG`
pub const Z: u8 = 0x2;

/// Bitfield on register `SREG`
pub const T: u8 = 0x40;

/// Bitfield on register `SREG`
pub const I: u8 = 0x80;

/// Bitfield on register `SREG`
pub const C: u8 = 0x1;

/// Bitfield on register `SREG`
pub const N: u8 = 0x4;

/// Bitfield on register `SSMCR`
pub const SETRPB: u8 = 0x80;

/// Bitfield on register `SSMCR`
pub const SSMTM: u8 = 0x2;

/// Bitfield on register `SSMCR`
pub const SSMTX: u8 = 0x1;

/// Bitfield on register `SSMCR`
pub const SETRPA: u8 = 0x40;

/// Bitfield on register `SSMFBR`
pub const SSMHADT: u8 = 0x10;

/// Bitfield on register `SSMFBR`
pub const SSMDFDT: u8 = 0x8;

/// Bitfield on register `SSMFBR`
pub const SSMFID: u8 = 0x7;

/// Bitfield on register `SSMFBR`
pub const SSMPLDT: u8 = 0x20;

/// Bitfield on register `SSMFCR`
pub const SSMCEH: u8 = 0x8;

/// Bitfield on register `SSMFCR`
pub const SSMSEH: u8 = 0x4;

/// Bitfield on register `SSMFCR`
pub const SSMIDSO: u8 = 0x1;

/// Bitfield on register `SSMFCR`
pub const SSMIDSF: u8 = 0x2;

/// Bitfield on register `SSMIFR`
pub const SSMIF: u8 = 0x1;

/// Bitfield on register `SSMIMR`
pub const SSMIM: u8 = 0x1;

/// Bitfield on register `SSMRCR`
pub const SSMPA: u8 = 0x1;

/// Bitfield on register `SSMRCR`
pub const SSMTMOE: u8 = 0x80;

/// Bitfield on register `SSMRCR`
pub const SSMPVS: u8 = 0x10;

/// Bitfield on register `SSMRCR`
pub const SSMIFA: u8 = 0x20;

/// Bitfield on register `SSMRCR`
pub const SSMAD: u8 = 0x4;

/// Bitfield on register `SSMRCR`
pub const SSMPB: u8 = 0x2;

/// Bitfield on register `SSMRCR`
pub const SSMHIS: u8 = 0x8;

/// Bitfield on register `SSMRCR`
pub const SSMIDSE: u8 = 0x40;

/// Bitfield on register `SSMRR`
pub const SSMST: u8 = 0x2;

/// Bitfield on register `SSMRR`
pub const SSMR: u8 = 0x1;

/// Bitfield on register `SSMSR`
pub const SSMESM: u8 = 0xF;

/// Bitfield on register `SSMSR`
pub const SSMERR: u8 = 0x80;

/// Bitfield on register `SSMSTR`
pub const SSMSTA: u8 = 0x3F;

/// Bitfield on register `SSMXSR`
pub const SSMSTB: u8 = 0x3F;

/// Bitfield on register `SUPCA1`
pub const VVCAL: u8 = 0xC;

/// Bitfield on register `SUPCA1`
pub const AVCAL: u8 = 0x30;

/// Bitfield on register `SUPCA1`
pub const DVCAL: u8 = 0x3;

/// Bitfield on register `SUPCA2`
pub const BGCAL: u8 = 0x3F;

/// Bitfield on register `SUPCA3`
pub const DMCAL: u8 = 0x3;

/// Bitfield on register `SUPCA3`
pub const VMOCAL: u8 = 0xC0;

/// Bitfield on register `SUPCA3`
pub const AMCAL: u8 = 0x30;

/// Bitfield on register `SUPCA3`
pub const VMCAL: u8 = 0xC;

/// Bitfield on register `SUPCR`
pub const AVEN: u8 = 0x20;

/// Bitfield on register `SUPCR`
pub const AVCCRM: u8 = 0x1;

/// Bitfield on register `SUPCR`
pub const AVDIC: u8 = 0x40;

/// Bitfield on register `SUPCR`
pub const AVCCLM: u8 = 0x2;

/// Bitfield on register `SUPFR`
pub const DCERF: u8 = 0x8;

/// Bitfield on register `SUPFR`
pub const AVCCLF: u8 = 0x2;

/// Bitfield on register `SUPFR`
pub const AVCCRF: u8 = 0x1;

/// Bitfield on register `SUPFR`
pub const DCRDYF: u8 = 0x4;

/// Bitfield on register `SYCA`
pub const SYCSA: u8 = 0xF;

/// Bitfield on register `SYCA`
pub const SYTLA: u8 = 0xF0;

/// Bitfield on register `SYCB`
pub const SYCSB: u8 = 0xF;

/// Bitfield on register `SYCB`
pub const SYTLB: u8 = 0xF0;

/// Bitfield on register `T0CR`
pub const T0PR: u8 = 0x10;

/// Bitfield on register `T0CR`
pub const T0IE: u8 = 0x8;

/// Bitfield on register `T0CR`
pub const T0PS: u8 = 0x7;

/// Bitfield on register `T0IFR`
pub const T0F: u8 = 0x1;

/// Bitfield on register `T1CR`
pub const T1OTM: u8 = 0x1;

/// Bitfield on register `T1CR`
pub const T1RES: u8 = 0x20;

/// Bitfield on register `T1CR`
pub const T1CTM: u8 = 0x2;

/// Bitfield on register `T1CR`
pub const T1TOP: u8 = 0x10;

/// Bitfield on register `T1CR`
pub const T1ENA: u8 = 0x80;

/// Bitfield on register `T1CR`
pub const T1TOS: u8 = 0x40;

/// Bitfield on register `T1CR`
pub const T1CRM: u8 = 0x4;

/// Bitfield on register `T1IFR`
pub const T1COF: u8 = 0x2;

/// Bitfield on register `T1IFR`
pub const T1OFF: u8 = 0x1;

/// Bitfield on register `T1IMR`
pub const T1CIM: u8 = 0x2;

/// Bitfield on register `T1IMR`
pub const T1OIM: u8 = 0x1;

/// Bitfield on register `T1MR`
pub const T1PS: u8 = 0x3C;

/// Bitfield on register `T1MR`
pub const T1CS: u8 = 0x3;

/// Bitfield on register `T1MR`
pub const T1DC: u8 = 0xC0;

/// Bitfield on register `T2CR`
pub const T2CTM: u8 = 0x2;

/// Bitfield on register `T2CR`
pub const T2TOP: u8 = 0x10;

/// Bitfield on register `T2CR`
pub const T2TOS: u8 = 0x40;

/// Bitfield on register `T2CR`
pub const T2RES: u8 = 0x20;

/// Bitfield on register `T2CR`
pub const T2CRM: u8 = 0x4;

/// Bitfield on register `T2CR`
pub const T2OTM: u8 = 0x1;

/// Bitfield on register `T2CR`
pub const T2ENA: u8 = 0x80;

/// Bitfield on register `T2IFR`
pub const T2OFF: u8 = 0x1;

/// Bitfield on register `T2IFR`
pub const T2COF: u8 = 0x2;

/// Bitfield on register `T2IMR`
pub const T2OIM: u8 = 0x1;

/// Bitfield on register `T2IMR`
pub const T2CIM: u8 = 0x2;

/// Bitfield on register `T2MR`
pub const T2DC: u8 = 0xC0;

/// Bitfield on register `T2MR`
pub const T2PS: u8 = 0x3C;

/// Bitfield on register `T2MR`
pub const T2CS: u8 = 0x3;

/// Bitfield on register `T3CR`
pub const T3TOS: u8 = 0x40;

/// Bitfield on register `T3CR`
pub const T3ENA: u8 = 0x80;

/// Bitfield on register `T3CR`
pub const T3CRM: u8 = 0x4;

/// Bitfield on register `T3CR`
pub const T3RES: u8 = 0x20;

/// Bitfield on register `T3CR`
pub const T3TOP: u8 = 0x10;

/// Bitfield on register `T3CR`
pub const T3OTM: u8 = 0x1;

/// Bitfield on register `T3CR`
pub const T3CTM: u8 = 0x2;

/// Bitfield on register `T3CR`
pub const T3CPRM: u8 = 0x8;

/// Bitfield on register `T3IFR`
pub const T3COF: u8 = 0x2;

/// Bitfield on register `T3IFR`
pub const T3ICF: u8 = 0x4;

/// Bitfield on register `T3IFR`
pub const T3OFF: u8 = 0x1;

/// Bitfield on register `T3IMR`
pub const T3CPIM: u8 = 0x4;

/// Bitfield on register `T3IMR`
pub const T3CIM: u8 = 0x2;

/// Bitfield on register `T3IMR`
pub const T3OIM: u8 = 0x1;

/// Bitfield on register `T3MRA`
pub const T3CS: u8 = 0x3;

/// Bitfield on register `T3MRA`
pub const T3PS: u8 = 0x1C;

/// Bitfield on register `T3MRB`
pub const T3CE: u8 = 0x18;

/// Bitfield on register `T3MRB`
pub const T3CNC: u8 = 0x4;

/// Bitfield on register `T3MRB`
pub const T3ICS: u8 = 0xE0;

/// Bitfield on register `T3MRB`
pub const T3SCE: u8 = 0x2;

/// Bitfield on register `T4CR`
pub const T4CRM: u8 = 0x4;

/// Bitfield on register `T4CR`
pub const T4ENA: u8 = 0x80;

/// Bitfield on register `T4CR`
pub const T4CPRM: u8 = 0x8;

/// Bitfield on register `T4CR`
pub const T4TOS: u8 = 0x40;

/// Bitfield on register `T4CR`
pub const T4CTM: u8 = 0x2;

/// Bitfield on register `T4CR`
pub const T4TOP: u8 = 0x10;

/// Bitfield on register `T4CR`
pub const T4RES: u8 = 0x20;

/// Bitfield on register `T4CR`
pub const T4OTM: u8 = 0x1;

/// Bitfield on register `T4IFR`
pub const T4OFF: u8 = 0x1;

/// Bitfield on register `T4IFR`
pub const T4ICF: u8 = 0x4;

/// Bitfield on register `T4IFR`
pub const T4COF: u8 = 0x2;

/// Bitfield on register `T4IMR`
pub const T4OIM: u8 = 0x1;

/// Bitfield on register `T4IMR`
pub const T4CIM: u8 = 0x2;

/// Bitfield on register `T4IMR`
pub const T4CPIM: u8 = 0x4;

/// Bitfield on register `T4MRA`
pub const T4CS: u8 = 0x3;

/// Bitfield on register `T4MRA`
pub const T4PS: u8 = 0x1C;

/// Bitfield on register `T4MRB`
pub const T4ICS: u8 = 0xE0;

/// Bitfield on register `T4MRB`
pub const T4CNC: u8 = 0x4;

/// Bitfield on register `T4MRB`
pub const T4CE: u8 = 0x18;

/// Bitfield on register `T4MRB`
pub const T4SCE: u8 = 0x2;

/// Bitfield on register `T5CCR`
pub const T5CTC: u8 = 0x8;

/// Bitfield on register `T5CCR`
pub const T5CS: u8 = 0x7;

/// Bitfield on register `T5IFR`
pub const T5OFF: u8 = 0x1;

/// Bitfield on register `T5IFR`
pub const T5COF: u8 = 0x2;

/// Bitfield on register `T5IMR`
pub const T5OIM: u8 = 0x1;

/// Bitfield on register `T5IMR`
pub const T5CIM: u8 = 0x2;

/// Bitfield on register `TESRA`
pub const EOTLA: u8 = 0x6;

/// Bitfield on register `TESRA`
pub const CRCOA: u8 = 0x1;

/// Bitfield on register `TESRB`
pub const EOTLB: u8 = 0x6;

/// Bitfield on register `TESRB`
pub const CRCOB: u8 = 0x1;

/// Bitfield on register `TRCCR`
pub const TRCEN: u8 = 0x2;

/// Bitfield on register `TRCCR`
pub const TRCCE: u8 = 0x1;

/// Bitfield on register `VMCSR`
pub const VMF: u8 = 0x20;

/// Bitfield on register `VMCSR`
pub const VMLS: u8 = 0xF;

/// Bitfield on register `VMCSR`
pub const VMIM: u8 = 0x10;

/// Bitfield on register `WDTCR`
pub const WDE: u8 = 0x8;

/// Bitfield on register `WDTCR`
pub const WDCE: u8 = 0x10;

/// Bitfield on register `WDTCR`
pub const WDPS: u8 = 0x7;

/// Bitfield on register `XFUSE`
pub const E2PT: u8 = 0xC;

/// Bitfield on register `XFUSE`
pub const FLPT: u8 = 0x3;

/// Bitfield on register `XFUSE`
pub const CKOUT: u8 = 0x40;

/// Bitfield on register `XFUSE`
pub const NVPTE: u8 = 0x10;

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

/// `CLOCK_MANAGEMENT_MODE` value group
#[allow(non_upper_case_globals)]
pub mod clock_management_mode {
   /// clk_src.
   pub const VAL_0x00: u32 = 0x0;
   /// clk_adiv.
   pub const VAL_0x01: u32 = 0x1;
   /// clk_ext.
   pub const VAL_0x02: u32 = 0x2;
   /// clk_xto6.
   pub const VAL_0x03: u32 = 0x3;
   /// clk_xto4.
   pub const VAL_0x04: u32 = 0x4;
}

/// `CLOCK_OUTPUT_SOURCE` value group
#[allow(non_upper_case_globals)]
pub mod clock_output_source {
   /// clk_src.
   pub const VAL_0x00: u32 = 0x0;
   /// clk_frc.
   pub const VAL_0x01: u32 = 0x1;
   /// clk_adiv.
   pub const VAL_0x02: u32 = 0x2;
   /// clk_xto.
   pub const VAL_0x03: u32 = 0x3;
}

/// `COMM_SCK_RATE_3BIT` value group
#[allow(non_upper_case_globals)]
pub mod comm_sck_rate_3bit {
   /// clkio/4.
   pub const VAL_0x00: u32 = 0x0;
   /// clkio/16.
   pub const VAL_0x01: u32 = 0x1;
   /// clkio/64.
   pub const VAL_0x02: u32 = 0x2;
   /// clkio/128.
   pub const VAL_0x03: u32 = 0x3;
   /// clkio/2.
   pub const VAL_0x04: u32 = 0x4;
   /// clkio/8.
   pub const VAL_0x05: u32 = 0x5;
   /// clkio/32.
   pub const VAL_0x06: u32 = 0x6;
   /// clkio/64.
   pub const VAL_0x07: u32 = 0x7;
}

/// CPU Busy Flag Output
#[allow(non_upper_case_globals)]
pub mod cpu_busy_flag_out {
   /// no output.
   pub const VAL_0x00: u32 = 0x0;
   /// PB0.
   pub const VAL_0x01: u32 = 0x1;
   /// PB4.
   pub const VAL_0x02: u32 = 0x2;
   /// PC1.
   pub const VAL_0x03: u32 = 0x3;
}

/// `CPU_CLK_PRESCALE_3BITS_SMALL` value group
#[allow(non_upper_case_globals)]
pub mod cpu_clk_prescale_3bits_small {
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
}

/// `CPU_CLT_PRESCALE_3BITS_SMALL` value group
#[allow(non_upper_case_globals)]
pub mod cpu_clt_prescale_3bits_small {
   /// disabled.
   pub const VAL_0x00: u32 = 0x0;
   /// 1.
   pub const VAL_0x01: u32 = 0x1;
   /// 2.
   pub const VAL_0x02: u32 = 0x2;
   /// 4.
   pub const VAL_0x03: u32 = 0x3;
   /// 8.
   pub const VAL_0x04: u32 = 0x4;
   /// 16.
   pub const VAL_0x05: u32 = 0x5;
   /// 32.
   pub const VAL_0x06: u32 = 0x6;
   /// 64.
   pub const VAL_0x07: u32 = 0x7;
}

/// Sleep Mode
#[allow(non_upper_case_globals)]
pub mod cpu_sleep_mode_2bits {
   /// Idle.
   pub const IDLE: u32 = 0x0;
   /// Power Down.
   pub const PDOWN: u32 = 0x1;
   /// Reserved.
   pub const VAL_0x02: u32 = 0x2;
   /// Power Off.
   pub const POFF: u32 = 0x3;
}

/// `DCDC_CLOCK_DIVIDER` value group
#[allow(non_upper_case_globals)]
pub mod dcdc_clock_divider {
   /// 16 (375kHz).
   pub const VAL_0x00: u32 = 0x0;
   /// 12 (500kHz).
   pub const VAL_0x01: u32 = 0x1;
   /// 8 (750kHz).
   pub const VAL_0x02: u32 = 0x2;
   /// 6 (1MHz).
   pub const VAL_0x03: u32 = 0x3;
}

/// `DCDC_OUTPUT_VOLTAGE` value group
#[allow(non_upper_case_globals)]
pub mod dcdc_output_voltage {
   /// 2.2V.
   pub const VAL_0x00: u32 = 0x0;
   /// 2.5V.
   pub const VAL_0x01: u32 = 0x1;
   /// 2.8V.
   pub const VAL_0x02: u32 = 0x2;
   /// 3.3V.
   pub const VAL_0x03: u32 = 0x3;
}

/// Pattern Select
#[allow(non_upper_case_globals)]
pub mod dm_pattern_select {
   /// 1T alternating (0x55 0xAA).
   pub const VAL_0x00: u32 = 0x0;
   /// 2T alternating (0x33 0xCC).
   pub const VAL_0x01: u32 = 0x1;
   /// DMPATx defined.
   pub const VAL_0x02: u32 = 0x2;
   /// Manchester conformal.
   pub const VAL_0x03: u32 = 0x3;
}

/// Pattern Severity
#[allow(non_upper_case_globals)]
pub mod dm_pattern_severity {
   /// 12.5%.
   pub const VAL_0x00: u32 = 0x0;
   /// 25%.
   pub const VAL_0x01: u32 = 0x1;
   /// 37.5%.
   pub const VAL_0x02: u32 = 0x2;
   /// 50%.
   pub const VAL_0x03: u32 = 0x3;
   /// 62.5%.
   pub const VAL_0x04: u32 = 0x4;
   /// 75%.
   pub const VAL_0x05: u32 = 0x5;
   /// 87.5%.
   pub const VAL_0x06: u32 = 0x6;
   /// 100%.
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

/// `ENUM_BLB0` value group
#[allow(non_upper_case_globals)]
pub mod enum_blb0 {
   /// LPM and SPM prohibited in Application Section.
   pub const VAL_0x00: u32 = 0x0;
   /// LPM prohibited in Application Section.
   pub const VAL_0x01: u32 = 0x1;
   /// SPM prohibited in Application Section.
   pub const VAL_0x02: u32 = 0x2;
   /// No lock on SPM and LPM in Application Section.
   pub const VAL_0x03: u32 = 0x3;
}

/// `ENUM_BLB1` value group
#[allow(non_upper_case_globals)]
pub mod enum_blb1 {
   /// LPM and SPM prohibited in Boot Loader Section.
   pub const VAL_0x00: u32 = 0x0;
   /// LPM prohibited in Boot Loader Section.
   pub const VAL_0x01: u32 = 0x1;
   /// SPM prohibited in Boot Loader Section.
   pub const VAL_0x02: u32 = 0x2;
   /// No lock on SPM and LPM in Boot Loader Section.
   pub const VAL_0x03: u32 = 0x3;
}

/// `ENUM_LB` value group
#[allow(non_upper_case_globals)]
pub mod enum_lb {
   /// Further programming and verification disabled.
   pub const VAL_0x00: u32 = 0x0;
   /// Further programming disabled.
   pub const VAL_0x02: u32 = 0x2;
   /// No memory lock features enable.
   pub const VAL_0x03: u32 = 0x3;
}

/// Interrupt Sense Control
#[allow(non_upper_case_globals)]
pub mod interrupt_sense_control {
   /// Low Level of INTX.
   pub const VAL_0x00: u32 = 0x0;
   /// Logical Change of INTX.
   pub const VAL_0x01: u32 = 0x1;
   /// Falling Edge of INTX.
   pub const VAL_0x02: u32 = 0x2;
   /// Rising Edge of INTX.
   pub const VAL_0x03: u32 = 0x3;
}

/// RX Buffer CRC Length
#[allow(non_upper_case_globals)]
pub mod rxbuf_crc_length {
   /// CRC 4-bit.
   pub const VAL_0x00: u32 = 0x0;
   /// CRC 8-bit.
   pub const VAL_0x01: u32 = 0x1;
   /// CRC 16-bit.
   pub const VAL_0x02: u32 = 0x2;
}

/// SSM EndOfTelegram Location
#[allow(non_upper_case_globals)]
pub mod ssm_eot_location {
   /// No EOT.
   pub const VAL_0x00: u32 = 0x0;
   /// Before WCO.
   pub const VAL_0x01: u32 = 0x1;
   /// Between WCO and SOT.
   pub const VAL_0x02: u32 = 0x2;
   /// After SOT.
   pub const VAL_0x03: u32 = 0x3;
}

/// `SSM_FILTER_DELAY` value group
#[allow(non_upper_case_globals)]
pub mod ssm_filter_delay {
   /// 380us (25kHz).
   pub const VAL_0x00: u32 = 0x0;
   /// 202us (50kHz).
   pub const VAL_0x01: u32 = 0x1;
   /// 135us (80kHz).
   pub const VAL_0x02: u32 = 0x2;
   /// 75us (165kHz).
   pub const VAL_0x03: u32 = 0x3;
   /// 58.5us (235kHz).
   pub const VAL_0x04: u32 = 0x4;
   /// 45us (360kHz).
   pub const VAL_0x05: u32 = 0x5;
}

/// `SSM_SUB_STATE_MACHINE` value group
#[allow(non_upper_case_globals)]
pub mod ssm_sub_state_machine {
   /// None/Stop.
   pub const VAL_0x00: u32 = 0x0;
   /// PLL en.
   pub const VAL_0x01: u32 = 0x1;
   /// PLL lock.
   pub const VAL_0x02: u32 = 0x2;
   /// RX DSP enable.
   pub const VAL_0x03: u32 = 0x3;
   /// RX DSP disable.
   pub const VAL_0x04: u32 = 0x4;
   /// TX DSP enable.
   pub const VAL_0x05: u32 = 0x5;
   /// TX DSP disable.
   pub const VAL_0x06: u32 = 0x6;
   /// RX to TX.
   pub const VAL_0x07: u32 = 0x7;
   /// TX to RX.
   pub const VAL_0x08: u32 = 0x8;
   /// Get telegram.
   pub const VAL_0x09: u32 = 0x9;
   /// Send telegram.
   pub const VAL_0x0A: u32 = 0xA;
   /// Shut down.
   pub const VAL_0x0B: u32 = 0xB;
   /// VCO Tuning.
   pub const VAL_0x0C: u32 = 0xC;
}

/// Timer1 Clock Select
#[allow(non_upper_case_globals)]
pub mod tim1_clock_select {
   /// clk_src.
   pub const VAL_0x00: u32 = 0x0;
   /// clk_frc.
   pub const VAL_0x01: u32 = 0x1;
   /// clk_T.
   pub const VAL_0x02: u32 = 0x2;
   /// clk_xto4.
   pub const VAL_0x03: u32 = 0x3;
}

/// Timer2 Clock Select
#[allow(non_upper_case_globals)]
pub mod tim2_clock_select {
   /// clk_src.
   pub const VAL_0x00: u32 = 0x0;
   /// clk_vdiv.
   pub const VAL_0x01: u32 = 0x1;
   /// clk_T.
   pub const VAL_0x02: u32 = 0x2;
   /// clk_xto4.
   pub const VAL_0x03: u32 = 0x3;
}

/// Timer3 Capture Edge Select
#[allow(non_upper_case_globals)]
pub mod tim3_capture_edge_select {
   /// disable.
   pub const VAL_0x00: u32 = 0x0;
   /// rising edge.
   pub const VAL_0x01: u32 = 0x1;
   /// falling edge.
   pub const VAL_0x02: u32 = 0x2;
   /// both edges.
   pub const VAL_0x03: u32 = 0x3;
}

/// Timer3 Capture Select
#[allow(non_upper_case_globals)]
pub mod tim3_capture_select {
   /// clk_T2.
   pub const VAL_0x00: u32 = 0x0;
   /// trpa.
   pub const VAL_0x01: u32 = 0x1;
   /// trpb.
   pub const VAL_0x02: u32 = 0x2;
   /// ticp.
   pub const VAL_0x03: u32 = 0x3;
   /// clk_src.
   pub const VAL_0x04: u32 = 0x4;
}

/// Timer3 Clock Select
#[allow(non_upper_case_globals)]
pub mod tim3_clock_select {
   /// clk_frc.
   pub const VAL_0x00: u32 = 0x0;
   /// clk_T.
   pub const VAL_0x01: u32 = 0x1;
   /// clk_xto4.
   pub const VAL_0x02: u32 = 0x2;
   /// clk_xto2.
   pub const VAL_0x03: u32 = 0x3;
}

/// Timer4 Capture Edge Select
#[allow(non_upper_case_globals)]
pub mod tim4_capture_edge_select {
   /// disable.
   pub const VAL_0x00: u32 = 0x0;
   /// rising edge.
   pub const VAL_0x01: u32 = 0x1;
   /// falling edge.
   pub const VAL_0x02: u32 = 0x2;
   /// both edges.
   pub const VAL_0x03: u32 = 0x3;
}

/// Timer4 Capture Select
#[allow(non_upper_case_globals)]
pub mod tim4_capture_select {
   /// clk_T2.
   pub const VAL_0x00: u32 = 0x0;
   /// trpa.
   pub const VAL_0x01: u32 = 0x1;
   /// trpb.
   pub const VAL_0x02: u32 = 0x2;
   /// ticp.
   pub const VAL_0x03: u32 = 0x3;
   /// clk_src.
   pub const VAL_0x04: u32 = 0x4;
}

/// Timer4 Clock Select
#[allow(non_upper_case_globals)]
pub mod tim4_clock_select {
   /// clk_src.
   pub const VAL_0x00: u32 = 0x0;
   /// clk_T.
   pub const VAL_0x01: u32 = 0x1;
   /// clk_xto6.
   pub const VAL_0x02: u32 = 0x2;
   /// clk_frc.
   pub const VAL_0x03: u32 = 0x3;
}

/// `VMON_LEVEL_SELECT` value group
#[allow(non_upper_case_globals)]
pub mod vmon_level_select {
   /// disable.
   pub const VAL_0x00: u32 = 0x0;
   /// 2.0V.
   pub const VAL_0x01: u32 = 0x1;
   /// 2.1V.
   pub const VAL_0x02: u32 = 0x2;
   /// 2.2V.
   pub const VAL_0x03: u32 = 0x3;
   /// 2.3V.
   pub const VAL_0x04: u32 = 0x4;
   /// 2.4V.
   pub const VAL_0x05: u32 = 0x5;
   /// 2.5V.
   pub const VAL_0x06: u32 = 0x6;
   /// 2.6V.
   pub const VAL_0x07: u32 = 0x7;
   /// 2.7V.
   pub const VAL_0x08: u32 = 0x8;
   /// 2.8V.
   pub const VAL_0x09: u32 = 0x9;
   /// 2.9V.
   pub const VAL_0x0A: u32 = 0xA;
   /// 3.0V.
   pub const VAL_0x0B: u32 = 0xB;
   /// 3.1V.
   pub const VAL_0x0C: u32 = 0xC;
   /// 3.2V.
   pub const VAL_0x0D: u32 = 0xD;
   /// 3.3V.
   pub const VAL_0x0E: u32 = 0xE;
   /// 3.4V.
   pub const VAL_0x0F: u32 = 0xF;
}

