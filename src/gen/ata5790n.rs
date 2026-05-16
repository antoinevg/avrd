//! The AVR ATA5790N microcontroller
//!
//! # Variants
//! |        | Pinout | Mcu age | Operating temperature | Operating voltage | Max speed |
//! |--------|--------|---------|-----------------------|-------------------|-----------|
//! | standard |  |  | 0°C - 0°C | 1.9V - 4.2V | 0 MHz |
//!

#![allow(non_upper_case_globals)]

/// `LOCKBIT` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BLB0 | 1100 |
/// | LB | 11 |
/// | BLB1 | 110000 |
pub const LOCKBIT: *mut u8 = 0x0 as *mut u8;

/// `LOW` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CKDIV8 | 10000000 |
/// | EESAVE | 1000 |
/// | DWEN | 1000000 |
/// | _32OEN | 10 |
/// | SPIEN | 100000 |
/// | Reserved | 100 |
/// | WDTON | 10000 |
pub const LOW: *mut u8 = 0x0 as *mut u8;

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

/// Timer3 Control Register 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T3GRES | 1 |
/// | T3C2TM | 10 |
/// | T3C2RM | 100 |
pub const T3CR2: *mut u8 = 0x2C as *mut u8;

/// Transponder Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TPPSD | 1000000 |
/// | TPD | 10000000 |
/// | TPMOD | 10 |
/// | TPMD | 110000 |
/// | TPMA | 1 |
/// | TPMS | 1100 |
pub const TPCR: *mut u8 = 0x2D as *mut u8;

/// Transponder Status & Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TPPSW | 1000 |
/// | TPF | 1 |
/// | TPA | 10 |
/// | TPGAP | 100 |
pub const TPFR: *mut u8 = 0x2E as *mut u8;

/// Clock Management Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ECINS | 100000 |
/// | CMCCE | 10000000 |
/// | SRCD | 100 |
/// | CO32D | 1000 |
/// | CMONEN | 1000000 |
/// | CCS | 10000 |
/// | CMM | 11 |
pub const CMCR: *mut u8 = 0x2F as *mut u8;

/// Clock Management Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RTCF | 100 |
/// | ECF | 1 |
/// | SXF | 10 |
pub const CMSR: *mut u8 = 0x30 as *mut u8;

/// Timer 2 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T2RES | 100000 |
/// | T2OTM | 1 |
/// | T2TOP | 10000 |
/// | T2E | 10000000 |
/// | T2CRM | 100 |
/// | T2CTM | 10 |
/// | T2TS | 1000000 |
/// | T2GRM | 1000 |
pub const T2CR: *mut u8 = 0x31 as *mut u8;

/// Timer 3 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T3E | 10000000 |
/// | T3CTM | 10 |
/// | T3RES | 100000 |
/// | T3CPRM | 1000 |
/// | T3CRM | 100 |
/// | T3CPTM | 1000000 |
/// | T3OTM | 1 |
/// | T3TOP | 10000 |
pub const T3CR: *mut u8 = 0x32 as *mut u8;

/// AES Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AESWD | 10 |
/// | AESIM | 100 |
/// | AESD | 1000 |
/// | AESE | 10000000 |
/// | AESWK | 1 |
/// | AESRES | 100000 |
/// | AESXOR | 10000 |
pub const AESCR: *mut u8 = 0x33 as *mut u8;

/// AES Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AESRF | 1 |
/// | AESERF | 10000000 |
pub const AESSR: *mut u8 = 0x34 as *mut u8;

/// Timer Modulator Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TMTXF | 10 |
/// | TMRXS | 1000 |
/// | TMTCF | 100 |
/// | TMRXF | 1 |
/// | TMTXS | 10000 |
pub const TMIFR: *mut u8 = 0x35 as *mut u8;

/// Voltage Monitor Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | VMF | 1 |
pub const VMSR: *mut u8 = 0x36 as *mut u8;

/// Pin Change Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCIF | 11 |
pub const PCIFR: *mut u8 = 0x37 as *mut u8;

/// LF Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LFCAF | 10000000 |
/// | LFDBF | 1000 |
/// | LFRSF | 10000 |
/// | LFMDF | 1000000 |
/// | LFSDF | 100000 |
/// | LFID1F | 10 |
/// | LFID0F | 1 |
/// | LFFEF | 100 |
pub const LFFR: *mut u8 = 0x38 as *mut u8;

/// Timer0 Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T0F | 1 |
pub const T0IFR: *mut u8 = 0x39 as *mut u8;

/// Timer1 Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T1F | 1 |
pub const T1IFR: *mut u8 = 0x3A as *mut u8;

/// Timer2 Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T2COF | 10 |
/// | T2OFF | 1 |
pub const T2IFR: *mut u8 = 0x3B as *mut u8;

/// Timer3 Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T3CO2F | 1000 |
/// | T3COF | 10 |
/// | T3ICF | 100 |
/// | T3OFF | 1 |
pub const T3IFR: *mut u8 = 0x3C as *mut u8;

