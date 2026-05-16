//! The AVR ATA5702M322 microcontroller
//!
//! # Variants
//! |        | Pinout | Mcu age | Operating temperature | Operating voltage | Max speed |
//! |--------|--------|---------|-----------------------|-------------------|-----------|
//! | standard |  |  | 0°C - 0°C | 2.1V - 4.2V | 0 MHz |
//!

#![allow(non_upper_case_globals)]

/// `LOW` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EESAVE | 1000 |
/// | WDTON | 10000 |
/// | PCEE1 | 1 |
/// | EEACC | 10 |
/// | BOOTRST | 100 |
/// | SPIEN | 100000 |
/// | CKDIV8 | 10000000 |
/// | DWEN | 1000000 |
pub const LOW: *mut u8 = 0x0 as *mut u8;

/// `LOCKBIT` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LB | 11 |
pub const LOCKBIT: *mut u8 = 0x0 as *mut u8;

/// General Purpose I/O Register 0.
pub const GPIOR0: *mut u8 = 0x20 as *mut u8;

/// Power reduction Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRT2 | 10 |
/// | PRT3 | 100 |
/// | PRT5 | 10000 |
/// | PRT1 | 1 |
/// | PRLFR | 100000 |
/// | PRLFPH | 10000000 |
/// | PRLFTP | 1000000 |
/// | PRT4 | 1000 |
pub const PRR1: *mut u8 = 0x21 as *mut u8;

/// Power reduction register 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRSPI2 | 1 |
/// | PRSF | 100 |
/// | PRDF | 1000 |
/// | PRTWI2 | 10 |
/// | PRTM | 1000000 |
/// | PRSSM | 10000000 |
pub const PRR2: *mut u8 = 0x22 as *mut u8;

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

/// Transponder Control 2 Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TPWDLV | 1100000 |
/// | TPPSD | 100 |
/// | TPMOD | 10 |
/// | TPMA | 1 |
/// | TPD | 1000 |
/// | TPNFTO | 10000 |
pub const TPCR2: *mut u8 = 0x2C as *mut u8;

/// Transponder Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TPF | 1 |
/// | TPNFTF | 100 |
/// | TPBERF | 1000 |
/// | TPFTF | 10 |
pub const TPFR: *mut u8 = 0x2D as *mut u8;

/// MCU control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PUD | 10000 |
/// | IVSEL | 10000000 |
/// | SPIIO | 100 |
/// | ENPS | 1000 |
/// | TRCCE | 100000 |
/// | TRCEN | 1000000 |
/// | IVL | 11 |
pub const MCUCR: *mut u8 = 0x2E as *mut u8;

/// Frequency Synthesizer Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PAON | 10000000 |
/// | TXMS | 1100 |
/// | TXMOD | 1 |
/// | PAOER | 10000 |
/// | SFM | 10 |
pub const FSCR: *mut u8 = 0x2F as *mut u8;

/// Timer1 control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T1TOP | 10000 |
/// | T1TOS | 1000000 |
/// | T1RES | 100000 |
/// | T1CRM | 100 |
/// | T1OTM | 1 |
/// | T1ENA | 10000000 |
/// | T1CTM | 10 |
pub const T1CR: *mut u8 = 0x31 as *mut u8;

/// Timer2 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T2TOP | 10000 |
/// | T2CRM | 100 |
/// | T2CTM | 10 |
/// | T2RES | 100000 |
/// | T2ENA | 10000000 |
/// | T2OTM | 1 |
/// | T2TOS | 1000000 |
pub const T2CR: *mut u8 = 0x32 as *mut u8;

/// Timer3 control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T3RES | 100000 |
/// | T3CTM | 10 |
/// | T3TOP | 10000 |
/// | T3TOS | 1000000 |
/// | T3OTM | 1 |
/// | T3CRM | 100 |
/// | T3CPRM | 1000 |
/// | T3ENA | 10000000 |
pub const T3CR: *mut u8 = 0x33 as *mut u8;

/// Timer4 control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T4CRM | 100 |
/// | T4OTM | 1 |
/// | T4CTM | 10 |
/// | T4ENA | 10000000 |
/// | T4RES | 100000 |
/// | T4TOP | 10000 |
/// | T4TOS | 1000000 |
/// | T4CPRM | 1000 |
pub const T4CR: *mut u8 = 0x34 as *mut u8;

/// LF Timer Control Mode Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LTPS0 | 1 |
/// | LTCM | 100000 |
/// | LTPS2 | 100 |
/// | LTSM | 1000000 |
/// | LTENA | 10000000 |
/// | LTCIM | 10000 |
/// | LTPS1 | 10 |
/// | LTCRM | 1000 |
pub const LTCMR: *mut u8 = 0x35 as *mut u8;

/// EEPROM Control Register 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | E2CIM | 10 |
/// | E2CF | 10000000 |
/// | E2FF | 1000000 |
/// | EEBRE | 1 |
/// | E2AVF | 100000 |
pub const EECR2: *mut u8 = 0x36 as *mut u8;

/// PH Telegram Configuration Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FRFIFO | 100000 |
/// | CSM | 10000000 |
/// | CPM | 1000000 |
pub const PHTCR: *mut u8 = 0x37 as *mut u8;

/// LF Data FIFO Fill Level Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LDFCLR | 10000000 |
pub const LDFFL: *mut u8 = 0x38 as *mut u8;

/// LF Data FIFO Data Register.
pub const LDFD: *mut u8 = 0x39 as *mut u8;

/// Power reduction Register 0.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRTXDC | 100 |
/// | PRLFRS | 10 |
/// | PRSPI | 1 |
/// | PRTWI1 | 10000000 |
/// | PRVM | 10000 |
/// | PRCU | 1000000 |
/// | PRCO | 100000 |
/// | PRCRC | 1000 |
pub const PRR0: *mut u8 = 0x3A as *mut u8;

/// Protocol Handler Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CRCEF | 1 |
/// | PHID1F | 100000 |
/// | PHID0F | 10000 |
/// | PHDFF | 100 |
/// | PHIDFF | 1000 |
/// | PHTBLF | 10 |
pub const PHFR: *mut u8 = 0x3B as *mut u8;

/// LF Receiver Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LFES | 10000000 |
/// | LFTOF | 1000 |
/// | LFDEF | 10 |
/// | LFEOF | 100 |
/// | LFSYDF | 1 |
/// | LFSD | 1000000 |
pub const LFFR: *mut u8 = 0x3C as *mut u8;

/// AES Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AESXOR | 10000 |
/// | AESD | 1000 |
/// | AESRES | 100000 |
/// | AESIM | 100 |
/// | AESLKM | 1000000 |
/// | AESWK | 1 |
/// | AESE | 10000000 |
/// | AESWD | 10 |
pub const AESCR: *mut u8 = 0x3D as *mut u8;

/// AES Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AESERF | 10000000 |
/// | AESRF | 1 |
pub const AESSR: *mut u8 = 0x3E as *mut u8;

/// EEPROM Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | NVMBSY | 10000000 |
/// | EERIE | 1000 |
/// | EEWE | 10 |
/// | EERE | 1 |
/// | EEPAGE | 1000000 |
/// | EEMWE | 100 |
/// | EEPM | 110000 |
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
/// | INT0 | 1 |
/// | INT1 | 10 |
pub const EIMSK: *mut u8 = 0x47 as *mut u8;

/// External Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | INTF0 | 1 |
/// | INTF1 | 10 |
pub const EIFR: *mut u8 = 0x48 as *mut u8;

/// LF Data FIFO Clock Switch Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LDFSCKS | 10 |
/// | LDFSCSW | 1 |
pub const LDFCKSW: *mut u8 = 0x49 as *mut u8;

/// Voltage Monitor Status and Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | VMDIH | 10 |
/// | VMF | 1 |
pub const VMSCR: *mut u8 = 0x4A as *mut u8;

/// MCU Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DWRF | 10000 |
/// | WDRF | 1000 |
/// | EXTRF | 10 |
/// | PORF | 1 |
/// | TPRF | 100000 |
pub const MCUSR: *mut u8 = 0x4B as *mut u8;

/// SPI control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CPOL | 1000 |
/// | SPR | 11 |
/// | SPE | 1000000 |
/// | MSTR | 10000 |
/// | CPHA | 100 |
/// | DORD | 100000 |
/// | SPIE | 10000000 |
pub const SPCR: *mut u8 = 0x4C as *mut u8;

/// SPI Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RXIF | 10000 |
/// | SPIF | 10000000 |
/// | SPI2X | 1 |
/// | TXIF | 100000 |
pub const SPSR: *mut u8 = 0x4D as *mut u8;

/// SPI Data Register.
pub const SPDR: *mut u8 = 0x4E as *mut u8;

/// LF Receiver Control Register 0.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LFMG | 100000 |
/// | LFCE1 | 1 |
/// | LFRRT | 11000000 |
/// | LFCE2 | 10 |
/// | LFBR | 11000 |
/// | LFCE3 | 100 |
pub const LFCR0: *mut u8 = 0x4F as *mut u8;

/// LF Receiver Control Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FLLEN | 10000 |
/// | ADTHEN | 100000 |
/// | LFPEEN | 1000000 |
/// | RSST | 11 |
/// | LFFM1 | 100 |
/// | ARMDE | 1000 |
/// | LFRE | 10000000 |
pub const LFCR1: *mut u8 = 0x50 as *mut u8;

/// Debug Wire Data Register.
pub const DWDR: *mut u8 = 0x51 as *mut u8;

/// Timer0 Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T0F | 1 |
pub const T0IFR: *mut u8 = 0x52 as *mut u8;

/// Store Program Memory Control and Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SPMIE | 10000000 |
/// | PGERS | 10 |
/// | FLSEL | 111000 |
/// | PGWRT | 100 |
/// | SELFPRGEN | 1 |
/// | RWWSB | 1000000 |
pub const SPMCSR: *mut u8 = 0x57 as *mut u8;

/// Sleep mode control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SM | 1110 |
/// | SE | 1 |
pub const SMCR: *mut u8 = 0x58 as *mut u8;

/// Transponder Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TPA | 1 |
/// | TPPSW | 100 |
/// | TPBCOK | 1000 |
/// | TPGAP | 10 |
pub const TPSR: *mut u8 = 0x59 as *mut u8;

/// LF Receiver Control Register 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LFDAMP | 100 |
/// | LFSEN | 11 |
/// | LFVC | 11100000 |
pub const LFCR2: *mut u8 = 0x5A as *mut u8;

/// LF Receiver Control Register 3.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LFTON | 1000 |
/// | LFSBEN | 10000000 |
/// | LFRCPM | 100 |
/// | LFRCTEN | 1 |
/// | LFRCPCEN | 10 |
/// | LFTS | 1110000 |
pub const LFCR3: *mut u8 = 0x5B as *mut u8;

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
/// | C | 1 |
/// | N | 100 |
/// | H | 100000 |
/// | T | 1000000 |
/// | S | 10000 |
/// | Z | 10 |
/// | V | 1000 |
/// | I | 10000000 |
pub const SREG: *mut u8 = 0x5F as *mut u8;

/// Frequency Synthesizer Enable register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PEEN | 1000 |
/// | ANTT | 100000 |
/// | ASEN | 10000 |
/// | SDEN | 10 |
/// | GAEN | 100 |
/// | SDPU | 1 |
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

/// Base Band Test Enable 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DITDIS | 10 |
/// | TDEPO | 1 |
pub const BBTE2: *mut u8 = 0x6A as *mut u8;

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
/// | PCINT2 | 100 |
/// | PCINT7 | 10000000 |
/// | PCINT6 | 1000000 |
/// | PCINT5 | 100000 |
/// | PCINT4 | 10000 |
/// | PCINT1 | 10 |
/// | PCINT0 | 1 |
/// | PCINT3 | 1000 |
pub const PCMSK0: *mut u8 = 0x6C as *mut u8;

/// Pin change Mask Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCINT13 | 100000 |
/// | PCINT15 | 10000000 |
/// | PCINT14 | 1000000 |
/// | PCINT12 | 10000 |
/// | PCINT8 | 1 |
/// | PCINT11 | 1000 |
/// | PCINT9 | 10 |
/// | PCINT10 | 100 |
pub const PCMSK1: *mut u8 = 0x6D as *mut u8;

/// Watchdog Timer0 control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDPS | 111 |
/// | WDCE | 10000 |
/// | WDE | 1000 |
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
/// | T1CS | 11 |
/// | T1PS | 111100 |
/// | T1DC | 11000000 |
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

/// Timer3 counter Register low byte.
pub const T3CNTL: *mut u8 = 0x77 as *mut u8;

/// Timer3 counter Register.
pub const T3CNT: *mut u16 = 0x77 as *mut u16;

/// Timer3 counter Register high byte.
pub const T3CNTH: *mut u8 = 0x78 as *mut u8;

/// Timer3 compare Register low byte.
pub const T3CORL: *mut u8 = 0x79 as *mut u8;

/// Timer3 compare Register.
pub const T3COR: *mut u16 = 0x79 as *mut u16;

/// Timer3 compare Register high byte.
pub const T3CORH: *mut u8 = 0x7A as *mut u8;

/// Timer3 input capture Register.
pub const T3ICR: *mut u16 = 0x7B as *mut u16;

/// Timer3 input capture Register low byte.
pub const T3ICRL: *mut u8 = 0x7B as *mut u8;

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
/// | T3SCE | 10 |
/// | T3CNC | 100 |
/// | T3CE | 11000 |
/// | T3ICS | 11100000 |
pub const T3MRB: *mut u8 = 0x7E as *mut u8;

/// Timer3 interrupt mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T3CIM | 10 |
/// | T3OIM | 1 |
/// | T3CPIM | 100 |
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
/// | T4CE | 11000 |
/// | T4ICS | 11100000 |
/// | T4SCE | 10 |
pub const T4MRB: *mut u8 = 0x87 as *mut u8;

/// Timer4 interrupt mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T4OIM | 1 |
/// | T4CPIM | 100 |
/// | T4CIM | 10 |
pub const T4IMR: *mut u8 = 0x88 as *mut u8;

/// Timer5 Temp Register.
pub const T5TEMP: *mut u8 = 0x89 as *mut u8;

/// Timer5 Output Compare Register.
pub const T5OCR: *mut u16 = 0x8A as *mut u16;

/// Timer5 Output Compare Register low byte.
pub const T5OCRL: *mut u8 = 0x8A as *mut u8;

/// Timer5 Output Compare Register high byte.
pub const T5OCRH: *mut u8 = 0x8B as *mut u8;

/// Timer5 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T5CTC | 1000 |
/// | T5CS | 111 |
pub const T5CCR: *mut u8 = 0x8C as *mut u8;

/// Timer5 Counter low byte.
pub const T5CNTL: *mut u8 = 0x8D as *mut u8;

/// Timer5 Counter.
pub const T5CNT: *mut u16 = 0x8D as *mut u16;

/// Timer5 Counter high byte.
pub const T5CNTH: *mut u8 = 0x8E as *mut u8;

/// Timer5 Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T5CIM | 10 |
/// | T5OIM | 1 |
pub const T5IMR: *mut u8 = 0x8F as *mut u8;

/// LF Receiver Calibration Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ICOMPRT | 11000 |
/// | SEL150M | 11100000 |
/// | LFSTC | 111 |
pub const LFCALR1: *mut u8 = 0x90 as *mut u8;

/// LF Receiver Calibration Register 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TIKOMPD | 10000000 |
/// | LFSRM | 1000000 |
/// | LFSTRES | 111111 |
pub const LFCALR2: *mut u8 = 0x91 as *mut u8;

/// LF Receiver Calibration Register 3.
pub const LFCALR3: *mut u8 = 0x92 as *mut u8;

/// LF Receiver Calibration Register 4.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TCGAIN27 | 10000000 |
/// | TCGAIN20 | 1 |
/// | TCGAIN23 | 1000 |
/// | TCGAIN21 | 10 |
/// | TCGAIN24 | 10000 |
/// | TCGAIN26 | 1000000 |
/// | TCGAIN22 | 100 |
/// | TCGAIN25 | 100000 |
pub const LFCALR4: *mut u8 = 0x93 as *mut u8;

/// LF Receiver Calibration Register 5.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TCGAIN37 | 10000000 |
/// | TCGAIN31 | 10 |
/// | TCGAIN32 | 100 |
/// | TCGAIN36 | 1000000 |
/// | TCGAIN35 | 100000 |
/// | TCGAIN30 | 1 |
/// | TCGAIN34 | 10000 |
pub const LFCALR5: *mut u8 = 0x94 as *mut u8;

/// LF Receiver Calibration Register 6.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TCGAIN42 | 100 |
/// | TCGAIN43 | 1000 |
/// | TCGAIN41 | 10 |
/// | TCGAIN40 | 1 |
/// | TCGAIN44 | 10000 |
pub const LFCALR6: *mut u8 = 0x95 as *mut u8;

/// LF Receiver Calibration Register 7.
pub const LFCALR7: *mut u8 = 0x96 as *mut u8;

/// LF Receiver Calibration Register 8.
pub const LFCALR8: *mut u8 = 0x97 as *mut u8;

/// LF Receiver Calibration Register 9.
pub const LFCALR9: *mut u8 = 0x98 as *mut u8;

/// LF Receiver Calibration Register 10.
pub const LFCALR10: *mut u8 = 0x99 as *mut u8;

/// LF Receiver Calibration Register 11.
pub const LFCALR11: *mut u8 = 0x9A as *mut u8;

/// LF Receiver Calibration Register 12.
pub const LFCALR12: *mut u8 = 0x9B as *mut u8;

/// LF Receiver Calibration Register 13.
pub const LFCALR13: *mut u8 = 0x9C as *mut u8;

/// LF Receiver Calibration Register 14.
pub const LFCALR14: *mut u8 = 0x9D as *mut u8;

/// LF Receiver Calibration Register 15.
pub const LFCALR15: *mut u8 = 0x9E as *mut u8;

/// LF Receiver Calibration Register 16.
pub const LFCALR16: *mut u8 = 0x9F as *mut u8;

/// LF Receiver Calibration Register 17.
pub const LFCALR17: *mut u8 = 0xA0 as *mut u8;

/// LF Receiver Calibration Register 18.
pub const LFCALR18: *mut u8 = 0xA1 as *mut u8;

/// LF Receiver Calibration Register 19.
pub const LFCALR19: *mut u8 = 0xA2 as *mut u8;

/// LF Receiver Calibration Register 20.
pub const LFCALR20: *mut u8 = 0xA3 as *mut u8;

/// LF Receiver Calibration Register 21.
pub const LFCALR21: *mut u8 = 0xA4 as *mut u8;

/// LF Receiver Calibration Register 22.
pub const LFCALR22: *mut u8 = 0xA5 as *mut u8;

/// LF Receiver Calibration Register 23.
pub const LFCALR23: *mut u8 = 0xA6 as *mut u8;

/// LF Receiver Calibration Register 24.
pub const LFCALR24: *mut u8 = 0xA7 as *mut u8;

/// LF Receiver Calibration Register 25.
pub const LFCALR25: *mut u8 = 0xA8 as *mut u8;

/// LF Receiver Calibration Register 26.
pub const LFCALR26: *mut u8 = 0xA9 as *mut u8;

/// LF Receiver Calibration Register 27.
pub const LFCALR27: *mut u8 = 0xAA as *mut u8;

