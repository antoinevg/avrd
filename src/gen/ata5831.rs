//! The AVR ATA5831 microcontroller
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
/// | AP | 1100 |
/// | LB | 11 |
/// | BLP | 110000 |
pub const LOCKBIT: *mut u8 = 0x0 as *mut u8;

/// `LOW` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DWEN | 1000000 |
/// | CKDIV8 | 10000000 |
/// | SPIEN | 100000 |
/// | BOOTRST | 100 |
/// | RSTDISBL | 10 |
/// | WDTON | 10000 |
/// | EXTCLKEN | 1 |
/// | EESAVE | 1000 |
pub const LOW: *mut u8 = 0x0 as *mut u8;

/// Power Reduction Register 0.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRRXDC | 10 |
/// | PRTXDC | 100 |
/// | PRSPI | 1 |
/// | PRCRC | 1000 |
/// | PRCO | 100000 |
/// | PRVM | 10000 |
pub const PRR0: *mut u8 = 0x21 as *mut u8;

/// Power Reduction Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRT4 | 1000 |
/// | PRT2 | 10 |
/// | PRT1 | 1 |
/// | PRT3 | 100 |
/// | PRT5 | 10000 |
pub const PRR1: *mut u8 = 0x22 as *mut u8;

/// Power Reduction Register 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRXA | 10 |
/// | PRSSM | 10000000 |
/// | PRSF | 100 |
/// | PRTM | 1000000 |
/// | PRRS | 100000 |
/// | PRDF | 1000 |
/// | PRXB | 1 |
/// | PRIDS | 10000 |
pub const PRR2: *mut u8 = 0x23 as *mut u8;

/// Rx DSP power reduction register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRFLT | 100 |
/// | RDPRF | 10000000 |
/// | APRPTB | 10000 |
/// | ARDPRF | 1000000 |
/// | PRPTB | 1 |
/// | APRPTA | 100000 |
/// | PRTMP | 1000 |
/// | PRPTA | 10 |
pub const RDPR: *mut u8 = 0x24 as *mut u8;

/// Port B Input Pins.
pub const PINB: *mut u8 = 0x25 as *mut u8;

/// Port B Data Direction Register.
pub const DDRB: *mut u8 = 0x26 as *mut u8;

/// Port B Data Register.
pub const PORTB: *mut u8 = 0x27 as *mut u8;

/// Port C Input Pins.
pub const PINC: *mut u8 = 0x28 as *mut u8;

/// Port C Data Direction Register.
pub const DDRC: *mut u8 = 0x29 as *mut u8;

/// Port C Data Register.
pub const PORTC: *mut u8 = 0x2A as *mut u8;

/// Frequency Synthesizer Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PAOER | 10000 |
/// | TXMOD | 1 |
/// | SFM | 10 |
/// | TXMS | 1100 |
/// | PAON | 10000000 |
pub const FSCR: *mut u8 = 0x2B as *mut u8;

/// Rx DSP status interrupt flag register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SOTB | 100000 |
/// | NBITA | 1 |
/// | WCOA | 1000000 |
/// | SOTA | 10000 |
/// | EOTA | 100 |
/// | EOTB | 1000 |
/// | NBITB | 10 |
/// | WCOB | 10000000 |
pub const RDSIFR: *mut u8 = 0x2D as *mut u8;

/// MCU Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PB7HS | 10000000 |
/// | IVCE | 1 |
/// | ENPS | 1000 |
/// | SPIIO | 100 |
/// | IVSEL | 10 |
/// | PB4HS | 100000 |
/// | PUD | 10000 |
/// | PB7LS | 1000000 |
pub const MCUCR: *mut u8 = 0x2E as *mut u8;

/// Pin change Interrupt flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCIF1 | 10 |
/// | PCIF0 | 1 |
pub const PCIFR: *mut u8 = 0x2F as *mut u8;

/// Timer0 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T0PS | 111 |
/// | T0IE | 1000 |
/// | T0PR | 10000 |
pub const T0CR: *mut u8 = 0x30 as *mut u8;

/// Timer1 control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T1OTM | 1 |
/// | T1CRM | 100 |
/// | T1RES | 100000 |
/// | T1CTM | 10 |
/// | T1TOS | 1000000 |
/// | T1ENA | 10000000 |
/// | T1TOP | 10000 |
pub const T1CR: *mut u8 = 0x31 as *mut u8;

/// Timer2 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T2OTM | 1 |
/// | T2CRM | 100 |
/// | T2RES | 100000 |
/// | T2TOP | 10000 |
/// | T2CTM | 10 |
/// | T2ENA | 10000000 |
/// | T2TOS | 1000000 |
pub const T2CR: *mut u8 = 0x32 as *mut u8;

/// Timer3 control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T3RES | 100000 |
/// | T3CRM | 100 |
/// | T3CTM | 10 |
/// | T3TOP | 10000 |
/// | T3CPRM | 1000 |
/// | T3OTM | 1 |
/// | T3TOS | 1000000 |
/// | T3ENA | 10000000 |
pub const T3CR: *mut u8 = 0x33 as *mut u8;

/// Timer4 control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T4RES | 100000 |
/// | T4OTM | 1 |
/// | T4TOS | 1000000 |
/// | T4CPRM | 1000 |
/// | T4TOP | 10000 |
/// | T4CRM | 100 |
/// | T4CTM | 10 |
/// | T4ENA | 10000000 |
pub const T4CR: *mut u8 = 0x34 as *mut u8;

/// Timer1 Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T1OFF | 1 |
/// | T1COF | 10 |
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

/// Timer3 interrupt flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T3OFF | 1 |
/// | T3ICF | 100 |
/// | T3COF | 10 |
pub const T3IFR: *mut u8 = 0x37 as *mut u8;

/// Timer4 interrupt flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T4ICF | 100 |
/// | T4OFF | 1 |
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
/// | EERE | 1 |
/// | NVMBSY | 10000000 |
/// | EEWE | 10 |
/// | EEPAGE | 1000000 |
/// | EEMWE | 100 |
/// | EEPM | 110000 |
/// | EERIE | 1000 |
pub const EECR: *mut u8 = 0x3F as *mut u8;

/// EEPROM Data Register.
pub const EEDR: *mut u8 = 0x40 as *mut u8;

/// EEPROM Address Register low byte.
pub const EEARL: *mut u8 = 0x41 as *mut u8;

/// EEPROM Address Register.
pub const EEAR: *mut u16 = 0x41 as *mut u16;

/// EEPROM Address Register high byte.
pub const EEARH: *mut u8 = 0x42 as *mut u8;

/// EEPROM Protection Register.
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

/// Pin change Interrupt control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCIE0 | 1 |
/// | PCIE1 | 10 |
pub const PCICR: *mut u8 = 0x46 as *mut u8;

/// External Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | INT1 | 10 |
/// | INT0 | 1 |
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
/// | VMLS | 1111 |
/// | VMF | 100000 |
/// | VMIM | 10000 |
pub const VMCSR: *mut u8 = 0x4A as *mut u8;

/// MCU Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDRF | 1000 |
/// | EXTRF | 10 |
/// | PORF | 1 |
pub const MCUSR: *mut u8 = 0x4B as *mut u8;

/// SPI Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CPOL | 1000 |
/// | MSTR | 10000 |
/// | DORD | 100000 |
/// | SPIE | 10000000 |
/// | CPHA | 100 |
/// | SPE | 1000000 |
/// | SPR | 11 |
pub const SPCR: *mut u8 = 0x4C as *mut u8;

/// SPI Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SPI2X | 1 |
/// | TXIF | 100000 |
/// | SPIF | 10000000 |
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

/// debugWire communication Register.
pub const DWDR: *mut u8 = 0x51 as *mut u8;

/// Rx DSP control register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RDPU | 1 |
/// | RDEN | 100 |
/// | ADIVEN | 10 |
pub const RDCR: *mut u8 = 0x53 as *mut u8;

/// End Of Telegram Status on path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AMPFA | 10 |
/// | EOTBF | 10000000 |
/// | TELRA | 100000 |
/// | TMOFA | 10000 |
/// | CARFA | 1 |
/// | MANFA | 1000 |
/// | RRFA | 1000000 |
/// | SYTFA | 100 |
pub const EOTSA: *mut u8 = 0x54 as *mut u8;

/// End Of Telegram Conditions for path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AMPFEA | 10 |
/// | EOTBFE | 10000000 |
/// | MANFEA | 1000 |
/// | CARFEA | 1 |
/// | TELREA | 100000 |
/// | SYTFEA | 100 |
/// | TMOFEA | 10000 |
/// | RRFEA | 1000000 |
pub const EOTCA: *mut u8 = 0x55 as *mut u8;

/// End Of Telegram Status on path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CARFB | 1 |
/// | SYTFB | 100 |
/// | RRFB | 1000000 |
/// | EOTAF | 10000000 |
/// | AMPFB | 10 |
/// | TELRB | 100000 |
/// | TMOFB | 10000 |
/// | MANFB | 1000 |
pub const EOTSB: *mut u8 = 0x56 as *mut u8;

/// End Of Telegram Conditions for path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SYTFEB | 100 |
/// | EOTAFE | 10000000 |
/// | AMPFEB | 10 |
/// | RRFEB | 1000000 |
/// | TMOFEB | 10000 |
/// | TELREB | 100000 |
/// | CARFEB | 1 |
/// | MANFEB | 1000 |
pub const EOTCB: *mut u8 = 0x57 as *mut u8;

/// Sleep mode control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SM | 1110 |
/// | SE | 1 |
pub const SMCR: *mut u8 = 0x58 as *mut u8;

/// Clock Management Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CMCCE | 10000000 |
/// | SRCD | 10000 |
/// | CCS | 1000 |
/// | CMONEN | 1000000 |
/// | CMM | 111 |
pub const CMCR: *mut u8 = 0x59 as *mut u8;

/// Clock Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ECIE | 1 |
pub const CMIMR: *mut u8 = 0x5A as *mut u8;

/// Clock Prescaler Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CLPCE | 10000000 |
/// | CLKPS | 111 |
/// | CLTPS | 111000 |
pub const CLPR: *mut u8 = 0x5B as *mut u8;

/// Store Program Memory Control and Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PGERS | 10 |
/// | PGWRT | 100 |
/// | BLBSET | 1000 |
/// | SPMIE | 10000000 |
/// | SELFPRGEN | 1 |
pub const SPMCSR: *mut u8 = 0x5C as *mut u8;

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
/// | S | 10000 |
/// | N | 100 |
/// | Z | 10 |
/// | V | 1000 |
/// | T | 1000000 |
/// | H | 100000 |
/// | C | 1 |
/// | I | 10000000 |
pub const SREG: *mut u8 = 0x5F as *mut u8;

/// Frequency Synthesizer Enable register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ASEN | 10000 |
/// | SDEN | 10 |
/// | PEEN | 1000 |
/// | SDPU | 1 |
/// | GAEN | 100 |
/// | ANTT | 100000 |
pub const FSEN: *mut u8 = 0x60 as *mut u8;

/// Frequency Synthesizer Filter Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BTSEL | 11 |
/// | ASDIV | 11110000 |
pub const FSFCR: *mut u8 = 0x61 as *mut u8;

/// Gauss Clock Divider.
pub const GACDIV: *mut u16 = 0x62 as *mut u16;

/// Gauss Clock Divider low byte.
pub const GACDIVL: *mut u8 = 0x62 as *mut u8;

/// Gauss Clock Divider high byte.
pub const GACDIVH: *mut u8 = 0x63 as *mut u8;

/// Fractional Frequency 1 Setting, Low Byte.
pub const FFREQ1L: *mut u8 = 0x64 as *mut u8;

/// Fractional Frequency 1 Setting, Middle Byte.
pub const FFREQ1M: *mut u8 = 0x65 as *mut u8;

/// Fractional Frequency 1 Setting, High Byte.
pub const FFREQ1H: *mut u8 = 0x66 as *mut u8;

/// Fractional Frequency 2 Setting, Low Byte.
pub const FFREQ2L: *mut u8 = 0x67 as *mut u8;

/// Fractional Frequency 2 Setting, Middle Byte.
pub const FFREQ2M: *mut u8 = 0x68 as *mut u8;

/// Fractional Frequency 2 Setting, High Byte.
pub const FFREQ2H: *mut u8 = 0x69 as *mut u8;

/// External Interrupt control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ISC0 | 11 |
/// | ISC1 | 1100 |
pub const EICRA: *mut u8 = 0x6B as *mut u8;

/// Pin change Mask Register 0.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCINT6 | 1000000 |
/// | PCINT7 | 10000000 |
/// | PCINT0 | 1 |
/// | PCINT2 | 100 |
/// | PCINT1 | 10 |
/// | PCINT5 | 100000 |
/// | PCINT3 | 1000 |
/// | PCINT4 | 10000 |
pub const PCMSK0: *mut u8 = 0x6C as *mut u8;

/// Pin change Mask Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCINT11 | 1000 |
/// | PCINT10 | 100 |
/// | PCINT12 | 10000 |
/// | PCINT9 | 10 |
/// | PCINT13 | 100000 |
/// | PCINT8 | 1 |
pub const PCMSK1: *mut u8 = 0x6D as *mut u8;

/// Watchdog Timer0 control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDE | 1000 |
/// | WDPS | 111 |
/// | WDCE | 10000 |
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
/// | T1DC | 11000000 |
/// | T1PS | 111100 |
/// | T1CS | 11 |
pub const T1MR: *mut u8 = 0x71 as *mut u8;

/// Timer1 Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T1OIM | 1 |
/// | T1CIM | 10 |
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
/// | T2CS | 11 |
/// | T2PS | 111100 |
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

/// Timer3 counter Register low byte.
pub const T3CNTL: *mut u8 = 0x77 as *mut u8;

/// Timer3 counter Register.
pub const T3CNT: *mut u16 = 0x77 as *mut u16;

/// Timer3 counter Register high byte.
pub const T3CNTH: *mut u8 = 0x78 as *mut u8;

/// Timer3 compare Register.
pub const T3COR: *mut u16 = 0x79 as *mut u16;

/// Timer3 compare Register low byte.
pub const T3CORL: *mut u8 = 0x79 as *mut u8;

/// Timer3 compare Register high byte.
pub const T3CORH: *mut u8 = 0x7A as *mut u8;

/// Timer3 input capture Register low byte.
pub const T3ICRL: *mut u8 = 0x7B as *mut u8;

