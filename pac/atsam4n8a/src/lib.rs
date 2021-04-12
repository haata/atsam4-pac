#![doc = "Peripheral access API for ATSAM4N8A microcontrollers (generated using svd2rust v0.17.0 (2bbb605 2020-05-16))\n\nYou can find an overview of the generated API [here].\n\nAPI features to be included in the [next]
svd2rust release can be generated by cloning the svd2rust [repository], checking out the above commit, and running `cargo doc --open`.\n\n[here]: https://docs.rs/svd2rust/0.17.0/svd2rust/#peripheral-api\n[next]: https://github.com/rust-embedded/svd2rust/blob/master/CHANGELOG.md#unreleased\n[repository]: https://github.com/rust-embedded/svd2rust"]
#![deny(const_err)]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(private_in_public)]
#![deny(unconditional_recursion)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 4;
#[cfg(feature = "rt")]
extern "C" {
    fn SUPC();
    fn RSTC();
    fn RTC();
    fn RTT();
    fn WDT();
    fn PMC();
    fn EFC();
    fn UART0();
    fn UART1();
    fn UART2();
    fn PIOA();
    fn PIOB();
    fn USART0();
    fn UART3();
    fn TWI0();
    fn TWI1();
    fn SPI();
    fn TWI2();
    fn TC0();
    fn TC1();
    fn TC2();
    fn ADC();
    fn PWM();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 32] = [
    Vector { _handler: SUPC },
    Vector { _handler: RSTC },
    Vector { _handler: RTC },
    Vector { _handler: RTT },
    Vector { _handler: WDT },
    Vector { _handler: PMC },
    Vector { _handler: EFC },
    Vector { _reserved: 0 },
    Vector { _handler: UART0 },
    Vector { _handler: UART1 },
    Vector { _handler: UART2 },
    Vector { _handler: PIOA },
    Vector { _handler: PIOB },
    Vector { _reserved: 0 },
    Vector { _handler: USART0 },
    Vector { _reserved: 0 },
    Vector { _handler: UART3 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: TWI0 },
    Vector { _handler: TWI1 },
    Vector { _handler: SPI },
    Vector { _handler: TWI2 },
    Vector { _handler: TC0 },
    Vector { _handler: TC1 },
    Vector { _handler: TC2 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: ADC },
    Vector { _reserved: 0 },
    Vector { _handler: PWM },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Interrupt {
    #[doc = "0 - SUPC"]
    SUPC = 0,
    #[doc = "1 - RSTC"]
    RSTC = 1,
    #[doc = "2 - RTC"]
    RTC = 2,
    #[doc = "3 - RTT"]
    RTT = 3,
    #[doc = "4 - WDT"]
    WDT = 4,
    #[doc = "5 - PMC"]
    PMC = 5,
    #[doc = "6 - EFC"]
    EFC = 6,
    #[doc = "8 - UART0"]
    UART0 = 8,
    #[doc = "9 - UART1"]
    UART1 = 9,
    #[doc = "10 - UART2"]
    UART2 = 10,
    #[doc = "11 - PIOA"]
    PIOA = 11,
    #[doc = "12 - PIOB"]
    PIOB = 12,
    #[doc = "14 - USART0"]
    USART0 = 14,
    #[doc = "16 - UART3"]
    UART3 = 16,
    #[doc = "19 - TWI0"]
    TWI0 = 19,
    #[doc = "20 - TWI1"]
    TWI1 = 20,
    #[doc = "21 - SPI"]
    SPI = 21,
    #[doc = "22 - TWI2"]
    TWI2 = 22,
    #[doc = "23 - TC0"]
    TC0 = 23,
    #[doc = "24 - TC1"]
    TC1 = 24,
    #[doc = "25 - TC2"]
    TC2 = 25,
    #[doc = "29 - ADC"]
    ADC = 29,
    #[doc = "31 - PWM"]
    PWM = 31,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline(always)]
    fn nr(&self) -> u8 {
        *self as u8
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[doc = "Serial Peripheral Interface"]
pub struct SPI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI {}
impl SPI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi::RegisterBlock {
        0x4000_8000 as *const _
    }
}
impl Deref for SPI {
    type Target = spi::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI::ptr() }
    }
}
#[doc = "Serial Peripheral Interface"]
pub mod spi;
#[doc = "Timer Counter 0"]
pub struct TC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TC0 {}
impl TC0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tc0::RegisterBlock {
        0x4001_0000 as *const _
    }
}
impl Deref for TC0 {
    type Target = tc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TC0::ptr() }
    }
}
#[doc = "Timer Counter 0"]
pub mod tc0;
#[doc = "Two-wire Interface 0"]
pub struct TWI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWI0 {}
impl TWI0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const twi0::RegisterBlock {
        0x4001_8000 as *const _
    }
}
impl Deref for TWI0 {
    type Target = twi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TWI0::ptr() }
    }
}
#[doc = "Two-wire Interface 0"]
pub mod twi0;
#[doc = "Two-wire Interface 1"]
pub struct TWI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWI1 {}
impl TWI1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const twi1::RegisterBlock {
        0x4001_c000 as *const _
    }
}
impl Deref for TWI1 {
    type Target = twi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TWI1::ptr() }
    }
}
#[doc = "Two-wire Interface 1"]
pub mod twi1;
#[doc = "Pulse Width Modulation Controller"]
pub struct PWM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM {}
impl PWM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pwm::RegisterBlock {
        0x4002_0000 as *const _
    }
}
impl Deref for PWM {
    type Target = pwm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PWM::ptr() }
    }
}
#[doc = "Pulse Width Modulation Controller"]
pub mod pwm;
#[doc = "Universal Synchronous Asynchronous Receiver Transmitter 0"]
pub struct USART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART0 {}
impl USART0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart0::RegisterBlock {
        0x4002_4000 as *const _
    }
}
impl Deref for USART0 {
    type Target = usart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART0::ptr() }
    }
}
#[doc = "Universal Synchronous Asynchronous Receiver Transmitter 0"]
pub mod usart0;
#[doc = "Analog-to-Digital Converter"]
pub struct ADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC {}
impl ADC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc::RegisterBlock {
        0x4003_8000 as *const _
    }
}
impl Deref for ADC {
    type Target = adc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC::ptr() }
    }
}
#[doc = "Analog-to-Digital Converter"]
pub mod adc;
#[doc = "Two-wire Interface 2"]
pub struct TWI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWI2 {}
impl TWI2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const twi2::RegisterBlock {
        0x4004_0000 as *const _
    }
}
impl Deref for TWI2 {
    type Target = twi2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TWI2::ptr() }
    }
}
#[doc = "Two-wire Interface 2"]
pub mod twi2;
#[doc = "Universal Asynchronous Receiver Transmitter 2"]
pub struct UART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART2 {}
impl UART2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart2::RegisterBlock {
        0x4004_4000 as *const _
    }
}
impl Deref for UART2 {
    type Target = uart2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART2::ptr() }
    }
}
#[doc = "Universal Asynchronous Receiver Transmitter 2"]
pub mod uart2;
#[doc = "Universal Asynchronous Receiver Transmitter 3"]
pub struct UART3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART3 {}
impl UART3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart3::RegisterBlock {
        0x4004_8000 as *const _
    }
}
impl Deref for UART3 {
    type Target = uart3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART3::ptr() }
    }
}
#[doc = "Universal Asynchronous Receiver Transmitter 3"]
pub mod uart3;
#[doc = "AHB Bus Matrix"]
pub struct MATRIX {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MATRIX {}
impl MATRIX {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const matrix::RegisterBlock {
        0x400e_0200 as *const _
    }
}
impl Deref for MATRIX {
    type Target = matrix::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*MATRIX::ptr() }
    }
}
#[doc = "AHB Bus Matrix"]
pub mod matrix;
#[doc = "Power Management Controller"]
pub struct PMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PMC {}
impl PMC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pmc::RegisterBlock {
        0x400e_0400 as *const _
    }
}
impl Deref for PMC {
    type Target = pmc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PMC::ptr() }
    }
}
#[doc = "Power Management Controller"]
pub mod pmc;
#[doc = "Universal Asynchronous Receiver Transmitter 0"]
pub struct UART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART0 {}
impl UART0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        0x400e_0600 as *const _
    }
}
impl Deref for UART0 {
    type Target = uart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART0::ptr() }
    }
}
#[doc = "Universal Asynchronous Receiver Transmitter 0"]
pub mod uart0;
#[doc = "Chip Identifier"]
pub struct CHIPID {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CHIPID {}
impl CHIPID {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const chipid::RegisterBlock {
        0x400e_0740 as *const _
    }
}
impl Deref for CHIPID {
    type Target = chipid::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CHIPID::ptr() }
    }
}
#[doc = "Chip Identifier"]
pub mod chipid;
#[doc = "Universal Asynchronous Receiver Transmitter 1"]
pub struct UART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART1 {}
impl UART1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart1::RegisterBlock {
        0x400e_0800 as *const _
    }
}
impl Deref for UART1 {
    type Target = uart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART1::ptr() }
    }
}
#[doc = "Universal Asynchronous Receiver Transmitter 1"]
pub mod uart1;
#[doc = "Embedded Flash Controller"]
pub struct EFC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EFC {}
impl EFC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const efc::RegisterBlock {
        0x400e_0a00 as *const _
    }
}
impl Deref for EFC {
    type Target = efc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EFC::ptr() }
    }
}
#[doc = "Embedded Flash Controller"]
pub mod efc;
#[doc = "Parallel Input/Output Controller A"]
pub struct PIOA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PIOA {}
impl PIOA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pioa::RegisterBlock {
        0x400e_0e00 as *const _
    }
}
impl Deref for PIOA {
    type Target = pioa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PIOA::ptr() }
    }
}
#[doc = "Parallel Input/Output Controller A"]
pub mod pioa;
#[doc = "Parallel Input/Output Controller B"]
pub struct PIOB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PIOB {}
impl PIOB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const piob::RegisterBlock {
        0x400e_1000 as *const _
    }
}
impl Deref for PIOB {
    type Target = piob::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PIOB::ptr() }
    }
}
#[doc = "Parallel Input/Output Controller B"]
pub mod piob;
#[doc = "Reset Controller"]
pub struct RSTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RSTC {}
impl RSTC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rstc::RegisterBlock {
        0x400e_1400 as *const _
    }
}
impl Deref for RSTC {
    type Target = rstc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RSTC::ptr() }
    }
}
#[doc = "Reset Controller"]
pub mod rstc;
#[doc = "Supply Controller"]
pub struct SUPC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SUPC {}
impl SUPC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const supc::RegisterBlock {
        0x400e_1410 as *const _
    }
}
impl Deref for SUPC {
    type Target = supc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SUPC::ptr() }
    }
}
#[doc = "Supply Controller"]
pub mod supc;
#[doc = "Real-time Timer"]
pub struct RTT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTT {}
impl RTT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtt::RegisterBlock {
        0x400e_1430 as *const _
    }
}
impl Deref for RTT {
    type Target = rtt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTT::ptr() }
    }
}
#[doc = "Real-time Timer"]
pub mod rtt;
#[doc = "Watchdog Timer"]
pub struct WDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT {}
impl WDT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wdt::RegisterBlock {
        0x400e_1450 as *const _
    }
}
impl Deref for WDT {
    type Target = wdt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*WDT::ptr() }
    }
}
#[doc = "Watchdog Timer"]
pub mod wdt;
#[doc = "Real-time Clock"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc::RegisterBlock {
        0x400e_1460 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "Real-time Clock"]