/// LF Receiver Calibration Register 28.
pub const LFCALR28: *mut u8 = 0xAB as *mut u8;

/// LF Receiver Calibration Register 29.
pub const LFCALR29: *mut u8 = 0xAC as *mut u8;

/// LF Receiver Calibration Register 30.
pub const LFCALR30: *mut u8 = 0xAD as *mut u8;

/// LF Receiver Calibration Register 31.
pub const LFCALR31: *mut u8 = 0xAE as *mut u8;

/// LF Receiver Calibration Register 32.
pub const LFCALR32: *mut u8 = 0xAF as *mut u8;

/// LF Receiver Calibration Register 33.
pub const LFCALR33: *mut u8 = 0xB0 as *mut u8;

/// LF Receiver Calibration Register 34.
pub const LFCALR34: *mut u8 = 0xB1 as *mut u8;

/// LF Receiver Calibration Register 35.
pub const LFCALR35: *mut u8 = 0xB2 as *mut u8;

/// LF Receiver Calibration Register 36.
pub const LFCALR36: *mut u8 = 0xB3 as *mut u8;

/// LF Receiver Calibration Register 37.
pub const LFCALR37: *mut u8 = 0xB4 as *mut u8;

/// LF Receiver Calibration Register 38.
pub const LFCALR38: *mut u8 = 0xB5 as *mut u8;

/// LF Receiver Calibration Register 39.
pub const LFCALR39: *mut u8 = 0xB6 as *mut u8;

/// LF Receiver Calibration Register 40.
pub const LFCALR40: *mut u8 = 0xB7 as *mut u8;

/// LF Receiver Calibration Register 41.
pub const LFCALR41: *mut u8 = 0xB8 as *mut u8;

/// LF Receiver Calibration Register 42.
pub const LFCALR42: *mut u8 = 0xB9 as *mut u8;

/// LF Receiver Calibration Register 43.
pub const LFCALR43: *mut u8 = 0xBA as *mut u8;

/// LF Receiver Calibration Register 44.
pub const LFCALR44: *mut u8 = 0xBB as *mut u8;

/// LF Receiver Calibration Register 45.
pub const LFCALR45: *mut u8 = 0xBC as *mut u8;

/// LF Receiver Calibration Register 46.
pub const LFCALR46: *mut u8 = 0xBD as *mut u8;

/// LF Receiver Calibration Register 47.
pub const LFCALR47: *mut u8 = 0xBE as *mut u8;

/// LF Receiver Calibration Register 48.
pub const LFCALR48: *mut u8 = 0xBF as *mut u8;

/// LF Receiver Calibration Register 49.
pub const LFCALR49: *mut u8 = 0xC0 as *mut u8;

/// LF Receiver Calibration Register 50.
pub const LFCALR50: *mut u8 = 0xC1 as *mut u8;

/// LF Receiver Calibration Register 51.
pub const LFCALR51: *mut u8 = 0xC2 as *mut u8;

/// LF Receiver Calibration Register 52.
pub const LFCALR52: *mut u8 = 0xC3 as *mut u8;

/// LF Receiver Calibration Register 53.
pub const LFCALR53: *mut u8 = 0xC4 as *mut u8;

/// `XFUSE` register
pub const XFUSE: *mut u8 = 0xC5 as *mut u8;

/// Middle RC oscillator calibration Register.
pub const MRCCAL: *mut u8 = 0xC6 as *mut u8;

/// Fast RC oscillator calibration Register.
pub const FRCCAL: *mut u8 = 0xC7 as *mut u8;

/// RC oscillator Temperature Compensation register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FRCTC | 1 |
/// | DI_MRCBG | 10000 |
/// | MRCTC | 1110 |
pub const RCTCAL: *mut u8 = 0xC8 as *mut u8;

/// Clock management status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ECF | 1 |
pub const CMSR: *mut u8 = 0xC9 as *mut u8;

/// Clock management override control register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FRCACT | 100 |
/// | MRCAO | 10 |
/// | FRCAO | 1 |
pub const CMOCR: *mut u8 = 0xCA as *mut u8;

/// Supply Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AVCCRF | 1 |
/// | AVCCLF | 10 |
pub const SUPFR: *mut u8 = 0xCB as *mut u8;

/// Supply Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DVHEN | 100000 |
/// | VMEMEN | 10000000 |
/// | AVDIC | 1000 |
/// | AVCCLM | 10 |
/// | AVEN | 10000 |
/// | PVEN | 100 |
/// | AVCCRM | 1 |
/// | VMRESM | 1000000 |
pub const SUPCR: *mut u8 = 0xCC as *mut u8;

/// Supply calibration register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PVCAL | 11110000 |
/// | PV22 | 100 |
/// | PVDIC | 1000 |
pub const SUPCA1: *mut u8 = 0xCD as *mut u8;

/// Supply calibration register 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BGCAL | 1111 |
pub const SUPCA2: *mut u8 = 0xCE as *mut u8;

/// Supply calibration register 3.
pub const SUPCA3: *mut u8 = 0xCF as *mut u8;

/// Supply calibration register 4.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ICONST | 111111 |
pub const SUPCA4: *mut u8 = 0xD0 as *mut u8;

/// Calibration ready signature.
pub const CALRDY: *mut u8 = 0xD1 as *mut u8;

/// Data FIFO Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DFOFL | 100 |
/// | DFFLRF | 1 |
/// | DFUFL | 10 |
pub const DFS: *mut u8 = 0xD2 as *mut u8;

/// Data FIFO Fill Level Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DFFLS | 111111 |
/// | DFCLR | 10000000 |
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
/// | DFFLIM | 1 |
/// | DFERIM | 10 |
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
/// | SFUFL | 10 |
/// | SFFLRF | 1 |
pub const SFS: *mut u8 = 0xDB as *mut u8;

/// Support FIFO Fill Level Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SFCLR | 10000000 |
/// | SFFLS | 11111 |
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
/// | SFFLIM | 1 |
/// | SFERIM | 10 |
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
/// | SSMTPE | 1000 |
/// | SSMTAE | 100000 |
/// | SSMPVE | 10000 |
/// | SSMTGE | 100 |
pub const SSMCR: *mut u8 = 0xE2 as *mut u8;

/// General Timer/Counter Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PSR10 | 1 |
/// | TSM | 10000000 |
pub const GTCCR: *mut u8 = 0xE3 as *mut u8;

/// SSM Filter Bandwidth Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SSMPLDT | 100000 |
pub const SSMFBR: *mut u8 = 0xE4 as *mut u8;

/// SSM Run Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SSMR | 1 |
/// | SSMST | 10 |
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

/// VX Mode Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | VX_SEL1 | 10 |
/// | EN_VX | 100 |
/// | VX_SEL0 | 1 |
/// | EN_VX_IN | 10000 |
/// | EN_VX_OUT | 1000 |
pub const VXMCTRL: *mut u8 = 0xEB as *mut u8;

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
/// | MSMSM5 | 11110000 |
/// | MSMSM4 | 1111 |
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

/// SPI2 control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CPHA2 | 100 |
/// | SP2E | 1000000 |
/// | SP2R | 11 |
/// | DORD2 | 100000 |
/// | CPOL2 | 1000 |
/// | SP2IE | 10000000 |
/// | MSTR2 | 10000 |
pub const SP2CR: *mut u8 = 0xF7 as *mut u8;

/// SPI2 Data Register.
pub const SP2DR: *mut u8 = 0xF8 as *mut u8;

/// SPI2 Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SPI22X | 1 |
/// | WCOL2 | 1000000 |
/// | SP2IF | 10000000 |
pub const SP2SR: *mut u8 = 0xF9 as *mut u8;

/// Trace ID Register low byte.
pub const TRCIDL: *mut u8 = 0xFC as *mut u8;

/// Trace ID Register.
pub const TRCID: *mut u16 = 0xFC as *mut u16;

/// Trace ID Register high byte.
pub const TRCIDH: *mut u8 = 0xFD as *mut u8;

/// Trace Data Register.
pub const TRCDR: *mut u8 = 0xFF as *mut u8;

/// Front-End Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PLCK | 1000 |
/// | ANTS | 10000 |
/// | XRDY | 100 |
pub const FESR: *mut u8 = 0x100 as *mut u8;

/// Front-End Enable Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PLEN | 1 |
/// | PLCAL | 10 |
/// | ATEN | 10000000 |
/// | XTOEN | 100 |
/// | PLSP1 | 1000000 |
pub const FEEN1: *mut u8 = 0x101 as *mut u8;

/// Front-End Enable Register 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CPBIA | 1000000 |
/// | PAEN | 100 |
/// | PLPEN | 10000 |
pub const FEEN2: *mut u8 = 0x102 as *mut u8;

/// Reserved.
pub const FELNA: *mut u8 = 0x103 as *mut u8;

/// Front-End Antenna Tuning.
pub const FEAT: *mut u8 = 0x104 as *mut u8;

/// Front-End Power Amplifier Control Register.
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
/// | RTN4 | 11110000 |
/// | CTN4 | 1111 |
pub const FETN4: *mut u8 = 0x109 as *mut u8;

/// Front-End Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ANPS | 100000 |
/// | PLCKG | 10000 |
/// | LBNHB | 1 |
/// | S4N3 | 10 |
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

/// Front-End Antenna.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LVLC | 1111 |
pub const FEANT: *mut u8 = 0x10D as *mut u8;

/// Reserved.
pub const FEBIA: *mut u8 = 0x10E as *mut u8;

/// Clock output divider settings Register.
pub const CLKOD: *mut u8 = 0x115 as *mut u8;

/// Clock output control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CLKOS | 11 |
/// | CLKOEN | 100 |
pub const CLKOCR: *mut u8 = 0x116 as *mut u8;

/// Front-End Test Enable Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | VCOT | 1000000 |
/// | LNLT | 100 |
/// | LNHT | 1000 |
/// | PATE | 10000 |
/// | XTOT | 10 |
/// | AMPT | 100000 |
/// | ADCT | 1 |
pub const FETE1: *mut u8 = 0x11C as *mut u8;

/// Front-End Test Enable Register 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CPT | 1000 |
/// | PRET | 1000000 |
/// | RCCT | 1 |
/// | PPFT | 10 |
/// | DADCT | 100000 |
/// | LFT | 100 |
/// | SWALT | 10000000 |
/// | PFDT | 10000 |
pub const FETE2: *mut u8 = 0x11D as *mut u8;

/// Front-End Test Enable Register 3.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RMPTST | 10 |
/// | BIOUT | 1 |
pub const FETE3: *mut u8 = 0x11E as *mut u8;

/// Front-End Test Data Register.
pub const FETD: *mut u8 = 0x11F as *mut u8;

/// Tx Modulator Finite State Machine.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TMMSM | 1110000 |
/// | TMSSM | 1111 |
pub const TMFSM: *mut u8 = 0x120 as *mut u8;

/// Tx Modulator CRC Result low byte.
pub const TMCRCL: *mut u8 = 0x121 as *mut u8;

/// Tx Modulator CRC Result.
pub const TMCRC: *mut u16 = 0x121 as *mut u16;

/// Tx Modulator CRC Result high byte.
pub const TMCRCH: *mut u8 = 0x122 as *mut u8;

/// Tx Modulator CRC Skip Bit Number.
pub const TMCSB: *mut u8 = 0x123 as *mut u8;

/// Tx Modulator CRC Init Value.
pub const TMCI: *mut u16 = 0x124 as *mut u16;

/// Tx Modulator CRC Init Value low byte.
pub const TMCIL: *mut u8 = 0x124 as *mut u8;

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

/// Tx Modulator Telegram Length Register low byte.
pub const TMTLLL: *mut u8 = 0x129 as *mut u8;

/// Tx Modulator Telegram Length Register.
pub const TMTLL: *mut u16 = 0x129 as *mut u16;

/// Tx Modulator Telegram Length Register high byte.
pub const TMTLLH: *mut u8 = 0x12A as *mut u8;

/// Tx Modulator Stop Sequence Configuration.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TMSSP | 1111 |
/// | TMSSH | 10000000 |
/// | TMSSL | 1110000 |
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
/// | TMLSB | 1000000 |
/// | TMCRCE | 1 |
/// | TMSSE | 100000 |
/// | TMCRCSE | 110 |
/// | TMPOL | 10000 |
/// | TMNRZE | 1000 |
pub const TMCR2: *mut u8 = 0x12D as *mut u8;

/// Tx Modulator Control Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TMPIS | 111 |
/// | TMCIM | 10000 |
/// | TMSCS | 1000 |
pub const TMCR1: *mut u8 = 0x12E as *mut u8;

/// LF Receiver Decoder Setting Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LOTHA | 11 |
/// | HITHA | 1100 |
/// | CTTHA | 110000 |
pub const LFDSR1: *mut u8 = 0x130 as *mut u8;

/// LF Receiver Decoder Setting Register 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CTTHB | 110000 |
/// | LOTHB | 11 |
/// | HITHB | 1100 |
pub const LFDSR2: *mut u8 = 0x131 as *mut u8;

/// LF Receiver Decoder Setting Register 3.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PBDTH | 11 |
/// | QCTH | 111000 |
pub const LFDSR3: *mut u8 = 0x132 as *mut u8;

/// LF Receiver Decoder Setting Register 4.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SCTHA | 111000 |
/// | SRSTC | 11000000 |
/// | SDTHA | 111 |
pub const LFDSR4: *mut u8 = 0x133 as *mut u8;

/// LF Decoder Setting 5 Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SSUTA | 1000000 |
/// | SSUTB | 10000000 |
/// | SDTHB | 111 |
/// | SCTHB | 111000 |
pub const LFDSR5: *mut u8 = 0x134 as *mut u8;

/// LF Decoder Setting 6 Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TODU | 111 |
/// | TODS | 111000 |
pub const LFDSR6: *mut u8 = 0x135 as *mut u8;

/// LF Decoder Setting 7 Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | MDG | 11000000 |
/// | PBG | 1100 |
/// | PBSP | 11 |
/// | MDSP | 110000 |
pub const LFDSR7: *mut u8 = 0x136 as *mut u8;

/// LF Decoder Setting 8 Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CLD | 111 |
/// | LGFE | 1000 |
/// | ASWTH | 1110000 |
pub const LFDSR8: *mut u8 = 0x137 as *mut u8;

/// LF Decoder Setting 9 Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | STW | 11111 |
pub const LFDSR9: *mut u8 = 0x138 as *mut u8;

/// LF Decoder Setting 10 Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FCL | 111111 |
/// | STBTH | 11000000 |
pub const LFDSR10: *mut u8 = 0x139 as *mut u8;

/// Low Frequency Decoder Setting Register 11.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TINITB | 11110000 |
/// | TINITA | 1111 |
pub const LFDSR11: *mut u8 = 0x13A as *mut u8;

/// EEPROM Protection Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EEPS4WD | 1 |
/// | EEPS5WD | 100 |
/// | EEPS6RD | 100000 |
/// | EEPS6WD | 10000 |
/// | EEPS7RD | 10000000 |
/// | EEPS5RD | 1000 |
/// | EEPS4RD | 10 |
/// | EEPS7WD | 1000000 |
pub const EEPR1: *mut u8 = 0x13B as *mut u8;

/// EEPROM Protection Register 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EEPS8RD | 10 |
/// | EEPS8WD | 1 |
/// | EEPS9WD | 100 |
/// | EEPS11WD | 1000000 |
/// | EEPS10RD | 100000 |
/// | EEPS11RD | 10000000 |
/// | EEPS9RD | 1000 |
/// | EEPS10WD | 10000 |
pub const EEPR2: *mut u8 = 0x13C as *mut u8;

/// EEPROM Protection Register 3.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EEPS12RD | 10 |
/// | EEPS12WD | 1 |
pub const EEPR3: *mut u8 = 0x13D as *mut u8;

/// CRC Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CRCRS | 1 |
/// | REFLI | 10 |
/// | REFLO | 100 |
pub const CRCCR: *mut u8 = 0x145 as *mut u8;

/// CRC Data Output Register.
pub const CRCDOR: *mut u8 = 0x146 as *mut u8;

/// LF Receiver SRC Tuning MSB.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LFSRCT1 | 1 |
/// | LFSRCT5 | 10000 |
/// | LFSRCT4 | 1000 |
/// | LFSRCT2 | 10 |
/// | LFSRCT6 | 100000 |
/// | LFSRCT8 | 10000000 |
/// | LFSRCT3 | 100 |
/// | LFSRCT7 | 1000000 |
pub const LFSRCTM: *mut u8 = 0x151 as *mut u8;

/// DeBounce Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DBTMS | 100 |
/// | DBHA | 1000 |
/// | DBCS | 10 |
/// | DBMD | 1 |
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
/// | DBGGS | 1111 |
/// | CPBF | 1000000 |
/// | CPBFOS | 110000 |
/// | ATEST | 10000000 |
pub const DBGSW: *mut u8 = 0x156 as *mut u8;

/// SPI FIFO Fill Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RFL | 111 |
/// | TFL | 1110000 |
/// | TFC | 10000000 |
/// | RFC | 1000 |
pub const SFFR: *mut u8 = 0x157 as *mut u8;

/// SPI FIFO Interrupt Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RIL | 111 |
/// | STIE | 10000000 |
/// | SRIE | 1000 |
/// | TIL | 1110000 |
pub const SFIR: *mut u8 = 0x158 as *mut u8;

/// Timer2 Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T2COF | 10 |
/// | T2OFF | 1 |
pub const T2IFR: *mut u8 = 0x159 as *mut u8;

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

/// LF Receiver SRC Tuning LSB.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LFSRCT0 | 1 |
pub const LFSRCTL: *mut u8 = 0x15C as *mut u8;

/// Pin change Interrupt flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCIF0 | 1 |
/// | PCIF1 | 10 |
pub const PCIFR: *mut u8 = 0x161 as *mut u8;

/// Timer0 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T0PS | 111 |
/// | T0IE | 1000 |
/// | T0PR | 10000 |
pub const T0CR: *mut u8 = 0x162 as *mut u8;

/// DeBounce Enable Port D.
pub const DBEND: *mut u8 = 0x164 as *mut u8;

/// Transponder Control Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TPQPLM | 100 |
/// | TPDFCP | 1100000 |
/// | TPMODE | 10000000 |
/// | TPBR | 10000 |
pub const TPCR1: *mut u8 = 0x165 as *mut u8;

/// Transponder Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TPIM | 1 |
/// | TPFTIM | 10 |
/// | TPNFTIM | 100 |
/// | TPBERIM | 1000 |
pub const TPIMR: *mut u8 = 0x166 as *mut u8;

/// Transponder Decoder Comparator Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TPDCL1 | 111111 |
pub const TPDCR1: *mut u8 = 0x167 as *mut u8;

/// Transponder Decoder Comparator Register 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TPDCL2 | 111111 |
pub const TPDCR2: *mut u8 = 0x168 as *mut u8;

/// Transponder Decoder Comparator Register 3.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TPDCL3 | 111111 |
pub const TPDCR3: *mut u8 = 0x169 as *mut u8;

/// Transponder Decoder Comparator Register 4.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TPDCL4 | 111111 |
pub const TPDCR4: *mut u8 = 0x16A as *mut u8;

/// Transponder Decoder Comparator Register 5.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TPDCL5 | 111111 |
pub const TPDCR5: *mut u8 = 0x16B as *mut u8;