/// External Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | INTF0 | 1 |
pub const EIFR: *mut u8 = 0x3D as *mut u8;

/// General Purpose I/O Register 0.
pub const GPIOR: *mut u8 = 0x3E as *mut u8;

/// EEPROM Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | NVMBSY | 10000000 |
/// | EELP | 1000000 |
/// | EEMWE | 100 |
/// | EEWE | 10 |
/// | EERE | 1 |
/// | EERIE | 1000 |
/// | EEPM | 110000 |
pub const EECR: *mut u8 = 0x3F as *mut u8;

/// EEPROM Data Register.
pub const EEDR: *mut u8 = 0x40 as *mut u8;

/// EEPROM Address Register  Bytes.
pub const EEAR: *mut u16 = 0x41 as *mut u16;

/// EEPROM Address Register  Bytes low byte.
pub const EEARL: *mut u8 = 0x41 as *mut u8;

/// EEPROM Address Register  Bytes high byte.
pub const EEARH: *mut u8 = 0x42 as *mut u8;

/// EEPROM Protect Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EEAP | 1111 |
pub const EEPR: *mut u8 = 0x43 as *mut u8;

/// EEPROM Error Correction Code Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EEL | 1111 |
pub const EECCR: *mut u8 = 0x44 as *mut u8;

/// EEPROM Control Register 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EEBRE | 1 |
/// | EEPAGE | 10 |
pub const EECR2: *mut u8 = 0x45 as *mut u8;

/// Pin Change Interrupt Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCIE | 11 |
pub const PCICR: *mut u8 = 0x46 as *mut u8;

/// External Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | INT0 | 1 |
pub const EIMSK: *mut u8 = 0x47 as *mut u8;

/// Timer Modulator Data Register.
pub const TMDR: *mut u8 = 0x48 as *mut u8;

/// AES Data Register.
pub const AESDR: *mut u8 = 0x49 as *mut u8;

/// AES Key Register.
pub const AESKR: *mut u8 = 0x4A as *mut u8;

/// Voltage Monitor Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | VMIM | 10000 |
/// | VMLS | 1111 |
/// | BODLS | 10000000 |
/// | BODPD | 1000000 |
/// | VMPS | 100000 |
pub const VMCR: *mut u8 = 0x4B as *mut u8;

/// SPI Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CPOL | 1000 |
/// | SPR | 11 |
/// | SPIE | 10000000 |
/// | CPHA | 100 |
/// | DORD | 100000 |
/// | MSTR | 10000 |
/// | SPE | 1000000 |
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

/// Low Frequency Receiver Control Register 0.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LFBRS | 1000 |
/// | LFRBS | 10000 |
/// | LFCE | 111 |
/// | LFVC | 11000000 |
/// | LFMG | 100000 |
pub const LFCR0: *mut u8 = 0x4F as *mut u8;

/// LF receiver Control Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LFRMS | 10000 |
/// | LFQCE | 1000000 |
/// | LFRE | 10000000 |
/// | LFM | 11 |
/// | LFRMSA | 100000 |
/// | LFFM | 1100 |
pub const LFCR1: *mut u8 = 0x50 as *mut u8;

/// LF Receiver Data Buffer.
pub const LFRDB: *mut u8 = 0x52 as *mut u8;

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
/// | PORF | 1 |
/// | TPRF | 100000 |
/// | WDRF | 1000 |
/// | BORF | 100 |
/// | EXTRF | 10 |
pub const MCUSR: *mut u8 = 0x54 as *mut u8;

/// MCU Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | IVCE | 1 |
/// | PUD | 10000 |
/// | IVSEL | 10 |
pub const MCUCR: *mut u8 = 0x55 as *mut u8;

/// LF Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LFSD | 10 |
/// | LFES | 1 |
pub const LFSR: *mut u8 = 0x56 as *mut u8;

/// Store Program Memory Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RWWSRE | 10000 |
/// | SPMIE | 10000000 |
/// | PGWRT | 100 |
/// | PGERS | 10 |
/// | SIGRD | 100000 |
/// | SPMEN | 1 |
/// | BLBSET | 1000 |
/// | RWWSB | 1000000 |
pub const SPMCSR: *mut u8 = 0x57 as *mut u8;

/// Timer 1 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T1PS | 11 |
/// | T1CS | 11000 |
/// | T1IE | 100 |
/// | T1E | 10000000 |
pub const T1CR: *mut u8 = 0x58 as *mut u8;

/// Timer 0 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T0IE | 1000 |
/// | T0PS | 111 |
/// | T0PR | 10000 |
pub const T0CR: *mut u8 = 0x59 as *mut u8;

/// Clock Management Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RTCIE | 100 |
/// | SXIE | 10 |
/// | ECIE | 1 |
pub const CMIMR: *mut u8 = 0x5B as *mut u8;