/// Timer3 input capture Register.
pub const T3ICR: *mut u16 = 0x7B as *mut u16;

/// Timer3 input capture Register high byte.
pub const T3ICRH: *mut u8 = 0x7C as *mut u8;

/// Timer3 mode Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T3PS | 11100 |
/// | T3CS | 11 |
pub const T3MRA: *mut u8 = 0x7D as *mut u8;

/// Timer3 mode Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T3CE | 11000 |
/// | T3SCE | 10 |
/// | T3CNC | 100 |
/// | T3ICS | 11100000 |
pub const T3MRB: *mut u8 = 0x7E as *mut u8;

/// Timer3 interrupt mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T3OIM | 1 |
/// | T3CPIM | 100 |
/// | T3CIM | 10 |
pub const T3IMR: *mut u8 = 0x7F as *mut u8;

/// Timer4 counter Register low byte.
pub const T4CNTL: *mut u8 = 0x80 as *mut u8;

/// Timer4 counter Register.
pub const T4CNT: *mut u16 = 0x80 as *mut u16;

/// Timer4 counter Register high byte.
pub const T4CNTH: *mut u8 = 0x81 as *mut u8;

/// Timer4 compare Register low byte.
pub const T4CORL: *mut u8 = 0x82 as *mut u8;

/// Timer4 compare Register.
pub const T4COR: *mut u16 = 0x82 as *mut u16;

/// Timer4 compare Register high byte.
pub const T4CORH: *mut u8 = 0x83 as *mut u8;

/// Timer4 input capture Register low byte.
pub const T4ICRL: *mut u8 = 0x84 as *mut u8;

/// Timer4 input capture Register.
pub const T4ICR: *mut u16 = 0x84 as *mut u16;

/// Timer4 input capture Register high byte.
pub const T4ICRH: *mut u8 = 0x85 as *mut u8;

/// Timer4 mode Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T4PS | 11100 |
/// | T4CS | 11 |
pub const T4MRA: *mut u8 = 0x86 as *mut u8;

/// Timer4 mode Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T4CNC | 100 |
/// | T4ICS | 11100000 |
/// | T4CE | 11000 |
/// | T4SCE | 10 |
pub const T4MRB: *mut u8 = 0x87 as *mut u8;

/// Timer4 interrupt mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T4CPIM | 100 |
/// | T4CIM | 10 |
/// | T4OIM | 1 |
pub const T4IMR: *mut u8 = 0x88 as *mut u8;

/// Timer5 Output Compare Register low byte.
pub const T5OCRL: *mut u8 = 0x8A as *mut u8;

/// Timer5 Output Compare Register.
pub const T5OCR: *mut u16 = 0x8A as *mut u16;

/// Timer5 Output Compare Register high byte.
pub const T5OCRH: *mut u8 = 0x8B as *mut u8;

/// Timer5 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T5CS | 111 |
/// | T5CTC | 1000 |
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
/// | TSM | 10000000 |
/// | PSR10 | 1 |
pub const GTCCR: *mut u8 = 0x90 as *mut u8;

/// Start Of Telegram Status for path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AMPOB | 10 |
/// | CAROB | 1 |
/// | SFIDOB | 100000 |
/// | RROB | 1000000 |
/// | SYTOB | 100 |
/// | WUPOB | 10000 |
/// | WCOAO | 10000000 |
/// | MANOB | 1000 |
pub const SOTSB: *mut u8 = 0x91 as *mut u8;

/// Start Of Telegram Status for path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AMPOA | 10 |
/// | RROA | 1000000 |
/// | MANOA | 1000 |
/// | SFIDOA | 100000 |
/// | CAROA | 1 |
/// | SYTOA | 100 |
/// | WCOBO | 10000000 |
/// | WUPOA | 10000 |
pub const SOTSA: *mut u8 = 0x92 as *mut u8;

/// Start Of Telegram Conditions for path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WUPEB | 10000 |
/// | SFIDEB | 100000 |
/// | MANOEB | 1000 |
/// | WCOAOE | 10000000 |
/// | RROEB | 1000000 |
/// | SYTOEB | 100 |
/// | CAROEB | 1 |
/// | AMPOEB | 10 |
pub const SOTCB: *mut u8 = 0x93 as *mut u8;

/// Start Of Telegram Conditions for path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CAROEA | 1 |
/// | RROEA | 1000000 |
/// | AMPOEA | 10 |
/// | WCOBOE | 10000000 |
/// | SYTOEA | 100 |
/// | SFIDEA | 100000 |
/// | MANOEA | 1000 |
/// | WUPEA | 10000 |
pub const SOTCA: *mut u8 = 0x94 as *mut u8;

/// Telegram Status Register on Path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CRCOB | 1 |
/// | EOTLB | 110 |
pub const TESRB: *mut u8 = 0x95 as *mut u8;

/// Telegram Status Register on Path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EOTLA | 110 |
/// | CRCOA | 1 |
pub const TESRA: *mut u8 = 0x96 as *mut u8;

/// Rx DSP status interrupt mask register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SOTBM | 100000 |
/// | WCOBM | 10000000 |
/// | EOTBM | 1000 |
/// | EOTAM | 100 |
/// | SOTAM | 10000 |
/// | NBITAM | 1 |
/// | WCOAM | 1000000 |
/// | NBITBM | 10 |
pub const RDSIMR: *mut u8 = 0x98 as *mut u8;

/// Rx DSP output control.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TMDS | 110 |
/// | ETRPB | 10000 |
/// | RDSIDB | 1000000 |
/// | RDSIDA | 100000 |
/// | ETRPA | 1000 |
pub const RDOCR: *mut u8 = 0x99 as *mut u8;

/// Temperature Low byte.
pub const TEMPL: *mut u8 = 0x9B as *mut u8;

/// Temperature High byte.
pub const TEMPH: *mut u8 = 0x9C as *mut u8;

/// Symbol check configuration for data path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SYTLB | 11110000 |
/// | SYCSB | 1111 |
pub const SYCB: *mut u8 = 0x9D as *mut u8;

/// Symbol check configuration for data path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SYTLA | 11110000 |
/// | SYCSA | 1111 |
pub const SYCA: *mut u8 = 0x9E as *mut u8;

/// Received Frequency Offset vs Intermediate Frequency on path B.
pub const RXFOB: *mut u8 = 0x9F as *mut u8;

/// Received Frequency Offset vs Intermediate Frequency on path A.
pub const RXFOA: *mut u8 = 0xA0 as *mut u8;

/// Demodulator Mode for Path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DMNEB | 10000000 |
/// | DMATB | 11111 |
/// | DMPB | 100000 |
/// | DMHB | 1000000 |
pub const DMMB: *mut u8 = 0xA1 as *mut u8;

/// Demodulator Mode for path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DMHA | 1000000 |
/// | DMPA | 100000 |
/// | DMNEA | 10000000 |
/// | DMATA | 11111 |
pub const DMMA: *mut u8 = 0xA2 as *mut u8;

/// Demodulator Carrier Detect for path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DMCTB | 11100000 |
/// | DMCLB | 11111 |
pub const DMCDB: *mut u8 = 0xA3 as *mut u8;

/// Demodulator Carrier Detect for path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DMCLA | 11111 |
/// | DMCTA | 11100000 |
pub const DMCDA: *mut u8 = 0xA4 as *mut u8;

/// Demodulator Control Register for path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DMARB | 10000000 |
/// | DMPGB | 11111 |
/// | SASKB | 100000 |
/// | SY1TB | 1000000 |
pub const DMCRB: *mut u8 = 0xA5 as *mut u8;

/// Demodulator Control Register for path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SY1TA | 1000000 |
/// | SASKA | 100000 |
/// | DMPGA | 11111 |
/// | DMARA | 10000000 |
pub const DMCRA: *mut u8 = 0xA6 as *mut u8;

/// Demodulator Data Rate on path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DMAB | 1111 |
/// | DMDNB | 11110000 |
pub const DMDRB: *mut u8 = 0xA7 as *mut u8;

/// Demodulator Data Rate on path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DMAA | 1111 |
/// | DMDNA | 11110000 |
pub const DMDRA: *mut u8 = 0xA8 as *mut u8;

/// Channel Filter Configuration Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BWM | 1111 |
pub const CHCR: *mut u8 = 0xA9 as *mut u8;

/// Channel Filter Down Sampling Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BBDN | 11111 |
/// | ADCDN | 100000 |
pub const CHDN: *mut u8 = 0xAA as *mut u8;

/// Start-Frame ID Control for data path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SFIDTB | 11111 |
/// | SEMEB | 10000000 |
pub const SFIDCB: *mut u8 = 0xAB as *mut u8;

/// Start-Frame ID Length for data path B.
pub const SFIDLB: *mut u8 = 0xAC as *mut u8;

/// Wake-Up Pattern Threshold for data path B.
pub const WUPTB: *mut u8 = 0xAD as *mut u8;

/// Wake-Up Pattern Length for data path B.
pub const WUPLB: *mut u8 = 0xAE as *mut u8;

/// Start-Frame ID byte 1 for data path B.
pub const SFID1B: *mut u8 = 0xAF as *mut u8;

/// Start-Frame ID byte 2 for data path B.
pub const SFID2B: *mut u8 = 0xB0 as *mut u8;

/// Start-Frame ID byte 3 for data path B.
pub const SFID3B: *mut u8 = 0xB1 as *mut u8;

/// Start-Frame ID byte 4 for data path B.
pub const SFID4B: *mut u8 = 0xB2 as *mut u8;

/// Wake-Up Pattern byte 1 for data path B.
pub const WUP1B: *mut u8 = 0xB3 as *mut u8;

/// Wake-Up Pattern byte 2 for data path B.
pub const WUP2B: *mut u8 = 0xB4 as *mut u8;

/// Wake-Up Pattern byte 3 for data path B.
pub const WUP3B: *mut u8 = 0xB5 as *mut u8;

/// Wake-Up Pattern byte 4 for data path B.
pub const WUP4B: *mut u8 = 0xB6 as *mut u8;

/// Start-Frame ID Control for data path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SEMEA | 10000000 |
/// | SFIDTA | 11111 |
pub const SFIDCA: *mut u8 = 0xB7 as *mut u8;

/// Start-Frame ID Length for data path A.
pub const SFIDLA: *mut u8 = 0xB8 as *mut u8;

/// Wake-Up Pattern Threshold for data path A.
pub const WUPTA: *mut u8 = 0xB9 as *mut u8;

/// Wake-Up Pattern Length for data path A.
pub const WUPLA: *mut u8 = 0xBA as *mut u8;

/// Start-Frame ID byte 1 for data path A.
pub const SFID1A: *mut u8 = 0xBB as *mut u8;

/// Start-Frame ID byte 2 for data path A.
pub const SFID2A: *mut u8 = 0xBC as *mut u8;

/// Start-Frame ID byte 3 for data path A.
pub const SFID3A: *mut u8 = 0xBD as *mut u8;

/// Start-Frame ID byte 4 for data path A.
pub const SFID4A: *mut u8 = 0xBE as *mut u8;

/// Wake-Up Pattern byte 1 for data path A.
pub const WUP1A: *mut u8 = 0xBF as *mut u8;

/// Wake-Up Pattern byte 2 for data path A.
pub const WUP2A: *mut u8 = 0xC0 as *mut u8;

/// Wake-Up Pattern byte 3 for data path A.
pub const WUP3A: *mut u8 = 0xC1 as *mut u8;

/// Wake-Up Pattern byte 4 for data path A.
pub const WUP4A: *mut u8 = 0xC2 as *mut u8;

/// Clock output divider settings Register.
pub const CLKOD: *mut u8 = 0xC3 as *mut u8;

/// Clock output control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CLKOEN | 100 |
/// | CLKOS | 11 |
pub const CLKOCR: *mut u8 = 0xC4 as *mut u8;

/// `XFUSE` register
pub const XFUSE: *mut u8 = 0xC5 as *mut u8;

/// Slow RC oscillator calibration Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SRCTC | 11000000 |
pub const SRCCAL: *mut u8 = 0xC6 as *mut u8;

/// Fast RC oscillator calibration Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FRCTC | 100000 |
pub const FRCCAL: *mut u8 = 0xC7 as *mut u8;

/// Clock management status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ECF | 1 |
pub const CMSR: *mut u8 = 0xC8 as *mut u8;

/// Clock management override control register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SRCAO | 10 |
/// | FRCAO | 1 |
/// | SRCACT | 1000 |
/// | FRCACT | 100 |
pub const CMOCR: *mut u8 = 0xC9 as *mut u8;

/// Supply Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AVCCLF | 10 |
/// | AVCCRF | 1 |
pub const SUPFR: *mut u8 = 0xCA as *mut u8;

/// Supply Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AVDIC | 1000000 |
/// | AVCCRM | 1 |
/// | PVEN | 100 |
/// | AVEN | 100000 |
/// | DVDIS | 10000 |
/// | AVCCLM | 10 |
pub const SUPCR: *mut u8 = 0xCB as *mut u8;

/// Supply calibration register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PVCAL | 11110000 |
/// | PV22 | 100 |
/// | PVDIC | 1000 |
pub const SUPCA1: *mut u8 = 0xCC as *mut u8;

/// Supply calibration register 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BGCAL | 1111 |
pub const SUPCA2: *mut u8 = 0xCD as *mut u8;

/// Supply calibration register 3.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ACAL6 | 100 |
/// | DCAL6 | 1000000 |
/// | ACAL5 | 10 |
/// | DCAL5 | 100000 |
/// | ACAL7 | 1000 |
/// | DCAL4 | 10000 |
/// | ACAL4 | 1 |
pub const SUPCA3: *mut u8 = 0xCE as *mut u8;

/// Supply calibration register 4.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DCAL3 | 10000000 |
/// | ACAL1 | 10 |
/// | DCAL0 | 10000 |
/// | DCAL2 | 1000000 |
/// | ACAL0 | 1 |
/// | ACAL2 | 100 |
/// | ACAL3 | 1000 |
/// | DCAL1 | 100000 |
pub const SUPCA4: *mut u8 = 0xCF as *mut u8;

/// Calibration ready signature.
pub const CALRDY: *mut u8 = 0xD0 as *mut u8;

/// Voltage Monitor Calibration register.
pub const VMCAL: *mut u8 = 0xD1 as *mut u8;

/// Data FIFO Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DFFLRF | 1 |
/// | DFUFL | 10 |
/// | DFOFL | 100 |
pub const DFS: *mut u8 = 0xD2 as *mut u8;