/// Transponder Encoder Comparator Register 1.
pub const TPECR1: *mut u8 = 0x16C as *mut u8;

/// Transponder Encoder Comparator Register 2.
pub const TPECR2: *mut u8 = 0x16D as *mut u8;

/// Transponder Encoder Comparator Register 3.
pub const TPECR3: *mut u8 = 0x16E as *mut u8;

/// Transponder Encoder Comparator Register 4.
pub const TPECR4: *mut u8 = 0x16F as *mut u8;

/// Transponder Encoder Mode Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TPECM3 | 110000 |
/// | TPECM1 | 11 |
/// | TPECM4 | 11000000 |
/// | TPECM2 | 1100 |
pub const TPECMR: *mut u8 = 0x170 as *mut u8;

/// Transponder Control Register 3.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TPRD | 10 |
/// | TPTLIW | 100 |
/// | TPTD | 1 |
/// | TPRCD | 100000 |
pub const TPCR3: *mut u8 = 0x171 as *mut u8;

/// Transponder Control Register 4.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TPBCCS | 1111 |
/// | TPBCM | 10000 |
pub const TPCR4: *mut u8 = 0x172 as *mut u8;

/// Transponder Control Register 5.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TPMUD | 111 |
/// | TPMD | 1110000 |
pub const TPCR5: *mut u8 = 0x173 as *mut u8;

/// Transponder Calibration Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TPBG_IREF | 111111 |
pub const TPCALR1: *mut u8 = 0x175 as *mut u8;

/// Transponder Calibration Register 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TPBG_UREF | 1111111 |
pub const TPCALR2: *mut u8 = 0x176 as *mut u8;

/// Transponder Calibration Register 3.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TPORTH | 11000 |
/// | LFVCC_TPCAL0 | 1 |
/// | LFVCC_TPCAL1 | 10 |
/// | LFVCC_TPCAL2 | 100 |
pub const TPCALR3: *mut u8 = 0x177 as *mut u8;

/// Transponder Calibration Register 4.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | COMPVC_CAL | 11000 |
/// | TPINIT_CAL | 111 |
pub const TPCALR4: *mut u8 = 0x178 as *mut u8;

/// Transponder Calibration Register 5.
pub const TPCALR5: *mut u8 = 0x179 as *mut u8;

/// Transponder Calibration Register 6.
pub const TPCALR6: *mut u8 = 0x17A as *mut u8;

/// Transponder Calibration Register 7.
pub const TPCALR7: *mut u8 = 0x17B as *mut u8;

/// Transponder Calibration Register 8.
pub const TPCALR8: *mut u8 = 0x17C as *mut u8;

/// Transponder Calibration Register 9.
pub const TPCALR9: *mut u8 = 0x17D as *mut u8;

/// Transponder Calibration Register 10.
pub const TPCALR10: *mut u8 = 0x17E as *mut u8;

/// AES Data Pointer Register.
pub const AESDPR: *mut u8 = 0x17F as *mut u8;

/// AES Key Register.
pub const AESKR: *mut u8 = 0x180 as *mut u8;

/// AES Data Register.
pub const AESDR: *mut u8 = 0x181 as *mut u8;

/// General Purpose I/O Register 3.
pub const GPIOR3: *mut u8 = 0x182 as *mut u8;

/// General Purpose I/O Register 4.
pub const GPIOR4: *mut u8 = 0x183 as *mut u8;

/// General Purpose I/O Register 5.
pub const GPIOR5: *mut u8 = 0x184 as *mut u8;

/// General Purpose I/O Register 6.
pub const GPIOR6: *mut u8 = 0x185 as *mut u8;

/// General Purpose I/O Register 7.
pub const GPIOR7: *mut u8 = 0x186 as *mut u8;

/// General Purpose I/O Register 8.
pub const GPIOR8: *mut u8 = 0x187 as *mut u8;

/// Protocol Handler Bit Counter Read Register.
pub const PHBCRR: *mut u8 = 0x188 as *mut u8;

/// LF Receiver Calibration Protect Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LFCPCE | 10000000 |
/// | TPCD | 1000000 |
/// | LFCALRY | 10 |
/// | LFCALP | 1 |
pub const LFCPR: *mut u8 = 0x18E as *mut u8;

/// LF Receiver Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LFDEIM | 10 |
/// | LFSYDIM | 1 |
/// | LFEOIM | 100 |
pub const LFIMR: *mut u8 = 0x18F as *mut u8;

/// PH ID0 Register.
pub const PHID0: *mut u32 = 0x190 as *mut u32;

/// PH Identifier 0 Length Register.
pub const PHID0L: *mut u8 = 0x194 as *mut u8;

/// PH ID1 Register.
pub const PHID1: *mut u32 = 0x195 as *mut u32;

/// PH Identifier 1 Length Register.
pub const PHID1L: *mut u8 = 0x199 as *mut u8;

/// Protocol Handler ID Frame Register.
pub const PHIDFR: *mut u8 = 0x19A as *mut u8;

/// LF Receiver Synchronization Symbols Register.
pub const LFSYSY: *mut u32 = 0x19B as *mut u32;

/// LF Receiver Synchronization Length Register.
pub const LFSYLE: *mut u8 = 0x19F as *mut u8;

/// LF Receiver Stop Bit Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LFSTL | 1110000 |
/// | LFSTSY | 1111 |
pub const LFSTOP: *mut u8 = 0x1A0 as *mut u8;

/// LF Timer Compare Register.
pub const LTCOR: *mut u8 = 0x1A1 as *mut u8;

/// Timer1 Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T1COF | 10 |
/// | T1OFF | 1 |
pub const T1IFR: *mut u8 = 0x1A2 as *mut u8;

/// Protocol Handler Telegram Bit Length Register.
pub const PHTBLR: *mut u8 = 0x1A4 as *mut u8;

/// Protocol Handler Data Frame end Register.
pub const PHDFR: *mut u8 = 0x1A5 as *mut u8;

/// LF Timer Event Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | IDFEM | 100 |
/// | ID1EM | 10 |
/// | LTCOF | 10000000 |
/// | DFEM | 1000 |
/// | FLEM | 100000 |
/// | TBLEM | 10000 |
/// | ID0EM | 1 |
/// | EOFEM | 1000000 |
pub const LTEMR: *mut u8 = 0x1A6 as *mut u8;

/// LF Receiver Channel 3 Quality Faktor Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LFCS3 | 11110000 |
/// | LFQS3 | 1111 |
pub const LFQC3: *mut u8 = 0x1A7 as *mut u8;

/// LF Receiver Channel 2 Quality Faktor Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LFQS2 | 1111 |
/// | LFCS2 | 11110000 |
pub const LFQC2: *mut u8 = 0x1A8 as *mut u8;

/// LF Receiver Channel 1 Quality Faktor Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LFQS1 | 1111 |
/// | LFCS1 | 11110000 |
pub const LFQC1: *mut u8 = 0x1A9 as *mut u8;

/// TWI2 Bit Rate Register.
pub const TW2BR: *mut u8 = 0x1AA as *mut u8;

/// TWI2 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TW2STA | 100000 |
/// | TW2WC | 1000 |
/// | TW2IE | 1 |
/// | TW2EA | 1000000 |
/// | TW2INT | 10000000 |
/// | TW2EN | 100 |
/// | TW2STO | 10000 |
pub const TW2CR: *mut u8 = 0x1AB as *mut u8;

/// TWI2 Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TW2PS | 11 |
/// | TW2S | 11111000 |
pub const TW2SR: *mut u8 = 0x1AC as *mut u8;

/// TWI2 Data Register.
pub const TW2DR: *mut u8 = 0x1AD as *mut u8;

/// TWI2 (Slave) Address Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TW2GCE | 1 |
/// | TW2A | 11111110 |
pub const TW2AR: *mut u8 = 0x1AE as *mut u8;

/// TWI2 Address Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TW2AM | 11111110 |
pub const TW2AMR: *mut u8 = 0x1AF as *mut u8;

/// RSSI Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RSSDEN | 1 |
/// | RSRES | 10000000 |
/// | RSMODE0 | 10000 |
/// | RSOFM | 1000 |
/// | RSOS | 10 |
/// | RSEOR | 100 |
/// | RSMODE1 | 100000 |
pub const RSCR: *mut u8 = 0x1B0 as *mut u8;

/// RSSI Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RSRDY | 1 |
/// | RSSVLD | 10 |
pub const RSSR: *mut u8 = 0x1B1 as *mut u8;

/// RSSI Measurement Setting 1 Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RSCH3E | 100 |
/// | RSSTIM | 10000 |
/// | RSSCAL | 10000000 |
/// | RSINTM | 1000 |
/// | RSCH1E | 1 |
/// | RSCH2E | 10 |
/// | RSSSV | 1000000 |
/// | RSCMS | 100000 |
pub const RSMS1R: *mut u8 = 0x1B2 as *mut u8;

/// RSSI Measurement Setting 2 Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RSAVGS2 | 1000000 |
/// | RSSADR0 | 1 |
/// | RSSADR1 | 10 |
/// | RSAVGS1 | 100000 |
/// | RSSADR2 | 100 |
/// | RSSADR3 | 1000 |
/// | RSAVGS3 | 10000000 |
/// | RSAVGS0 | 10000 |
pub const RSMS2R: *mut u8 = 0x1B3 as *mut u8;

/// RSSI Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RSAOOR1 | 100000 |
/// | RSOOR1 | 1 |
/// | RSAOOR2 | 1000000 |
/// | RSOOR2 | 10 |
/// | RSOFF | 1000 |
/// | RSOOR3 | 100 |
/// | RSAOOR3 | 10000000 |
pub const RSFR: *mut u8 = 0x1B4 as *mut u8;

/// RSSI Calibration Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RSCALIB2 | 100 |
/// | RSCALIB7 | 10000000 |
/// | RSCALIB0 | 1 |
/// | RSCALIB3 | 1000 |
/// | RSCALIB1 | 10 |
/// | RSCALIB4 | 10000 |
/// | RSCALIB6 | 1000000 |
/// | RSCALIB5 | 100000 |
pub const RSCALIB: *mut u8 = 0x1B6 as *mut u8;

/// RSSI Delay Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RSTRD3 | 1000 |
/// | RSTRD2 | 100 |
/// | RSTRD0 | 1 |
/// | RSTRD5 | 100000 |
/// | RSTRD1 | 10 |
/// | RSTRD4 | 10000 |
/// | RSRD0 | 1000000 |
/// | RSRD1 | 10000000 |
pub const RSDLYR: *mut u8 = 0x1B7 as *mut u8;

/// RSSI Result 1 Low Byte Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RSRES1L4 | 10000 |
/// | RSRES1L3 | 1000 |
/// | RSRES1L7 | 10000000 |
/// | RSRES1L0 | 1 |
/// | RSRES1L6 | 1000000 |
/// | RSRES1L2 | 100 |
/// | RSRES1L5 | 100000 |
/// | RSRES1L1 | 10 |
pub const RSRES1L: *mut u8 = 0x1B8 as *mut u8;

/// RSSI Result 1 High Byte Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RSRES1H0 | 1 |
/// | RSRES1H3 | 1000 |
/// | RSRES1H1 | 10 |
/// | RSRES1H2 | 100 |
/// | RSRES1H7 | 10000000 |
/// | RSRES1H6 | 1000000 |
/// | RSRES1H5 | 100000 |
/// | RSRES1H4 | 10000 |
pub const RSRES1H: *mut u8 = 0x1B9 as *mut u8;

/// RSSI Result 2 Low Byte Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RSRES2L0 | 1 |
/// | RSRES2L4 | 10000 |
/// | RSRES2L6 | 1000000 |
/// | RSRES2L7 | 10000000 |
/// | RSRES2L2 | 100 |
/// | RSRES2L5 | 100000 |
/// | RSRES2L3 | 1000 |
/// | RSRES2L1 | 10 |
pub const RSRES2L: *mut u8 = 0x1BA as *mut u8;

/// RSSI Result 2 High Byte Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RSRES2H3 | 1000 |
/// | RSRES2H0 | 1 |
/// | RSRES2H6 | 1000000 |
/// | RSRES2H4 | 10000 |
/// | RSRES2H5 | 100000 |
/// | RSRES2H7 | 10000000 |
/// | RSRES2H2 | 100 |
/// | RSRES2H1 | 10 |
pub const RSRES2H: *mut u8 = 0x1BB as *mut u8;

/// RSSI Result 3 Low Byte Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RSRES3L7 | 10000000 |
/// | RSRES3L3 | 1000 |
/// | RSRES3L6 | 1000000 |
/// | RSRES3L5 | 100000 |
/// | RSRES3L2 | 100 |
/// | RSRES3L4 | 10000 |
/// | RSRES3L0 | 1 |
/// | RSRES3L1 | 10 |
pub const RSRES3L: *mut u8 = 0x1BC as *mut u8;

/// RSSI Result 3 High Byte Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RSRES3H1 | 10 |
/// | RSRES3H3 | 1000 |
/// | RSRES3H7 | 10000000 |
/// | RSRES3H6 | 1000000 |
/// | RSRES3H0 | 1 |
/// | RSRES3H5 | 100000 |
/// | RSRES3H2 | 100 |
/// | RSRES3H4 | 10000 |
pub const RSRES3H: *mut u8 = 0x1BD as *mut u8;

/// RSSI Result 4 Low Byte Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RSRES4L4 | 10000 |
/// | RSRES4L7 | 10000000 |
/// | RSRES4L3 | 1000 |
/// | RSRES4L0 | 1 |
/// | RSRES4L2 | 100 |
/// | RSRES4L5 | 100000 |
/// | RSRES4L6 | 1000000 |
/// | RSRES4L1 | 10 |
pub const RSRES4L: *mut u8 = 0x1BE as *mut u8;

/// RSSI Result 4 High Byte Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RSRES4H1 | 10 |
/// | RSRES4H4 | 10000 |
/// | RSRES4H3 | 1000 |
/// | RSRES4H5 | 100000 |
/// | RSRES4H2 | 100 |
/// | RSRES4H6 | 1000000 |
/// | RSRES4H7 | 10000000 |
/// | RSRES4H0 | 1 |
pub const RSRES4H: *mut u8 = 0x1BF as *mut u8;

/// RSSI SRC Calibration Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SRCMODE0 | 1 |
/// | SRCSTEP1 | 10000000 |
/// | SRCMIN0 | 100 |
/// | SRCSTEP0 | 1000000 |
/// | SRCMODE1 | 10 |
/// | SRCCLR | 10000 |
/// | SRCMIN1 | 1000 |
pub const RSSRCR: *mut u8 = 0x1C0 as *mut u8;

/// Sign Detection Channel 1 vs 2 Result Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SD12RR3 | 1000 |
/// | SD12RR1 | 10 |
/// | SD12RR4 | 10000 |
/// | SD12RR2 | 100 |
/// | SD12RR5 | 100000 |
/// | SD12RR6 | 1000000 |
/// | SD12RR7 | 10000000 |
/// | SD12RR0 | 1 |
pub const SD12RR: *mut u8 = 0x1C1 as *mut u8;

/// Sign Detection Channel 1 vs 3 Result Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SD13RR7 | 10000000 |
/// | SD13RR4 | 10000 |
/// | SD13RR5 | 100000 |
/// | SD13RR1 | 10 |
/// | SD13RR0 | 1 |
/// | SD13RR3 | 1000 |
/// | SD13RR2 | 100 |
/// | SD13RR6 | 1000000 |
pub const SD13RR: *mut u8 = 0x1C2 as *mut u8;

/// Sign Detection Channel 2 vs 3 Result Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SD23RR7 | 10000000 |
/// | SD23RR0 | 1 |
/// | SD23RR1 | 10 |
/// | SD23RR6 | 1000000 |
/// | SD23RR5 | 100000 |
/// | SD23RR3 | 1000 |
/// | SD23RR2 | 100 |
/// | SD23RR4 | 10000 |
pub const SD23RR: *mut u8 = 0x1C3 as *mut u8;

/// Sign Detection 360 Degree Result Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SD360R6 | 1000000 |
/// | SD360R3 | 1000 |
/// | SD360R7 | 10000000 |
/// | SD360R4 | 10000 |
/// | SD360R1 | 10 |
/// | SD360R0 | 1 |
/// | SD360R5 | 100000 |
/// | SD360R2 | 100 |
pub const SD360R: *mut u8 = 0x1C4 as *mut u8;

/// RSSI Debug Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RSDBGS0 | 100000 |
/// | RSSANA | 1 |
/// | RSHOME | 10000 |
/// | RSINFM | 100 |
/// | RSDBGS1 | 1000000 |
/// | RSDBGEN | 10000000 |
/// | RSFPD | 1000 |
pub const RSDBGR: *mut u8 = 0x1C5 as *mut u8;

/// LF Data FIFO Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LDFUF | 10 |
/// | LDFOF | 100 |
/// | LDFFLR | 1 |
pub const LDFS: *mut u8 = 0x1D1 as *mut u8;

/// Timer4 interrupt flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T4ICF | 100 |
/// | T4OFF | 1 |
/// | T4COF | 10 |
pub const T4IFR: *mut u8 = 0x1D2 as *mut u8;

/// LF Data FIFO Write Pointer.
pub const LDFWP: *mut u8 = 0x1D3 as *mut u8;

/// LF Data FIFO Read Pointer.
pub const LDFRP: *mut u8 = 0x1D4 as *mut u8;

/// Timer5 Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T5OFF | 1 |
/// | T5COF | 10 |
pub const T5IFR: *mut u8 = 0x1D5 as *mut u8;

/// LF Data FIFO Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LDFFLIM | 1 |
/// | LDFEIM | 10 |
pub const LDFIM: *mut u8 = 0x1D6 as *mut u8;

/// LF Data FIFO Configuration Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LDFMSB | 1000000 |
/// | LDFFLC | 111111 |
pub const LDFC: *mut u8 = 0x1D7 as *mut u8;

/// Protocol Handler Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PHID1IM | 100000 |
/// | PHID0IM | 10000 |
/// | PHIDFIM | 1000 |
/// | PHDFIM | 100 |
/// | PHTBLIM | 10 |
pub const PHIMR: *mut u8 = 0x1D8 as *mut u8;

/// Protocol Handler CRC Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CRCSE0 | 10000 |
/// | CRCSE1 | 100000 |
/// | CRCFR | 100 |
/// | CRCEN | 10000000 |
pub const PHCRCR: *mut u8 = 0x1D9 as *mut u8;

/// PH CRC Start Value Register low byte.
pub const PHCSTL: *mut u8 = 0x1DA as *mut u8;

/// PH CRC Start Value Register.
pub const PHCST: *mut u16 = 0x1DA as *mut u16;

/// PH CRC Start Value Register high byte.
pub const PHCSTH: *mut u8 = 0x1DB as *mut u8;

/// PH CRC Polynomial Register low byte.
pub const PHCRPL: *mut u8 = 0x1DC as *mut u8;

/// PH CRC Polynomial Register.
pub const PHCRP: *mut u16 = 0x1DC as *mut u16;

/// PH CRC Polynomial Register high byte.
pub const PHCRPH: *mut u8 = 0x1DD as *mut u8;

/// PH CRC Checksum Register.
pub const PHCSR: *mut u16 = 0x1DE as *mut u16;

/// PH CRC Checksum Register low byte.
pub const PHCSRL: *mut u8 = 0x1DE as *mut u8;