/// Clock Prescaler Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CLKPS | 111 |
/// | CLKPCE | 10000000 |
/// | CLTPS | 111000 |
pub const CLKPR: *mut u8 = 0x5C as *mut u8;

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
/// | H | 100000 |
/// | T | 1000000 |
/// | S | 10000 |
/// | I | 10000000 |
/// | N | 100 |
/// | C | 1 |
/// | V | 1000 |
/// | Z | 10 |
pub const SREG: *mut u8 = 0x5F as *mut u8;

/// Watchdog Timer Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDE | 1000 |
/// | WDPS | 111 |
/// | WDCE | 10000 |
pub const WDTCR: *mut u8 = 0x60 as *mut u8;

/// Power Reduction Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRDS | 1000000 |
/// | PRTM | 10000 |
/// | PRLFR | 1 |
/// | PRVM | 10000000 |
/// | PRT2 | 100 |
/// | PRCU | 100000 |
/// | PRT1 | 10 |
/// | PRT3 | 1000 |
pub const PRR0: *mut u8 = 0x63 as *mut u8;

/// Power Reduction Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRSPI | 10 |
/// | PRCI | 1 |
pub const PRR1: *mut u8 = 0x64 as *mut u8;

/// SRC-Oscillator Calibration Register.
pub const SRCCAL: *mut u8 = 0x65 as *mut u8;

/// FRC-Oscillator Calibration Register.
pub const FRCCAL: *mut u8 = 0x66 as *mut u8;

/// External Interrupt Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ISC00 | 1 |
/// | ISC01 | 10 |
pub const EICRA: *mut u8 = 0x69 as *mut u8;

/// Pin Change Mask Register 0.
pub const PCMSK0: *mut u8 = 0x6A as *mut u8;

/// Pin Change Mask Register 1.
pub const PCMSK1: *mut u8 = 0x6B as *mut u8;

/// LED Driver Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LDE | 1 |
/// | LDCS | 110 |
pub const LDCR: *mut u8 = 0x6D as *mut u8;

/// Timer 2 Counter Register.
pub const T2CNT: *mut u8 = 0x70 as *mut u8;

/// Timer2 Compare Register.
pub const T2COR: *mut u8 = 0x71 as *mut u8;

/// Timer 2 Mode Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T2PS | 111000 |
/// | T2CS | 111 |
/// | T2D | 11000000 |
pub const T2MR: *mut u8 = 0x73 as *mut u8;

/// Timer 2 Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T2OIM | 1 |
/// | T2CIM | 10 |
pub const T2IMR: *mut u8 = 0x74 as *mut u8;

/// Timer3 COmpare2 Register.
pub const T3CO2R: *mut u8 = 0x75 as *mut u8;

/// Timer3 Counter Register.
pub const T3CNT: *mut u8 = 0x76 as *mut u8;

/// Timer3 COmpare Register.
pub const T3COR: *mut u8 = 0x77 as *mut u8;

/// Timer3 Input Capture Register.
pub const T3ICR: *mut u8 = 0x78 as *mut u8;

/// Timer 3 Mode Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T3CE | 11000 |
/// | T3SCE | 100 |
/// | T3ICS | 11000000 |
/// | T3CS | 11 |
/// | T3CNC | 100000 |
pub const T3MRA: *mut u8 = 0x79 as *mut u8;

/// Timer 3 Mode Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T3PS | 111 |
pub const T3MRB: *mut u8 = 0x7A as *mut u8;

/// Timer3 Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T3CPIM | 100 |
/// | T3OIM | 1 |
/// | T3C2IM | 1000 |
/// | T3CIM | 10 |
pub const T3IMR: *mut u8 = 0x7B as *mut u8;

/// Timer Modulator Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | MI1S | 11 |
/// | MI2S | 1100 |
/// | TMSSIE | 10000000 |
/// | TMCPOL | 1000000 |
/// | MI4S | 110000 |
pub const TMCR: *mut u8 = 0x7D as *mut u8;

/// Timer Modulator Mode Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | MOUTC | 10000 |
/// | MSCS | 1100 |
/// | TM12S | 10000000 |
/// | TMMS | 1100000 |
/// | MOS | 11 |
pub const TMMR: *mut u8 = 0x7E as *mut u8;

/// Timer Modulator Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TMTCIM | 100 |
/// | TMTXIM | 10 |
/// | TMRXIM | 1 |
pub const TMIMR: *mut u8 = 0x7F as *mut u8;

/// LF Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LFDBIM | 1000 |
/// | LFID0IM | 1 |
/// | LFSDIM | 100000 |
/// | LFID1IM | 10 |
/// | LFRSIM | 10000 |
/// | LFFEIM | 100 |
/// | LFMDIM | 1000000 |
pub const LFIMR: *mut u8 = 0x82 as *mut u8;

/// LF Clock Adjustment Data Register.
pub const LFCAD: *mut u8 = 0x83 as *mut u8;

/// LF ID 0 Data Register Byte 0.
pub const LFID00: *mut u8 = 0x84 as *mut u8;

/// LF ID 0 Data Register Byte 1.
pub const LFID01: *mut u8 = 0x85 as *mut u8;

/// LF ID 0 Data Register Byte 2.
pub const LFID02: *mut u8 = 0x86 as *mut u8;