/// Data FIFO Telegram Length low byte.
pub const DFTLL: *mut u8 = 0xD3 as *mut u8;

/// Data FIFO Telegram Length.
pub const DFTL: *mut u16 = 0xD3 as *mut u16;

/// Data FIFO Telegram Length high byte.
pub const DFTLH: *mut u8 = 0xD4 as *mut u8;

/// Data FIFO Fill Level Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DFCLR | 10000000 |
/// | DFFLS | 111111 |
pub const DFL: *mut u8 = 0xD5 as *mut u8;

/// Data FIFO Write Pointer.
pub const DFWP: *mut u8 = 0xD6 as *mut u8;

/// Data FIFO Read Pointer.
pub const DFRP: *mut u8 = 0xD7 as *mut u8;

/// Data FIFO Data Register.
pub const DFD: *mut u8 = 0xD8 as *mut u8;

/// Data FIFO Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DFERIM | 10 |
/// | DFFLIM | 1 |
pub const DFI: *mut u8 = 0xD9 as *mut u8;

/// Data FIFO Configuration Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DFFLC | 111111 |
/// | DFDRA | 10000000 |
pub const DFC: *mut u8 = 0xDA as *mut u8;

/// Support FIFO Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SFOFL | 100 |
/// | SFFLRF | 1 |
/// | SFUFL | 10 |
pub const SFS: *mut u8 = 0xDB as *mut u8;

/// Support FIFO Fill Level Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SFFLS | 11111 |
/// | SFCLR | 10000000 |
pub const SFL: *mut u8 = 0xDC as *mut u8;

/// Support FIFO Write Pointer.
pub const SFWP: *mut u8 = 0xDD as *mut u8;

/// Support FIFO Read Pointer.
pub const SFRP: *mut u8 = 0xDE as *mut u8;

/// Support FIFO Data Register.
pub const SFD: *mut u8 = 0xDF as *mut u8;

/// Support FIFO Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SFERIM | 10 |
/// | SFFLIM | 1 |
pub const SFI: *mut u8 = 0xE0 as *mut u8;

/// Support FIFO Configuration Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SFDRA | 10000000 |
/// | SFFLC | 11111 |
pub const SFC: *mut u8 = 0xE1 as *mut u8;

/// SSM Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SSMTM | 10 |
/// | SETRPB | 10000000 |
/// | SSMPVE | 10000 |
/// | SSMTX | 1 |
/// | SSMTPE | 1000 |
/// | SSMTAE | 100000 |
/// | SSMTGE | 100 |
/// | SETRPA | 1000000 |
pub const SSMCR: *mut u8 = 0xE2 as *mut u8;

/// SSM Rx Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SSMADB | 1000 |
/// | SSMIFA | 100000 |
/// | SSMPB | 10 |
/// | SSMTMOE | 10000000 |
/// | SSMPVS | 10000 |
/// | SSMIDSE | 1000000 |
/// | SSMADA | 100 |
/// | SSMPA | 1 |
pub const SSMRCR: *mut u8 = 0xE3 as *mut u8;

/// SSM Filter Bandwidth Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SSMDFDT | 1000 |
/// | SSMHADT | 10000 |
/// | SSMFID | 111 |
/// | SSMPLDT | 100000 |
pub const SSMFBR: *mut u8 = 0xE4 as *mut u8;

/// SSM Run Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SSMST | 10 |
/// | SSMR | 1 |
pub const SSMRR: *mut u8 = 0xE5 as *mut u8;

/// SSM Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SSMESM | 1111 |
/// | SSMERR | 10000000 |
pub const SSMSR: *mut u8 = 0xE6 as *mut u8;

/// SSM Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SSMIF | 1 |
pub const SSMIFR: *mut u8 = 0xE7 as *mut u8;

/// SSM interrupt mask register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SSMIM | 1 |
pub const SSMIMR: *mut u8 = 0xE8 as *mut u8;

/// Master State Machine state register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SSMMST | 11111 |
pub const MSMSTR: *mut u8 = 0xE9 as *mut u8;

/// SSM State Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SSMSTA | 111111 |
pub const SSMSTR: *mut u8 = 0xEA as *mut u8;

/// SSM extended State Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SSMSTB | 111111 |
pub const SSMXSR: *mut u8 = 0xEB as *mut u8;

/// Master State Machine Control Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | MSMSM0 | 1111 |
/// | MSMSM1 | 11110000 |
pub const MSMCR1: *mut u8 = 0xEC as *mut u8;

/// Master State Machine Control Register 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | MSMSM3 | 11110000 |
/// | MSMSM2 | 1111 |
pub const MSMCR2: *mut u8 = 0xED as *mut u8;

/// Master State Machine Control Register 3.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | MSMSM4 | 1111 |
/// | MSMSM5 | 11110000 |
pub const MSMCR3: *mut u8 = 0xEE as *mut u8;

/// Master State Machine Control Register 4.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | MSMSM6 | 1111 |
/// | MSMSM7 | 11110000 |
pub const MSMCR4: *mut u8 = 0xEF as *mut u8;

/// Get Telegram Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | IWUPA | 1000 |
/// | IWUPB | 10000000 |
/// | DARA | 100 |
/// | GAPMB | 100000 |
/// | RXTEHA | 1 |
/// | GAPMA | 10 |
/// | RXTEHB | 10000 |
/// | DARB | 1000000 |
pub const GTCR: *mut u8 = 0xF0 as *mut u8;

/// Start Of Telegram Conditions 1 for Path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SYTOEA1 | 100 |
/// | RROEA1 | 1000000 |
/// | AMPOEA1 | 10 |
/// | CAROEA1 | 1 |
/// | WUPEA1 | 10000 |
/// | SFIDEA1 | 100000 |
/// | WCOBOE1 | 10000000 |
/// | MANOEA1 | 1000 |
pub const SOTC1A: *mut u8 = 0xF1 as *mut u8;

/// Start Of Telegram Conditions 2 for Path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RROEA2 | 1000000 |
/// | WCOBOE2 | 10000000 |
/// | WUPEA2 | 10000 |
/// | CAROEA2 | 1 |
/// | MANOEA2 | 1000 |
/// | SFIDEA2 | 100000 |
/// | AMPOEA2 | 10 |
/// | SYTOEA2 | 100 |
pub const SOTC2A: *mut u8 = 0xF2 as *mut u8;

/// Start Of Telegram Conditions 1 for Path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | MANOEB1 | 1000 |
/// | WCOAOE1 | 10000000 |
/// | SYTOEB1 | 100 |
/// | WUPEB1 | 10000 |
/// | RROEB1 | 1000000 |
/// | CAROEB1 | 1 |
/// | AMPOEB1 | 10 |
/// | SFIDEB1 | 100000 |
pub const SOTC1B: *mut u8 = 0xF3 as *mut u8;

/// Start Of Telegram Conditions 2 for Path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WCOAOE2 | 10000000 |
/// | AMPOEB2 | 10 |
/// | WUPEB2 | 10000 |
/// | MANOEB2 | 1000 |
/// | RROEB2 | 1000000 |
/// | SFIDEB2 | 100000 |
/// | SYTOEB2 | 100 |
/// | CAROEB2 | 1 |
pub const SOTC2B: *mut u8 = 0xF4 as *mut u8;

/// End Of Telegram Conditions 1 for Path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RRFEA1 | 1000000 |
/// | MANFEA1 | 1000 |
/// | EOTBFE1 | 10000000 |
/// | TMOFEA1 | 10000 |
/// | AMPFEA1 | 10 |
/// | TELREA1 | 100000 |
/// | SYTFEA1 | 100 |
/// | CARFEA1 | 1 |
pub const EOTC1A: *mut u8 = 0xF5 as *mut u8;

/// End Of Telegram Conditions 2 for Path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EOTBFE2 | 10000000 |
/// | CARFEA2 | 1 |
/// | RRFEA2 | 1000000 |
/// | TELREA2 | 100000 |
/// | TMOFEA2 | 10000 |
/// | MANFEA2 | 1000 |
/// | SYTFEA2 | 100 |
/// | AMPFEA2 | 10 |
pub const EOTC2A: *mut u8 = 0xF6 as *mut u8;

/// End Of Telegram Conditions 3 for Path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RRFEA3 | 1000000 |
/// | SYTFEA3 | 100 |
/// | TELREA3 | 100000 |
/// | CARFEA3 | 1 |
/// | TMOFEA3 | 10000 |
/// | MANFEA3 | 1000 |
/// | AMPFEA3 | 10 |
/// | EOTBFE3 | 10000000 |
pub const EOTC3A: *mut u8 = 0xF7 as *mut u8;

/// End Of Telegram Conditions 1 for Path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CARFEB1 | 1 |
/// | AMPFEB1 | 10 |
/// | TMOFEB1 | 10000 |
/// | EOTAFE1 | 10000000 |
/// | TELREB1 | 100000 |
/// | SYTFEB1 | 100 |
/// | MANFEB1 | 1000 |
/// | RRFEB1 | 1000000 |
pub const EOTC1B: *mut u8 = 0xF8 as *mut u8;

/// End Of Telegram Conditions 2 for Path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CARFEB2 | 1 |
/// | TMOFEB2 | 10000 |
/// | MANFEB2 | 1000 |
/// | RRFEB2 | 1000000 |
/// | TELREB2 | 100000 |
/// | EOTAFE2 | 10000000 |
/// | AMPFEB2 | 10 |
/// | SYTFEB2 | 100 |
pub const EOTC2B: *mut u8 = 0xF9 as *mut u8;

/// End Of Telegram Conditions 3 for Path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TELREB3 | 100000 |
/// | TMOFEB3 | 10000 |
/// | MANFEB3 | 1000 |
/// | AMPFEB3 | 10 |
/// | RRFEB3 | 1000000 |
/// | EOTAFE3 | 10000000 |
/// | SYTFEB3 | 100 |
/// | CARFEB3 | 1 |
pub const EOTC3B: *mut u8 = 0xFA as *mut u8;

/// Wait check ok time out for path A.
pub const WCOTOA: *mut u8 = 0xFB as *mut u8;

/// Wait check ok time out for path B.
pub const WCOTOB: *mut u8 = 0xFC as *mut u8;

/// Start Of Telegram Time Out for path A.
pub const SOTTOA: *mut u8 = 0xFD as *mut u8;

/// Start Of Telegram Time Out for path B.
pub const SOTTOB: *mut u8 = 0xFE as *mut u8;

/// SSM Flow Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SSMIDSF | 10 |
/// | SSMIDSO | 1 |
pub const SSMFCR: *mut u8 = 0xFF as *mut u8;

/// Front-End Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | HBSAT | 10 |
/// | LBSAT | 1 |
/// | XRDY | 100 |
/// | PLCK | 1000 |
/// | ANTS | 10000 |
pub const FESR: *mut u8 = 0x100 as *mut u8;

/// Front-End Enable Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADEN | 10000 |
/// | PLEN | 1 |
/// | PLSP1 | 1000000 |
/// | XTOEN | 100 |
/// | LNAEN | 1000 |
/// | ADCLK | 100000 |
/// | ATEN | 10000000 |
/// | PLCAL | 10 |
pub const FEEN1: *mut u8 = 0x101 as *mut u8;

/// Front-End Enable Register 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CPBIA | 1000000 |
/// | SDTX | 10 |
/// | PAEN | 100 |
/// | PLPEN | 10000 |
/// | XTPEN | 100000 |
/// | SDRX | 1 |
/// | TMPM | 1000 |
pub const FEEN2: *mut u8 = 0x102 as *mut u8;

/// Front-End LNA Bias Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LBH | 1111 |
/// | LBL | 11110000 |
pub const FELNA: *mut u8 = 0x103 as *mut u8;

/// Front-End Antenna Tuning.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ANTN | 1111 |
pub const FEAT: *mut u8 = 0x104 as *mut u8;

/// Front-End Power Amplifier Control Register Pout/dBm LowBand HighBand.
pub const FEPAC: *mut u8 = 0x105 as *mut u8;

/// Front-End VCO Tuning Register.
pub const FEVCT: *mut u8 = 0x106 as *mut u8;

/// Front-End RC Tuning Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RTN2 | 1100 |
/// | CTN2 | 11 |
pub const FEBT: *mut u8 = 0x107 as *mut u8;

/// Front-End Main and Swallow Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PLLM | 11110000 |
/// | PLLS | 1111 |
pub const FEMS: *mut u8 = 0x108 as *mut u8;

/// Front-End RC Tuning 4bit Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CTN4 | 1111 |
/// | RTN4 | 11110000 |
pub const FETN4: *mut u8 = 0x109 as *mut u8;

/// Front-End Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PLCKG | 10000 |
/// | ANDP | 100 |
/// | ANPS | 100000 |
/// | LBNHB | 1 |
/// | S4N3 | 10 |
/// | ADHS | 1000 |
pub const FECR: *mut u8 = 0x10A as *mut u8;

/// Front-End VCO and PLL control.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CPCC | 1111 |
/// | VCOB | 11110000 |
pub const FEVCO: *mut u8 = 0x10B as *mut u8;

/// Front-End Antenna Level Detector Range.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RNGE | 11 |
pub const FEALR: *mut u8 = 0x10C as *mut u8;

/// Front-End ANTenna.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LVLC | 1111 |
pub const FEANT: *mut u8 = 0x10D as *mut u8;

/// Front-End IF Amplifier BIAS.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | IFAEN | 10000000 |
pub const FEBIA: *mut u8 = 0x10E as *mut u8;

/// Tx Modulator Finite State Machine.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TMSSM | 1111 |
/// | TMMSM | 1110000 |
pub const TMFSM: *mut u8 = 0x120 as *mut u8;

/// Tx Modulator CRC Result.
pub const TMCR: *mut u16 = 0x121 as *mut u16;

/// Tx Modulator CRC Result low byte.
pub const TMCRL: *mut u8 = 0x121 as *mut u8;

/// Tx Modulator CRC Result high byte.
pub const TMCRH: *mut u8 = 0x122 as *mut u8;

/// Tx Modulator CRC Skip Bit Number.
pub const TMCSB: *mut u8 = 0x123 as *mut u8;

/// Tx Modulator CRC Init Value low byte.
pub const TMCIL: *mut u8 = 0x124 as *mut u8;

/// Tx Modulator CRC Init Value.
pub const TMCI: *mut u16 = 0x124 as *mut u16;

/// Tx Modulator CRC Init Value high byte.
pub const TMCIH: *mut u8 = 0x125 as *mut u8;