/// PH CRC Checksum Register high byte.
pub const PHCSRH: *mut u8 = 0x1DF as *mut u8;

/// CRC Data Input Register.
pub const CRCDIR: *mut u8 = 0x1E0 as *mut u8;

/// Timer3 interrupt flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T3COF | 10 |
/// | T3OFF | 1 |
/// | T3ICF | 100 |
pub const T3IFR: *mut u8 = 0x1E1 as *mut u8;

/// Clock Management Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CMCCE | 10000000 |
/// | CMONEN | 1000000 |
/// | CMM | 111 |
/// | CCS | 1000 |
pub const CMCR: *mut u8 = 0x1E3 as *mut u8;

/// Clock interrupt mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ECIE | 1 |
pub const CMIMR: *mut u8 = 0x1E4 as *mut u8;

/// Clock Prescaler Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CLPCE | 10000000 |
/// | CLTPS | 111000 |
/// | CLKPS | 111 |
pub const CLPR: *mut u8 = 0x1E5 as *mut u8;

/// Voltage Monitor Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | VMPS | 1100000 |
/// | VMLS | 1111 |
/// | VMIM | 10000 |
/// | VMRS | 10000000 |
pub const VMCR: *mut u8 = 0x1E6 as *mut u8;

/// Downbond Test Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ISO_GND | 100 |
/// | BBESD | 1 |
/// | BTEST4 | 10000 |
/// | BTEST5 | 100000 |
/// | AGND_LF | 1000 |
/// | AGND_BB | 10 |
/// | BTEST6 | 1000000 |
pub const DBONDR: *mut u8 = 0x1E7 as *mut u8;

/// Calibration ready signature LFVCC.
pub const CALRDYLF: *mut u8 = 0x1E8 as *mut u8;

/// TWI1 Bit Rate Register.
pub const TW1BR: *mut u8 = 0x1E9 as *mut u8;

/// TWI1 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TW1WC | 1000 |
/// | TW1EN | 100 |
/// | TW1EA | 1000000 |
/// | TW1STA | 100000 |
/// | TW1STO | 10000 |
/// | TW1IE | 1 |
/// | TW1INT | 10000000 |
pub const TW1CR: *mut u8 = 0x1EA as *mut u8;

/// TWI1 Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TW1S | 11111000 |
/// | TW1PS | 11 |
pub const TW1SR: *mut u8 = 0x1EB as *mut u8;

/// TWI1 Data Register.
pub const TW1DR: *mut u8 = 0x1EC as *mut u8;

/// TWI1 (Slave) Address Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TW1A | 11111110 |
/// | TW1GCE | 1 |
pub const TW1AR: *mut u8 = 0x1ED as *mut u8;

/// TWI1 Address Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TW1AM | 11111110 |
pub const TW1AMR: *mut u8 = 0x1EE as *mut u8;

/// Pad Driver Strength Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ATBSEL | 10000000 |
/// | RSSISEL | 1000000 |
/// | PDSC | 11111 |
/// | STBTEST | 100000 |
pub const PDSCR: *mut u8 = 0x1EF as *mut u8;

/// Timer Modulator Output Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TO4PIS | 11000000 |
/// | TO1PIS | 11 |
/// | TO3PIS | 110000 |
/// | TO2PIS | 1100 |
pub const TMOCR: *mut u8 = 0x1F0 as *mut u8;

/// Slow RC oscillator calibration.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SRCCAL6 | 100000 |
/// | SRCCAL8 | 10000000 |
/// | SRCCAL3 | 100 |
/// | SRCCAL1 | 1 |
/// | SRCCAL5 | 10000 |
/// | SRCCAL2 | 10 |
/// | SRCCAL4 | 1000 |
/// | SRCCAL7 | 1000000 |
pub const SRCCAL: *mut u8 = 0x1F1 as *mut u8;

/// SRC oscillator Temperature Compensation register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SRCTC | 111 |
/// | SRCS | 11000 |
/// | DIS_SRC | 1000000 |
/// | HOLD_SRC | 10000000 |
pub const SRCTCAL: *mut u8 = 0x1F2 as *mut u8;

/// Supply calibration register 5.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | IPTAT | 111111 |
pub const SUPCA5: *mut u8 = 0x1F3 as *mut u8;

/// Supply calibration register 6.
pub const SUPCA6: *mut u8 = 0x1F4 as *mut u8;

/// Supply calibration register 7.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | VCCCAL | 111 |
/// | LFVCCBD | 111000 |
pub const SUPCA7: *mut u8 = 0x1F5 as *mut u8;

/// Supply calibration register 8.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | VSWBD | 111 |
/// | DVCCBD | 111000 |
pub const SUPCA8: *mut u8 = 0x1F6 as *mut u8;

/// Supply calibration register 9.
pub const SUPCA9: *mut u8 = 0x1F7 as *mut u8;

/// Supply calibration register 10.
pub const SUPCA10: *mut u8 = 0x1F8 as *mut u8;

/// Transponder Calibration Register 11.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | MTBTR0 | 1 |
/// | ENDVBD | 100 |
/// | ENVSWBD | 10000 |
/// | TPCALR117 | 10000000 |
/// | MTBTR1 | 10 |
/// | TPCALR115 | 100000 |
/// | TPCALR116 | 1000000 |
/// | ENLFBD | 1000 |
pub const TPCALR11: *mut u8 = 0x1F9 as *mut u8;

/// Transponder Calibration Register 12.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TPCALR121 | 10 |
/// | TPCALR123 | 1000 |
/// | TPCALR127 | 10000000 |
/// | TPCALR126 | 1000000 |
/// | TPDMOD | 1 |
/// | TPCALR125 | 100000 |
/// | TPCALR122 | 100 |
/// | TPCALR124 | 10000 |
pub const TPCALR12: *mut u8 = 0x1FA as *mut u8;

/// Transponder Calibration Register 13.
pub const TPCALR13: *mut u8 = 0x1FB as *mut u8;

/// Power Management Test Enable Register.
pub const PMTER: *mut u8 = 0x1FE as *mut u8;

/// Slow RC oscillator calibration LSB.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SRCCAL0 | 1 |
pub const SRCCALL: *mut u8 = 0x1FF as *mut u8;

/// Bitfield on register `AESCR`
pub const AESXOR: u8 = 0x10;

/// Bitfield on register `AESCR`
pub const AESD: u8 = 0x8;

/// Bitfield on register `AESCR`
pub const AESRES: u8 = 0x20;

/// Bitfield on register `AESCR`
pub const AESIM: u8 = 0x4;

/// Bitfield on register `AESCR`
pub const AESLKM: u8 = 0x40;

/// Bitfield on register `AESCR`
pub const AESWK: u8 = 0x1;

/// Bitfield on register `AESCR`
pub const AESE: u8 = 0x80;

/// Bitfield on register `AESCR`
pub const AESWD: u8 = 0x2;

/// Bitfield on register `AESSR`
pub const AESERF: u8 = 0x80;

/// Bitfield on register `AESSR`
pub const AESRF: u8 = 0x1;

/// Bitfield on register `BBTE2`
pub const DITDIS: u8 = 0x2;

/// Bitfield on register `BBTE2`
pub const TDEPO: u8 = 0x1;

/// Bitfield on register `CLKOCR`
pub const CLKOS: u8 = 0x3;

/// Bitfield on register `CLKOCR`
pub const CLKOEN: u8 = 0x4;

/// Bitfield on register `CLPR`
pub const CLPCE: u8 = 0x80;

/// Bitfield on register `CLPR`
pub const CLTPS: u8 = 0x38;

/// Bitfield on register `CLPR`
pub const CLKPS: u8 = 0x7;

/// Bitfield on register `CMCR`
pub const CMCCE: u8 = 0x80;

/// Bitfield on register `CMCR`
pub const CMONEN: u8 = 0x40;

/// Bitfield on register `CMCR`
pub const CMM: u8 = 0x7;

/// Bitfield on register `CMCR`
pub const CCS: u8 = 0x8;

/// Bitfield on register `CMIMR`
pub const ECIE: u8 = 0x1;

/// Bitfield on register `CMOCR`
pub const FRCACT: u8 = 0x4;

/// Bitfield on register `CMOCR`
pub const MRCAO: u8 = 0x2;

/// Bitfield on register `CMOCR`
pub const FRCAO: u8 = 0x1;

/// Bitfield on register `CMSR`
pub const ECF: u8 = 0x1;

/// Bitfield on register `CRCCR`
pub const CRCRS: u8 = 0x1;

/// Bitfield on register `CRCCR`
pub const REFLI: u8 = 0x2;

/// Bitfield on register `CRCCR`
pub const REFLO: u8 = 0x4;

/// Bitfield on register `DBCR`
pub const DBTMS: u8 = 0x4;

/// Bitfield on register `DBCR`
pub const DBHA: u8 = 0x8;

/// Bitfield on register `DBCR`
pub const DBCS: u8 = 0x2;

/// Bitfield on register `DBCR`
pub const DBMD: u8 = 0x1;

/// Bitfield on register `DBGSW`
pub const DBGGS: u8 = 0xF;

/// Bitfield on register `DBGSW`
pub const CPBF: u8 = 0x40;

/// Bitfield on register `DBGSW`
pub const CPBFOS: u8 = 0x30;

/// Bitfield on register `DBGSW`
pub const ATEST: u8 = 0x80;

/// Bitfield on register `DBONDR`
pub const ISO_GND: u8 = 0x4;

/// Bitfield on register `DBONDR`
pub const BBESD: u8 = 0x1;

/// Bitfield on register `DBONDR`
pub const BTEST4: u8 = 0x10;

/// Bitfield on register `DBONDR`
pub const BTEST5: u8 = 0x20;

/// Bitfield on register `DBONDR`
pub const AGND_LF: u8 = 0x8;

/// Bitfield on register `DBONDR`
pub const AGND_BB: u8 = 0x2;

/// Bitfield on register `DBONDR`
pub const BTEST6: u8 = 0x40;

/// Bitfield on register `DFC`
pub const DFFLC: u8 = 0x3F;

/// Bitfield on register `DFC`
pub const DFDRA: u8 = 0x80;

/// Bitfield on register `DFI`
pub const DFFLIM: u8 = 0x1;

/// Bitfield on register `DFI`
pub const DFERIM: u8 = 0x2;

/// Bitfield on register `DFL`
pub const DFFLS: u8 = 0x3F;

/// Bitfield on register `DFL`
pub const DFCLR: u8 = 0x80;

/// Bitfield on register `DFS`
pub const DFOFL: u8 = 0x4;

/// Bitfield on register `DFS`
pub const DFFLRF: u8 = 0x1;

/// Bitfield on register `DFS`
pub const DFUFL: u8 = 0x2;

/// Bitfield on register `EECR`
pub const NVMBSY: u8 = 0x80;

/// Bitfield on register `EECR`
pub const EERIE: u8 = 0x8;

/// Bitfield on register `EECR`
pub const EEWE: u8 = 0x2;

/// Bitfield on register `EECR`
pub const EERE: u8 = 0x1;

/// Bitfield on register `EECR`
pub const EEPAGE: u8 = 0x40;

/// Bitfield on register `EECR`
pub const EEMWE: u8 = 0x4;

/// Bitfield on register `EECR`
pub const EEPM: u8 = 0x30;

/// Bitfield on register `EECR2`
pub const E2CIM: u8 = 0x2;

/// Bitfield on register `EECR2`
pub const E2CF: u8 = 0x80;

/// Bitfield on register `EECR2`
pub const E2FF: u8 = 0x40;

/// Bitfield on register `EECR2`
pub const EEBRE: u8 = 0x1;

/// Bitfield on register `EECR2`
pub const E2AVF: u8 = 0x20;

/// Bitfield on register `EEPR`
pub const EEAP: u8 = 0xF;

/// Bitfield on register `EEPR1`
pub const EEPS4WD: u8 = 0x1;

/// Bitfield on register `EEPR1`
pub const EEPS5WD: u8 = 0x4;

/// Bitfield on register `EEPR1`
pub const EEPS6RD: u8 = 0x20;

/// Bitfield on register `EEPR1`
pub const EEPS6WD: u8 = 0x10;

/// Bitfield on register `EEPR1`
pub const EEPS7RD: u8 = 0x80;

/// Bitfield on register `EEPR1`
pub const EEPS5RD: u8 = 0x8;

/// Bitfield on register `EEPR1`
pub const EEPS4RD: u8 = 0x2;

/// Bitfield on register `EEPR1`
pub const EEPS7WD: u8 = 0x40;

/// Bitfield on register `EEPR2`
pub const EEPS8RD: u8 = 0x2;

/// Bitfield on register `EEPR2`
pub const EEPS8WD: u8 = 0x1;

/// Bitfield on register `EEPR2`
pub const EEPS9WD: u8 = 0x4;

/// Bitfield on register `EEPR2`
pub const EEPS11WD: u8 = 0x40;

/// Bitfield on register `EEPR2`
pub const EEPS10RD: u8 = 0x20;

/// Bitfield on register `EEPR2`
pub const EEPS11RD: u8 = 0x80;

/// Bitfield on register `EEPR2`
pub const EEPS9RD: u8 = 0x8;

/// Bitfield on register `EEPR2`
pub const EEPS10WD: u8 = 0x10;

/// Bitfield on register `EEPR3`
pub const EEPS12RD: u8 = 0x2;

/// Bitfield on register `EEPR3`
pub const EEPS12WD: u8 = 0x1;

/// Bitfield on register `EEST`
pub const EESYN: u8 = 0xF;

/// Bitfield on register `EICRA`
pub const ISC0: u8 = 0x3;

/// Bitfield on register `EICRA`
pub const ISC1: u8 = 0xC;

/// Bitfield on register `EIFR`
pub const INTF0: u8 = 0x1;

/// Bitfield on register `EIFR`
pub const INTF1: u8 = 0x2;

/// Bitfield on register `EIMSK`
pub const INT0: u8 = 0x1;

/// Bitfield on register `EIMSK`
pub const INT1: u8 = 0x2;

/// Bitfield on register `FEALR`
pub const RNGE: u8 = 0x3;

/// Bitfield on register `FEANT`
pub const LVLC: u8 = 0xF;

/// Bitfield on register `FEBT`
pub const RTN2: u8 = 0xC;

/// Bitfield on register `FEBT`
pub const CTN2: u8 = 0x3;

/// Bitfield on register `FECR`
pub const ANPS: u8 = 0x20;

/// Bitfield on register `FECR`
pub const PLCKG: u8 = 0x10;

/// Bitfield on register `FECR`
pub const LBNHB: u8 = 0x1;

/// Bitfield on register `FECR`
pub const S4N3: u8 = 0x2;

/// Bitfield on register `FEEN1`
pub const PLEN: u8 = 0x1;

/// Bitfield on register `FEEN1`
pub const PLCAL: u8 = 0x2;

/// Bitfield on register `FEEN1`
pub const ATEN: u8 = 0x80;

/// Bitfield on register `FEEN1`
pub const XTOEN: u8 = 0x4;

/// Bitfield on register `FEEN1`
pub const PLSP1: u8 = 0x40;

/// Bitfield on register `FEEN2`
pub const CPBIA: u8 = 0x40;

/// Bitfield on register `FEEN2`
pub const PAEN: u8 = 0x4;

/// Bitfield on register `FEEN2`
pub const PLPEN: u8 = 0x10;

/// Bitfield on register `FEMS`
pub const PLLM: u8 = 0xF0;

/// Bitfield on register `FEMS`
pub const PLLS: u8 = 0xF;

/// Bitfield on register `FESR`
pub const PLCK: u8 = 0x8;

/// Bitfield on register `FESR`
pub const ANTS: u8 = 0x10;

/// Bitfield on register `FESR`
pub const XRDY: u8 = 0x4;

/// Bitfield on register `FETE1`
pub const VCOT: u8 = 0x40;

/// Bitfield on register `FETE1`
pub const LNLT: u8 = 0x4;

/// Bitfield on register `FETE1`
pub const LNHT: u8 = 0x8;

/// Bitfield on register `FETE1`
pub const PATE: u8 = 0x10;

/// Bitfield on register `FETE1`
pub const XTOT: u8 = 0x2;

/// Bitfield on register `FETE1`
pub const AMPT: u8 = 0x20;

/// Bitfield on register `FETE1`
pub const ADCT: u8 = 0x1;

/// Bitfield on register `FETE2`
pub const CPT: u8 = 0x8;

/// Bitfield on register `FETE2`
pub const PRET: u8 = 0x40;

/// Bitfield on register `FETE2`
pub const RCCT: u8 = 0x1;

/// Bitfield on register `FETE2`
pub const PPFT: u8 = 0x2;

/// Bitfield on register `FETE2`
pub const DADCT: u8 = 0x20;

/// Bitfield on register `FETE2`
pub const LFT: u8 = 0x4;

/// Bitfield on register `FETE2`
pub const SWALT: u8 = 0x80;

/// Bitfield on register `FETE2`
pub const PFDT: u8 = 0x10;

/// Bitfield on register `FETE3`
pub const RMPTST: u8 = 0x2;

/// Bitfield on register `FETE3`
pub const BIOUT: u8 = 0x1;

/// Bitfield on register `FETN4`
pub const RTN4: u8 = 0xF0;

/// Bitfield on register `FETN4`
pub const CTN4: u8 = 0xF;

/// Bitfield on register `FEVCO`
pub const CPCC: u8 = 0xF;

/// Bitfield on register `FEVCO`
pub const VCOB: u8 = 0xF0;

/// Bitfield on register `FSCR`
pub const PAON: u8 = 0x80;

/// Bitfield on register `FSCR`
pub const TXMS: u8 = 0xC;

/// Bitfield on register `FSCR`
pub const TXMOD: u8 = 0x1;

/// Bitfield on register `FSCR`
pub const PAOER: u8 = 0x10;

/// Bitfield on register `FSCR`
pub const SFM: u8 = 0x2;

/// Bitfield on register `FSEN`
pub const PEEN: u8 = 0x8;

/// Bitfield on register `FSEN`
pub const ANTT: u8 = 0x20;

/// Bitfield on register `FSEN`
pub const ASEN: u8 = 0x10;

/// Bitfield on register `FSEN`
pub const SDEN: u8 = 0x2;

/// Bitfield on register `FSEN`
pub const GAEN: u8 = 0x4;

/// Bitfield on register `FSEN`
pub const SDPU: u8 = 0x1;

/// Bitfield on register `FSFCR`
pub const BTSEL: u8 = 0x3;

/// Bitfield on register `FSFCR`
pub const ASDIV: u8 = 0xF0;

/// Bitfield on register `GTCCR`
pub const PSR10: u8 = 0x1;

/// Bitfield on register `GTCCR`
pub const TSM: u8 = 0x80;

/// Bitfield on register `LDFC`
pub const LDFMSB: u8 = 0x40;

/// Bitfield on register `LDFC`
pub const LDFFLC: u8 = 0x3F;

/// Bitfield on register `LDFCKSW`
pub const LDFSCKS: u8 = 0x2;

/// Bitfield on register `LDFCKSW`
pub const LDFSCSW: u8 = 0x1;

/// Bitfield on register `LDFFL`
pub const LDFCLR: u8 = 0x80;

/// Bitfield on register `LDFIM`
pub const LDFFLIM: u8 = 0x1;