/// LF ID 0 Data Register Byte 3.
pub const LFID03: *mut u8 = 0x87 as *mut u8;

/// LF ID 1 Data Register Byte 0.
pub const LFID10: *mut u8 = 0x88 as *mut u8;

/// LF ID 1 Data Register Byte 1.
pub const LFID11: *mut u8 = 0x89 as *mut u8;

/// LF ID 1 Data Register Byte 2.
pub const LFID12: *mut u8 = 0x8A as *mut u8;

/// LF ID 1 Data Register Byte 3.
pub const LFID13: *mut u8 = 0x8B as *mut u8;

/// LF Receive Data Register Byte 0.
pub const LFRD0: *mut u8 = 0x8C as *mut u8;

/// LF Receive Data Register Byte 1.
pub const LFRD1: *mut u8 = 0x8D as *mut u8;

/// LF Receive Data Register Byte 2.
pub const LFRD2: *mut u8 = 0x8E as *mut u8;

/// LF Receive Data Register Byte 3.
pub const LFRD3: *mut u8 = 0x8F as *mut u8;

/// LF Identifier 0 Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ID0FS | 11111 |
/// | ID0E | 10000000 |
pub const LFID0M: *mut u8 = 0x90 as *mut u8;

/// LF Identifier 1 Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ID1E | 10000000 |
/// | ID1FS | 11111 |
pub const LFID1M: *mut u8 = 0x91 as *mut u8;

/// LF Receive Data Frame Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RDFE | 10000000 |
/// | RDFS | 11111 |
pub const LFRDF: *mut u8 = 0x92 as *mut u8;

/// LF RSSI Data Register 1.
pub const LFRSD1: *mut u8 = 0x93 as *mut u8;

/// LF RSSI Data Register 2.
pub const LFRSD2: *mut u8 = 0x94 as *mut u8;

/// LF RSSI Data Register 3.
pub const LFRSD3: *mut u8 = 0x95 as *mut u8;

/// Low Frequency Channel Capacity select register 1.
pub const LFCC1: *mut u8 = 0x96 as *mut u8;

/// Low Frequency Channel Capacity select register 2.
pub const LFCC2: *mut u8 = 0x97 as *mut u8;

/// Low Frequency Channel Capacity select register 3.
pub const LFCC3: *mut u8 = 0x98 as *mut u8;

/// Low Frequency Receiver Quality Controll Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LFQCLL | 1 |
pub const LFQCR: *mut u8 = 0x99 as *mut u8;

/// Transponder Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TPIM | 1 |
pub const TPIMR: *mut u8 = 0x9C as *mut u8;

/// Real-Time Clock Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RTCR | 1 |
pub const RTCCR: *mut u8 = 0x9E as *mut u8;

/// Real Time Clock Data Register.
pub const RTCDR: *mut u8 = 0x9F as *mut u8;

/// Timer Modulator Manchester Data Register.
pub const TMMDR: *mut u8 = 0xA8 as *mut u8;

/// Timer Modulator Biphase Data Register.
pub const TMBDR: *mut u8 = 0xA9 as *mut u8;

/// Timer Modulator Transmit Data Register.
pub const TMTDR: *mut u8 = 0xAA as *mut u8;

/// Timer Modulator Shift Register.
pub const TMSR: *mut u8 = 0xAB as *mut u8;

/// CRC Polynomial Data Register.
pub const CRCPOL: *mut u8 = 0xAC as *mut u8;

/// CRC Data Register.
pub const CRCDR: *mut u8 = 0xAD as *mut u8;

/// CRC Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CRCN | 111 |
/// | STVAL | 1000000 |
/// | REFLO | 100000 |
/// | REFLI | 10000 |
/// | CRCSEL | 1000 |
/// | CRCRS | 10000000 |
pub const CRCCR: *mut u8 = 0xAE as *mut u8;

/// CRC Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CRCBF | 1 |
pub const CRCSR: *mut u8 = 0xAF as *mut u8;

/// Bitfield on register `AESCR`
pub const AESWD: u8 = 0x2;

/// Bitfield on register `AESCR`
pub const AESIM: u8 = 0x4;

/// Bitfield on register `AESCR`
pub const AESD: u8 = 0x8;

/// Bitfield on register `AESCR`
pub const AESE: u8 = 0x80;

/// Bitfield on register `AESCR`
pub const AESWK: u8 = 0x1;

/// Bitfield on register `AESCR`
pub const AESRES: u8 = 0x20;

/// Bitfield on register `AESCR`
pub const AESXOR: u8 = 0x10;

/// Bitfield on register `AESSR`
pub const AESRF: u8 = 0x1;

/// Bitfield on register `AESSR`
pub const AESERF: u8 = 0x80;

/// Bitfield on register `CLKPR`
pub const CLKPS: u8 = 0x7;

/// Bitfield on register `CLKPR`
pub const CLKPCE: u8 = 0x80;

/// Bitfield on register `CLKPR`
pub const CLTPS: u8 = 0x38;