/// Tx Modulator CRC Polynomial.
pub const TMCP: *mut u16 = 0x126 as *mut u16;

/// Tx Modulator CRC Polynomial low byte.
pub const TMCPL: *mut u8 = 0x126 as *mut u8;

/// Tx Modulator CRC Polynomial high byte.
pub const TMCPH: *mut u8 = 0x127 as *mut u8;

/// Tx Modulator Shift Register.
pub const TMSHR: *mut u8 = 0x128 as *mut u8;

/// Tx Modulator Telegram Length Register.
pub const TMTL: *mut u16 = 0x129 as *mut u16;

/// Tx Modulator Telegram Length Register low byte.
pub const TMTLL: *mut u8 = 0x129 as *mut u8;

/// Tx Modulator Telegram Length Register high byte.
pub const TMTLH: *mut u8 = 0x12A as *mut u8;

/// Tx Modulator Stop Sequence Configuration.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TMSSP | 1111 |
/// | TMSSL | 1110000 |
/// | TMSSH | 10000000 |
pub const TMSSC: *mut u8 = 0x12B as *mut u8;

/// Tx Modulator Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TMTCF | 1 |
pub const TMSR: *mut u8 = 0x12C as *mut u8;

/// Tx Modulator Control Register 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TMCRCE | 1 |
/// | TMPOL | 10000 |
/// | TMSSE | 100000 |
/// | TMMSB | 1000000 |
/// | TMNRZE | 1000 |
/// | TMCRCL | 110 |
pub const TMCR2: *mut u8 = 0x12D as *mut u8;

/// Tx Modulator Control Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TMPIS | 111 |
/// | TMSCS | 1000 |
/// | TMCIM | 10000 |
pub const TMCR1: *mut u8 = 0x12E as *mut u8;

/// Rx Buffer configuration register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RXMSBA | 1000 |
/// | RXCEB | 10000 |
/// | RXCBLB | 1100000 |
/// | RXCBLA | 110 |
/// | RXCEA | 1 |
/// | RXMSBB | 10000000 |
pub const RXBC1: *mut u8 = 0x12F as *mut u8;

/// Rx Buffer configuration register 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RXBF | 10 |
/// | RXBPB | 1 |
/// | RXBCLR | 100 |
pub const RXBC2: *mut u8 = 0x130 as *mut u8;

/// Rx data telegram length register low byte for data path B.
pub const RXTLLB: *mut u8 = 0x131 as *mut u8;

/// Rx data telegram length register high byte for data path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RXTLHB3 | 1000 |
/// | RXTLHB1 | 10 |
/// | RXTLHB2 | 100 |
/// | RXTLHB0 | 1 |
pub const RXTLHB: *mut u8 = 0x132 as *mut u8;

/// Rx CRC result register low byte for data path B.
pub const RXCRLB: *mut u8 = 0x133 as *mut u8;

/// Rx CRC result register high byte for data path B.
pub const RXCRHB: *mut u8 = 0x134 as *mut u8;

/// Rx CRC skip bit number for data path B.
pub const RXCSBB: *mut u8 = 0x135 as *mut u8;

/// Rx CRC Init value (16-bit RXCI) low byte for data path B.
pub const RXCILB: *mut u8 = 0x136 as *mut u8;

/// Rx CRC Init value (16-bit RXCI) high byte for data path B.
pub const RXCIHB: *mut u8 = 0x137 as *mut u8;

/// Rx CRC polynomial low byte for data path B.
pub const RXCPLB: *mut u8 = 0x138 as *mut u8;

/// Rx CRC polynomial (15 bit RXCPB) high byte for data path B.
pub const RXCPHB: *mut u8 = 0x139 as *mut u8;

/// Rx data shift register for data path B.
pub const RXDSB: *mut u8 = 0x13A as *mut u8;

/// Rx data telegram length register low byte for data path A.
pub const RXTLLA: *mut u8 = 0x13B as *mut u8;

/// Rx data telegram length register high byte for data path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RXTLHA2 | 100 |
/// | RXTLHA3 | 1000 |
/// | RXTLHA0 | 1 |
/// | RXTLHA1 | 10 |
pub const RXTLHA: *mut u8 = 0x13C as *mut u8;

/// Rx CRC result register low byte for data path A.
pub const RXCRLA: *mut u8 = 0x13D as *mut u8;

/// Rx CRC result register high byte for data path A.
pub const RXCRHA: *mut u8 = 0x13E as *mut u8;

/// Rx CRC skip bit number for data path A.
pub const RXCSBA: *mut u8 = 0x13F as *mut u8;

/// Rx CRC Init value (16-bit RXCI) low byte for data path A.
pub const RXCILA: *mut u8 = 0x140 as *mut u8;

/// Rx CRC Init value (16-bit RXCI) high byte for data path A.
pub const RXCIHA: *mut u8 = 0x141 as *mut u8;

/// Rx CRC polynomial low byte for data path A.
pub const RXCPLA: *mut u8 = 0x142 as *mut u8;

/// Rx CRC polynomial (15 bit RXCPA) high byte for data path A.
pub const RXCPHA: *mut u8 = 0x143 as *mut u8;

/// Rx data shift register for data path A.
pub const RXDSA: *mut u8 = 0x144 as *mut u8;

/// CRC Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | REFLI | 10 |
/// | CRCRS | 1 |
/// | REFLO | 100 |
pub const CRCCR: *mut u8 = 0x145 as *mut u8;

/// CRC Data Output Register.
pub const CRCDOR: *mut u8 = 0x146 as *mut u8;

/// ID Byte 0.
pub const IDB0: *mut u8 = 0x147 as *mut u8;

/// ID Byte 1.
pub const IDB1: *mut u8 = 0x148 as *mut u8;

/// ID Byte 2.
pub const IDB2: *mut u8 = 0x149 as *mut u8;

/// ID Byte 3.
pub const IDB3: *mut u8 = 0x14A as *mut u8;

/// ID Configuration.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | IDBO | 1100 |
/// | IDCLR | 1000000 |
/// | IDFIM | 100000 |
/// | IDL | 11 |
/// | IDCE | 10000000 |
pub const IDC: *mut u8 = 0x14B as *mut u8;

/// ID Status.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | IDFULL | 10 |
/// | IDOK | 1 |
pub const IDS: *mut u8 = 0x14C as *mut u8;

/// RSSI Average Value.
pub const RSSAV: *mut u8 = 0x14D as *mut u8;

/// RSSI Peak Value.
pub const RSSPK: *mut u8 = 0x14E as *mut u8;

/// RSSI Low Threshold for Signal Check.
pub const RSSL: *mut u8 = 0x14F as *mut u8;

/// RSSI High Threshold for Signal Check.
pub const RSSH: *mut u8 = 0x150 as *mut u8;

/// RSSI Configuration Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RSWLH | 10000 |
/// | RSUP | 1111 |
/// | RSHRX | 100000 |
/// | RSPKF | 1000000 |
pub const RSSC: *mut u8 = 0x151 as *mut u8;

/// DeBounce Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DBHA | 1000 |
/// | DBMD | 1 |
/// | DBTMS | 100 |
/// | DBCS | 10 |
pub const DBCR: *mut u8 = 0x152 as *mut u8;

/// Debounce Timer Compare Register.
pub const DBTC: *mut u8 = 0x153 as *mut u8;

/// DeBounce Enable Port B.
pub const DBENB: *mut u8 = 0x154 as *mut u8;

/// DeBounce Enable Port C.
pub const DBENC: *mut u8 = 0x155 as *mut u8;

/// Debugging Support Switch.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CPBF | 1000000 |
/// | DBGSE | 10000000 |
/// | DBGGS | 1111 |
/// | CPBFOS | 110000 |
pub const DBGSW: *mut u8 = 0x156 as *mut u8;

/// SPI FIFO Fill Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TFL | 1110000 |
/// | RFL | 111 |
/// | RFC | 1000 |
/// | TFC | 10000000 |
pub const SFFR: *mut u8 = 0x157 as *mut u8;

/// SPI FIFO Interrupt Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SRIE | 1000 |
/// | STIE | 10000000 |
/// | TIL | 1110000 |
/// | RIL | 111 |
pub const SFIR: *mut u8 = 0x158 as *mut u8;

/// EEPROM Control Register 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EEBRE | 1 |
pub const EECR2: *mut u8 = 0x159 as *mut u8;

/// Program Memory Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PGMSYN | 11111 |
pub const PGMST: *mut u8 = 0x15A as *mut u8;

/// EEPROM Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EESYN | 1111 |
pub const EEST: *mut u8 = 0x15B as *mut u8;

/// RSSI High IF Amplifier Gain.
pub const RSIFG: *mut u8 = 0x15C as *mut u8;

/// RSSI Low Band Damping Value.
pub const RSLDV: *mut u8 = 0x15D as *mut u8;

/// RSSI High Band Damping Value.
pub const RSHDV: *mut u8 = 0x15E as *mut u8;

/// RSSI Compensation Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RSIFC | 10 |
/// | RSDC | 1 |
pub const RSCOM: *mut u8 = 0x15F as *mut u8;

/// Bitfield on register `CHCR`
pub const BWM: u8 = 0xF;

/// Bitfield on register `CHDN`
pub const BBDN: u8 = 0x1F;

/// Bitfield on register `CHDN`
pub const ADCDN: u8 = 0x20;

/// Bitfield on register `CLKOCR`
pub const CLKOEN: u8 = 0x4;

/// Bitfield on register `CLKOCR`
pub const CLKOS: u8 = 0x3;

/// Bitfield on register `CLPR`
pub const CLPCE: u8 = 0x80;

/// Bitfield on register `CLPR`
pub const CLKPS: u8 = 0x7;

/// Bitfield on register `CLPR`
pub const CLTPS: u8 = 0x38;

/// Bitfield on register `CMCR`
pub const CMCCE: u8 = 0x80;

/// Bitfield on register `CMCR`
pub const SRCD: u8 = 0x10;

/// Bitfield on register `CMCR`
pub const CCS: u8 = 0x8;

/// Bitfield on register `CMCR`
pub const CMONEN: u8 = 0x40;

/// Bitfield on register `CMCR`
pub const CMM: u8 = 0x7;

/// Bitfield on register `CMIMR`
pub const ECIE: u8 = 0x1;

/// Bitfield on register `CMOCR`
pub const SRCAO: u8 = 0x2;

/// Bitfield on register `CMOCR`
pub const FRCAO: u8 = 0x1;

/// Bitfield on register `CMOCR`
pub const SRCACT: u8 = 0x8;

/// Bitfield on register `CMOCR`
pub const FRCACT: u8 = 0x4;

/// Bitfield on register `CMSR`
pub const ECF: u8 = 0x1;

/// Bitfield on register `CRCCR`
pub const REFLI: u8 = 0x2;

/// Bitfield on register `CRCCR`
pub const CRCRS: u8 = 0x1;

/// Bitfield on register `CRCCR`
pub const REFLO: u8 = 0x4;

/// Bitfield on register `DBCR`
pub const DBHA: u8 = 0x8;

/// Bitfield on register `DBCR`
pub const DBMD: u8 = 0x1;

/// Bitfield on register `DBCR`
pub const DBTMS: u8 = 0x4;

/// Bitfield on register `DBCR`
pub const DBCS: u8 = 0x2;

/// Bitfield on register `DBGSW`
pub const CPBF: u8 = 0x40;

/// Bitfield on register `DBGSW`
pub const DBGSE: u8 = 0x80;

/// Bitfield on register `DBGSW`
pub const DBGGS: u8 = 0xF;

/// Bitfield on register `DBGSW`
pub const CPBFOS: u8 = 0x30;

/// Bitfield on register `DFC`
pub const DFFLC: u8 = 0x3F;

/// Bitfield on register `DFC`
pub const DFDRA: u8 = 0x80;

/// Bitfield on register `DFI`
pub const DFERIM: u8 = 0x2;

/// Bitfield on register `DFI`
pub const DFFLIM: u8 = 0x1;

/// Bitfield on register `DFL`
pub const DFCLR: u8 = 0x80;

/// Bitfield on register `DFL`
pub const DFFLS: u8 = 0x3F;

/// Bitfield on register `DFS`
pub const DFFLRF: u8 = 0x1;

/// Bitfield on register `DFS`
pub const DFUFL: u8 = 0x2;

/// Bitfield on register `DFS`
pub const DFOFL: u8 = 0x4;

/// Bitfield on register `DMCDA`
pub const DMCLA: u8 = 0x1F;

/// Bitfield on register `DMCDA`
pub const DMCTA: u8 = 0xE0;

/// Bitfield on register `DMCDB`
pub const DMCTB: u8 = 0xE0;

/// Bitfield on register `DMCDB`
pub const DMCLB: u8 = 0x1F;

/// Bitfield on register `DMCRA`
pub const SY1TA: u8 = 0x40;

/// Bitfield on register `DMCRA`
pub const SASKA: u8 = 0x20;

/// Bitfield on register `DMCRA`
pub const DMPGA: u8 = 0x1F;

/// Bitfield on register `DMCRA`
pub const DMARA: u8 = 0x80;

/// Bitfield on register `DMCRB`
pub const DMARB: u8 = 0x80;

/// Bitfield on register `DMCRB`
pub const DMPGB: u8 = 0x1F;

/// Bitfield on register `DMCRB`
pub const SASKB: u8 = 0x20;

/// Bitfield on register `DMCRB`
pub const SY1TB: u8 = 0x40;

/// Bitfield on register `DMDRA`
pub const DMAA: u8 = 0xF;

/// Bitfield on register `DMDRA`
pub const DMDNA: u8 = 0xF0;

/// Bitfield on register `DMDRB`
pub const DMAB: u8 = 0xF;

/// Bitfield on register `DMDRB`
pub const DMDNB: u8 = 0xF0;

/// Bitfield on register `DMMA`
pub const DMHA: u8 = 0x40;

/// Bitfield on register `DMMA`
pub const DMPA: u8 = 0x20;

/// Bitfield on register `DMMA`
pub const DMNEA: u8 = 0x80;

/// Bitfield on register `DMMA`
pub const DMATA: u8 = 0x1F;

/// Bitfield on register `DMMB`
pub const DMNEB: u8 = 0x80;

/// Bitfield on register `DMMB`
pub const DMATB: u8 = 0x1F;

/// Bitfield on register `DMMB`
pub const DMPB: u8 = 0x20;

/// Bitfield on register `DMMB`
pub const DMHB: u8 = 0x40;