/// Bitfield on register `LDFIM`
pub const LDFEIM: u8 = 0x2;

/// Bitfield on register `LDFS`
pub const LDFUF: u8 = 0x2;

/// Bitfield on register `LDFS`
pub const LDFOF: u8 = 0x4;

/// Bitfield on register `LDFS`
pub const LDFFLR: u8 = 0x1;

/// Bitfield on register `LFCALR1`
pub const ICOMPRT: u8 = 0x18;

/// Bitfield on register `LFCALR1`
pub const SEL150M: u8 = 0xE0;

/// Bitfield on register `LFCALR1`
pub const LFSTC: u8 = 0x7;

/// Bitfield on register `LFCALR2`
pub const TIKOMPD: u8 = 0x80;

/// Bitfield on register `LFCALR2`
pub const LFSRM: u8 = 0x40;

/// Bitfield on register `LFCALR2`
pub const LFSTRES: u8 = 0x3F;

/// Bitfield on register `LFCALR4`
pub const TCGAIN27: u8 = 0x80;

/// Bitfield on register `LFCALR4`
pub const TCGAIN20: u8 = 0x1;

/// Bitfield on register `LFCALR4`
pub const TCGAIN23: u8 = 0x8;

/// Bitfield on register `LFCALR4`
pub const TCGAIN21: u8 = 0x2;

/// Bitfield on register `LFCALR4`
pub const TCGAIN24: u8 = 0x10;

/// Bitfield on register `LFCALR4`
pub const TCGAIN26: u8 = 0x40;

/// Bitfield on register `LFCALR4`
pub const TCGAIN22: u8 = 0x4;

/// Bitfield on register `LFCALR4`
pub const TCGAIN25: u8 = 0x20;

/// Bitfield on register `LFCALR5`
pub const TCGAIN37: u8 = 0x80;

/// Bitfield on register `LFCALR5`
pub const TCGAIN31: u8 = 0x2;

/// Bitfield on register `LFCALR5`
pub const TCGAIN32: u8 = 0x4;

/// Bitfield on register `LFCALR5`
pub const TCGAIN36: u8 = 0x40;

/// Bitfield on register `LFCALR5`
pub const TCGAIN35: u8 = 0x20;

/// Bitfield on register `LFCALR5`
pub const TCGAIN30: u8 = 0x1;

/// Bitfield on register `LFCALR5`
pub const TCGAIN34: u8 = 0x10;

/// Bitfield on register `LFCALR6`
pub const TCGAIN42: u8 = 0x4;

/// Bitfield on register `LFCALR6`
pub const TCGAIN43: u8 = 0x8;

/// Bitfield on register `LFCALR6`
pub const TCGAIN41: u8 = 0x2;

/// Bitfield on register `LFCALR6`
pub const TCGAIN40: u8 = 0x1;

/// Bitfield on register `LFCALR6`
pub const TCGAIN44: u8 = 0x10;

/// Bitfield on register `LFCPR`
pub const LFCPCE: u8 = 0x80;

/// Bitfield on register `LFCPR`
pub const TPCD: u8 = 0x40;

/// Bitfield on register `LFCPR`
pub const LFCALRY: u8 = 0x2;

/// Bitfield on register `LFCPR`
pub const LFCALP: u8 = 0x1;

/// Bitfield on register `LFCR0`
pub const LFMG: u8 = 0x20;

/// Bitfield on register `LFCR0`
pub const LFCE1: u8 = 0x1;

/// Bitfield on register `LFCR0`
pub const LFRRT: u8 = 0xC0;

/// Bitfield on register `LFCR0`
pub const LFCE2: u8 = 0x2;

/// Bitfield on register `LFCR0`
pub const LFBR: u8 = 0x18;

/// Bitfield on register `LFCR0`
pub const LFCE3: u8 = 0x4;

/// Bitfield on register `LFCR1`
pub const FLLEN: u8 = 0x10;

/// Bitfield on register `LFCR1`
pub const ADTHEN: u8 = 0x20;

/// Bitfield on register `LFCR1`
pub const LFPEEN: u8 = 0x40;

/// Bitfield on register `LFCR1`
pub const RSST: u8 = 0x3;

/// Bitfield on register `LFCR1`
pub const LFFM1: u8 = 0x4;

/// Bitfield on register `LFCR1`
pub const ARMDE: u8 = 0x8;

/// Bitfield on register `LFCR1`
pub const LFRE: u8 = 0x80;

/// Bitfield on register `LFCR2`
pub const LFDAMP: u8 = 0x4;

/// Bitfield on register `LFCR2`
pub const LFSEN: u8 = 0x3;

/// Bitfield on register `LFCR2`
pub const LFVC: u8 = 0xE0;

/// Bitfield on register `LFCR3`
pub const LFTON: u8 = 0x8;

/// Bitfield on register `LFCR3`
pub const LFSBEN: u8 = 0x80;

/// Bitfield on register `LFCR3`
pub const LFRCPM: u8 = 0x4;

/// Bitfield on register `LFCR3`
pub const LFRCTEN: u8 = 0x1;

/// Bitfield on register `LFCR3`
pub const LFRCPCEN: u8 = 0x2;

/// Bitfield on register `LFCR3`
pub const LFTS: u8 = 0x70;

/// Bitfield on register `LFDSR1`
pub const LOTHA: u8 = 0x3;

/// Bitfield on register `LFDSR1`
pub const HITHA: u8 = 0xC;

/// Bitfield on register `LFDSR1`
pub const CTTHA: u8 = 0x30;

/// Bitfield on register `LFDSR10`
pub const FCL: u8 = 0x3F;

/// Bitfield on register `LFDSR10`
pub const STBTH: u8 = 0xC0;

/// Bitfield on register `LFDSR11`
pub const TINITB: u8 = 0xF0;

/// Bitfield on register `LFDSR11`
pub const TINITA: u8 = 0xF;

/// Bitfield on register `LFDSR2`
pub const CTTHB: u8 = 0x30;

/// Bitfield on register `LFDSR2`
pub const LOTHB: u8 = 0x3;

/// Bitfield on register `LFDSR2`
pub const HITHB: u8 = 0xC;

/// Bitfield on register `LFDSR3`
pub const PBDTH: u8 = 0x3;

/// Bitfield on register `LFDSR3`
pub const QCTH: u8 = 0x38;

/// Bitfield on register `LFDSR4`
pub const SCTHA: u8 = 0x38;

/// Bitfield on register `LFDSR4`
pub const SRSTC: u8 = 0xC0;

/// Bitfield on register `LFDSR4`
pub const SDTHA: u8 = 0x7;

/// Bitfield on register `LFDSR5`
pub const SSUTA: u8 = 0x40;

/// Bitfield on register `LFDSR5`
pub const SSUTB: u8 = 0x80;

/// Bitfield on register `LFDSR5`
pub const SDTHB: u8 = 0x7;

/// Bitfield on register `LFDSR5`
pub const SCTHB: u8 = 0x38;

/// Bitfield on register `LFDSR6`
pub const TODU: u8 = 0x7;

/// Bitfield on register `LFDSR6`
pub const TODS: u8 = 0x38;

/// Bitfield on register `LFDSR7`
pub const MDG: u8 = 0xC0;

/// Bitfield on register `LFDSR7`
pub const PBG: u8 = 0xC;

/// Bitfield on register `LFDSR7`
pub const PBSP: u8 = 0x3;

/// Bitfield on register `LFDSR7`
pub const MDSP: u8 = 0x30;

/// Bitfield on register `LFDSR8`
pub const CLD: u8 = 0x7;

/// Bitfield on register `LFDSR8`
pub const LGFE: u8 = 0x8;

/// Bitfield on register `LFDSR8`
pub const ASWTH: u8 = 0x70;

/// Bitfield on register `LFDSR9`
pub const STW: u8 = 0x1F;

/// Bitfield on register `LFFR`
pub const LFES: u8 = 0x80;

/// Bitfield on register `LFFR`
pub const LFTOF: u8 = 0x8;

/// Bitfield on register `LFFR`
pub const LFDEF: u8 = 0x2;

/// Bitfield on register `LFFR`
pub const LFEOF: u8 = 0x4;

/// Bitfield on register `LFFR`
pub const LFSYDF: u8 = 0x1;

/// Bitfield on register `LFFR`
pub const LFSD: u8 = 0x40;

/// Bitfield on register `LFIMR`
pub const LFDEIM: u8 = 0x2;

/// Bitfield on register `LFIMR`
pub const LFSYDIM: u8 = 0x1;

/// Bitfield on register `LFIMR`
pub const LFEOIM: u8 = 0x4;

/// Bitfield on register `LFQC1`
pub const LFQS1: u8 = 0xF;

/// Bitfield on register `LFQC1`
pub const LFCS1: u8 = 0xF0;

/// Bitfield on register `LFQC2`
pub const LFQS2: u8 = 0xF;

/// Bitfield on register `LFQC2`
pub const LFCS2: u8 = 0xF0;

/// Bitfield on register `LFQC3`
pub const LFCS3: u8 = 0xF0;

/// Bitfield on register `LFQC3`
pub const LFQS3: u8 = 0xF;

/// Bitfield on register `LFSRCTL`
pub const LFSRCT0: u8 = 0x1;

/// Bitfield on register `LFSRCTM`
pub const LFSRCT1: u8 = 0x1;

/// Bitfield on register `LFSRCTM`
pub const LFSRCT5: u8 = 0x10;

/// Bitfield on register `LFSRCTM`
pub const LFSRCT4: u8 = 0x8;

/// Bitfield on register `LFSRCTM`
pub const LFSRCT2: u8 = 0x2;

/// Bitfield on register `LFSRCTM`
pub const LFSRCT6: u8 = 0x20;

/// Bitfield on register `LFSRCTM`
pub const LFSRCT8: u8 = 0x80;

/// Bitfield on register `LFSRCTM`
pub const LFSRCT3: u8 = 0x4;

/// Bitfield on register `LFSRCTM`
pub const LFSRCT7: u8 = 0x40;

/// Bitfield on register `LFSTOP`
pub const LFSTL: u8 = 0x70;

/// Bitfield on register `LFSTOP`
pub const LFSTSY: u8 = 0xF;

/// Bitfield on register `LOCKBIT`
pub const LB: u8 = 0x3;

/// Bitfield on register `LOW`
pub const EESAVE: u8 = 0x8;

/// Bitfield on register `LOW`
pub const WDTON: u8 = 0x10;

/// Bitfield on register `LOW`
pub const PCEE1: u8 = 0x1;

/// Bitfield on register `LOW`
pub const EEACC: u8 = 0x2;

/// Bitfield on register `LOW`
pub const BOOTRST: u8 = 0x4;

/// Bitfield on register `LOW`
pub const SPIEN: u8 = 0x20;

/// Bitfield on register `LOW`
pub const CKDIV8: u8 = 0x80;

/// Bitfield on register `LOW`
pub const DWEN: u8 = 0x40;

/// Bitfield on register `LTCMR`
pub const LTPS0: u8 = 0x1;

/// Bitfield on register `LTCMR`
pub const LTCM: u8 = 0x20;

/// Bitfield on register `LTCMR`
pub const LTPS2: u8 = 0x4;

/// Bitfield on register `LTCMR`
pub const LTSM: u8 = 0x40;

/// Bitfield on register `LTCMR`
pub const LTENA: u8 = 0x80;

/// Bitfield on register `LTCMR`
pub const LTCIM: u8 = 0x10;

/// Bitfield on register `LTCMR`
pub const LTPS1: u8 = 0x2;

/// Bitfield on register `LTCMR`
pub const LTCRM: u8 = 0x8;

/// Bitfield on register `LTEMR`
pub const IDFEM: u8 = 0x4;

/// Bitfield on register `LTEMR`
pub const ID1EM: u8 = 0x2;

/// Bitfield on register `LTEMR`
pub const LTCOF: u8 = 0x80;

/// Bitfield on register `LTEMR`
pub const DFEM: u8 = 0x8;

/// Bitfield on register `LTEMR`
pub const FLEM: u8 = 0x20;

/// Bitfield on register `LTEMR`
pub const TBLEM: u8 = 0x10;

/// Bitfield on register `LTEMR`
pub const ID0EM: u8 = 0x1;

/// Bitfield on register `LTEMR`
pub const EOFEM: u8 = 0x40;

/// Bitfield on register `MCUCR`
pub const PUD: u8 = 0x10;

/// Bitfield on register `MCUCR`
pub const IVSEL: u8 = 0x80;

/// Bitfield on register `MCUCR`
pub const SPIIO: u8 = 0x4;

/// Bitfield on register `MCUCR`
pub const ENPS: u8 = 0x8;

/// Bitfield on register `MCUCR`
pub const TRCCE: u8 = 0x20;

/// Bitfield on register `MCUCR`
pub const TRCEN: u8 = 0x40;

/// Bitfield on register `MCUCR`
pub const IVL: u8 = 0x3;

/// Bitfield on register `MCUSR`
pub const DWRF: u8 = 0x10;

/// Bitfield on register `MCUSR`
pub const WDRF: u8 = 0x8;

/// Bitfield on register `MCUSR`
pub const EXTRF: u8 = 0x2;

/// Bitfield on register `MCUSR`
pub const PORF: u8 = 0x1;

/// Bitfield on register `MCUSR`
pub const TPRF: u8 = 0x20;

/// Bitfield on register `MSMCR1`
pub const MSMSM0: u8 = 0xF;

/// Bitfield on register `MSMCR1`
pub const MSMSM1: u8 = 0xF0;

/// Bitfield on register `MSMCR2`
pub const MSMSM3: u8 = 0xF0;

/// Bitfield on register `MSMCR2`
pub const MSMSM2: u8 = 0xF;

/// Bitfield on register `MSMCR3`
pub const MSMSM5: u8 = 0xF0;

/// Bitfield on register `MSMCR3`
pub const MSMSM4: u8 = 0xF;

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
pub const PCIF0: u8 = 0x1;

/// Bitfield on register `PCIFR`
pub const PCIF1: u8 = 0x2;

/// Bitfield on register `PCMSK0`
pub const PCINT2: u8 = 0x4;

/// Bitfield on register `PCMSK0`
pub const PCINT7: u8 = 0x80;

/// Bitfield on register `PCMSK0`
pub const PCINT6: u8 = 0x40;

/// Bitfield on register `PCMSK0`
pub const PCINT5: u8 = 0x20;

/// Bitfield on register `PCMSK0`
pub const PCINT4: u8 = 0x10;

/// Bitfield on register `PCMSK0`
pub const PCINT1: u8 = 0x2;

/// Bitfield on register `PCMSK0`
pub const PCINT0: u8 = 0x1;

/// Bitfield on register `PCMSK0`
pub const PCINT3: u8 = 0x8;

/// Bitfield on register `PCMSK1`
pub const PCINT13: u8 = 0x20;

/// Bitfield on register `PCMSK1`
pub const PCINT15: u8 = 0x80;

/// Bitfield on register `PCMSK1`
pub const PCINT14: u8 = 0x40;

/// Bitfield on register `PCMSK1`
pub const PCINT12: u8 = 0x10;

/// Bitfield on register `PCMSK1`
pub const PCINT8: u8 = 0x1;

/// Bitfield on register `PCMSK1`
pub const PCINT11: u8 = 0x8;

/// Bitfield on register `PCMSK1`
pub const PCINT9: u8 = 0x2;

/// Bitfield on register `PCMSK1`
pub const PCINT10: u8 = 0x4;

/// Bitfield on register `PDSCR`
pub const ATBSEL: u8 = 0x80;

/// Bitfield on register `PDSCR`
pub const RSSISEL: u8 = 0x40;

/// Bitfield on register `PDSCR`
pub const PDSC: u8 = 0x1F;

/// Bitfield on register `PDSCR`
pub const STBTEST: u8 = 0x20;

/// Bitfield on register `PGMST`
pub const PGMSYN: u8 = 0x1F;

/// Bitfield on register `PHCRCR`
pub const CRCSE0: u8 = 0x10;

/// Bitfield on register `PHCRCR`
pub const CRCSE1: u8 = 0x20;

/// Bitfield on register `PHCRCR`
pub const CRCFR: u8 = 0x4;

/// Bitfield on register `PHCRCR`
pub const CRCEN: u8 = 0x80;

/// Bitfield on register `PHFR`
pub const CRCEF: u8 = 0x1;

/// Bitfield on register `PHFR`
pub const PHID1F: u8 = 0x20;

/// Bitfield on register `PHFR`
pub const PHID0F: u8 = 0x10;

/// Bitfield on register `PHFR`
pub const PHDFF: u8 = 0x4;

/// Bitfield on register `PHFR`
pub const PHIDFF: u8 = 0x8;

/// Bitfield on register `PHFR`
pub const PHTBLF: u8 = 0x2;

/// Bitfield on register `PHIMR`
pub const PHID1IM: u8 = 0x20;

/// Bitfield on register `PHIMR`
pub const PHID0IM: u8 = 0x10;

/// Bitfield on register `PHIMR`
pub const PHIDFIM: u8 = 0x8;

/// Bitfield on register `PHIMR`
pub const PHDFIM: u8 = 0x4;

/// Bitfield on register `PHIMR`
pub const PHTBLIM: u8 = 0x2;

/// Bitfield on register `PHTCR`
pub const FRFIFO: u8 = 0x20;

/// Bitfield on register `PHTCR`
pub const CSM: u8 = 0x80;

/// Bitfield on register `PHTCR`
pub const CPM: u8 = 0x40;

/// Bitfield on register `PRR0`
pub const PRTXDC: u8 = 0x4;

/// Bitfield on register `PRR0`
pub const PRLFRS: u8 = 0x2;

/// Bitfield on register `PRR0`
pub const PRSPI: u8 = 0x1;

/// Bitfield on register `PRR0`
pub const PRTWI1: u8 = 0x80;

/// Bitfield on register `PRR0`
pub const PRVM: u8 = 0x10;

/// Bitfield on register `PRR0`
pub const PRCU: u8 = 0x40;

/// Bitfield on register `PRR0`
pub const PRCO: u8 = 0x20;

/// Bitfield on register `PRR0`
pub const PRCRC: u8 = 0x8;

/// Bitfield on register `PRR1`
pub const PRT2: u8 = 0x2;

/// Bitfield on register `PRR1`
pub const PRT3: u8 = 0x4;

/// Bitfield on register `PRR1`
pub const PRT5: u8 = 0x10;

/// Bitfield on register `PRR1`
pub const PRT1: u8 = 0x1;

/// Bitfield on register `PRR1`
pub const PRLFR: u8 = 0x20;

/// Bitfield on register `PRR1`
pub const PRLFPH: u8 = 0x80;

/// Bitfield on register `PRR1`
pub const PRLFTP: u8 = 0x40;

/// Bitfield on register `PRR1`
pub const PRT4: u8 = 0x8;

/// Bitfield on register `PRR2`
pub const PRSPI2: u8 = 0x1;

/// Bitfield on register `PRR2`
pub const PRSF: u8 = 0x4;

/// Bitfield on register `PRR2`
pub const PRDF: u8 = 0x8;

/// Bitfield on register `PRR2`
pub const PRTWI2: u8 = 0x2;

/// Bitfield on register `PRR2`
pub const PRTM: u8 = 0x40;