/// Bitfield on register `CMCR`
pub const ECINS: u8 = 0x20;

/// Bitfield on register `CMCR`
pub const CMCCE: u8 = 0x80;

/// Bitfield on register `CMCR`
pub const SRCD: u8 = 0x4;

/// Bitfield on register `CMCR`
pub const CO32D: u8 = 0x8;

/// Bitfield on register `CMCR`
pub const CMONEN: u8 = 0x40;

/// Bitfield on register `CMCR`
pub const CCS: u8 = 0x10;

/// Bitfield on register `CMCR`
pub const CMM: u8 = 0x3;

/// Bitfield on register `CMIMR`
pub const RTCIE: u8 = 0x4;

/// Bitfield on register `CMIMR`
pub const SXIE: u8 = 0x2;

/// Bitfield on register `CMIMR`
pub const ECIE: u8 = 0x1;

/// Bitfield on register `CMSR`
pub const RTCF: u8 = 0x4;

/// Bitfield on register `CMSR`
pub const ECF: u8 = 0x1;

/// Bitfield on register `CMSR`
pub const SXF: u8 = 0x2;

/// Bitfield on register `CRCCR`
pub const CRCN: u8 = 0x7;

/// Bitfield on register `CRCCR`
pub const STVAL: u8 = 0x40;

/// Bitfield on register `CRCCR`
pub const REFLO: u8 = 0x20;

/// Bitfield on register `CRCCR`
pub const REFLI: u8 = 0x10;

/// Bitfield on register `CRCCR`
pub const CRCSEL: u8 = 0x8;

/// Bitfield on register `CRCCR`
pub const CRCRS: u8 = 0x80;

/// Bitfield on register `CRCSR`
pub const CRCBF: u8 = 0x1;

/// Bitfield on register `EECCR`
pub const EEL: u8 = 0xF;

/// Bitfield on register `EECR`
pub const NVMBSY: u8 = 0x80;

/// Bitfield on register `EECR`
pub const EELP: u8 = 0x40;

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

/// Bitfield on register `EECR2`
pub const EEBRE: u8 = 0x1;

/// Bitfield on register `EECR2`
pub const EEPAGE: u8 = 0x2;

/// Bitfield on register `EEPR`
pub const EEAP: u8 = 0xF;

/// Bitfield on register `EICRA`
pub const ISC00: u8 = 0x1;

/// Bitfield on register `EICRA`
pub const ISC01: u8 = 0x2;

/// Bitfield on register `EIFR`
pub const INTF0: u8 = 0x1;

/// Bitfield on register `EIMSK`
pub const INT0: u8 = 0x1;

/// Bitfield on register `LDCR`
pub const LDE: u8 = 0x1;

/// Bitfield on register `LDCR`
pub const LDCS: u8 = 0x6;

/// Bitfield on register `LFCR0`
pub const LFBRS: u8 = 0x8;

/// Bitfield on register `LFCR0`
pub const LFRBS: u8 = 0x10;

/// Bitfield on register `LFCR0`
pub const LFCE: u8 = 0x7;

/// Bitfield on register `LFCR0`
pub const LFVC: u8 = 0xC0;

/// Bitfield on register `LFCR0`
pub const LFMG: u8 = 0x20;

/// Bitfield on register `LFCR1`
pub const LFRMS: u8 = 0x10;

/// Bitfield on register `LFCR1`
pub const LFQCE: u8 = 0x40;

/// Bitfield on register `LFCR1`
pub const LFRE: u8 = 0x80;

/// Bitfield on register `LFCR1`
pub const LFM: u8 = 0x3;

/// Bitfield on register `LFCR1`
pub const LFRMSA: u8 = 0x20;

/// Bitfield on register `LFCR1`
pub const LFFM: u8 = 0xC;

/// Bitfield on register `LFFR`
pub const LFCAF: u8 = 0x80;

/// Bitfield on register `LFFR`
pub const LFDBF: u8 = 0x8;

/// Bitfield on register `LFFR`
pub const LFRSF: u8 = 0x10;

/// Bitfield on register `LFFR`
pub const LFMDF: u8 = 0x40;

/// Bitfield on register `LFFR`
pub const LFSDF: u8 = 0x20;

/// Bitfield on register `LFFR`
pub const LFID1F: u8 = 0x2;

/// Bitfield on register `LFFR`
pub const LFID0F: u8 = 0x1;

/// Bitfield on register `LFFR`
pub const LFFEF: u8 = 0x4;

/// Bitfield on register `LFID0M`
pub const ID0FS: u8 = 0x1F;

/// Bitfield on register `LFID0M`
pub const ID0E: u8 = 0x80;

/// Bitfield on register `LFID1M`
pub const ID1E: u8 = 0x80;

/// Bitfield on register `LFID1M`
pub const ID1FS: u8 = 0x1F;

/// Bitfield on register `LFIMR`
pub const LFDBIM: u8 = 0x8;

/// Bitfield on register `LFIMR`
pub const LFID0IM: u8 = 0x1;