/// Bitfield on register `EECR`
pub const EERE: u8 = 0x1;

/// Bitfield on register `EECR`
pub const NVMBSY: u8 = 0x80;

/// Bitfield on register `EECR`
pub const EEWE: u8 = 0x2;

/// Bitfield on register `EECR`
pub const EEPAGE: u8 = 0x40;

/// Bitfield on register `EECR`
pub const EEMWE: u8 = 0x4;

/// Bitfield on register `EECR`
pub const EEPM: u8 = 0x30;

/// Bitfield on register `EECR`
pub const EERIE: u8 = 0x8;

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
pub const INT1: u8 = 0x2;

/// Bitfield on register `EIMSK`
pub const INT0: u8 = 0x1;

/// Bitfield on register `EOTC1A`
pub const RRFEA1: u8 = 0x40;

/// Bitfield on register `EOTC1A`
pub const MANFEA1: u8 = 0x8;

/// Bitfield on register `EOTC1A`
pub const EOTBFE1: u8 = 0x80;

/// Bitfield on register `EOTC1A`
pub const TMOFEA1: u8 = 0x10;

/// Bitfield on register `EOTC1A`
pub const AMPFEA1: u8 = 0x2;

/// Bitfield on register `EOTC1A`
pub const TELREA1: u8 = 0x20;

/// Bitfield on register `EOTC1A`
pub const SYTFEA1: u8 = 0x4;

/// Bitfield on register `EOTC1A`
pub const CARFEA1: u8 = 0x1;

/// Bitfield on register `EOTC1B`
pub const CARFEB1: u8 = 0x1;

/// Bitfield on register `EOTC1B`
pub const AMPFEB1: u8 = 0x2;

/// Bitfield on register `EOTC1B`
pub const TMOFEB1: u8 = 0x10;

/// Bitfield on register `EOTC1B`
pub const EOTAFE1: u8 = 0x80;

/// Bitfield on register `EOTC1B`
pub const TELREB1: u8 = 0x20;

/// Bitfield on register `EOTC1B`
pub const SYTFEB1: u8 = 0x4;

/// Bitfield on register `EOTC1B`
pub const MANFEB1: u8 = 0x8;

/// Bitfield on register `EOTC1B`
pub const RRFEB1: u8 = 0x40;

/// Bitfield on register `EOTC2A`
pub const EOTBFE2: u8 = 0x80;

/// Bitfield on register `EOTC2A`
pub const CARFEA2: u8 = 0x1;

/// Bitfield on register `EOTC2A`
pub const RRFEA2: u8 = 0x40;

/// Bitfield on register `EOTC2A`
pub const TELREA2: u8 = 0x20;

/// Bitfield on register `EOTC2A`
pub const TMOFEA2: u8 = 0x10;

/// Bitfield on register `EOTC2A`
pub const MANFEA2: u8 = 0x8;

/// Bitfield on register `EOTC2A`
pub const SYTFEA2: u8 = 0x4;

/// Bitfield on register `EOTC2A`
pub const AMPFEA2: u8 = 0x2;

/// Bitfield on register `EOTC2B`
pub const CARFEB2: u8 = 0x1;

/// Bitfield on register `EOTC2B`
pub const TMOFEB2: u8 = 0x10;

/// Bitfield on register `EOTC2B`
pub const MANFEB2: u8 = 0x8;

/// Bitfield on register `EOTC2B`
pub const RRFEB2: u8 = 0x40;

/// Bitfield on register `EOTC2B`
pub const TELREB2: u8 = 0x20;

/// Bitfield on register `EOTC2B`
pub const EOTAFE2: u8 = 0x80;

/// Bitfield on register `EOTC2B`
pub const AMPFEB2: u8 = 0x2;

/// Bitfield on register `EOTC2B`
pub const SYTFEB2: u8 = 0x4;

/// Bitfield on register `EOTC3A`
pub const RRFEA3: u8 = 0x40;

/// Bitfield on register `EOTC3A`
pub const SYTFEA3: u8 = 0x4;

/// Bitfield on register `EOTC3A`
pub const TELREA3: u8 = 0x20;

/// Bitfield on register `EOTC3A`
pub const CARFEA3: u8 = 0x1;

/// Bitfield on register `EOTC3A`
pub const TMOFEA3: u8 = 0x10;

/// Bitfield on register `EOTC3A`
pub const MANFEA3: u8 = 0x8;

/// Bitfield on register `EOTC3A`
pub const AMPFEA3: u8 = 0x2;

/// Bitfield on register `EOTC3A`
pub const EOTBFE3: u8 = 0x80;

/// Bitfield on register `EOTC3B`
pub const TELREB3: u8 = 0x20;

/// Bitfield on register `EOTC3B`
pub const TMOFEB3: u8 = 0x10;

/// Bitfield on register `EOTC3B`
pub const MANFEB3: u8 = 0x8;

/// Bitfield on register `EOTC3B`
pub const AMPFEB3: u8 = 0x2;

/// Bitfield on register `EOTC3B`
pub const RRFEB3: u8 = 0x40;

/// Bitfield on register `EOTC3B`
pub const EOTAFE3: u8 = 0x80;

/// Bitfield on register `EOTC3B`
pub const SYTFEB3: u8 = 0x4;

/// Bitfield on register `EOTC3B`
pub const CARFEB3: u8 = 0x1;

/// Bitfield on register `EOTCA`
pub const AMPFEA: u8 = 0x2;

/// Bitfield on register `EOTCA`
pub const EOTBFE: u8 = 0x80;

/// Bitfield on register `EOTCA`
pub const MANFEA: u8 = 0x8;

/// Bitfield on register `EOTCA`
pub const CARFEA: u8 = 0x1;

/// Bitfield on register `EOTCA`
pub const TELREA: u8 = 0x20;

/// Bitfield on register `EOTCA`
pub const SYTFEA: u8 = 0x4;

/// Bitfield on register `EOTCA`
pub const TMOFEA: u8 = 0x10;

/// Bitfield on register `EOTCA`
pub const RRFEA: u8 = 0x40;

/// Bitfield on register `EOTCB`
pub const SYTFEB: u8 = 0x4;

/// Bitfield on register `EOTCB`
pub const EOTAFE: u8 = 0x80;

/// Bitfield on register `EOTCB`
pub const AMPFEB: u8 = 0x2;

/// Bitfield on register `EOTCB`
pub const RRFEB: u8 = 0x40;

/// Bitfield on register `EOTCB`
pub const TMOFEB: u8 = 0x10;

/// Bitfield on register `EOTCB`
pub const TELREB: u8 = 0x20;

/// Bitfield on register `EOTCB`
pub const CARFEB: u8 = 0x1;

/// Bitfield on register `EOTCB`
pub const MANFEB: u8 = 0x8;

/// Bitfield on register `EOTSA`
pub const AMPFA: u8 = 0x2;

/// Bitfield on register `EOTSA`
pub const EOTBF: u8 = 0x80;

/// Bitfield on register `EOTSA`
pub const TELRA: u8 = 0x20;

/// Bitfield on register `EOTSA`
pub const TMOFA: u8 = 0x10;

/// Bitfield on register `EOTSA`
pub const CARFA: u8 = 0x1;

/// Bitfield on register `EOTSA`
pub const MANFA: u8 = 0x8;

/// Bitfield on register `EOTSA`
pub const RRFA: u8 = 0x40;

/// Bitfield on register `EOTSA`
pub const SYTFA: u8 = 0x4;

/// Bitfield on register `EOTSB`
pub const CARFB: u8 = 0x1;

/// Bitfield on register `EOTSB`
pub const SYTFB: u8 = 0x4;

/// Bitfield on register `EOTSB`
pub const RRFB: u8 = 0x40;

/// Bitfield on register `EOTSB`
pub const EOTAF: u8 = 0x80;

/// Bitfield on register `EOTSB`
pub const AMPFB: u8 = 0x2;

/// Bitfield on register `EOTSB`
pub const TELRB: u8 = 0x20;

/// Bitfield on register `EOTSB`
pub const TMOFB: u8 = 0x10;

/// Bitfield on register `EOTSB`
pub const MANFB: u8 = 0x8;

/// Bitfield on register `FEALR`
pub const RNGE: u8 = 0x3;

/// Bitfield on register `FEANT`
pub const LVLC: u8 = 0xF;

/// Bitfield on register `FEAT`
pub const ANTN: u8 = 0xF;

/// Bitfield on register `FEBIA`
pub const IFAEN: u8 = 0x80;

/// Bitfield on register `FEBT`
pub const RTN2: u8 = 0xC;

/// Bitfield on register `FEBT`
pub const CTN2: u8 = 0x3;

/// Bitfield on register `FECR`
pub const PLCKG: u8 = 0x10;

/// Bitfield on register `FECR`
pub const ANDP: u8 = 0x4;

/// Bitfield on register `FECR`
pub const ANPS: u8 = 0x20;

/// Bitfield on register `FECR`
pub const LBNHB: u8 = 0x1;

/// Bitfield on register `FECR`
pub const S4N3: u8 = 0x2;

/// Bitfield on register `FECR`
pub const ADHS: u8 = 0x8;

/// Bitfield on register `FEEN1`
pub const ADEN: u8 = 0x10;

/// Bitfield on register `FEEN1`
pub const PLEN: u8 = 0x1;

/// Bitfield on register `FEEN1`
pub const PLSP1: u8 = 0x40;

/// Bitfield on register `FEEN1`
pub const XTOEN: u8 = 0x4;

/// Bitfield on register `FEEN1`
pub const LNAEN: u8 = 0x8;

/// Bitfield on register `FEEN1`
pub const ADCLK: u8 = 0x20;

/// Bitfield on register `FEEN1`
pub const ATEN: u8 = 0x80;

/// Bitfield on register `FEEN1`
pub const PLCAL: u8 = 0x2;

/// Bitfield on register `FEEN2`
pub const CPBIA: u8 = 0x40;

/// Bitfield on register `FEEN2`
pub const SDTX: u8 = 0x2;

/// Bitfield on register `FEEN2`
pub const PAEN: u8 = 0x4;

/// Bitfield on register `FEEN2`
pub const PLPEN: u8 = 0x10;

/// Bitfield on register `FEEN2`
pub const XTPEN: u8 = 0x20;

/// Bitfield on register `FEEN2`
pub const SDRX: u8 = 0x1;

/// Bitfield on register `FEEN2`
pub const TMPM: u8 = 0x8;

/// Bitfield on register `FELNA`
pub const LBH: u8 = 0xF;

/// Bitfield on register `FELNA`
pub const LBL: u8 = 0xF0;

/// Bitfield on register `FEMS`
pub const PLLM: u8 = 0xF0;

/// Bitfield on register `FEMS`
pub const PLLS: u8 = 0xF;

/// Bitfield on register `FESR`
pub const HBSAT: u8 = 0x2;

/// Bitfield on register `FESR`
pub const LBSAT: u8 = 0x1;

/// Bitfield on register `FESR`
pub const XRDY: u8 = 0x4;

/// Bitfield on register `FESR`
pub const PLCK: u8 = 0x8;

/// Bitfield on register `FESR`
pub const ANTS: u8 = 0x10;

/// Bitfield on register `FETN4`
pub const CTN4: u8 = 0xF;

/// Bitfield on register `FETN4`
pub const RTN4: u8 = 0xF0;

/// Bitfield on register `FEVCO`
pub const CPCC: u8 = 0xF;

/// Bitfield on register `FEVCO`
pub const VCOB: u8 = 0xF0;

/// Bitfield on register `FRCCAL`
pub const FRCTC: u8 = 0x20;

/// Bitfield on register `FSCR`
pub const PAOER: u8 = 0x10;

/// Bitfield on register `FSCR`
pub const TXMOD: u8 = 0x1;

/// Bitfield on register `FSCR`
pub const SFM: u8 = 0x2;

/// Bitfield on register `FSCR`
pub const TXMS: u8 = 0xC;

/// Bitfield on register `FSCR`
pub const PAON: u8 = 0x80;

/// Bitfield on register `FSEN`
pub const ASEN: u8 = 0x10;

/// Bitfield on register `FSEN`
pub const SDEN: u8 = 0x2;

/// Bitfield on register `FSEN`
pub const PEEN: u8 = 0x8;

/// Bitfield on register `FSEN`
pub const SDPU: u8 = 0x1;

/// Bitfield on register `FSEN`
pub const GAEN: u8 = 0x4;

/// Bitfield on register `FSEN`
pub const ANTT: u8 = 0x20;

/// Bitfield on register `FSFCR`
pub const BTSEL: u8 = 0x3;

/// Bitfield on register `FSFCR`
pub const ASDIV: u8 = 0xF0;

/// Bitfield on register `GTCCR`
pub const TSM: u8 = 0x80;

/// Bitfield on register `GTCCR`
pub const PSR10: u8 = 0x1;

/// Bitfield on register `GTCR`
pub const IWUPA: u8 = 0x8;

/// Bitfield on register `GTCR`
pub const IWUPB: u8 = 0x80;

/// Bitfield on register `GTCR`
pub const DARA: u8 = 0x4;

/// Bitfield on register `GTCR`
pub const GAPMB: u8 = 0x20;

/// Bitfield on register `GTCR`
pub const RXTEHA: u8 = 0x1;

/// Bitfield on register `GTCR`
pub const GAPMA: u8 = 0x2;

/// Bitfield on register `GTCR`
pub const RXTEHB: u8 = 0x10;

/// Bitfield on register `GTCR`
pub const DARB: u8 = 0x40;

/// Bitfield on register `IDC`
pub const IDBO: u8 = 0xC;

/// Bitfield on register `IDC`
pub const IDCLR: u8 = 0x40;

/// Bitfield on register `IDC`
pub const IDFIM: u8 = 0x20;

/// Bitfield on register `IDC`
pub const IDL: u8 = 0x3;

/// Bitfield on register `IDC`
pub const IDCE: u8 = 0x80;

/// Bitfield on register `IDS`
pub const IDFULL: u8 = 0x2;

/// Bitfield on register `IDS`
pub const IDOK: u8 = 0x1;

/// Bitfield on register `LOCKBIT`
pub const AP: u8 = 0xC;

/// Bitfield on register `LOCKBIT`
pub const LB: u8 = 0x3;

/// Bitfield on register `LOCKBIT`
pub const BLP: u8 = 0x30;

/// Bitfield on register `LOW`
pub const DWEN: u8 = 0x40;