/// Bitfield on register `PRR2`
pub const PRSSM: u8 = 0x80;

/// Bitfield on register `RCTCAL`
pub const FRCTC: u8 = 0x1;

/// Bitfield on register `RCTCAL`
pub const DI_MRCBG: u8 = 0x10;

/// Bitfield on register `RCTCAL`
pub const MRCTC: u8 = 0xE;

/// Bitfield on register `RSCALIB`
pub const RSCALIB2: u8 = 0x4;

/// Bitfield on register `RSCALIB`
pub const RSCALIB7: u8 = 0x80;

/// Bitfield on register `RSCALIB`
pub const RSCALIB0: u8 = 0x1;

/// Bitfield on register `RSCALIB`
pub const RSCALIB3: u8 = 0x8;

/// Bitfield on register `RSCALIB`
pub const RSCALIB1: u8 = 0x2;

/// Bitfield on register `RSCALIB`
pub const RSCALIB4: u8 = 0x10;

/// Bitfield on register `RSCALIB`
pub const RSCALIB6: u8 = 0x40;

/// Bitfield on register `RSCALIB`
pub const RSCALIB5: u8 = 0x20;

/// Bitfield on register `RSCR`
pub const RSSDEN: u8 = 0x1;

/// Bitfield on register `RSCR`
pub const RSRES: u8 = 0x80;

/// Bitfield on register `RSCR`
pub const RSMODE0: u8 = 0x10;

/// Bitfield on register `RSCR`
pub const RSOFM: u8 = 0x8;

/// Bitfield on register `RSCR`
pub const RSOS: u8 = 0x2;

/// Bitfield on register `RSCR`
pub const RSEOR: u8 = 0x4;

/// Bitfield on register `RSCR`
pub const RSMODE1: u8 = 0x20;

/// Bitfield on register `RSDBGR`
pub const RSDBGS0: u8 = 0x20;

/// Bitfield on register `RSDBGR`
pub const RSSANA: u8 = 0x1;

/// Bitfield on register `RSDBGR`
pub const RSHOME: u8 = 0x10;

/// Bitfield on register `RSDBGR`
pub const RSINFM: u8 = 0x4;

/// Bitfield on register `RSDBGR`
pub const RSDBGS1: u8 = 0x40;

/// Bitfield on register `RSDBGR`
pub const RSDBGEN: u8 = 0x80;

/// Bitfield on register `RSDBGR`
pub const RSFPD: u8 = 0x8;

/// Bitfield on register `RSDLYR`
pub const RSTRD3: u8 = 0x8;

/// Bitfield on register `RSDLYR`
pub const RSTRD2: u8 = 0x4;

/// Bitfield on register `RSDLYR`
pub const RSTRD0: u8 = 0x1;

/// Bitfield on register `RSDLYR`
pub const RSTRD5: u8 = 0x20;

/// Bitfield on register `RSDLYR`
pub const RSTRD1: u8 = 0x2;

/// Bitfield on register `RSDLYR`
pub const RSTRD4: u8 = 0x10;

/// Bitfield on register `RSDLYR`
pub const RSRD0: u8 = 0x40;

/// Bitfield on register `RSDLYR`
pub const RSRD1: u8 = 0x80;

/// Bitfield on register `RSFR`
pub const RSAOOR1: u8 = 0x20;

/// Bitfield on register `RSFR`
pub const RSOOR1: u8 = 0x1;

/// Bitfield on register `RSFR`
pub const RSAOOR2: u8 = 0x40;

/// Bitfield on register `RSFR`
pub const RSOOR2: u8 = 0x2;

/// Bitfield on register `RSFR`
pub const RSOFF: u8 = 0x8;

/// Bitfield on register `RSFR`
pub const RSOOR3: u8 = 0x4;

/// Bitfield on register `RSFR`
pub const RSAOOR3: u8 = 0x80;

/// Bitfield on register `RSMS1R`
pub const RSCH3E: u8 = 0x4;

/// Bitfield on register `RSMS1R`
pub const RSSTIM: u8 = 0x10;

/// Bitfield on register `RSMS1R`
pub const RSSCAL: u8 = 0x80;

/// Bitfield on register `RSMS1R`
pub const RSINTM: u8 = 0x8;

/// Bitfield on register `RSMS1R`
pub const RSCH1E: u8 = 0x1;

/// Bitfield on register `RSMS1R`
pub const RSCH2E: u8 = 0x2;

/// Bitfield on register `RSMS1R`
pub const RSSSV: u8 = 0x40;

/// Bitfield on register `RSMS1R`
pub const RSCMS: u8 = 0x20;

/// Bitfield on register `RSMS2R`
pub const RSAVGS2: u8 = 0x40;

/// Bitfield on register `RSMS2R`
pub const RSSADR0: u8 = 0x1;

/// Bitfield on register `RSMS2R`
pub const RSSADR1: u8 = 0x2;

/// Bitfield on register `RSMS2R`
pub const RSAVGS1: u8 = 0x20;

/// Bitfield on register `RSMS2R`
pub const RSSADR2: u8 = 0x4;

/// Bitfield on register `RSMS2R`
pub const RSSADR3: u8 = 0x8;

/// Bitfield on register `RSMS2R`
pub const RSAVGS3: u8 = 0x80;

/// Bitfield on register `RSMS2R`
pub const RSAVGS0: u8 = 0x10;

/// Bitfield on register `RSRES1H`
pub const RSRES1H0: u8 = 0x1;

/// Bitfield on register `RSRES1H`
pub const RSRES1H3: u8 = 0x8;

/// Bitfield on register `RSRES1H`
pub const RSRES1H1: u8 = 0x2;

/// Bitfield on register `RSRES1H`
pub const RSRES1H2: u8 = 0x4;

/// Bitfield on register `RSRES1H`
pub const RSRES1H7: u8 = 0x80;

/// Bitfield on register `RSRES1H`
pub const RSRES1H6: u8 = 0x40;

/// Bitfield on register `RSRES1H`
pub const RSRES1H5: u8 = 0x20;

/// Bitfield on register `RSRES1H`
pub const RSRES1H4: u8 = 0x10;

/// Bitfield on register `RSRES1L`
pub const RSRES1L4: u8 = 0x10;

/// Bitfield on register `RSRES1L`
pub const RSRES1L3: u8 = 0x8;

/// Bitfield on register `RSRES1L`
pub const RSRES1L7: u8 = 0x80;

/// Bitfield on register `RSRES1L`
pub const RSRES1L0: u8 = 0x1;

/// Bitfield on register `RSRES1L`
pub const RSRES1L6: u8 = 0x40;

/// Bitfield on register `RSRES1L`
pub const RSRES1L2: u8 = 0x4;

/// Bitfield on register `RSRES1L`
pub const RSRES1L5: u8 = 0x20;

/// Bitfield on register `RSRES1L`
pub const RSRES1L1: u8 = 0x2;

/// Bitfield on register `RSRES2H`
pub const RSRES2H3: u8 = 0x8;

/// Bitfield on register `RSRES2H`
pub const RSRES2H0: u8 = 0x1;

/// Bitfield on register `RSRES2H`
pub const RSRES2H6: u8 = 0x40;

/// Bitfield on register `RSRES2H`
pub const RSRES2H4: u8 = 0x10;

/// Bitfield on register `RSRES2H`
pub const RSRES2H5: u8 = 0x20;

/// Bitfield on register `RSRES2H`
pub const RSRES2H7: u8 = 0x80;

/// Bitfield on register `RSRES2H`
pub const RSRES2H2: u8 = 0x4;

/// Bitfield on register `RSRES2H`
pub const RSRES2H1: u8 = 0x2;

/// Bitfield on register `RSRES2L`
pub const RSRES2L0: u8 = 0x1;

/// Bitfield on register `RSRES2L`
pub const RSRES2L4: u8 = 0x10;

/// Bitfield on register `RSRES2L`
pub const RSRES2L6: u8 = 0x40;

/// Bitfield on register `RSRES2L`
pub const RSRES2L7: u8 = 0x80;

/// Bitfield on register `RSRES2L`
pub const RSRES2L2: u8 = 0x4;

/// Bitfield on register `RSRES2L`
pub const RSRES2L5: u8 = 0x20;

/// Bitfield on register `RSRES2L`
pub const RSRES2L3: u8 = 0x8;

/// Bitfield on register `RSRES2L`
pub const RSRES2L1: u8 = 0x2;

/// Bitfield on register `RSRES3H`
pub const RSRES3H1: u8 = 0x2;

/// Bitfield on register `RSRES3H`
pub const RSRES3H3: u8 = 0x8;

/// Bitfield on register `RSRES3H`
pub const RSRES3H7: u8 = 0x80;

/// Bitfield on register `RSRES3H`
pub const RSRES3H6: u8 = 0x40;

/// Bitfield on register `RSRES3H`
pub const RSRES3H0: u8 = 0x1;

/// Bitfield on register `RSRES3H`
pub const RSRES3H5: u8 = 0x20;

/// Bitfield on register `RSRES3H`
pub const RSRES3H2: u8 = 0x4;

/// Bitfield on register `RSRES3H`
pub const RSRES3H4: u8 = 0x10;

/// Bitfield on register `RSRES3L`
pub const RSRES3L7: u8 = 0x80;

/// Bitfield on register `RSRES3L`
pub const RSRES3L3: u8 = 0x8;

/// Bitfield on register `RSRES3L`
pub const RSRES3L6: u8 = 0x40;

/// Bitfield on register `RSRES3L`
pub const RSRES3L5: u8 = 0x20;

/// Bitfield on register `RSRES3L`
pub const RSRES3L2: u8 = 0x4;

/// Bitfield on register `RSRES3L`
pub const RSRES3L4: u8 = 0x10;

/// Bitfield on register `RSRES3L`
pub const RSRES3L0: u8 = 0x1;

/// Bitfield on register `RSRES3L`
pub const RSRES3L1: u8 = 0x2;

/// Bitfield on register `RSRES4H`
pub const RSRES4H1: u8 = 0x2;

/// Bitfield on register `RSRES4H`
pub const RSRES4H4: u8 = 0x10;

/// Bitfield on register `RSRES4H`
pub const RSRES4H3: u8 = 0x8;

/// Bitfield on register `RSRES4H`
pub const RSRES4H5: u8 = 0x20;

/// Bitfield on register `RSRES4H`
pub const RSRES4H2: u8 = 0x4;

/// Bitfield on register `RSRES4H`
pub const RSRES4H6: u8 = 0x40;

/// Bitfield on register `RSRES4H`
pub const RSRES4H7: u8 = 0x80;

/// Bitfield on register `RSRES4H`
pub const RSRES4H0: u8 = 0x1;

/// Bitfield on register `RSRES4L`
pub const RSRES4L4: u8 = 0x10;

/// Bitfield on register `RSRES4L`
pub const RSRES4L7: u8 = 0x80;

/// Bitfield on register `RSRES4L`
pub const RSRES4L3: u8 = 0x8;

/// Bitfield on register `RSRES4L`
pub const RSRES4L0: u8 = 0x1;

/// Bitfield on register `RSRES4L`
pub const RSRES4L2: u8 = 0x4;

/// Bitfield on register `RSRES4L`
pub const RSRES4L5: u8 = 0x20;

/// Bitfield on register `RSRES4L`
pub const RSRES4L6: u8 = 0x40;

/// Bitfield on register `RSRES4L`
pub const RSRES4L1: u8 = 0x2;

/// Bitfield on register `RSSR`
pub const RSRDY: u8 = 0x1;

/// Bitfield on register `RSSR`
pub const RSSVLD: u8 = 0x2;

/// Bitfield on register `RSSRCR`
pub const SRCMODE0: u8 = 0x1;

/// Bitfield on register `RSSRCR`
pub const SRCSTEP1: u8 = 0x80;

/// Bitfield on register `RSSRCR`
pub const SRCMIN0: u8 = 0x4;

/// Bitfield on register `RSSRCR`
pub const SRCSTEP0: u8 = 0x40;

/// Bitfield on register `RSSRCR`
pub const SRCMODE1: u8 = 0x2;

/// Bitfield on register `RSSRCR`
pub const SRCCLR: u8 = 0x10;

/// Bitfield on register `RSSRCR`
pub const SRCMIN1: u8 = 0x8;

/// Bitfield on register `SD12RR`
pub const SD12RR3: u8 = 0x8;

/// Bitfield on register `SD12RR`
pub const SD12RR1: u8 = 0x2;

/// Bitfield on register `SD12RR`
pub const SD12RR4: u8 = 0x10;

/// Bitfield on register `SD12RR`
pub const SD12RR2: u8 = 0x4;

/// Bitfield on register `SD12RR`
pub const SD12RR5: u8 = 0x20;

/// Bitfield on register `SD12RR`
pub const SD12RR6: u8 = 0x40;

/// Bitfield on register `SD12RR`
pub const SD12RR7: u8 = 0x80;

/// Bitfield on register `SD12RR`
pub const SD12RR0: u8 = 0x1;

/// Bitfield on register `SD13RR`
pub const SD13RR7: u8 = 0x80;

/// Bitfield on register `SD13RR`
pub const SD13RR4: u8 = 0x10;

/// Bitfield on register `SD13RR`
pub const SD13RR5: u8 = 0x20;

/// Bitfield on register `SD13RR`
pub const SD13RR1: u8 = 0x2;

/// Bitfield on register `SD13RR`
pub const SD13RR0: u8 = 0x1;

/// Bitfield on register `SD13RR`
pub const SD13RR3: u8 = 0x8;

/// Bitfield on register `SD13RR`
pub const SD13RR2: u8 = 0x4;

/// Bitfield on register `SD13RR`
pub const SD13RR6: u8 = 0x40;

/// Bitfield on register `SD23RR`
pub const SD23RR7: u8 = 0x80;

/// Bitfield on register `SD23RR`
pub const SD23RR0: u8 = 0x1;

/// Bitfield on register `SD23RR`
pub const SD23RR1: u8 = 0x2;

/// Bitfield on register `SD23RR`
pub const SD23RR6: u8 = 0x40;

/// Bitfield on register `SD23RR`
pub const SD23RR5: u8 = 0x20;

/// Bitfield on register `SD23RR`
pub const SD23RR3: u8 = 0x8;

/// Bitfield on register `SD23RR`
pub const SD23RR2: u8 = 0x4;

/// Bitfield on register `SD23RR`
pub const SD23RR4: u8 = 0x10;

/// Bitfield on register `SD360R`
pub const SD360R6: u8 = 0x40;

/// Bitfield on register `SD360R`
pub const SD360R3: u8 = 0x8;

/// Bitfield on register `SD360R`
pub const SD360R7: u8 = 0x80;

/// Bitfield on register `SD360R`
pub const SD360R4: u8 = 0x10;

/// Bitfield on register `SD360R`
pub const SD360R1: u8 = 0x2;

/// Bitfield on register `SD360R`
pub const SD360R0: u8 = 0x1;

/// Bitfield on register `SD360R`
pub const SD360R5: u8 = 0x20;

/// Bitfield on register `SD360R`
pub const SD360R2: u8 = 0x4;

/// Bitfield on register `SFC`
pub const SFDRA: u8 = 0x80;

/// Bitfield on register `SFC`
pub const SFFLC: u8 = 0x1F;

/// Bitfield on register `SFFR`
pub const RFL: u8 = 0x7;

/// Bitfield on register `SFFR`
pub const TFL: u8 = 0x70;

/// Bitfield on register `SFFR`
pub const TFC: u8 = 0x80;

/// Bitfield on register `SFFR`
pub const RFC: u8 = 0x8;

/// Bitfield on register `SFI`
pub const SFFLIM: u8 = 0x1;

/// Bitfield on register `SFI`
pub const SFERIM: u8 = 0x2;

/// Bitfield on register `SFIR`
pub const RIL: u8 = 0x7;

/// Bitfield on register `SFIR`
pub const STIE: u8 = 0x80;

/// Bitfield on register `SFIR`
pub const SRIE: u8 = 0x8;

/// Bitfield on register `SFIR`
pub const TIL: u8 = 0x70;

/// Bitfield on register `SFL`
pub const SFCLR: u8 = 0x80;

/// Bitfield on register `SFL`
pub const SFFLS: u8 = 0x1F;

/// Bitfield on register `SFS`
pub const SFOFL: u8 = 0x4;

/// Bitfield on register `SFS`
pub const SFUFL: u8 = 0x2;

/// Bitfield on register `SFS`
pub const SFFLRF: u8 = 0x1;

/// Bitfield on register `SMCR`
pub const SM: u8 = 0xE;

/// Bitfield on register `SMCR`
pub const SE: u8 = 0x1;

/// Bitfield on register `SP2CR`
pub const CPHA2: u8 = 0x4;

/// Bitfield on register `SP2CR`
pub const SP2E: u8 = 0x40;

/// Bitfield on register `SP2CR`
pub const SP2R: u8 = 0x3;

/// Bitfield on register `SP2CR`
pub const DORD2: u8 = 0x20;

/// Bitfield on register `SP2CR`
pub const CPOL2: u8 = 0x8;

/// Bitfield on register `SP2CR`
pub const SP2IE: u8 = 0x80;

/// Bitfield on register `SP2CR`
pub const MSTR2: u8 = 0x10;

/// Bitfield on register `SP2SR`
pub const SPI22X: u8 = 0x1;

/// Bitfield on register `SP2SR`
pub const WCOL2: u8 = 0x40;

/// Bitfield on register `SP2SR`
pub const SP2IF: u8 = 0x80;

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

/// Bitfield on register `SPCR`
pub const DORD: u8 = 0x20;

/// Bitfield on register `SPCR`
pub const SPIE: u8 = 0x80;

/// Bitfield on register `SPMCSR`
pub const SPMIE: u8 = 0x80;

/// Bitfield on register `SPMCSR`
pub const PGERS: u8 = 0x2;

/// Bitfield on register `SPMCSR`
pub const FLSEL: u8 = 0x38;

/// Bitfield on register `SPMCSR`
pub const PGWRT: u8 = 0x4;

/// Bitfield on register `SPMCSR`
pub const SELFPRGEN: u8 = 0x1;

/// Bitfield on register `SPMCSR`
pub const RWWSB: u8 = 0x40;

/// Bitfield on register `SPSR`
pub const RXIF: u8 = 0x10;

/// Bitfield on register `SPSR`
pub const SPIF: u8 = 0x80;

/// Bitfield on register `SPSR`
pub const SPI2X: u8 = 0x1;

/// Bitfield on register `SPSR`
pub const TXIF: u8 = 0x20;

/// Bitfield on register `SRCCAL`
pub const SRCCAL6: u8 = 0x20;

/// Bitfield on register `SRCCAL`
pub const SRCCAL8: u8 = 0x80;

/// Bitfield on register `SRCCAL`
pub const SRCCAL3: u8 = 0x4;

/// Bitfield on register `SRCCAL`
pub const SRCCAL1: u8 = 0x1;

/// Bitfield on register `SRCCAL`
pub const SRCCAL5: u8 = 0x10;

/// Bitfield on register `SRCCAL`
pub const SRCCAL2: u8 = 0x2;

/// Bitfield on register `SRCCAL`
pub const SRCCAL4: u8 = 0x8;

/// Bitfield on register `SRCCAL`
pub const SRCCAL7: u8 = 0x40;

