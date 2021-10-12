#![doc = "Peripheral access API for ATSAM4S2C microcontrollers (generated using svd2rust v0.19.0 ( ))\n\nYou can find an overview of the generated API [here].\n\nAPI features to be included in the [next]
svd2rust release can be generated by cloning the svd2rust [repository], checking out the above commit, and running `cargo doc --open`.\n\n[here]: https://docs.rs/svd2rust/0.19.0/svd2rust/#peripheral-api\n[next]: https://github.com/rust-embedded/svd2rust/blob/master/CHANGELOG.md#unreleased\n[repository]: https://github.com/rust-embedded/svd2rust"]
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
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 4;
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
#[cfg(feature = "rt")]
extern "C" {
    fn SUPC();
    fn RSTC();
    fn RTC();
    fn RTT();
    fn WDT();
    fn PMC();
    fn EFC0();
    fn UART0();
    fn UART1();
    fn PIOA();
    fn PIOB();
    fn PIOC();
    fn USART0();
    fn USART1();
    fn HSMCI();
    fn TWI0();
    fn TWI1();
    fn SPI();
    fn SSC();
    fn TC0();
    fn TC1();
    fn TC2();
    fn TC3();
    fn TC4();
    fn TC5();
    fn ADC();
    fn DACC();
    fn PWM();
    fn CRCCU();
    fn ACC();
    fn UDP();
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
pub static __INTERRUPTS: [Vector; 35] = [
    Vector { _handler: SUPC },
    Vector { _handler: RSTC },
    Vector { _handler: RTC },
    Vector { _handler: RTT },
    Vector { _handler: WDT },
    Vector { _handler: PMC },
    Vector { _handler: EFC0 },
    Vector { _reserved: 0 },
    Vector { _handler: UART0 },
    Vector { _handler: UART1 },
    Vector { _reserved: 0 },
    Vector { _handler: PIOA },
    Vector { _handler: PIOB },
    Vector { _handler: PIOC },
    Vector { _handler: USART0 },
    Vector { _handler: USART1 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: HSMCI },
    Vector { _handler: TWI0 },
    Vector { _handler: TWI1 },
    Vector { _handler: SPI },
    Vector { _handler: SSC },
    Vector { _handler: TC0 },
    Vector { _handler: TC1 },
    Vector { _handler: TC2 },
    Vector { _handler: TC3 },
    Vector { _handler: TC4 },
    Vector { _handler: TC5 },
    Vector { _handler: ADC },
    Vector { _handler: DACC },
    Vector { _handler: PWM },
    Vector { _handler: CRCCU },
    Vector { _handler: ACC },
    Vector { _handler: UDP },
];
#[doc = r"Enumeration of all the interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
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
    #[doc = "6 - EFC0"]
    EFC0 = 6,
    #[doc = "8 - UART0"]
    UART0 = 8,
    #[doc = "9 - UART1"]
    UART1 = 9,
    #[doc = "11 - PIOA"]
    PIOA = 11,
    #[doc = "12 - PIOB"]
    PIOB = 12,
    #[doc = "13 - PIOC"]
    PIOC = 13,
    #[doc = "14 - USART0"]
    USART0 = 14,
    #[doc = "15 - USART1"]
    USART1 = 15,
    #[doc = "18 - HSMCI"]
    HSMCI = 18,
    #[doc = "19 - TWI0"]
    TWI0 = 19,
    #[doc = "20 - TWI1"]
    TWI1 = 20,
    #[doc = "21 - SPI"]
    SPI = 21,
    #[doc = "22 - SSC"]
    SSC = 22,
    #[doc = "23 - TC0"]
    TC0 = 23,
    #[doc = "24 - TC1"]
    TC1 = 24,
    #[doc = "25 - TC2"]
    TC2 = 25,
    #[doc = "26 - TC3"]
    TC3 = 26,
    #[doc = "27 - TC4"]
    TC4 = 27,
    #[doc = "28 - TC5"]
    TC5 = 28,
    #[doc = "29 - ADC"]
    ADC = 29,
    #[doc = "30 - DACC"]
    DACC = 30,
    #[doc = "31 - PWM"]
    PWM = 31,
    #[doc = "32 - CRCCU"]
    CRCCU = 32,
    #[doc = "33 - ACC"]
    ACC = 33,
    #[doc = "34 - UDP"]
    UDP = 34,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[doc = "High Speed MultiMedia Card Interface"]
pub struct HSMCI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HSMCI {}
impl HSMCI {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const hsmci::RegisterBlock = 0x4000_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hsmci::RegisterBlock {
        Self::PTR
    }
}
impl Deref for HSMCI {
    type Target = hsmci::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for HSMCI {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSMCI").finish()
    }
}
#[doc = "High Speed MultiMedia Card Interface"]
pub mod hsmci;
#[doc = "Synchronous Serial Controller"]
pub struct SSC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SSC {}
impl SSC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const ssc::RegisterBlock = 0x4000_4000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ssc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SSC {
    type Target = ssc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SSC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SSC").finish()
    }
}
#[doc = "Synchronous Serial Controller"]
pub mod ssc;
#[doc = "Serial Peripheral Interface"]
pub struct SPI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI {}
impl SPI {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const spi::RegisterBlock = 0x4000_8000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SPI {
    type Target = spi::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SPI {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI").finish()
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
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const tc0::RegisterBlock = 0x4001_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tc0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TC0 {
    type Target = tc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TC0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TC0").finish()
    }
}
#[doc = "Timer Counter 0"]
pub mod tc0;
#[doc = "Timer Counter 1"]
pub struct TC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TC1 {}
impl TC1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const tc1::RegisterBlock = 0x4001_4000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tc1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TC1 {
    type Target = tc1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TC1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TC1").finish()
    }
}
#[doc = "Timer Counter 1"]
pub mod tc1;
#[doc = "Two-wire Interface 0"]
pub struct TWI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWI0 {}
impl TWI0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const twi0::RegisterBlock = 0x4001_8000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const twi0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TWI0 {
    type Target = twi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TWI0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TWI0").finish()
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
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const twi1::RegisterBlock = 0x4001_c000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const twi1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TWI1 {
    type Target = twi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TWI1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TWI1").finish()
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
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const pwm::RegisterBlock = 0x4002_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pwm::RegisterBlock {
        Self::PTR
    }
}
impl Deref for PWM {
    type Target = pwm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for PWM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWM").finish()
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
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const usart0::RegisterBlock = 0x4002_4000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for USART0 {
    type Target = usart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for USART0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USART0").finish()
    }
}
#[doc = "Universal Synchronous Asynchronous Receiver Transmitter 0"]
pub mod usart0;
#[doc = "Universal Synchronous Asynchronous Receiver Transmitter 1"]
pub struct USART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART1 {}
impl USART1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const usart1::RegisterBlock = 0x4002_8000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for USART1 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for USART1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USART1").finish()
    }
}
#[doc = "Universal Synchronous Asynchronous Receiver Transmitter 1"]
pub mod usart1;
#[doc = "USB Device Port"]
pub struct UDP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UDP {}
impl UDP {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const udp::RegisterBlock = 0x4003_4000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const udp::RegisterBlock {
        Self::PTR
    }
}
impl Deref for UDP {
    type Target = udp::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for UDP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDP").finish()
    }
}
#[doc = "USB Device Port"]
pub mod udp;
#[doc = "Analog-to-Digital Converter"]
pub struct ADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC {}
impl ADC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const adc::RegisterBlock = 0x4003_8000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for ADC {
    type Target = adc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for ADC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC").finish()
    }
}
#[doc = "Analog-to-Digital Converter"]
pub mod adc;
#[doc = "Digital-to-Analog Converter Controller"]
pub struct DACC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DACC {}
impl DACC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const dacc::RegisterBlock = 0x4003_c000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dacc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for DACC {
    type Target = dacc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for DACC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DACC").finish()
    }
}
#[doc = "Digital-to-Analog Converter Controller"]
pub mod dacc;
#[doc = "Analog Comparator Controller"]
pub struct ACC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ACC {}
impl ACC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const acc::RegisterBlock = 0x4004_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const acc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for ACC {
    type Target = acc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for ACC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACC").finish()
    }
}
#[doc = "Analog Comparator Controller"]
pub mod acc;
#[doc = "Cyclic Redundancy Check Calculation Unit"]
pub struct CRCCU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRCCU {}
impl CRCCU {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const crccu::RegisterBlock = 0x4004_4000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const crccu::RegisterBlock {
        Self::PTR
    }
}
impl Deref for CRCCU {
    type Target = crccu::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for CRCCU {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRCCU").finish()
    }
}
#[doc = "Cyclic Redundancy Check Calculation Unit"]
pub mod crccu;
#[doc = "Static Memory Controller"]
pub struct SMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SMC {}
impl SMC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const smc::RegisterBlock = 0x400e_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const smc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SMC {
    type Target = smc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SMC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMC").finish()
    }
}
#[doc = "Static Memory Controller"]
pub mod smc;
#[doc = "AHB Bus Matrix"]
pub struct MATRIX {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MATRIX {}
impl MATRIX {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const matrix::RegisterBlock = 0x400e_0200 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const matrix::RegisterBlock {
        Self::PTR
    }
}
impl Deref for MATRIX {
    type Target = matrix::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for MATRIX {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MATRIX").finish()
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
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const pmc::RegisterBlock = 0x400e_0400 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pmc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for PMC {
    type Target = pmc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for PMC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMC").finish()
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
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const uart0::RegisterBlock = 0x400e_0600 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for UART0 {
    type Target = uart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for UART0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART0").finish()
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
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const chipid::RegisterBlock = 0x400e_0740 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const chipid::RegisterBlock {
        Self::PTR
    }
}
impl Deref for CHIPID {
    type Target = chipid::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for CHIPID {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHIPID").finish()
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
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const uart1::RegisterBlock = 0x400e_0800 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for UART1 {
    type Target = uart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for UART1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART1").finish()
    }
}
#[doc = "Universal Asynchronous Receiver Transmitter 1"]
pub mod uart1;
#[doc = "Embedded Flash Controller 0"]
pub struct EFC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EFC0 {}
impl EFC0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const efc0::RegisterBlock = 0x400e_0a00 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const efc0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for EFC0 {
    type Target = efc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for EFC0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EFC0").finish()
    }
}
#[doc = "Embedded Flash Controller 0"]
pub mod efc0;
#[doc = "Parallel Input/Output Controller A"]
pub struct PIOA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PIOA {}
impl PIOA {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const pioa::RegisterBlock = 0x400e_0e00 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pioa::RegisterBlock {
        Self::PTR
    }
}
impl Deref for PIOA {
    type Target = pioa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for PIOA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIOA").finish()
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
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const piob::RegisterBlock = 0x400e_1000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const piob::RegisterBlock {
        Self::PTR
    }
}
impl Deref for PIOB {
    type Target = piob::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for PIOB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIOB").finish()
    }
}
#[doc = "Parallel Input/Output Controller B"]
pub mod piob;
#[doc = "Parallel Input/Output Controller C"]
pub struct PIOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PIOC {}
impl PIOC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const pioc::RegisterBlock = 0x400e_1200 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pioc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for PIOC {
    type Target = pioc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for PIOC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIOC").finish()
    }
}
#[doc = "Parallel Input/Output Controller C"]
pub mod pioc;
#[doc = "Reset Controller"]
pub struct RSTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RSTC {}
impl RSTC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const rstc::RegisterBlock = 0x400e_1400 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rstc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for RSTC {
    type Target = rstc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for RSTC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSTC").finish()
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
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const supc::RegisterBlock = 0x400e_1410 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const supc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SUPC {
    type Target = supc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SUPC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SUPC").finish()
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
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const rtt::RegisterBlock = 0x400e_1430 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtt::RegisterBlock {
        Self::PTR
    }
}
impl Deref for RTT {
    type Target = rtt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for RTT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTT").finish()
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
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const wdt::RegisterBlock = 0x400e_1450 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wdt::RegisterBlock {
        Self::PTR
    }
}
impl Deref for WDT {
    type Target = wdt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for WDT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDT").finish()
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
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const rtc::RegisterBlock = 0x400e_1460 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for RTC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC").finish()
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
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const gpbr::RegisterBlock = 0x400e_1490 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpbr::RegisterBlock {
        Self::PTR
    }
}
impl Deref for GPBR {
    type Target = gpbr::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for GPBR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPBR").finish()
    }
}
#[doc = "General Purpose Backup Registers"]
pub mod gpbr;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "HSMCI"]
    pub HSMCI: HSMCI,
    #[doc = "SSC"]
    pub SSC: SSC,
    #[doc = "SPI"]
    pub SPI: SPI,
    #[doc = "TC0"]
    pub TC0: TC0,
    #[doc = "TC1"]
    pub TC1: TC1,
    #[doc = "TWI0"]
    pub TWI0: TWI0,
    #[doc = "TWI1"]
    pub TWI1: TWI1,
    #[doc = "PWM"]
    pub PWM: PWM,
    #[doc = "USART0"]
    pub USART0: USART0,
    #[doc = "USART1"]
    pub USART1: USART1,
    #[doc = "UDP"]
    pub UDP: UDP,
    #[doc = "ADC"]
    pub ADC: ADC,
    #[doc = "DACC"]
    pub DACC: DACC,
    #[doc = "ACC"]
    pub ACC: ACC,
    #[doc = "CRCCU"]
    pub CRCCU: CRCCU,
    #[doc = "SMC"]
    pub SMC: SMC,
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
    #[doc = "EFC0"]
    pub EFC0: EFC0,
    #[doc = "PIOA"]
    pub PIOA: PIOA,
    #[doc = "PIOB"]
    pub PIOB: PIOB,
    #[doc = "PIOC"]
    pub PIOC: PIOC,
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
            HSMCI: HSMCI {
                _marker: PhantomData,
            },
            SSC: SSC {
                _marker: PhantomData,
            },
            SPI: SPI {
                _marker: PhantomData,
            },
            TC0: TC0 {
                _marker: PhantomData,
            },
            TC1: TC1 {
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
            USART1: USART1 {
                _marker: PhantomData,
            },
            UDP: UDP {
                _marker: PhantomData,
            },
            ADC: ADC {
                _marker: PhantomData,
            },
            DACC: DACC {
                _marker: PhantomData,
            },
            ACC: ACC {
                _marker: PhantomData,
            },
            CRCCU: CRCCU {
                _marker: PhantomData,
            },
            SMC: SMC {
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
            EFC0: EFC0 {
                _marker: PhantomData,
            },
            PIOA: PIOA {
                _marker: PhantomData,
            },
            PIOB: PIOB {
                _marker: PhantomData,
            },
            PIOC: PIOC {
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