/// Bitfield on register `LFIMR`
pub const LFSDIM: u8 = 0x20;

/// Bitfield on register `LFIMR`
pub const LFID1IM: u8 = 0x2;

/// Bitfield on register `LFIMR`
pub const LFRSIM: u8 = 0x10;

/// Bitfield on register `LFIMR`
pub const LFFEIM: u8 = 0x4;

/// Bitfield on register `LFIMR`
pub const LFMDIM: u8 = 0x40;

/// Bitfield on register `LFQCR`
pub const LFQCLL: u8 = 0x1;

/// Bitfield on register `LFRDF`
pub const RDFE: u8 = 0x80;

/// Bitfield on register `LFRDF`
pub const RDFS: u8 = 0x1F;

/// Bitfield on register `LFSR`
pub const LFSD: u8 = 0x2;

/// Bitfield on register `LFSR`
pub const LFES: u8 = 0x1;

/// Bitfield on register `LOCKBIT`
pub const BLB0: u8 = 0xC;

/// Bitfield on register `LOCKBIT`
pub const LB: u8 = 0x3;

/// Bitfield on register `LOCKBIT`
pub const BLB1: u8 = 0x30;

/// Bitfield on register `LOW`
pub const CKDIV8: u8 = 0x80;

/// Bitfield on register `LOW`
pub const EESAVE: u8 = 0x8;

/// Bitfield on register `LOW`
pub const DWEN: u8 = 0x40;

/// Bitfield on register `LOW`
pub const _32OEN: u8 = 0x2;

/// Bitfield on register `LOW`
pub const SPIEN: u8 = 0x20;

/// Bitfield on register `LOW`
pub const Reserved: u8 = 0x4;

/// Bitfield on register `LOW`
pub const WDTON: u8 = 0x10;

/// Bitfield on register `MCUCR`
pub const IVCE: u8 = 0x1;

/// Bitfield on register `MCUCR`
pub const PUD: u8 = 0x10;

/// Bitfield on register `MCUCR`
pub const IVSEL: u8 = 0x2;

/// Bitfield on register `MCUSR`
pub const PORF: u8 = 0x1;

/// Bitfield on register `MCUSR`
pub const TPRF: u8 = 0x20;

/// Bitfield on register `MCUSR`
pub const WDRF: u8 = 0x8;

/// Bitfield on register `MCUSR`
pub const BORF: u8 = 0x4;

/// Bitfield on register `MCUSR`
pub const EXTRF: u8 = 0x2;

/// Bitfield on register `PCICR`
pub const PCIE: u8 = 0x3;

/// Bitfield on register `PCIFR`
pub const PCIF: u8 = 0x3;

/// Bitfield on register `PRR0`
pub const PRDS: u8 = 0x40;

/// Bitfield on register `PRR0`
pub const PRTM: u8 = 0x10;

/// Bitfield on register `PRR0`
pub const PRLFR: u8 = 0x1;

/// Bitfield on register `PRR0`
pub const PRVM: u8 = 0x80;

/// Bitfield on register `PRR0`
pub const PRT2: u8 = 0x4;

/// Bitfield on register `PRR0`
pub const PRCU: u8 = 0x20;

/// Bitfield on register `PRR0`
pub const PRT1: u8 = 0x2;

/// Bitfield on register `PRR0`
pub const PRT3: u8 = 0x8;

/// Bitfield on register `PRR1`
pub const PRSPI: u8 = 0x2;

/// Bitfield on register `PRR1`
pub const PRCI: u8 = 0x1;

/// Bitfield on register `RTCCR`
pub const RTCR: u8 = 0x1;

/// Bitfield on register `SMCR`
pub const SE: u8 = 0x1;

/// Bitfield on register `SMCR`
pub const SM: u8 = 0xE;

/// Bitfield on register `SPCR`
pub const CPOL: u8 = 0x8;

/// Bitfield on register `SPCR`
pub const SPR: u8 = 0x3;

/// Bitfield on register `SPCR`
pub const SPIE: u8 = 0x80;

/// Bitfield on register `SPCR`
pub const CPHA: u8 = 0x4;

/// Bitfield on register `SPCR`
pub const DORD: u8 = 0x20;

/// Bitfield on register `SPCR`
pub const MSTR: u8 = 0x10;

/// Bitfield on register `SPCR`
pub const SPE: u8 = 0x40;

/// Bitfield on register `SPMCSR`
pub const RWWSRE: u8 = 0x10;

/// Bitfield on register `SPMCSR`
pub const SPMIE: u8 = 0x80;

/// Bitfield on register `SPMCSR`
pub const PGWRT: u8 = 0x4;

/// Bitfield on register `SPMCSR`
pub const PGERS: u8 = 0x2;

/// Bitfield on register `SPMCSR`
pub const SIGRD: u8 = 0x20;

/// Bitfield on register `SPMCSR`
pub const SPMEN: u8 = 0x1;

/// Bitfield on register `SPMCSR`
pub const BLBSET: u8 = 0x8;