/// Bitfield on register `SRCCALL`
pub const SRCCAL0: u8 = 0x1;

/// Bitfield on register `SRCTCAL`
pub const SRCTC: u8 = 0x7;

/// Bitfield on register `SRCTCAL`
pub const SRCS: u8 = 0x18;

/// Bitfield on register `SRCTCAL`
pub const DIS_SRC: u8 = 0x40;

/// Bitfield on register `SRCTCAL`
pub const HOLD_SRC: u8 = 0x80;

/// Bitfield on register `SREG`
pub const C: u8 = 0x1;

/// Bitfield on register `SREG`
pub const N: u8 = 0x4;

/// Bitfield on register `SREG`
pub const H: u8 = 0x20;

/// Bitfield on register `SREG`
pub const T: u8 = 0x40;

/// Bitfield on register `SREG`
pub const S: u8 = 0x10;

/// Bitfield on register `SREG`
pub const Z: u8 = 0x2;

/// Bitfield on register `SREG`
pub const V: u8 = 0x8;

/// Bitfield on register `SREG`
pub const I: u8 = 0x80;

/// Bitfield on register `SSMCR`
pub const SSMTPE: u8 = 0x8;

/// Bitfield on register `SSMCR`
pub const SSMTAE: u8 = 0x20;

/// Bitfield on register `SSMCR`
pub const SSMPVE: u8 = 0x10;

/// Bitfield on register `SSMCR`
pub const SSMTGE: u8 = 0x4;

/// Bitfield on register `SSMFBR`
pub const SSMPLDT: u8 = 0x20;

/// Bitfield on register `SSMIFR`
pub const SSMIF: u8 = 0x1;

/// Bitfield on register `SSMIMR`
pub const SSMIM: u8 = 0x1;

/// Bitfield on register `SSMRR`
pub const SSMR: u8 = 0x1;

/// Bitfield on register `SSMRR`
pub const SSMST: u8 = 0x2;

/// Bitfield on register `SSMSR`
pub const SSMESM: u8 = 0xF;

/// Bitfield on register `SSMSR`
pub const SSMERR: u8 = 0x80;

/// Bitfield on register `SSMSTR`
pub const SSMSTA: u8 = 0x3F;

/// Bitfield on register `SUPCA1`
pub const PVCAL: u8 = 0xF0;

/// Bitfield on register `SUPCA1`
pub const PV22: u8 = 0x4;

/// Bitfield on register `SUPCA1`
pub const PVDIC: u8 = 0x8;

/// Bitfield on register `SUPCA2`
pub const BGCAL: u8 = 0xF;

/// Bitfield on register `SUPCA4`
pub const ICONST: u8 = 0x3F;

/// Bitfield on register `SUPCA5`
pub const IPTAT: u8 = 0x3F;

/// Bitfield on register `SUPCA7`
pub const VCCCAL: u8 = 0x7;

/// Bitfield on register `SUPCA7`
pub const LFVCCBD: u8 = 0x38;

/// Bitfield on register `SUPCA8`
pub const VSWBD: u8 = 0x7;

/// Bitfield on register `SUPCA8`
pub const DVCCBD: u8 = 0x38;

/// Bitfield on register `SUPCR`
pub const DVHEN: u8 = 0x20;

/// Bitfield on register `SUPCR`
pub const VMEMEN: u8 = 0x80;

/// Bitfield on register `SUPCR`
pub const AVDIC: u8 = 0x8;

/// Bitfield on register `SUPCR`
pub const AVCCLM: u8 = 0x2;

/// Bitfield on register `SUPCR`
pub const AVEN: u8 = 0x10;

/// Bitfield on register `SUPCR`
pub const PVEN: u8 = 0x4;

/// Bitfield on register `SUPCR`
pub const AVCCRM: u8 = 0x1;

/// Bitfield on register `SUPCR`
pub const VMRESM: u8 = 0x40;

/// Bitfield on register `SUPFR`
pub const AVCCRF: u8 = 0x1;

/// Bitfield on register `SUPFR`
pub const AVCCLF: u8 = 0x2;

/// Bitfield on register `T0CR`
pub const T0PS: u8 = 0x7;

/// Bitfield on register `T0CR`
pub const T0IE: u8 = 0x8;

/// Bitfield on register `T0CR`
pub const T0PR: u8 = 0x10;

/// Bitfield on register `T0IFR`
pub const T0F: u8 = 0x1;

/// Bitfield on register `T1CR`
pub const T1TOP: u8 = 0x10;

/// Bitfield on register `T1CR`
pub const T1TOS: u8 = 0x40;

/// Bitfield on register `T1CR`
pub const T1RES: u8 = 0x20;

/// Bitfield on register `T1CR`
pub const T1CRM: u8 = 0x4;

/// Bitfield on register `T1CR`
pub const T1OTM: u8 = 0x1;

/// Bitfield on register `T1CR`
pub const T1ENA: u8 = 0x80;

/// Bitfield on register `T1CR`
pub const T1CTM: u8 = 0x2;

/// Bitfield on register `T1IFR`
pub const T1COF: u8 = 0x2;

/// Bitfield on register `T1IFR`
pub const T1OFF: u8 = 0x1;

/// Bitfield on register `T1IMR`
pub const T1OIM: u8 = 0x1;

/// Bitfield on register `T1IMR`
pub const T1CIM: u8 = 0x2;

/// Bitfield on register `T1MR`
pub const T1CS: u8 = 0x3;

/// Bitfield on register `T1MR`
pub const T1PS: u8 = 0x3C;

/// Bitfield on register `T1MR`
pub const T1DC: u8 = 0xC0;

/// Bitfield on register `T2CR`
pub const T2TOP: u8 = 0x10;

/// Bitfield on register `T2CR`
pub const T2CRM: u8 = 0x4;

/// Bitfield on register `T2CR`
pub const T2CTM: u8 = 0x2;

/// Bitfield on register `T2CR`
pub const T2RES: u8 = 0x20;

/// Bitfield on register `T2CR`
pub const T2ENA: u8 = 0x80;

/// Bitfield on register `T2CR`
pub const T2OTM: u8 = 0x1;

/// Bitfield on register `T2CR`
pub const T2TOS: u8 = 0x40;

/// Bitfield on register `T2IFR`
pub const T2COF: u8 = 0x2;

/// Bitfield on register `T2IFR`
pub const T2OFF: u8 = 0x1;

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
pub const T3RES: u8 = 0x20;

/// Bitfield on register `T3CR`
pub const T3CTM: u8 = 0x2;

/// Bitfield on register `T3CR`
pub const T3TOP: u8 = 0x10;

/// Bitfield on register `T3CR`
pub const T3TOS: u8 = 0x40;

/// Bitfield on register `T3CR`
pub const T3OTM: u8 = 0x1;

/// Bitfield on register `T3CR`
pub const T3CRM: u8 = 0x4;

/// Bitfield on register `T3CR`
pub const T3CPRM: u8 = 0x8;

/// Bitfield on register `T3CR`
pub const T3ENA: u8 = 0x80;

/// Bitfield on register `T3IFR`
pub const T3COF: u8 = 0x2;

/// Bitfield on register `T3IFR`
pub const T3OFF: u8 = 0x1;

/// Bitfield on register `T3IFR`
pub const T3ICF: u8 = 0x4;

/// Bitfield on register `T3IMR`
pub const T3CIM: u8 = 0x2;

/// Bitfield on register `T3IMR`
pub const T3OIM: u8 = 0x1;

/// Bitfield on register `T3IMR`
pub const T3CPIM: u8 = 0x4;

/// Bitfield on register `T3MRA`
pub const T3PS: u8 = 0x1C;

/// Bitfield on register `T3MRA`
pub const T3CS: u8 = 0x3;

/// Bitfield on register `T3MRB`
pub const T3SCE: u8 = 0x2;

/// Bitfield on register `T3MRB`
pub const T3CNC: u8 = 0x4;

/// Bitfield on register `T3MRB`
pub const T3CE: u8 = 0x18;

/// Bitfield on register `T3MRB`
pub const T3ICS: u8 = 0xE0;

/// Bitfield on register `T4CR`
pub const T4CRM: u8 = 0x4;

/// Bitfield on register `T4CR`
pub const T4OTM: u8 = 0x1;

/// Bitfield on register `T4CR`
pub const T4CTM: u8 = 0x2;

/// Bitfield on register `T4CR`
pub const T4ENA: u8 = 0x80;

/// Bitfield on register `T4CR`
pub const T4RES: u8 = 0x20;

/// Bitfield on register `T4CR`
pub const T4TOP: u8 = 0x10;

/// Bitfield on register `T4CR`
pub const T4TOS: u8 = 0x40;

/// Bitfield on register `T4CR`
pub const T4CPRM: u8 = 0x8;

/// Bitfield on register `T4IFR`
pub const T4ICF: u8 = 0x4;

/// Bitfield on register `T4IFR`
pub const T4OFF: u8 = 0x1;

/// Bitfield on register `T4IFR`
pub const T4COF: u8 = 0x2;

/// Bitfield on register `T4IMR`
pub const T4OIM: u8 = 0x1;

/// Bitfield on register `T4IMR`
pub const T4CPIM: u8 = 0x4;

/// Bitfield on register `T4IMR`
pub const T4CIM: u8 = 0x2;

/// Bitfield on register `T4MRA`
pub const T4PS: u8 = 0x1C;

/// Bitfield on register `T4MRA`
pub const T4CS: u8 = 0x3;

/// Bitfield on register `T4MRB`
pub const T4CNC: u8 = 0x4;

/// Bitfield on register `T4MRB`
pub const T4CE: u8 = 0x18;

/// Bitfield on register `T4MRB`
pub const T4ICS: u8 = 0xE0;

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
pub const T5CIM: u8 = 0x2;

/// Bitfield on register `T5IMR`
pub const T5OIM: u8 = 0x1;

/// Bitfield on register `TMCR1`
pub const TMPIS: u8 = 0x7;

/// Bitfield on register `TMCR1`
pub const TMCIM: u8 = 0x10;

/// Bitfield on register `TMCR1`
pub const TMSCS: u8 = 0x8;

/// Bitfield on register `TMCR2`
pub const TMLSB: u8 = 0x40;

/// Bitfield on register `TMCR2`
pub const TMCRCE: u8 = 0x1;

/// Bitfield on register `TMCR2`
pub const TMSSE: u8 = 0x20;

/// Bitfield on register `TMCR2`
pub const TMCRCSE: u8 = 0x6;

/// Bitfield on register `TMCR2`
pub const TMPOL: u8 = 0x10;

/// Bitfield on register `TMCR2`
pub const TMNRZE: u8 = 0x8;

/// Bitfield on register `TMFSM`
pub const TMMSM: u8 = 0x70;

/// Bitfield on register `TMFSM`
pub const TMSSM: u8 = 0xF;

/// Bitfield on register `TMOCR`
pub const TO4PIS: u8 = 0xC0;

/// Bitfield on register `TMOCR`
pub const TO1PIS: u8 = 0x3;

/// Bitfield on register `TMOCR`
pub const TO3PIS: u8 = 0x30;

/// Bitfield on register `TMOCR`
pub const TO2PIS: u8 = 0xC;

/// Bitfield on register `TMSR`
pub const TMTCF: u8 = 0x1;

/// Bitfield on register `TMSSC`
pub const TMSSP: u8 = 0xF;

/// Bitfield on register `TMSSC`
pub const TMSSH: u8 = 0x80;

/// Bitfield on register `TMSSC`
pub const TMSSL: u8 = 0x70;

/// Bitfield on register `TPCALR1`
pub const TPBG_IREF: u8 = 0x3F;

/// Bitfield on register `TPCALR11`
pub const MTBTR0: u8 = 0x1;

/// Bitfield on register `TPCALR11`
pub const ENDVBD: u8 = 0x4;

/// Bitfield on register `TPCALR11`
pub const ENVSWBD: u8 = 0x10;

/// Bitfield on register `TPCALR11`
pub const TPCALR117: u8 = 0x80;

/// Bitfield on register `TPCALR11`
pub const MTBTR1: u8 = 0x2;

/// Bitfield on register `TPCALR11`
pub const TPCALR115: u8 = 0x20;

/// Bitfield on register `TPCALR11`
pub const TPCALR116: u8 = 0x40;

/// Bitfield on register `TPCALR11`
pub const ENLFBD: u8 = 0x8;

/// Bitfield on register `TPCALR12`
pub const TPCALR121: u8 = 0x2;

/// Bitfield on register `TPCALR12`
pub const TPCALR123: u8 = 0x8;

/// Bitfield on register `TPCALR12`
pub const TPCALR127: u8 = 0x80;

/// Bitfield on register `TPCALR12`
pub const TPCALR126: u8 = 0x40;

/// Bitfield on register `TPCALR12`
pub const TPDMOD: u8 = 0x1;

/// Bitfield on register `TPCALR12`
pub const TPCALR125: u8 = 0x20;

/// Bitfield on register `TPCALR12`
pub const TPCALR122: u8 = 0x4;

/// Bitfield on register `TPCALR12`
pub const TPCALR124: u8 = 0x10;

/// Bitfield on register `TPCALR2`
pub const TPBG_UREF: u8 = 0x7F;

/// Bitfield on register `TPCALR3`
pub const TPORTH: u8 = 0x18;

/// Bitfield on register `TPCALR3`
pub const LFVCC_TPCAL0: u8 = 0x1;

/// Bitfield on register `TPCALR3`
pub const LFVCC_TPCAL1: u8 = 0x2;

/// Bitfield on register `TPCALR3`
pub const LFVCC_TPCAL2: u8 = 0x4;

/// Bitfield on register `TPCALR4`
pub const COMPVC_CAL: u8 = 0x18;

/// Bitfield on register `TPCALR4`
pub const TPINIT_CAL: u8 = 0x7;

/// Bitfield on register `TPCR1`
pub const TPQPLM: u8 = 0x4;

/// Bitfield on register `TPCR1`
pub const TPDFCP: u8 = 0x60;

/// Bitfield on register `TPCR1`
pub const TPMODE: u8 = 0x80;

/// Bitfield on register `TPCR1`
pub const TPBR: u8 = 0x10;

/// Bitfield on register `TPCR2`
pub const TPWDLV: u8 = 0x60;

/// Bitfield on register `TPCR2`
pub const TPPSD: u8 = 0x4;

/// Bitfield on register `TPCR2`
pub const TPMOD: u8 = 0x2;

/// Bitfield on register `TPCR2`
pub const TPMA: u8 = 0x1;

/// Bitfield on register `TPCR2`
pub const TPD: u8 = 0x8;

/// Bitfield on register `TPCR2`
pub const TPNFTO: u8 = 0x10;

/// Bitfield on register `TPCR3`
pub const TPRD: u8 = 0x2;

/// Bitfield on register `TPCR3`
pub const TPTLIW: u8 = 0x4;

/// Bitfield on register `TPCR3`
pub const TPTD: u8 = 0x1;

/// Bitfield on register `TPCR3`
pub const TPRCD: u8 = 0x20;

/// Bitfield on register `TPCR4`
pub const TPBCCS: u8 = 0xF;

/// Bitfield on register `TPCR4`
pub const TPBCM: u8 = 0x10;

/// Bitfield on register `TPCR5`
pub const TPMUD: u8 = 0x7;

/// Bitfield on register `TPCR5`
pub const TPMD: u8 = 0x70;

/// Bitfield on register `TPDCR1`
pub const TPDCL1: u8 = 0x3F;

/// Bitfield on register `TPDCR2`
pub const TPDCL2: u8 = 0x3F;

/// Bitfield on register `TPDCR3`
pub const TPDCL3: u8 = 0x3F;

/// Bitfield on register `TPDCR4`
pub const TPDCL4: u8 = 0x3F;

/// Bitfield on register `TPDCR5`
pub const TPDCL5: u8 = 0x3F;

/// Bitfield on register `TPECMR`
pub const TPECM3: u8 = 0x30;

/// Bitfield on register `TPECMR`
pub const TPECM1: u8 = 0x3;

/// Bitfield on register `TPECMR`
pub const TPECM4: u8 = 0xC0;

/// Bitfield on register `TPECMR`
pub const TPECM2: u8 = 0xC;

/// Bitfield on register `TPFR`
pub const TPF: u8 = 0x1;

/// Bitfield on register `TPFR`
pub const TPNFTF: u8 = 0x4;

/// Bitfield on register `TPFR`
pub const TPBERF: u8 = 0x8;

/// Bitfield on register `TPFR`
pub const TPFTF: u8 = 0x2;

/// Bitfield on register `TPIMR`
pub const TPIM: u8 = 0x1;

/// Bitfield on register `TPIMR`
pub const TPFTIM: u8 = 0x2;

/// Bitfield on register `TPIMR`
pub const TPNFTIM: u8 = 0x4;

/// Bitfield on register `TPIMR`
pub const TPBERIM: u8 = 0x8;

/// Bitfield on register `TPSR`
pub const TPA: u8 = 0x1;

/// Bitfield on register `TPSR`
pub const TPPSW: u8 = 0x4;

/// Bitfield on register `TPSR`
pub const TPBCOK: u8 = 0x8;

/// Bitfield on register `TPSR`
pub const TPGAP: u8 = 0x2;

/// Bitfield on register `TW1AMR`
pub const TW1AM: u8 = 0xFE;

/// Bitfield on register `TW1AR`
pub const TW1A: u8 = 0xFE;

/// Bitfield on register `TW1AR`
pub const TW1GCE: u8 = 0x1;

/// Bitfield on register `TW1CR`
pub const TW1WC: u8 = 0x8;

/// Bitfield on register `TW1CR`
pub const TW1EN: u8 = 0x4;

/// Bitfield on register `TW1CR`
pub const TW1EA: u8 = 0x40;

/// Bitfield on register `TW1CR`
pub const TW1STA: u8 = 0x20;

/// Bitfield on register `TW1CR`
pub const TW1STO: u8 = 0x10;

/// Bitfield on register `TW1CR`
pub const TW1IE: u8 = 0x1;

/// Bitfield on register `TW1CR`
pub const TW1INT: u8 = 0x80;

/// Bitfield on register `TW1SR`
pub const TW1S: u8 = 0xF8;

/// Bitfield on register `TW1SR`
pub const TW1PS: u8 = 0x3;

/// Bitfield on register `TW2AMR`
pub const TW2AM: u8 = 0xFE;

/// Bitfield on register `TW2AR`
pub const TW2GCE: u8 = 0x1;

/// Bitfield on register `TW2AR`
pub const TW2A: u8 = 0xFE;

/// Bitfield on register `TW2CR`
pub const TW2STA: u8 = 0x20;

/// Bitfield on register `TW2CR`
pub const TW2WC: u8 = 0x8;

/// Bitfield on register `TW2CR`
pub const TW2IE: u8 = 0x1;

/// Bitfield on register `TW2CR`
pub const TW2EA: u8 = 0x40;

/// Bitfield on register `TW2CR`
pub const TW2INT: u8 = 0x80;

/// Bitfield on register `TW2CR`
pub const TW2EN: u8 = 0x4;

/// Bitfield on register `TW2CR`
pub const TW2STO: u8 = 0x10;

/// Bitfield on register `TW2SR`
pub const TW2PS: u8 = 0x3;