/// Bitfield on register `LOW`
pub const CKDIV8: u8 = 0x80;

/// Bitfield on register `LOW`
pub const SPIEN: u8 = 0x20;

/// Bitfield on register `LOW`
pub const BOOTRST: u8 = 0x4;

/// Bitfield on register `LOW`
pub const RSTDISBL: u8 = 0x2;

/// Bitfield on register `LOW`
pub const WDTON: u8 = 0x10;

/// Bitfield on register `LOW`
pub const EXTCLKEN: u8 = 0x1;

/// Bitfield on register `LOW`
pub const EESAVE: u8 = 0x8;

/// Bitfield on register `MCUCR`
pub const PB7HS: u8 = 0x80;

/// Bitfield on register `MCUCR`
pub const IVCE: u8 = 0x1;

/// Bitfield on register `MCUCR`
pub const ENPS: u8 = 0x8;

/// Bitfield on register `MCUCR`
pub const SPIIO: u8 = 0x4;

/// Bitfield on register `MCUCR`
pub const IVSEL: u8 = 0x2;

/// Bitfield on register `MCUCR`
pub const PB4HS: u8 = 0x20;

/// Bitfield on register `MCUCR`
pub const PUD: u8 = 0x10;

/// Bitfield on register `MCUCR`
pub const PB7LS: u8 = 0x40;

/// Bitfield on register `MCUSR`
pub const WDRF: u8 = 0x8;

/// Bitfield on register `MCUSR`
pub const EXTRF: u8 = 0x2;

/// Bitfield on register `MCUSR`
pub const PORF: u8 = 0x1;

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

/// Bitfield on register `PCICR`
pub const PCIE0: u8 = 0x1;

/// Bitfield on register `PCICR`
pub const PCIE1: u8 = 0x2;

/// Bitfield on register `PCIFR`
pub const PCIF1: u8 = 0x2;

/// Bitfield on register `PCIFR`
pub const PCIF0: u8 = 0x1;

/// Bitfield on register `PCMSK0`
pub const PCINT6: u8 = 0x40;

/// Bitfield on register `PCMSK0`
pub const PCINT7: u8 = 0x80;

/// Bitfield on register `PCMSK0`
pub const PCINT0: u8 = 0x1;

/// Bitfield on register `PCMSK0`
pub const PCINT2: u8 = 0x4;

/// Bitfield on register `PCMSK0`
pub const PCINT1: u8 = 0x2;

/// Bitfield on register `PCMSK0`
pub const PCINT5: u8 = 0x20;

/// Bitfield on register `PCMSK0`
pub const PCINT3: u8 = 0x8;

/// Bitfield on register `PCMSK0`
pub const PCINT4: u8 = 0x10;

/// Bitfield on register `PCMSK1`
pub const PCINT11: u8 = 0x8;

/// Bitfield on register `PCMSK1`
pub const PCINT10: u8 = 0x4;

/// Bitfield on register `PCMSK1`
pub const PCINT12: u8 = 0x10;

/// Bitfield on register `PCMSK1`
pub const PCINT9: u8 = 0x2;

/// Bitfield on register `PCMSK1`
pub const PCINT13: u8 = 0x20;

/// Bitfield on register `PCMSK1`
pub const PCINT8: u8 = 0x1;

/// Bitfield on register `PGMST`
pub const PGMSYN: u8 = 0x1F;

/// Bitfield on register `PRR0`
pub const PRRXDC: u8 = 0x2;

/// Bitfield on register `PRR0`
pub const PRTXDC: u8 = 0x4;

/// Bitfield on register `PRR0`
pub const PRSPI: u8 = 0x1;

/// Bitfield on register `PRR0`
pub const PRCRC: u8 = 0x8;

/// Bitfield on register `PRR0`
pub const PRCO: u8 = 0x20;

/// Bitfield on register `PRR0`
pub const PRVM: u8 = 0x10;

/// Bitfield on register `PRR1`
pub const PRT4: u8 = 0x8;

/// Bitfield on register `PRR1`
pub const PRT2: u8 = 0x2;

/// Bitfield on register `PRR1`
pub const PRT1: u8 = 0x1;

/// Bitfield on register `PRR1`
pub const PRT3: u8 = 0x4;

/// Bitfield on register `PRR1`
pub const PRT5: u8 = 0x10;

/// Bitfield on register `PRR2`
pub const PRXA: u8 = 0x2;

/// Bitfield on register `PRR2`
pub const PRSSM: u8 = 0x80;

/// Bitfield on register `PRR2`
pub const PRSF: u8 = 0x4;

/// Bitfield on register `PRR2`
pub const PRTM: u8 = 0x40;

/// Bitfield on register `PRR2`
pub const PRRS: u8 = 0x20;

/// Bitfield on register `PRR2`
pub const PRDF: u8 = 0x8;

/// Bitfield on register `PRR2`
pub const PRXB: u8 = 0x1;

/// Bitfield on register `PRR2`
pub const PRIDS: u8 = 0x10;

/// Bitfield on register `RDCR`
pub const RDPU: u8 = 0x1;

/// Bitfield on register `RDCR`
pub const RDEN: u8 = 0x4;

/// Bitfield on register `RDCR`
pub const ADIVEN: u8 = 0x2;

/// Bitfield on register `RDOCR`
pub const TMDS: u8 = 0x6;

/// Bitfield on register `RDOCR`
pub const ETRPB: u8 = 0x10;

/// Bitfield on register `RDOCR`
pub const RDSIDB: u8 = 0x40;

/// Bitfield on register `RDOCR`
pub const RDSIDA: u8 = 0x20;

/// Bitfield on register `RDOCR`
pub const ETRPA: u8 = 0x8;

/// Bitfield on register `RDPR`
pub const PRFLT: u8 = 0x4;

/// Bitfield on register `RDPR`
pub const RDPRF: u8 = 0x80;

/// Bitfield on register `RDPR`
pub const APRPTB: u8 = 0x10;

/// Bitfield on register `RDPR`
pub const ARDPRF: u8 = 0x40;

/// Bitfield on register `RDPR`
pub const PRPTB: u8 = 0x1;

/// Bitfield on register `RDPR`
pub const APRPTA: u8 = 0x20;

/// Bitfield on register `RDPR`
pub const PRTMP: u8 = 0x8;

/// Bitfield on register `RDPR`
pub const PRPTA: u8 = 0x2;

/// Bitfield on register `RDSIFR`
pub const SOTB: u8 = 0x20;

/// Bitfield on register `RDSIFR`
pub const NBITA: u8 = 0x1;

/// Bitfield on register `RDSIFR`
pub const WCOA: u8 = 0x40;

/// Bitfield on register `RDSIFR`
pub const SOTA: u8 = 0x10;

/// Bitfield on register `RDSIFR`
pub const EOTA: u8 = 0x4;

/// Bitfield on register `RDSIFR`
pub const EOTB: u8 = 0x8;

/// Bitfield on register `RDSIFR`
pub const NBITB: u8 = 0x2;

/// Bitfield on register `RDSIFR`
pub const WCOB: u8 = 0x80;

/// Bitfield on register `RDSIMR`
pub const SOTBM: u8 = 0x20;

/// Bitfield on register `RDSIMR`
pub const WCOBM: u8 = 0x80;

/// Bitfield on register `RDSIMR`
pub const EOTBM: u8 = 0x8;

/// Bitfield on register `RDSIMR`
pub const EOTAM: u8 = 0x4;

/// Bitfield on register `RDSIMR`
pub const SOTAM: u8 = 0x10;

/// Bitfield on register `RDSIMR`
pub const NBITAM: u8 = 0x1;

/// Bitfield on register `RDSIMR`
pub const WCOAM: u8 = 0x40;

/// Bitfield on register `RDSIMR`
pub const NBITBM: u8 = 0x2;

/// Bitfield on register `RSCOM`
pub const RSIFC: u8 = 0x2;

/// Bitfield on register `RSCOM`
pub const RSDC: u8 = 0x1;

/// Bitfield on register `RSSC`
pub const RSWLH: u8 = 0x10;

/// Bitfield on register `RSSC`
pub const RSUP: u8 = 0xF;

/// Bitfield on register `RSSC`
pub const RSHRX: u8 = 0x20;

/// Bitfield on register `RSSC`
pub const RSPKF: u8 = 0x40;

/// Bitfield on register `RXBC1`
pub const RXMSBA: u8 = 0x8;

/// Bitfield on register `RXBC1`
pub const RXCEB: u8 = 0x10;

/// Bitfield on register `RXBC1`
pub const RXCBLB: u8 = 0x60;

/// Bitfield on register `RXBC1`
pub const RXCBLA: u8 = 0x6;

/// Bitfield on register `RXBC1`
pub const RXCEA: u8 = 0x1;

/// Bitfield on register `RXBC1`
pub const RXMSBB: u8 = 0x80;

/// Bitfield on register `RXBC2`
pub const RXBF: u8 = 0x2;

/// Bitfield on register `RXBC2`
pub const RXBPB: u8 = 0x1;

/// Bitfield on register `RXBC2`
pub const RXBCLR: u8 = 0x4;

/// Bitfield on register `RXTLHA`
pub const RXTLHA2: u8 = 0x4;

/// Bitfield on register `RXTLHA`
pub const RXTLHA3: u8 = 0x8;

/// Bitfield on register `RXTLHA`
pub const RXTLHA0: u8 = 0x1;

/// Bitfield on register `RXTLHA`
pub const RXTLHA1: u8 = 0x2;

/// Bitfield on register `RXTLHB`
pub const RXTLHB3: u8 = 0x8;

/// Bitfield on register `RXTLHB`
pub const RXTLHB1: u8 = 0x2;

/// Bitfield on register `RXTLHB`
pub const RXTLHB2: u8 = 0x4;

/// Bitfield on register `RXTLHB`
pub const RXTLHB0: u8 = 0x1;

/// Bitfield on register `SFC`
pub const SFDRA: u8 = 0x80;

/// Bitfield on register `SFC`
pub const SFFLC: u8 = 0x1F;

/// Bitfield on register `SFFR`
pub const TFL: u8 = 0x70;

/// Bitfield on register `SFFR`
pub const RFL: u8 = 0x7;

/// Bitfield on register `SFFR`
pub const RFC: u8 = 0x8;

/// Bitfield on register `SFFR`
pub const TFC: u8 = 0x80;

/// Bitfield on register `SFI`
pub const SFERIM: u8 = 0x2;

/// Bitfield on register `SFI`
pub const SFFLIM: u8 = 0x1;

/// Bitfield on register `SFIDCA`
pub const SEMEA: u8 = 0x80;

/// Bitfield on register `SFIDCA`
pub const SFIDTA: u8 = 0x1F;

/// Bitfield on register `SFIDCB`
pub const SFIDTB: u8 = 0x1F;

/// Bitfield on register `SFIDCB`
pub const SEMEB: u8 = 0x80;

/// Bitfield on register `SFIR`
pub const SRIE: u8 = 0x8;

/// Bitfield on register `SFIR`
pub const STIE: u8 = 0x80;

/// Bitfield on register `SFIR`
pub const TIL: u8 = 0x70;

/// Bitfield on register `SFIR`
pub const RIL: u8 = 0x7;

/// Bitfield on register `SFL`
pub const SFFLS: u8 = 0x1F;

/// Bitfield on register `SFL`
pub const SFCLR: u8 = 0x80;

/// Bitfield on register `SFS`
pub const SFOFL: u8 = 0x4;

/// Bitfield on register `SFS`
pub const SFFLRF: u8 = 0x1;

/// Bitfield on register `SFS`
pub const SFUFL: u8 = 0x2;

/// Bitfield on register `SMCR`
pub const SM: u8 = 0xE;

/// Bitfield on register `SMCR`
pub const SE: u8 = 0x1;

/// Bitfield on register `SOTC1A`
pub const SYTOEA1: u8 = 0x4;

/// Bitfield on register `SOTC1A`
pub const RROEA1: u8 = 0x40;

/// Bitfield on register `SOTC1A`
pub const AMPOEA1: u8 = 0x2;

/// Bitfield on register `SOTC1A`
pub const CAROEA1: u8 = 0x1;

/// Bitfield on register `SOTC1A`
pub const WUPEA1: u8 = 0x10;

/// Bitfield on register `SOTC1A`
pub const SFIDEA1: u8 = 0x20;

/// Bitfield on register `SOTC1A`
pub const WCOBOE1: u8 = 0x80;

/// Bitfield on register `SOTC1A`
pub const MANOEA1: u8 = 0x8;

/// Bitfield on register `SOTC1B`
pub const MANOEB1: u8 = 0x8;

/// Bitfield on register `SOTC1B`
pub const WCOAOE1: u8 = 0x80;

/// Bitfield on register `SOTC1B`
pub const SYTOEB1: u8 = 0x4;

/// Bitfield on register `SOTC1B`
pub const WUPEB1: u8 = 0x10;

/// Bitfield on register `SOTC1B`
pub const RROEB1: u8 = 0x40;

/// Bitfield on register `SOTC1B`
pub const CAROEB1: u8 = 0x1;

/// Bitfield on register `SOTC1B`
pub const AMPOEB1: u8 = 0x2;

/// Bitfield on register `SOTC1B`
pub const SFIDEB1: u8 = 0x20;

/// Bitfield on register `SOTC2A`
pub const RROEA2: u8 = 0x40;

/// Bitfield on register `SOTC2A`
pub const WCOBOE2: u8 = 0x80;

/// Bitfield on register `SOTC2A`
pub const WUPEA2: u8 = 0x10;

/// Bitfield on register `SOTC2A`
pub const CAROEA2: u8 = 0x1;

/// Bitfield on register `SOTC2A`
pub const MANOEA2: u8 = 0x8;

/// Bitfield on register `SOTC2A`
pub const SFIDEA2: u8 = 0x20;

/// Bitfield on register `SOTC2A`
pub const AMPOEA2: u8 = 0x2;

/// Bitfield on register `SOTC2A`
pub const SYTOEA2: u8 = 0x4;

/// Bitfield on register `SOTC2B`
pub const WCOAOE2: u8 = 0x80;

/// Bitfield on register `SOTC2B`
pub const AMPOEB2: u8 = 0x2;

/// Bitfield on register `SOTC2B`
pub const WUPEB2: u8 = 0x10;

/// Bitfield on register `SOTC2B`
pub const MANOEB2: u8 = 0x8;