/// Bitfield on register `SPMCSR`
pub const RWWSB: u8 = 0x40;

/// Bitfield on register `SPSR`
pub const WCOL: u8 = 0x40;

/// Bitfield on register `SPSR`
pub const SPIF: u8 = 0x80;

/// Bitfield on register `SPSR`
pub const SPI2X: u8 = 0x1;

/// Bitfield on register `SREG`
pub const H: u8 = 0x20;

/// Bitfield on register `SREG`
pub const T: u8 = 0x40;

/// Bitfield on register `SREG`
pub const S: u8 = 0x10;

/// Bitfield on register `SREG`
pub const I: u8 = 0x80;

/// Bitfield on register `SREG`
pub const N: u8 = 0x4;

/// Bitfield on register `SREG`
pub const C: u8 = 0x1;

/// Bitfield on register `SREG`
pub const V: u8 = 0x8;

/// Bitfield on register `SREG`
pub const Z: u8 = 0x2;

/// Bitfield on register `T0CR`
pub const T0IE: u8 = 0x8;

/// Bitfield on register `T0CR`
pub const T0PS: u8 = 0x7;

/// Bitfield on register `T0CR`
pub const T0PR: u8 = 0x10;

/// Bitfield on register `T0IFR`
pub const T0F: u8 = 0x1;

/// Bitfield on register `T1CR`
pub const T1PS: u8 = 0x3;

/// Bitfield on register `T1CR`
pub const T1CS: u8 = 0x18;

/// Bitfield on register `T1CR`
pub const T1IE: u8 = 0x4;

/// Bitfield on register `T1CR`
pub const T1E: u8 = 0x80;

/// Bitfield on register `T1IFR`
pub const T1F: u8 = 0x1;

/// Bitfield on register `T2CR`
pub const T2RES: u8 = 0x20;

/// Bitfield on register `T2CR`
pub const T2OTM: u8 = 0x1;

/// Bitfield on register `T2CR`
pub const T2TOP: u8 = 0x10;

/// Bitfield on register `T2CR`
pub const T2E: u8 = 0x80;

/// Bitfield on register `T2CR`
pub const T2CRM: u8 = 0x4;

/// Bitfield on register `T2CR`
pub const T2CTM: u8 = 0x2;

/// Bitfield on register `T2CR`
pub const T2TS: u8 = 0x40;

/// Bitfield on register `T2CR`
pub const T2GRM: u8 = 0x8;

/// Bitfield on register `T2IFR`
pub const T2COF: u8 = 0x2;

/// Bitfield on register `T2IFR`
pub const T2OFF: u8 = 0x1;

/// Bitfield on register `T2IMR`
pub const T2OIM: u8 = 0x1;

/// Bitfield on register `T2IMR`
pub const T2CIM: u8 = 0x2;

/// Bitfield on register `T2MR`
pub const T2PS: u8 = 0x38;

/// Bitfield on register `T2MR`
pub const T2CS: u8 = 0x7;

/// Bitfield on register `T2MR`
pub const T2D: u8 = 0xC0;

/// Bitfield on register `T3CR`
pub const T3E: u8 = 0x80;

/// Bitfield on register `T3CR`
pub const T3CTM: u8 = 0x2;

/// Bitfield on register `T3CR`
pub const T3RES: u8 = 0x20;

/// Bitfield on register `T3CR`
pub const T3CPRM: u8 = 0x8;

/// Bitfield on register `T3CR`
pub const T3CRM: u8 = 0x4;

/// Bitfield on register `T3CR`
pub const T3CPTM: u8 = 0x40;

/// Bitfield on register `T3CR`
pub const T3OTM: u8 = 0x1;

/// Bitfield on register `T3CR`
pub const T3TOP: u8 = 0x10;

/// Bitfield on register `T3CR2`
pub const T3GRES: u8 = 0x1;

/// Bitfield on register `T3CR2`
pub const T3C2TM: u8 = 0x2;

/// Bitfield on register `T3CR2`
pub const T3C2RM: u8 = 0x4;

/// Bitfield on register `T3IFR`
pub const T3CO2F: u8 = 0x8;

/// Bitfield on register `T3IFR`
pub const T3COF: u8 = 0x2;

/// Bitfield on register `T3IFR`
pub const T3ICF: u8 = 0x4;

/// Bitfield on register `T3IFR`
pub const T3OFF: u8 = 0x1;

/// Bitfield on register `T3IMR`
pub const T3CPIM: u8 = 0x4;

/// Bitfield on register `T3IMR`
pub const T3OIM: u8 = 0x1;

/// Bitfield on register `T3IMR`
pub const T3C2IM: u8 = 0x8;

/// Bitfield on register `T3IMR`
pub const T3CIM: u8 = 0x2;

/// Bitfield on register `T3MRA`
pub const T3CE: u8 = 0x18;

/// Bitfield on register `T3MRA`
pub const T3SCE: u8 = 0x4;

/// Bitfield on register `T3MRA`
pub const T3ICS: u8 = 0xC0;