/// Bitfield on register `TW2SR`
pub const TW2S: u8 = 0xF8;

/// Bitfield on register `VMCR`
pub const VMPS: u8 = 0x60;

/// Bitfield on register `VMCR`
pub const VMLS: u8 = 0xF;

/// Bitfield on register `VMCR`
pub const VMIM: u8 = 0x10;

/// Bitfield on register `VMCR`
pub const VMRS: u8 = 0x80;

/// Bitfield on register `VMSCR`
pub const VMDIH: u8 = 0x2;

/// Bitfield on register `VMSCR`
pub const VMF: u8 = 0x1;

/// Bitfield on register `VXMCTRL`
pub const VX_SEL1: u8 = 0x2;

/// Bitfield on register `VXMCTRL`
pub const EN_VX: u8 = 0x4;

/// Bitfield on register `VXMCTRL`
pub const VX_SEL0: u8 = 0x1;

/// Bitfield on register `VXMCTRL`
pub const EN_VX_IN: u8 = 0x10;

/// Bitfield on register `VXMCTRL`
pub const EN_VX_OUT: u8 = 0x8;

/// Bitfield on register `WDTCR`
pub const WDPS: u8 = 0x7;

/// Bitfield on register `WDTCR`
pub const WDCE: u8 = 0x10;

/// Bitfield on register `WDTCR`
pub const WDE: u8 = 0x8;

/// `CLKOUT_CLOCK_SELECT` value group
#[allow(non_upper_case_globals)]
pub mod clkout_clock_select {
   /// clk_src.
   pub const VAL_0x00: u32 = 0x0;
   /// clk_frc.
   pub const VAL_0x01: u32 = 0x1;
   /// clk_mrc.
   pub const VAL_0x02: u32 = 0x2;
   /// clk_xto.
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

/// `COMM_TWI1_PRESCALE` value group
#[allow(non_upper_case_globals)]
pub mod comm_twi1_prescale {
   /// 1.
   pub const VAL_0x00: u32 = 0x0;
   /// 4.
   pub const VAL_0x01: u32 = 0x1;
   /// 16.
   pub const VAL_0x02: u32 = 0x2;
   /// 64.
   pub const VAL_0x03: u32 = 0x3;
}

/// `COMM_TWI2_PRESCALE` value group
#[allow(non_upper_case_globals)]
pub mod comm_twi2_prescale {
   /// 1.
   pub const VAL_0x00: u32 = 0x0;
   /// 4.
   pub const VAL_0x01: u32 = 0x1;
   /// 16.
   pub const VAL_0x02: u32 = 0x2;
   /// 64.
   pub const VAL_0x03: u32 = 0x3;
}

/// `CPU_BUSY_OUT` value group
#[allow(non_upper_case_globals)]
pub mod cpu_busy_out {
   /// disabled.
   pub const VAL_0x00: u32 = 0x0;
   /// PB0.
   pub const VAL_0x01: u32 = 0x1;
   /// PB3.
   pub const VAL_0x02: u32 = 0x2;
   /// PC1.
   pub const VAL_0x03: u32 = 0x3;
}

/// `CPU_CLK_PRESCALE_3BITS` value group
#[allow(non_upper_case_globals)]
pub mod cpu_clk_prescale_3bits {
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

/// `CPU_CLT_PRESCALE_3BITS` value group
#[allow(non_upper_case_globals)]
pub mod cpu_clt_prescale_3bits {
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

/// `CPU_FUSE_LOCK_SEL_3BITS` value group
#[allow(non_upper_case_globals)]
pub mod cpu_fuse_lock_sel_3bits {
   /// ROM/FLASH.
   pub const VAL_0x00: u32 = 0x0;
   /// Lockbits.
   pub const VAL_0x01: u32 = 0x8;
   /// Security Fuses.
   pub const VAL_0x03: u32 = 0x18;
   /// EEPROM Protection Fuse Low.
   pub const VAL_0x05: u32 = 0x28;
   /// EEPROM Protection Fuse High.
   pub const VALR_0x07: u32 = 0x38;
}

/// `CPU_IVL_2BITS` value group
#[allow(non_upper_case_globals)]
pub mod cpu_ivl_2bits {
   /// 0x3600.
   pub const VAL_0x00: u32 = 0x0;
   /// 0x4000.
   pub const VAL_0x01: u32 = 0x1;
   /// 0x7000.
   pub const VAL_0x02: u32 = 0x2;
   /// 0x8000.
   pub const VAL_0x03: u32 = 0x3;
}

/// `CPU_SLEEP_MODE_3BITS` value group
#[allow(non_upper_case_globals)]
pub mod cpu_sleep_mode_3bits {
   /// Idle.
   pub const IDLE: u32 = 0x0;
   /// Power save.
   pub const PSAVE: u32 = 0x1;
   /// Power down.
   pub const PDOWN: u32 = 0x2;
   /// Extended power save.
   pub const EPSAVE: u32 = 0x3;
   /// Extended power down.
   pub const EPDOWN: u32 = 0x4;
   /// Power off.
   pub const POFF: u32 = 0x5;
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

/// `GAUSS_BT_SELECT` value group
#[allow(non_upper_case_globals)]
pub mod gauss_bt_select {
   /// BT = 2.
   pub const VAL_0x00: u32 = 0x0;
   /// BT = 1.5.
   pub const VAL_0x01: u32 = 0x1;
   /// BT = 1.
   pub const VAL_0x02: u32 = 0x2;
   /// BT = 0.5.
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

/// `LFREC_BIT_RATE` value group
#[allow(non_upper_case_globals)]
pub mod lfrec_bit_rate {
   /// 1.95 kBit/s.
   pub const VAL_0x00: u32 = 0x0;
   /// 3.90 kBit/s.
   pub const VAL_0x01: u32 = 0x1;
   /// 7.81 kBit/s.
   pub const VAL_0x02: u32 = 0x2;
}

/// `LFREC_RESET_TIME` value group
#[allow(non_upper_case_globals)]
pub mod lfrec_reset_time {
   /// 128 us.
   pub const VAL_0x00: u32 = 0x0;
   /// 160 us.
   pub const VAL_0x01: u32 = 0x1;
   /// 192 us.
   pub const VAL_0x02: u32 = 0x2;
   /// 224 us.
   pub const VAL_0x03: u32 = 0x3;
}

/// `LFREC_RSSI_RESET_TIME` value group
#[allow(non_upper_case_globals)]
pub mod lfrec_rssi_reset_time {
   /// 256 us.
   pub const VAL_0x00: u32 = 0x0;
   /// 384 us.
   pub const VAL_0x01: u32 = 0x1;
   /// 512 us.
   pub const VAL_0x02: u32 = 0x2;
   /// 640 us.
   pub const VAL_0x03: u32 = 0x3;
}

/// `LFREC_SENSITIVITY_MODE` value group
#[allow(non_upper_case_globals)]
pub mod lfrec_sensitivity_mode {
   /// High Sensitivity.
   pub const VAL_0x00: u32 = 0x0;
   /// Medium Sensitivity.
   pub const VAL_0x01: u32 = 0x1;
   /// Low Sensitivity.
   pub const VAL_0x02: u32 = 0x2;
}

/// `LFREC_STANDBY_TIME` value group
#[allow(non_upper_case_globals)]
pub mod lfrec_standby_time {
   /// 384 us.
   pub const VAL_0x00: u32 = 0x0;
   /// 768 us.
   pub const VAL_0x01: u32 = 0x1;
   /// 1152 us.
   pub const VAL_0x02: u32 = 0x2;
   /// 1536 us.
   pub const VAL_0x03: u32 = 0x3;
   /// 2304 us.
   pub const VAL_0x04: u32 = 0x4;
   /// 3072 us.
   pub const VAL_0x05: u32 = 0x5;
   /// 4608 us.
   pub const VAL_0x06: u32 = 0x6;
   /// 6144 us.
   pub const VAL_0x07: u32 = 0x7;
}

/// `LFTP_FIELDCLK_PRESCALER` value group
#[allow(non_upper_case_globals)]
pub mod lftp_fieldclk_prescaler {
   /// Field Clock / 1.
   pub const VAL_0x00: u32 = 0x0;
   /// Field Clock / 1.
   pub const VAL_0x01: u32 = 0x1;
   /// Field Clock / 2.
   pub const VAL_0x02: u32 = 0x2;
   /// Field Clock / 4.
   pub const VAL_0x03: u32 = 0x3;
}

/// `LFTP_TPECM` value group
#[allow(non_upper_case_globals)]
pub mod lftp_tpecm {
   /// Manchester.
   pub const VAL_0x00: u32 = 0x0;
   /// Biphase.
   pub const VAL_0x01: u32 = 0x1;
   /// NRZ.
   pub const VAL_0x02: u32 = 0x2;
   /// Manchester.
   pub const VAL_0x03: u32 = 0x3;
}

/// `LFTP_TPMUD` value group
#[allow(non_upper_case_globals)]
pub mod lftp_tpmud {
   /// 5.0 V.
   pub const VAL_0x00: u32 = 0x0;
   /// 5.4 V.
   pub const VAL_0x01: u32 = 0x1;
   /// 5.8 V.
   pub const VAL_0x02: u32 = 0x2;
   /// 6.2 V.
   pub const VAL_0x03: u32 = 0x3;
   /// 6.6 V.
   pub const VAL_0x04: u32 = 0x4;
   /// 7.0 V.
   pub const VAL_0x05: u32 = 0x5;
   /// Up to OVP.
   pub const VAL_0x07: u32 = 0x7;
}

/// `LFTP_TPWDLV` value group
#[allow(non_upper_case_globals)]
pub mod lftp_tpwdlv {
   /// 1.024 ms.
   pub const VAL_0x00: u32 = 0x0;
   /// 2.048 ms.
   pub const VAL_0x01: u32 = 0x1;
   /// 3.072 ms.
   pub const VAL_0x02: u32 = 0x2;
   /// 4.096 ms.
   pub const VAL_0x03: u32 = 0x3;
}

/// `SPI2_SP2R` value group
#[allow(non_upper_case_globals)]
pub mod spi2_sp2r {
   /// clkio/4.
   pub const VAL_0x00: u32 = 0x0;
   /// clkio/16.
   pub const VAL_0x01: u32 = 0x1;
   /// clkio/64.
   pub const VAL_0x02: u32 = 0x2;
   /// clkio/128.
   pub const VAL_0x03: u32 = 0x3;
}

/// `SPI_SPR` value group
#[allow(non_upper_case_globals)]
pub mod spi_spr {
   /// clkio/4.
   pub const VAL_0x00: u32 = 0x0;
   /// clkio/16.
   pub const VAL_0x01: u32 = 0x1;
   /// clkio/64.
   pub const VAL_0x02: u32 = 0x2;
   /// clkio/128.
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
   /// TX DSP enable.
   pub const VAL_0x03: u32 = 0x3;
   /// TX DSP disable.
   pub const VAL_0x04: u32 = 0x4;
   /// Send telegram.
   pub const VAL_0x05: u32 = 0x5;
   /// Shut down.
   pub const VAL_0x06: u32 = 0x6;
   /// VCO Tuning.
   pub const VAL_0x07: u32 = 0x7;
   /// Antenna Tuning.
   pub const VAL_0x08: u32 = 0x8;
}

/// `TIM0_PS_SELECT` value group
#[allow(non_upper_case_globals)]
pub mod tim0_ps_select {
   /// 0.256ms typ.
   pub const VAL_0x00: u32 = 0x0;
   /// 1ms typ.
   pub const VAL_0x01: u32 = 0x1;
   /// 8ms typ.
   pub const VAL_0x02: u32 = 0x2;
   /// 0.5s typ.
   pub const VAL_0x03: u32 = 0x3;
   /// 1s typ.
   pub const VAL_0x04: u32 = 0x4;
   /// 8s typ.
   pub const VAL_0x05: u32 = 0x5;
   /// 67s typ.
   pub const VAL_0x06: u32 = 0x6;
   /// 134s typ.
   pub const VAL_0x07: u32 = 0x7;
}

/// `TIM0_WDPS_SELECT` value group
#[allow(non_upper_case_globals)]
pub mod tim0_wdps_select {
   /// 1ms typ (0.85ms min).
   pub const VAL_0x00: u32 = 0x0;
   /// 4ms typ (3.4ms min).
   pub const VAL_0x01: u32 = 0x1;
   /// 32ms typ (27ms min).
   pub const VAL_0x02: u32 = 0x2;
   /// 2.1s typ (1.75s min).
   pub const VAL_0x03: u32 = 0x3;
   /// 4.2s typ (3.5s min).
   pub const VAL_0x04: u32 = 0x4;
   /// 16.8s typ (14s min).
   pub const VAL_0x05: u32 = 0x5;
   /// 134s typ (110s min).
   pub const VAL_0x06: u32 = 0x6;
   /// 268s typ (220s min).
   pub const VAL_0x07: u32 = 0x7;
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
   /// clk_mrc.
   pub const VAL_0x03: u32 = 0x3;
}

/// `TIM1_DC_SELECT` value group
#[allow(non_upper_case_globals)]
pub mod tim1_dc_select {
   /// Bypass.
   pub const VAL_0x00: u32 = 0x0;
   /// Duty cycle 1/1 (div 2).
   pub const VAL_0x01: u32 = 0x1;
   /// Duty cycle 1/2 (div 3).
   pub const VAL_0x02: u32 = 0x2;
   /// Duty cycle 1/3 (div 4).
   pub const VAL_0x03: u32 = 0x3;
}

/// `TIM2_CLOCK_SELECT` value group
#[allow(non_upper_case_globals)]
pub mod tim2_clock_select {
   /// clk_src.
   pub const VAL_0x00: u32 = 0x0;
   /// clk_mrc.
   pub const VAL_0x01: u32 = 0x1;
   /// clk_T.
   pub const VAL_0x02: u32 = 0x2;
   /// clk_xto4.
   pub const VAL_0x03: u32 = 0x3;
}

/// `TIM2_DC_SELECT` value group
#[allow(non_upper_case_globals)]
pub mod tim2_dc_select {
   /// Bypass.
   pub const VAL_0x00: u32 = 0x0;
   /// Duty cycle 1/1 (div 2).
   pub const VAL_0x01: u32 = 0x1;
   /// Duty cycle 1/2 (div 3).
   pub const VAL_0x02: u32 = 0x2;
   /// Duty cycle 1/3 (div 4).
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

/// `TIM3_CAPTURE_SELECT` value group
#[allow(non_upper_case_globals)]
pub mod tim3_capture_select {
   /// clk_T2.
   pub const VAL_0x00: u32 = 0x0;
   /// clk_T1.
   pub const VAL_0x01: u32 = 0x1;
   /// clk_T4.
   pub const VAL_0x02: u32 = 0x2;
   /// TICP.
   pub const VAL_0x03: u32 = 0x3;
   /// LFES.
   pub const VAL_0x04: u32 = 0x4;
   /// clk_src.
   pub const VAL_0x05: u32 = 0x5;
   /// TPGAP.
   pub const VAL_0x06: u32 = 0x6;
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
   /// clk_TEI.
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

/// `TIM4_CAPTURE_SELECT` value group
#[allow(non_upper_case_globals)]
pub mod tim4_capture_select {
   /// clk_T2.
   pub const VAL_0x00: u32 = 0x0;
   /// clk_T1.
   pub const VAL_0x01: u32 = 0x1;
   /// clk_T3.
   pub const VAL_0x02: u32 = 0x2;
   /// TICP.
   pub const VAL_0x03: u32 = 0x3;
   /// LFES.
   pub const VAL_0x04: u32 = 0x4;
   /// clk_src.
   pub const VAL_0x05: u32 = 0x5;
   /// TPGAP.
   pub const VAL_0x06: u32 = 0x6;
}

/// `TIM4_CLOCK_SELECT` value group
#[allow(non_upper_case_globals)]
pub mod tim4_clock_select {
   /// clk_src.
   pub const VAL_0x00: u32 = 0x0;
   /// clk_T.
   pub const VAL_0x01: u32 = 0x1;
   /// clk_mrc.
   pub const VAL_0x02: u32 = 0x2;
   /// clk_frc.
   pub const VAL_0x03: u32 = 0x3;
}

/// `TO1PIS_SELECT` value group
#[allow(non_upper_case_globals)]
pub mod to1pis_select {
   /// Port D2 Data Register.
   pub const VAL_0x00: u32 = 0x0;
   /// M1 - Toggle Register Timer1.
   pub const VAL_0x01: u32 = 0x1;
   /// M2 - Toggle Register Timer2.
   pub const VAL_0x02: u32 = 0x2;
   /// M3 - Toggle Register Timer3.
   pub const VAL_0x03: u32 = 0x3;
}

/// `TO2PIS_SELECT` value group
#[allow(non_upper_case_globals)]
pub mod to2pis_select {
   /// Port D3 Data Register.
   pub const VAL_0x00: u32 = 0x0;
   /// M1 - Toggle Register Timer1.
   pub const VAL_0x01: u32 = 0x1;
   /// M2 - Toggle Register Timer2.
   pub const VAL_0x02: u32 = 0x2;
   /// M4 - Toggle Register Timer4.
   pub const VAL_0x03: u32 = 0x3;
}

/// `TO3PIS_SELECT` value group
#[allow(non_upper_case_globals)]
pub mod to3pis_select {
   /// Port D4 Data Register.
   pub const VAL_0x00: u32 = 0x0;
   /// M1 - Toggle Register Timer1.
   pub const VAL_0x01: u32 = 0x1;
   /// M3 - Toggle Register Timer3.
   pub const VAL_0x02: u32 = 0x2;
   /// M4 - Toggle Register Timer4.
   pub const VAL_0x03: u32 = 0x3;
}

/// `TO4PIS_SELECT` value group
#[allow(non_upper_case_globals)]
pub mod to4pis_select {
   /// Port D5 Data Register.
   pub const VAL_0x00: u32 = 0x0;
   /// M1 - Toggle Register Timer1.
   pub const VAL_0x01: u32 = 0x1;
   /// M2 - Toggle Register Timer2.
   pub const VAL_0x02: u32 = 0x2;
   /// M3 - Toggle Register Timer3.
   pub const VAL_0x03: u32 = 0x3;
}

/// `TXM_CRC_SELECT` value group
#[allow(non_upper_case_globals)]
pub mod txm_crc_select {
   /// CRC 4-bit.
   pub const VAL_0x00: u32 = 0x0;
   /// CRC 8-bit.
   pub const VAL_0x01: u32 = 0x1;
   /// CRC 16-bit.
   pub const VAL_0x03: u32 = 0x3;
}

/// `TXM_PINTERFACE_SELECT` value group
#[allow(non_upper_case_globals)]
pub mod txm_pinterface_select {
   /// Port D1.
   pub const VAL_0x00: u32 = 0x0;
   /// M2 - Toggle Register Timer2.
   pub const VAL_0x01: u32 = 0x1;
   /// M3 - Toggle Register Timer3.
   pub const VAL_0x02: u32 = 0x2;
   /// M4 - Toggle Register Timer4.
   pub const VAL_0x03: u32 = 0x3;
   /// SO Tx Modulator Serial Output.
   pub const VAL_0x04: u32 = 0x4;
   /// M1 - Toggle Register Timer1.
   pub const VAL_0x05: u32 = 0x5;
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