/// Bitfield on register `SOTC2B`
pub const RROEB2: u8 = 0x40;

/// Bitfield on register `SOTC2B`
pub const SFIDEB2: u8 = 0x20;

/// Bitfield on register `SOTC2B`
pub const SYTOEB2: u8 = 0x4;

/// Bitfield on register `SOTC2B`
pub const CAROEB2: u8 = 0x1;

/// Bitfield on register `SOTCA`
pub const CAROEA: u8 = 0x1;

/// Bitfield on register `SOTCA`
pub const RROEA: u8 = 0x40;

/// Bitfield on register `SOTCA`
pub const AMPOEA: u8 = 0x2;

/// Bitfield on register `SOTCA`
pub const WCOBOE: u8 = 0x80;

/// Bitfield on register `SOTCA`
pub const SYTOEA: u8 = 0x4;

/// Bitfield on register `SOTCA`
pub const SFIDEA: u8 = 0x20;

/// Bitfield on register `SOTCA`
pub const MANOEA: u8 = 0x8;

/// Bitfield on register `SOTCA`
pub const WUPEA: u8 = 0x10;

/// Bitfield on register `SOTCB`
pub const WUPEB: u8 = 0x10;

/// Bitfield on register `SOTCB`
pub const SFIDEB: u8 = 0x20;

/// Bitfield on register `SOTCB`
pub const MANOEB: u8 = 0x8;

/// Bitfield on register `SOTCB`
pub const WCOAOE: u8 = 0x80;

/// Bitfield on register `SOTCB`
pub const RROEB: u8 = 0x40;

/// Bitfield on register `SOTCB`
pub const SYTOEB: u8 = 0x4;

/// Bitfield on register `SOTCB`
pub const CAROEB: u8 = 0x1;

/// Bitfield on register `SOTCB`
pub const AMPOEB: u8 = 0x2;

/// Bitfield on register `SOTSA`
pub const AMPOA: u8 = 0x2;

/// Bitfield on register `SOTSA`
pub const RROA: u8 = 0x40;

/// Bitfield on register `SOTSA`
pub const MANOA: u8 = 0x8;

/// Bitfield on register `SOTSA`
pub const SFIDOA: u8 = 0x20;

/// Bitfield on register `SOTSA`
pub const CAROA: u8 = 0x1;

/// Bitfield on register `SOTSA`
pub const SYTOA: u8 = 0x4;

/// Bitfield on register `SOTSA`
pub const WCOBO: u8 = 0x80;

/// Bitfield on register `SOTSA`
pub const WUPOA: u8 = 0x10;

/// Bitfield on register `SOTSB`
pub const AMPOB: u8 = 0x2;

/// Bitfield on register `SOTSB`
pub const CAROB: u8 = 0x1;

/// Bitfield on register `SOTSB`
pub const SFIDOB: u8 = 0x20;

/// Bitfield on register `SOTSB`
pub const RROB: u8 = 0x40;

/// Bitfield on register `SOTSB`
pub const SYTOB: u8 = 0x4;

/// Bitfield on register `SOTSB`
pub const WUPOB: u8 = 0x10;

/// Bitfield on register `SOTSB`
pub const WCOAO: u8 = 0x80;

/// Bitfield on register `SOTSB`
pub const MANOB: u8 = 0x8;

/// Bitfield on register `SPCR`
pub const CPOL: u8 = 0x8;

/// Bitfield on register `SPCR`
pub const MSTR: u8 = 0x10;

/// Bitfield on register `SPCR`
pub const DORD: u8 = 0x20;

/// Bitfield on register `SPCR`
pub const SPIE: u8 = 0x80;

/// Bitfield on register `SPCR`
pub const CPHA: u8 = 0x4;

/// Bitfield on register `SPCR`
pub const SPE: u8 = 0x40;

/// Bitfield on register `SPCR`
pub const SPR: u8 = 0x3;

/// Bitfield on register `SPMCSR`
pub const PGERS: u8 = 0x2;

/// Bitfield on register `SPMCSR`
pub const PGWRT: u8 = 0x4;

/// Bitfield on register `SPMCSR`
pub const BLBSET: u8 = 0x8;

/// Bitfield on register `SPMCSR`
pub const SPMIE: u8 = 0x80;

/// Bitfield on register `SPMCSR`
pub const SELFPRGEN: u8 = 0x1;

/// Bitfield on register `SPSR`
pub const SPI2X: u8 = 0x1;

/// Bitfield on register `SPSR`
pub const TXIF: u8 = 0x20;

/// Bitfield on register `SPSR`
pub const SPIF: u8 = 0x80;

/// Bitfield on register `SPSR`
pub const RXIF: u8 = 0x10;

/// Bitfield on register `SRCCAL`
pub const SRCTC: u8 = 0xC0;

/// Bitfield on register `SREG`
pub const S: u8 = 0x10;

/// Bitfield on register `SREG`
pub const N: u8 = 0x4;

/// Bitfield on register `SREG`
pub const Z: u8 = 0x2;

/// Bitfield on register `SREG`
pub const V: u8 = 0x8;

/// Bitfield on register `SREG`
pub const T: u8 = 0x40;

/// Bitfield on register `SREG`
pub const H: u8 = 0x20;

/// Bitfield on register `SREG`
pub const C: u8 = 0x1;

/// Bitfield on register `SREG`
pub const I: u8 = 0x80;

/// Bitfield on register `SSMCR`
pub const SSMTM: u8 = 0x2;

/// Bitfield on register `SSMCR`
pub const SETRPB: u8 = 0x80;

/// Bitfield on register `SSMCR`
pub const SSMPVE: u8 = 0x10;

/// Bitfield on register `SSMCR`
pub const SSMTX: u8 = 0x1;

/// Bitfield on register `SSMCR`
pub const SSMTPE: u8 = 0x8;

/// Bitfield on register `SSMCR`
pub const SSMTAE: u8 = 0x20;

/// Bitfield on register `SSMCR`
pub const SSMTGE: u8 = 0x4;

/// Bitfield on register `SSMCR`
pub const SETRPA: u8 = 0x40;

/// Bitfield on register `SSMFBR`
pub const SSMDFDT: u8 = 0x8;

/// Bitfield on register `SSMFBR`
pub const SSMHADT: u8 = 0x10;

/// Bitfield on register `SSMFBR`
pub const SSMFID: u8 = 0x7;

/// Bitfield on register `SSMFBR`
pub const SSMPLDT: u8 = 0x20;

/// Bitfield on register `SSMFCR`
pub const SSMIDSF: u8 = 0x2;

/// Bitfield on register `SSMFCR`
pub const SSMIDSO: u8 = 0x1;

/// Bitfield on register `SSMIFR`
pub const SSMIF: u8 = 0x1;

/// Bitfield on register `SSMIMR`
pub const SSMIM: u8 = 0x1;

/// Bitfield on register `SSMRCR`
pub const SSMADB: u8 = 0x8;

/// Bitfield on register `SSMRCR`
pub const SSMIFA: u8 = 0x20;

/// Bitfield on register `SSMRCR`
pub const SSMPB: u8 = 0x2;

/// Bitfield on register `SSMRCR`
pub const SSMTMOE: u8 = 0x80;

/// Bitfield on register `SSMRCR`
pub const SSMPVS: u8 = 0x10;

/// Bitfield on register `SSMRCR`
pub const SSMIDSE: u8 = 0x40;

/// Bitfield on register `SSMRCR`
pub const SSMADA: u8 = 0x4;

/// Bitfield on register `SSMRCR`
pub const SSMPA: u8 = 0x1;

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
pub const PVCAL: u8 = 0xF0;

/// Bitfield on register `SUPCA1`
pub const PV22: u8 = 0x4;

/// Bitfield on register `SUPCA1`
pub const PVDIC: u8 = 0x8;

/// Bitfield on register `SUPCA2`
pub const BGCAL: u8 = 0xF;

/// Bitfield on register `SUPCA3`
pub const ACAL6: u8 = 0x4;

/// Bitfield on register `SUPCA3`
pub const DCAL6: u8 = 0x40;

/// Bitfield on register `SUPCA3`
pub const ACAL5: u8 = 0x2;

/// Bitfield on register `SUPCA3`
pub const DCAL5: u8 = 0x20;

/// Bitfield on register `SUPCA3`
pub const ACAL7: u8 = 0x8;

/// Bitfield on register `SUPCA3`
pub const DCAL4: u8 = 0x10;

/// Bitfield on register `SUPCA3`
pub const ACAL4: u8 = 0x1;

/// Bitfield on register `SUPCA4`
pub const DCAL3: u8 = 0x80;

/// Bitfield on register `SUPCA4`
pub const ACAL1: u8 = 0x2;

/// Bitfield on register `SUPCA4`
pub const DCAL0: u8 = 0x10;

/// Bitfield on register `SUPCA4`
pub const DCAL2: u8 = 0x40;

/// Bitfield on register `SUPCA4`
pub const ACAL0: u8 = 0x1;

/// Bitfield on register `SUPCA4`
pub const ACAL2: u8 = 0x4;

/// Bitfield on register `SUPCA4`
pub const ACAL3: u8 = 0x8;

/// Bitfield on register `SUPCA4`
pub const DCAL1: u8 = 0x20;

/// Bitfield on register `SUPCR`
pub const AVDIC: u8 = 0x40;

/// Bitfield on register `SUPCR`
pub const AVCCRM: u8 = 0x1;

/// Bitfield on register `SUPCR`
pub const PVEN: u8 = 0x4;

/// Bitfield on register `SUPCR`
pub const AVEN: u8 = 0x20;

/// Bitfield on register `SUPCR`
pub const DVDIS: u8 = 0x10;

/// Bitfield on register `SUPCR`
pub const AVCCLM: u8 = 0x2;

/// Bitfield on register `SUPFR`
pub const AVCCLF: u8 = 0x2;

/// Bitfield on register `SUPFR`
pub const AVCCRF: u8 = 0x1;

/// Bitfield on register `SYCA`
pub const SYTLA: u8 = 0xF0;

/// Bitfield on register `SYCA`
pub const SYCSA: u8 = 0xF;

/// Bitfield on register `SYCB`
pub const SYTLB: u8 = 0xF0;

/// Bitfield on register `SYCB`
pub const SYCSB: u8 = 0xF;

/// Bitfield on register `T0CR`
pub const T0PS: u8 = 0x7;

/// Bitfield on register `T0CR`
pub const T0IE: u8 = 0x8;

/// Bitfield on register `T0CR`
pub const T0PR: u8 = 0x10;

/// Bitfield on register `T0IFR`
pub const T0F: u8 = 0x1;

/// Bitfield on register `T1CR`
pub const T1OTM: u8 = 0x1;

/// Bitfield on register `T1CR`
pub const T1CRM: u8 = 0x4;

/// Bitfield on register `T1CR`
pub const T1RES: u8 = 0x20;

/// Bitfield on register `T1CR`
pub const T1CTM: u8 = 0x2;

/// Bitfield on register `T1CR`
pub const T1TOS: u8 = 0x40;

/// Bitfield on register `T1CR`
pub const T1ENA: u8 = 0x80;

/// Bitfield on register `T1CR`
pub const T1TOP: u8 = 0x10;

/// Bitfield on register `T1IFR`
pub const T1OFF: u8 = 0x1;

/// Bitfield on register `T1IFR`
pub const T1COF: u8 = 0x2;

/// Bitfield on register `T1IMR`
pub const T1OIM: u8 = 0x1;

/// Bitfield on register `T1IMR`
pub const T1CIM: u8 = 0x2;

/// Bitfield on register `T1MR`
pub const T1DC: u8 = 0xC0;

/// Bitfield on register `T1MR`
pub const T1PS: u8 = 0x3C;

/// Bitfield on register `T1MR`
pub const T1CS: u8 = 0x3;

/// Bitfield on register `T2CR`
pub const T2OTM: u8 = 0x1;

/// Bitfield on register `T2CR`
pub const T2CRM: u8 = 0x4;

/// Bitfield on register `T2CR`
pub const T2RES: u8 = 0x20;

/// Bitfield on register `T2CR`
pub const T2TOP: u8 = 0x10;

/// Bitfield on register `T2CR`
pub const T2CTM: u8 = 0x2;

/// Bitfield on register `T2CR`
pub const T2ENA: u8 = 0x80;

/// Bitfield on register `T2CR`
pub const T2TOS: u8 = 0x40;

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
pub const T2CS: u8 = 0x3;

/// Bitfield on register `T2MR`
pub const T2PS: u8 = 0x3C;

/// Bitfield on register `T3CR`
pub const T3RES: u8 = 0x20;

/// Bitfield on register `T3CR`
pub const T3CRM: u8 = 0x4;

/// Bitfield on register `T3CR`
pub const T3CTM: u8 = 0x2;

/// Bitfield on register `T3CR`
pub const T3TOP: u8 = 0x10;

/// Bitfield on register `T3CR`
pub const T3CPRM: u8 = 0x8;

/// Bitfield on register `T3CR`
pub const T3OTM: u8 = 0x1;

/// Bitfield on register `T3CR`
pub const T3TOS: u8 = 0x40;

/// Bitfield on register `T3CR`
pub const T3ENA: u8 = 0x80;

/// Bitfield on register `T3IFR`
pub const T3OFF: u8 = 0x1;

/// Bitfield on register `T3IFR`
pub const T3ICF: u8 = 0x4;

/// Bitfield on register `T3IFR`
pub const T3COF: u8 = 0x2;

/// Bitfield on register `T3IMR`
pub const T3OIM: u8 = 0x1;

/// Bitfield on register `T3IMR`
pub const T3CPIM: u8 = 0x4;

/// Bitfield on register `T3IMR`
pub const T3CIM: u8 = 0x2;

/// Bitfield on register `T3MRA`
pub const T3PS: u8 = 0x1C;

/// Bitfield on register `T3MRA`
pub const T3CS: u8 = 0x3;

/// Bitfield on register `T3MRB`
pub const T3CE: u8 = 0x18;

/// Bitfield on register `T3MRB`
pub const T3SCE: u8 = 0x2;

/// Bitfield on register `T3MRB`
pub const T3CNC: u8 = 0x4;

/// Bitfield on register `T3MRB`
pub const T3ICS: u8 = 0xE0;

/// Bitfield on register `T4CR`
pub const T4RES: u8 = 0x20;

/// Bitfield on register `T4CR`
pub const T4OTM: u8 = 0x1;