pub mod rtc;
#[doc = "General Purpose Backup Registers"]
pub struct GPBR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPBR {}
impl GPBR {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpbr::RegisterBlock {
        0x400e_1490 as *const _
    }
}
impl Deref for GPBR {
    type Target = gpbr::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPBR::ptr() }
    }
}
#[doc = "General Purpose Backup Registers"]
pub mod gpbr;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "SPI"]
    pub SPI: SPI,
    #[doc = "TC0"]
    pub TC0: TC0,
    #[doc = "TWI0"]
    pub TWI0: TWI0,
    #[doc = "TWI1"]
    pub TWI1: TWI1,
    #[doc = "PWM"]
    pub PWM: PWM,
    #[doc = "USART0"]
    pub USART0: USART0,
    #[doc = "ADC"]
    pub ADC: ADC,
    #[doc = "TWI2"]
    pub TWI2: TWI2,
    #[doc = "UART2"]
    pub UART2: UART2,
    #[doc = "UART3"]
    pub UART3: UART3,
    #[doc = "MATRIX"]
    pub MATRIX: MATRIX,
    #[doc = "PMC"]
    pub PMC: PMC,
    #[doc = "UART0"]
    pub UART0: UART0,
    #[doc = "CHIPID"]
    pub CHIPID: CHIPID,
    #[doc = "UART1"]
    pub UART1: UART1,
    #[doc = "EFC"]
    pub EFC: EFC,
    #[doc = "PIOA"]
    pub PIOA: PIOA,
    #[doc = "PIOB"]
    pub PIOB: PIOB,
    #[doc = "RSTC"]
    pub RSTC: RSTC,
    #[doc = "SUPC"]
    pub SUPC: SUPC,
    #[doc = "RTT"]
    pub RTT: RTT,
    #[doc = "WDT"]
    pub WDT: WDT,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "GPBR"]
    pub GPBR: GPBR,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            SPI: SPI {
                _marker: PhantomData,
            },
            TC0: TC0 {
                _marker: PhantomData,
            },
            TWI0: TWI0 {
                _marker: PhantomData,
            },
            TWI1: TWI1 {
                _marker: PhantomData,
            },
            PWM: PWM {
                _marker: PhantomData,
            },
            USART0: USART0 {
                _marker: PhantomData,
            },
            ADC: ADC {
                _marker: PhantomData,
            },
            TWI2: TWI2 {
                _marker: PhantomData,
            },
            UART2: UART2 {
                _marker: PhantomData,
            },
            UART3: UART3 {
                _marker: PhantomData,
            },
            MATRIX: MATRIX {
                _marker: PhantomData,
            },
            PMC: PMC {
                _marker: PhantomData,
            },
            UART0: UART0 {
                _marker: PhantomData,
            },
            CHIPID: CHIPID {
                _marker: PhantomData,
            },
            UART1: UART1 {
                _marker: PhantomData,
            },
            EFC: EFC {
                _marker: PhantomData,
            },
            PIOA: PIOA {
                _marker: PhantomData,
            },
            PIOB: PIOB {
                _marker: PhantomData,
            },
            RSTC: RSTC {
                _marker: PhantomData,
            },
            SUPC: SUPC {
                _marker: PhantomData,
            },
            RTT: RTT {
                _marker: PhantomData,
            },
            WDT: WDT {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            GPBR: GPBR {
                _marker: PhantomData,
            },
        }
    }
}