/// Bitfield on register `T3MRA`
pub const T3CS: u8 = 0x3;

/// Bitfield on register `T3MRA`
pub const T3CNC: u8 = 0x20;

/// Bitfield on register `T3MRB`
pub const T3PS: u8 = 0x7;

/// Bitfield on register `TMCR`
pub const MI1S: u8 = 0x3;

/// Bitfield on register `TMCR`
pub const MI2S: u8 = 0xC;

/// Bitfield on register `TMCR`
pub const TMSSIE: u8 = 0x80;

/// Bitfield on register `TMCR`
pub const TMCPOL: u8 = 0x40;

/// Bitfield on register `TMCR`
pub const MI4S: u8 = 0x30;

/// Bitfield on register `TMIFR`
pub const TMTXF: u8 = 0x2;

/// Bitfield on register `TMIFR`
pub const TMRXS: u8 = 0x8;

/// Bitfield on register `TMIFR`
pub const TMTCF: u8 = 0x4;

/// Bitfield on register `TMIFR`
pub const TMRXF: u8 = 0x1;

/// Bitfield on register `TMIFR`
pub const TMTXS: u8 = 0x10;

/// Bitfield on register `TMIMR`
pub const TMTCIM: u8 = 0x4;

/// Bitfield on register `TMIMR`
pub const TMTXIM: u8 = 0x2;

/// Bitfield on register `TMIMR`
pub const TMRXIM: u8 = 0x1;

/// Bitfield on register `TMMR`
pub const MOUTC: u8 = 0x10;

/// Bitfield on register `TMMR`
pub const MSCS: u8 = 0xC;

/// Bitfield on register `TMMR`
pub const TM12S: u8 = 0x80;

/// Bitfield on register `TMMR`
pub const TMMS: u8 = 0x60;

/// Bitfield on register `TMMR`
pub const MOS: u8 = 0x3;

/// Bitfield on register `TPCR`
pub const TPPSD: u8 = 0x40;

/// Bitfield on register `TPCR`
pub const TPD: u8 = 0x80;

/// Bitfield on register `TPCR`
pub const TPMOD: u8 = 0x2;

/// Bitfield on register `TPCR`
pub const TPMD: u8 = 0x30;

/// Bitfield on register `TPCR`
pub const TPMA: u8 = 0x1;

/// Bitfield on register `TPCR`
pub const TPMS: u8 = 0xC;

/// Bitfield on register `TPFR`
pub const TPPSW: u8 = 0x8;

/// Bitfield on register `TPFR`
pub const TPF: u8 = 0x1;

/// Bitfield on register `TPFR`
pub const TPA: u8 = 0x2;

/// Bitfield on register `TPFR`
pub const TPGAP: u8 = 0x4;

/// Bitfield on register `TPIMR`
pub const TPIM: u8 = 0x1;

/// Bitfield on register `VMCR`
pub const VMIM: u8 = 0x10;

/// Bitfield on register `VMCR`
pub const VMLS: u8 = 0xF;

/// Bitfield on register `VMCR`
pub const BODLS: u8 = 0x80;

/// Bitfield on register `VMCR`
pub const BODPD: u8 = 0x40;

/// Bitfield on register `VMCR`
pub const VMPS: u8 = 0x20;

/// Bitfield on register `VMSR`
pub const VMF: u8 = 0x1;

/// Bitfield on register `WDTCR`
pub const WDE: u8 = 0x8;

/// Bitfield on register `WDTCR`
pub const WDPS: u8 = 0x7;

/// Bitfield on register `WDTCR`
pub const WDCE: u8 = 0x10;

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

/// `ENUM_BLB` value group
#[allow(non_upper_case_globals)]
pub mod enum_blb {
   /// LPM and SPM prohibited in Application Section.
   pub const VAL_0x00: u32 = 0x0;
   /// LPM prohibited in Application Section.
   pub const VAL_0x01: u32 = 0x1;
   /// SPM prohibited in Application Section.
   pub const VAL_0x02: u32 = 0x2;
   /// No lock on SPM and LPM in Application Section.
   pub const VAL_0x03: u32 = 0x3;
}

/// `ENUM_BLB2` value group
#[allow(non_upper_case_globals)]
pub mod enum_blb2 {
   /// LPM and SPM prohibited in Boot Section.
   pub const VAL_0x00: u32 = 0x0;
   /// LPM prohibited in Boot Section.
   pub const VAL_0x01: u32 = 0x1;
   /// SPM prohibited in Boot Section.
   pub const VAL_0x02: u32 = 0x2;
   /// No lock on SPM and LPM in Boot Section.
   pub const VAL_0x03: u32 = 0x3;
}

/// `ENUM_LB` value group
#[allow(non_upper_case_globals)]
pub mod enum_lb {
   /// Further programming and verification disabled.
   pub const VAL_0x00: u32 = 0x0;
   /// Further programming disabled.
   pub const VAL_0x02: u32 = 0x2;
   /// No memory lock features enabled.
   pub const VAL_0x03: u32 = 0x3;
}