/// Bitfield on register `T4CR`
pub const T4TOS: u8 = 0x40;

/// Bitfield on register `T4CR`
pub const T4CPRM: u8 = 0x8;

/// Bitfield on register `T4CR`
pub const T4TOP: u8 = 0x10;

/// Bitfield on register `T4CR`
pub const T4CRM: u8 = 0x4;

/// Bitfield on register `T4CR`
pub const T4CTM: u8 = 0x2;

/// Bitfield on register `T4CR`
pub const T4ENA: u8 = 0x80;

/// Bitfield on register `T4IFR`
pub const T4ICF: u8 = 0x4;

/// Bitfield on register `T4IFR`
pub const T4OFF: u8 = 0x1;

/// Bitfield on register `T4IFR`
pub const T4COF: u8 = 0x2;

/// Bitfield on register `T4IMR`
pub const T4CPIM: u8 = 0x4;

/// Bitfield on register `T4IMR`
pub const T4CIM: u8 = 0x2;

/// Bitfield on register `T4IMR`
pub const T4OIM: u8 = 0x1;

/// Bitfield on register `T4MRA`
pub const T4PS: u8 = 0x1C;

/// Bitfield on register `T4MRA`
pub const T4CS: u8 = 0x3;

/// Bitfield on register `T4MRB`
pub const T4CNC: u8 = 0x4;

/// Bitfield on register `T4MRB`
pub const T4ICS: u8 = 0xE0;

/// Bitfield on register `T4MRB`
pub const T4CE: u8 = 0x18;

/// Bitfield on register `T4MRB`
pub const T4SCE: u8 = 0x2;

/// Bitfield on register `T5CCR`
pub const T5CS: u8 = 0x7;

/// Bitfield on register `T5CCR`
pub const T5CTC: u8 = 0x8;

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
pub const CRCOB: u8 = 0x1;

/// Bitfield on register `TESRB`
pub const EOTLB: u8 = 0x6;

/// Bitfield on register `TMCR1`
pub const TMPIS: u8 = 0x7;

/// Bitfield on register `TMCR1`
pub const TMSCS: u8 = 0x8;

/// Bitfield on register `TMCR1`
pub const TMCIM: u8 = 0x10;

/// Bitfield on register `TMCR2`
pub const TMCRCE: u8 = 0x1;

/// Bitfield on register `TMCR2`
pub const TMPOL: u8 = 0x10;

/// Bitfield on register `TMCR2`
pub const TMSSE: u8 = 0x20;

/// Bitfield on register `TMCR2`
pub const TMMSB: u8 = 0x40;

/// Bitfield on register `TMCR2`
pub const TMNRZE: u8 = 0x8;

/// Bitfield on register `TMCR2`
pub const TMCRCL: u8 = 0x6;

/// Bitfield on register `TMFSM`
pub const TMSSM: u8 = 0xF;

/// Bitfield on register `TMFSM`
pub const TMMSM: u8 = 0x70;

/// Bitfield on register `TMSR`
pub const TMTCF: u8 = 0x1;

/// Bitfield on register `TMSSC`
pub const TMSSP: u8 = 0xF;

/// Bitfield on register `TMSSC`
pub const TMSSL: u8 = 0x70;

/// Bitfield on register `TMSSC`
pub const TMSSH: u8 = 0x80;

/// Bitfield on register `VMCSR`
pub const VMLS: u8 = 0xF;

/// Bitfield on register `VMCSR`
pub const VMF: u8 = 0x20;

/// Bitfield on register `VMCSR`
pub const VMIM: u8 = 0x10;

/// Bitfield on register `WDTCR`
pub const WDE: u8 = 0x8;

/// Bitfield on register `WDTCR`
pub const WDPS: u8 = 0x7;

/// Bitfield on register `WDTCR`
pub const WDCE: u8 = 0x10;

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

/// `CPU_CLK_PRESCALE_3_BITS_SMALL` value group
#[allow(non_upper_case_globals)]
pub mod cpu_clk_prescale_3_bits_small {
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

/// `CPU_CLT_PRESCALE_3_BITS_SMALL` value group
#[allow(non_upper_case_globals)]
pub mod cpu_clt_prescale_3_bits_small {
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

/// `CPU_SLEEP_MODE_3BITS2` value group
#[allow(non_upper_case_globals)]
pub mod cpu_sleep_mode_3bits2 {
   /// Idle.
   pub const IDLE: u32 = 0x0;
   /// Extended power-save.
   pub const EPSAVE: u32 = 0x1;
   /// Power Down.
   pub const PDOWN: u32 = 0x2;
   /// Power Save.
   pub const PSAVE: u32 = 0x3;
   /// Reserved.
   pub const VAL_0x04: u32 = 0x4;
   /// Reserved.
   pub const VAL_0x05: u32 = 0x5;
   /// Reserved.
   pub const VAL_0x06: u32 = 0x6;
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

/// `ENUM_AP` value group
#[allow(non_upper_case_globals)]
pub mod enum_ap {
   /// LPM and SPM prohibited in Application Section.
   pub const VAL_0x00: u32 = 0x0;
   /// LPM prohibited in Application Section.
   pub const VAL_0x04: u32 = 0x4;
   /// SPM prohibited in Application Section.
   pub const VAL_0x08: u32 = 0x8;
   /// No lock on SPM and LPM in Application Section.
   pub const VAL_0x0C: u32 = 0xC;
}

/// `ENUM_BLP` value group
#[allow(non_upper_case_globals)]
pub mod enum_blp {
   /// LPM and SPM prohibited in Boot Loader Section.
   pub const VAL_0x00: u32 = 0x0;
   /// LPM prohibited in Boot Loader Section.
   pub const VAL_0x10: u32 = 0x10;
   /// SPM prohibited in Boot Loader Section.
   pub const VAL_0x20: u32 = 0x20;
   /// No lock on SPM and LPM in Boot Loader Section.
   pub const VAL_0x30: u32 = 0x30;
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

/// `FE_ALR_RANGE` value group
#[allow(non_upper_case_globals)]
pub mod fe_alr_range {
   /// 0..3 dBm.
   pub const VAL_0x00: u32 = 0x0;
   /// 4..7 dBm.
   pub const VAL_0x01: u32 = 0x1;
   /// 8..14 dBm.
   pub const VAL_0x02: u32 = 0x2;
   /// Secure Measurement.
   pub const VAL_0x03: u32 = 0x3;
}

/// `FE_POWER_AMPLIFIER_CONTROL` value group
#[allow(non_upper_case_globals)]
pub mod fe_power_amplifier_control {
   /// -11.80  -12.90.
   pub const VAL_0x00: u32 = 0x0;
   /// -11.30  -12.33.
   pub const VAL_0x01: u32 = 0x1;
   /// -10.70  -11.76.
   pub const VAL_0x02: u32 = 0x2;
   /// -10.20  -11.10.
   pub const VAL_0x03: u32 = 0x3;
   /// -9.70  -10.60.
   pub const VAL_0x04: u32 = 0x4;
   /// -9.20  -10.00.
   pub const VAL_0x05: u32 = 0x5;
   /// -8.60   -9.50.
   pub const VAL_0x06: u32 = 0x6;
   /// -8.00   -9.00.
   pub const VAL_0x07: u32 = 0x7;
   /// -7.50   -8.50.
   pub const VAL_0x08: u32 = 0x8;
   /// -7.00   -7.90.
   pub const VAL_0x09: u32 = 0x9;
   /// -6.40   -7.30.
   pub const VAL_0x0A: u32 = 0xA;
   /// -5.90   -6.80.
   pub const VAL_0x0B: u32 = 0xB;
   /// -5.30   -6.30.
   pub const VAL_0x0C: u32 = 0xC;
   /// -4.77   -5.70.
   pub const VAL_0x0D: u32 = 0xD;
   /// -4.17   -5.20.
   pub const VAL_0x0E: u32 = 0xE;
   /// -3.67   -4.60.
   pub const VAL_0x0F: u32 = 0xF;
   /// -3.12   -4.07.
   pub const VAL_0x10: u32 = 0x10;
   /// -2.56   -3.47.
   pub const VAL_0x11: u32 = 0x11;
   /// -2.10   -2.97.
   pub const VAL_0x12: u32 = 0x12;
   /// -1.58   -2.42.
   pub const VAL_0x13: u32 = 0x13;
   /// -1.08   -1.86.
   pub const VAL_0x14: u32 = 0x14;
   /// -0.50   -1.40.
   pub const VAL_0x15: u32 = 0x15;
   /// 0.00   -0.88.
   pub const VAL_0x16: u32 = 0x16;
   /// 0.41   -0.38.
   pub const VAL_0x17: u32 = 0x17;
   /// 1.00    0.20.
   pub const VAL_0x18: u32 = 0x18;
   /// 1.42    0.70.
   pub const VAL_0x19: u32 = 0x19;
   /// 1.83    1.11.
   pub const VAL_0x1A: u32 = 0x1A;
   /// 2.42    1.70.
   pub const VAL_0x1B: u32 = 0x1B;
   /// 2.88    2.12.
   pub const VAL_0x1C: u32 = 0x1C;
   /// 3.38    2.53.
   pub const VAL_0x1D: u32 = 0x1D;
   /// 3.81    3.12.
   pub const VAL_0x1E: u32 = 0x1E;
   /// 4.31    3.58.
   pub const VAL_0x1F: u32 = 0x1F;
   /// 4.72    4.08.
   pub const VAL_0x20: u32 = 0x20;
   /// 5.09    4.51.
   pub const VAL_0x21: u32 = 0x21;
   /// 5.57    5.01.
   pub const VAL_0x22: u32 = 0x22;
   /// 6.00    5.42.
   pub const VAL_0x23: u32 = 0x23;
   /// 6.41    5.79.
   pub const VAL_0x24: u32 = 0x24;
   /// 6.77    6.27.
   pub const VAL_0x25: u32 = 0x25;
   /// 7.19    6.70.
   pub const VAL_0x26: u32 = 0x26;
   /// 7.55    7.11.
   pub const VAL_0x27: u32 = 0x27;
   /// 7.98    7.47.
   pub const VAL_0x28: u32 = 0x28;
   /// 8.40    7.89.
   pub const VAL_0x29: u32 = 0x29;
   /// 8.79    8.25.
   pub const VAL_0x2A: u32 = 0x2A;
   /// 9.11    8.68.
   pub const VAL_0x2B: u32 = 0x2B;
   /// 9.46    9.10.
   pub const VAL_0x2C: u32 = 0x2C;
   /// 9.82    9.49.
   pub const VAL_0x2D: u32 = 0x2D;
   /// 10.18    9.81.
   pub const VAL_0x2E: u32 = 0x2E;
   /// 10.60   10.16.
   pub const VAL_0x2F: u32 = 0x2F;
   /// 10.89   10.52.
   pub const VAL_0x30: u32 = 0x30;
   /// 11.30   10.88.
   pub const VAL_0x31: u32 = 0x31;
   /// 11.62   11.30.
   pub const VAL_0x32: u32 = 0x32;
   /// 12.06   11.59.
   pub const VAL_0x33: u32 = 0x33;
   /// 12.39   12.00.
   pub const VAL_0x34: u32 = 0x34;
   /// 12.82   12.32.
   pub const VAL_0x35: u32 = 0x35;
   /// 13.22   12.76.
   pub const VAL_0x36: u32 = 0x36;
   /// 13.58   13.09.
   pub const VAL_0x37: u32 = 0x37;
   /// 13.95   13.52.
   pub const VAL_0x38: u32 = 0x38;
   /// 14.22   13.92.
   pub const VAL_0x39: u32 = 0x39;
   /// 14.41   14.28.
   pub const VAL_0x3A: u32 = 0x3A;
   /// 14.49   14.65.
   pub const VAL_0x3B: u32 = 0x3B;
   /// 14.60   14.65.
   pub const VAL_0x3C: u32 = 0x3C;
   /// 14.60   14.65.
   pub const VAL_0x3D: u32 = 0x3D;
   /// 14.60   14.65.
   pub const VAL_0x3E: u32 = 0x3E;
   /// 14.60   14.65.
   pub const VAL_0x3F: u32 = 0x3F;
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

/// `RXBUF_CRC_LENGTH` value group
#[allow(non_upper_case_globals)]
pub mod rxbuf_crc_length {
   /// CRC 4-bit.
   pub const VAL_0x00: u32 = 0x0;
   /// CRC 8-bit.
   pub const VAL_0x01: u32 = 0x1;
   /// CRC 16-bit.
   pub const VAL_0x02: u32 = 0x2;
}

/// `SSM_EOT_LOCATION` value group
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
   /// Antenna Tuning.
   pub const VAL_0x0D: u32 = 0xD;
}

/// `TIM1_CLOCK_SELECT` value group
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

/// `TIM2_CLOCK_SELECT` value group
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

/// `TIM3_CAPTURE_EDGE_SELECT` value group
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

/// `TIM3_CLOCK_SELECT` value group
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

/// `TIM4_CAPTURE_EDGE_SELECT` value group
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

/// `TIM4_CLOCK_SELECT` value group
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

/// `TXM_CRC_LENGTH` value group
#[allow(non_upper_case_globals)]
pub mod txm_crc_length {
   /// CRC 4-bit.
   pub const VAL_0x00: u32 = 0x0;
   /// CRC 8-bit.
   pub const VAL_0x01: u32 = 0x1;
   /// CRC 16-bit.
   pub const VAL_0x02: u32 = 0x2;
}

/// `TXM_PINTERFACE_SELECT` value group
#[allow(non_upper_case_globals)]
pub mod txm_pinterface_select {
   /// Port C3.
   pub const VAL_0x00: u32 = 0x0;
   /// M2 - Toggle Register Timer2.
   pub const VAL_0x01: u32 = 0x1;
   /// M3 - Toggle Register Timer3.
   pub const VAL_0x02: u32 = 0x2;
   /// M4 - Toggle Register Timer4.
   pub const VAL_0x03: u32 = 0x3;
   /// SO Tx Modulator Serial Output.
   pub const VAL_0x04: u32 = 0x4;
}

/// `TX_MODULATION_SOURCE` value group
#[allow(non_upper_case_globals)]
pub mod tx_modulation_source {
   /// TXMOD Register.
   pub const VAL_0x00: u32 = 0x0;
   /// TMDI Input.
   pub const VAL_0x01: u32 = 0x1;
   /// Tx Modulator Serial Out.
   pub const VAL_0x02: u32 = 0x2;
}

