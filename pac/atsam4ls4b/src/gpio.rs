#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO Enable Register"]
    pub gper0: crate::Reg<gper::GPER_SPEC>,
    #[doc = "0x04 - GPIO Enable Register - Set"]
    pub gpers0: crate::Reg<gpers::GPERS_SPEC>,
    #[doc = "0x08 - GPIO Enable Register - Clear"]
    pub gperc0: crate::Reg<gperc::GPERC_SPEC>,
    #[doc = "0x0c - GPIO Enable Register - Toggle"]
    pub gpert0: crate::Reg<gpert::GPERT_SPEC>,
    #[doc = "0x10 - Peripheral Mux Register 0"]
    pub pmr00: crate::Reg<pmr0::PMR0_SPEC>,
    #[doc = "0x14 - Peripheral Mux Register 0 - Set"]
    pub pmr0s0: crate::Reg<pmr0s::PMR0S_SPEC>,
    #[doc = "0x18 - Peripheral Mux Register 0 - Clear"]
    pub pmr0c0: crate::Reg<pmr0c::PMR0C_SPEC>,
    #[doc = "0x1c - Peripheral Mux Register 0 - Toggle"]
    pub pmr0t0: crate::Reg<pmr0t::PMR0T_SPEC>,
    #[doc = "0x20 - Peripheral Mux Register 1"]
    pub pmr10: crate::Reg<pmr1::PMR1_SPEC>,
    #[doc = "0x24 - Peripheral Mux Register 1 - Set"]
    pub pmr1s0: crate::Reg<pmr1s::PMR1S_SPEC>,
    #[doc = "0x28 - Peripheral Mux Register 1 - Clear"]
    pub pmr1c0: crate::Reg<pmr1c::PMR1C_SPEC>,
    #[doc = "0x2c - Peripheral Mux Register 1 - Toggle"]
    pub pmr1t0: crate::Reg<pmr1t::PMR1T_SPEC>,
    #[doc = "0x30 - Peripheral Mux Register 2"]
    pub pmr20: crate::Reg<pmr2::PMR2_SPEC>,
    #[doc = "0x34 - Peripheral Mux Register 2 - Set"]
    pub pmr2s0: crate::Reg<pmr2s::PMR2S_SPEC>,
    #[doc = "0x38 - Peripheral Mux Register 2 - Clear"]
    pub pmr2c0: crate::Reg<pmr2c::PMR2C_SPEC>,
    #[doc = "0x3c - Peripheral Mux Register 2 - Toggle"]
    pub pmr2t0: crate::Reg<pmr2t::PMR2T_SPEC>,
    #[doc = "0x40 - Output Driver Enable Register"]
    pub oder0: crate::Reg<oder::ODER_SPEC>,
    #[doc = "0x44 - Output Driver Enable Register - Set"]
    pub oders0: crate::Reg<oders::ODERS_SPEC>,
    #[doc = "0x48 - Output Driver Enable Register - Clear"]
    pub oderc0: crate::Reg<oderc::ODERC_SPEC>,
    #[doc = "0x4c - Output Driver Enable Register - Toggle"]
    pub odert0: crate::Reg<odert::ODERT_SPEC>,
    #[doc = "0x50 - Output Value Register"]
    pub ovr0: crate::Reg<ovr::OVR_SPEC>,
    #[doc = "0x54 - Output Value Register - Set"]
    pub ovrs0: crate::Reg<ovrs::OVRS_SPEC>,
    #[doc = "0x58 - Output Value Register - Clear"]
    pub ovrc0: crate::Reg<ovrc::OVRC_SPEC>,
    #[doc = "0x5c - Output Value Register - Toggle"]
    pub ovrt0: crate::Reg<ovrt::OVRT_SPEC>,
    #[doc = "0x60 - Pin Value Register"]
    pub pvr0: crate::Reg<pvr::PVR_SPEC>,
    _reserved25: [u8; 0x0c],
    #[doc = "0x70 - Pull-up Enable Register"]
    pub puer0: crate::Reg<puer::PUER_SPEC>,
    #[doc = "0x74 - Pull-up Enable Register - Set"]
    pub puers0: crate::Reg<puers::PUERS_SPEC>,
    #[doc = "0x78 - Pull-up Enable Register - Clear"]
    pub puerc0: crate::Reg<puerc::PUERC_SPEC>,
    #[doc = "0x7c - Pull-up Enable Register - Toggle"]
    pub puert0: crate::Reg<puert::PUERT_SPEC>,
    #[doc = "0x80 - Pull-down Enable Register"]
    pub pder0: crate::Reg<pder::PDER_SPEC>,
    #[doc = "0x84 - Pull-down Enable Register - Set"]
    pub pders0: crate::Reg<pders::PDERS_SPEC>,
    #[doc = "0x88 - Pull-down Enable Register - Clear"]
    pub pderc0: crate::Reg<pderc::PDERC_SPEC>,
    #[doc = "0x8c - Pull-down Enable Register - Toggle"]
    pub pdert0: crate::Reg<pdert::PDERT_SPEC>,
    #[doc = "0x90 - Interrupt Enable Register"]
    pub ier0: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x94 - Interrupt Enable Register - Set"]
    pub iers0: crate::Reg<iers::IERS_SPEC>,
    #[doc = "0x98 - Interrupt Enable Register - Clear"]
    pub ierc0: crate::Reg<ierc::IERC_SPEC>,
    #[doc = "0x9c - Interrupt Enable Register - Toggle"]
    pub iert0: crate::Reg<iert::IERT_SPEC>,
    #[doc = "0xa0 - Interrupt Mode Register 0"]
    pub imr00: crate::Reg<imr0::IMR0_SPEC>,
    #[doc = "0xa4 - Interrupt Mode Register 0 - Set"]
    pub imr0s0: crate::Reg<imr0s::IMR0S_SPEC>,
    #[doc = "0xa8 - Interrupt Mode Register 0 - Clear"]
    pub imr0c0: crate::Reg<imr0c::IMR0C_SPEC>,
    #[doc = "0xac - Interrupt Mode Register 0 - Toggle"]
    pub imr0t0: crate::Reg<imr0t::IMR0T_SPEC>,
    #[doc = "0xb0 - Interrupt Mode Register 1"]
    pub imr10: crate::Reg<imr1::IMR1_SPEC>,
    #[doc = "0xb4 - Interrupt Mode Register 1 - Set"]
    pub imr1s0: crate::Reg<imr1s::IMR1S_SPEC>,
    #[doc = "0xb8 - Interrupt Mode Register 1 - Clear"]
    pub imr1c0: crate::Reg<imr1c::IMR1C_SPEC>,
    #[doc = "0xbc - Interrupt Mode Register 1 - Toggle"]
    pub imr1t0: crate::Reg<imr1t::IMR1T_SPEC>,
    #[doc = "0xc0 - Glitch Filter Enable Register"]
    pub gfer0: crate::Reg<gfer::GFER_SPEC>,
    #[doc = "0xc4 - Glitch Filter Enable Register - Set"]
    pub gfers0: crate::Reg<gfers::GFERS_SPEC>,
    #[doc = "0xc8 - Glitch Filter Enable Register - Clear"]
    pub gferc0: crate::Reg<gferc::GFERC_SPEC>,
    #[doc = "0xcc - Glitch Filter Enable Register - Toggle"]
    pub gfert0: crate::Reg<gfert::GFERT_SPEC>,
    #[doc = "0xd0 - Interrupt Flag Register"]
    pub ifr0: crate::Reg<ifr::IFR_SPEC>,
    _reserved50: [u8; 0x04],
    #[doc = "0xd8 - Interrupt Flag Register - Clear"]
    pub ifrc0: crate::Reg<ifrc::IFRC_SPEC>,
    _reserved51: [u8; 0x04],
    #[doc = "0xe0 - Open Drain Mode Register"]
    pub odmer0: crate::Reg<odmer::ODMER_SPEC>,
    #[doc = "0xe4 - Open Drain Mode Register - Set"]
    pub odmers0: crate::Reg<odmers::ODMERS_SPEC>,
    #[doc = "0xe8 - Open Drain Mode Register - Clear"]
    pub odmerc0: crate::Reg<odmerc::ODMERC_SPEC>,
    #[doc = "0xec - Open Drain Mode Register - Toggle"]
    pub odmert0: crate::Reg<odmert::ODMERT_SPEC>,
    _reserved55: [u8; 0x10],
    #[doc = "0x100 - Output Driving Capability Register 0"]
    pub odcr00: crate::Reg<odcr0::ODCR0_SPEC>,
    #[doc = "0x104 - Output Driving Capability Register 0 - Set"]
    pub odcr0s0: crate::Reg<odcr0s::ODCR0S_SPEC>,
    #[doc = "0x108 - Output Driving Capability Register 0 - Clear"]
    pub odcr0c0: crate::Reg<odcr0c::ODCR0C_SPEC>,
    #[doc = "0x10c - Output Driving Capability Register 0 - Toggle"]
    pub odcr0t0: crate::Reg<odcr0t::ODCR0T_SPEC>,
    #[doc = "0x110 - Output Driving Capability Register 1"]
    pub odcr10: crate::Reg<odcr1::ODCR1_SPEC>,
    #[doc = "0x114 - Output Driving Capability Register 1 - Set"]
    pub odcr1s0: crate::Reg<odcr1s::ODCR1S_SPEC>,
    #[doc = "0x118 - Output Driving Capability Register 1 - Clear"]
    pub odcr1c0: crate::Reg<odcr1c::ODCR1C_SPEC>,
    #[doc = "0x11c - Output Driving Capability Register 1 - Toggle"]
    pub odcr1t0: crate::Reg<odcr1t::ODCR1T_SPEC>,
    _reserved63: [u8; 0x10],
    #[doc = "0x130 - Output Slew Rate Register 0"]
    pub osrr00: crate::Reg<osrr0::OSRR0_SPEC>,
    #[doc = "0x134 - Output Slew Rate Register 0 - Set"]
    pub osrr0s0: crate::Reg<osrr0s::OSRR0S_SPEC>,
    #[doc = "0x138 - Output Slew Rate Register 0 - Clear"]
    pub osrr0c0: crate::Reg<osrr0c::OSRR0C_SPEC>,
    #[doc = "0x13c - Output Slew Rate Register 0 - Toggle"]
    pub osrr0t0: crate::Reg<osrr0t::OSRR0T_SPEC>,
    _reserved67: [u8; 0x20],
    #[doc = "0x160 - Schmitt Trigger Enable Register"]
    pub ster0: crate::Reg<ster::STER_SPEC>,
    #[doc = "0x164 - Schmitt Trigger Enable Register - Set"]
    pub sters0: crate::Reg<sters::STERS_SPEC>,
    #[doc = "0x168 - Schmitt Trigger Enable Register - Clear"]
    pub sterc0: crate::Reg<sterc::STERC_SPEC>,
    #[doc = "0x16c - Schmitt Trigger Enable Register - Toggle"]
    pub stert0: crate::Reg<stert::STERT_SPEC>,
    _reserved71: [u8; 0x10],
    #[doc = "0x180 - Event Enable Register"]
    pub ever0: crate::Reg<ever::EVER_SPEC>,
    #[doc = "0x184 - Event Enable Register - Set"]
    pub evers0: crate::Reg<evers::EVERS_SPEC>,
    #[doc = "0x188 - Event Enable Register - Clear"]
    pub everc0: crate::Reg<everc::EVERC_SPEC>,
    #[doc = "0x18c - Event Enable Register - Toggle"]
    pub evert0: crate::Reg<evert::EVERT_SPEC>,
    _reserved75: [u8; 0x10],
    #[doc = "0x1a0 - Lock Register"]
    pub lock0: crate::Reg<lock::LOCK_SPEC>,
    #[doc = "0x1a4 - Lock Register - Set"]
    pub locks0: crate::Reg<locks::LOCKS_SPEC>,
    #[doc = "0x1a8 - Lock Register - Clear"]
    pub lockc0: crate::Reg<lockc::LOCKC_SPEC>,
    #[doc = "0x1ac - Lock Register - Toggle"]
    pub lockt0: crate::Reg<lockt::LOCKT_SPEC>,
    _reserved79: [u8; 0x30],
    #[doc = "0x1e0 - Unlock Register"]
    pub unlock0: crate::Reg<unlock::UNLOCK_SPEC>,
    #[doc = "0x1e4 - Access Status Register"]
    pub asr0: crate::Reg<asr::ASR_SPEC>,
    _reserved81: [u8; 0x10],
    #[doc = "0x1f8 - Parameter Register"]
    pub parameter0: crate::Reg<parameter::PARAMETER_SPEC>,
    #[doc = "0x1fc - Version Register"]
    pub version0: crate::Reg<version::VERSION_SPEC>,
    #[doc = "0x200 - GPIO Enable Register"]
    pub gper1: crate::Reg<gper::GPER_SPEC>,
    #[doc = "0x204 - GPIO Enable Register - Set"]
    pub gpers1: crate::Reg<gpers::GPERS_SPEC>,
    #[doc = "0x208 - GPIO Enable Register - Clear"]
    pub gperc1: crate::Reg<gperc::GPERC_SPEC>,
    #[doc = "0x20c - GPIO Enable Register - Toggle"]
    pub gpert1: crate::Reg<gpert::GPERT_SPEC>,
    #[doc = "0x210 - Peripheral Mux Register 0"]
    pub pmr01: crate::Reg<pmr0::PMR0_SPEC>,
    #[doc = "0x214 - Peripheral Mux Register 0 - Set"]
    pub pmr0s1: crate::Reg<pmr0s::PMR0S_SPEC>,
    #[doc = "0x218 - Peripheral Mux Register 0 - Clear"]
    pub pmr0c1: crate::Reg<pmr0c::PMR0C_SPEC>,
    #[doc = "0x21c - Peripheral Mux Register 0 - Toggle"]
    pub pmr0t1: crate::Reg<pmr0t::PMR0T_SPEC>,
    #[doc = "0x220 - Peripheral Mux Register 1"]
    pub pmr11: crate::Reg<pmr1::PMR1_SPEC>,
    #[doc = "0x224 - Peripheral Mux Register 1 - Set"]
    pub pmr1s1: crate::Reg<pmr1s::PMR1S_SPEC>,
    #[doc = "0x228 - Peripheral Mux Register 1 - Clear"]
    pub pmr1c1: crate::Reg<pmr1c::PMR1C_SPEC>,
    #[doc = "0x22c - Peripheral Mux Register 1 - Toggle"]
    pub pmr1t1: crate::Reg<pmr1t::PMR1T_SPEC>,
    #[doc = "0x230 - Peripheral Mux Register 2"]
    pub pmr21: crate::Reg<pmr2::PMR2_SPEC>,
    #[doc = "0x234 - Peripheral Mux Register 2 - Set"]
    pub pmr2s1: crate::Reg<pmr2s::PMR2S_SPEC>,
    #[doc = "0x238 - Peripheral Mux Register 2 - Clear"]
    pub pmr2c1: crate::Reg<pmr2c::PMR2C_SPEC>,
    #[doc = "0x23c - Peripheral Mux Register 2 - Toggle"]
    pub pmr2t1: crate::Reg<pmr2t::PMR2T_SPEC>,
    #[doc = "0x240 - Output Driver Enable Register"]
    pub oder1: crate::Reg<oder::ODER_SPEC>,
    #[doc = "0x244 - Output Driver Enable Register - Set"]
    pub oders1: crate::Reg<oders::ODERS_SPEC>,
    #[doc = "0x248 - Output Driver Enable Register - Clear"]
    pub oderc1: crate::Reg<oderc::ODERC_SPEC>,
    #[doc = "0x24c - Output Driver Enable Register - Toggle"]
    pub odert1: crate::Reg<odert::ODERT_SPEC>,
    #[doc = "0x250 - Output Value Register"]
    pub ovr1: crate::Reg<ovr::OVR_SPEC>,
    #[doc = "0x254 - Output Value Register - Set"]
    pub ovrs1: crate::Reg<ovrs::OVRS_SPEC>,
    #[doc = "0x258 - Output Value Register - Clear"]
    pub ovrc1: crate::Reg<ovrc::OVRC_SPEC>,
    #[doc = "0x25c - Output Value Register - Toggle"]
    pub ovrt1: crate::Reg<ovrt::OVRT_SPEC>,
    #[doc = "0x260 - Pin Value Register"]
    pub pvr1: crate::Reg<pvr::PVR_SPEC>,
    _reserved108: [u8; 0x0c],
    #[doc = "0x270 - Pull-up Enable Register"]
    pub puer1: crate::Reg<puer::PUER_SPEC>,
    #[doc = "0x274 - Pull-up Enable Register - Set"]
    pub puers1: crate::Reg<puers::PUERS_SPEC>,
    #[doc = "0x278 - Pull-up Enable Register - Clear"]
    pub puerc1: crate::Reg<puerc::PUERC_SPEC>,
    #[doc = "0x27c - Pull-up Enable Register - Toggle"]
    pub puert1: crate::Reg<puert::PUERT_SPEC>,
    #[doc = "0x280 - Pull-down Enable Register"]
    pub pder1: crate::Reg<pder::PDER_SPEC>,
    #[doc = "0x284 - Pull-down Enable Register - Set"]
    pub pders1: crate::Reg<pders::PDERS_SPEC>,
    #[doc = "0x288 - Pull-down Enable Register - Clear"]
    pub pderc1: crate::Reg<pderc::PDERC_SPEC>,
    #[doc = "0x28c - Pull-down Enable Register - Toggle"]
    pub pdert1: crate::Reg<pdert::PDERT_SPEC>,
    #[doc = "0x290 - Interrupt Enable Register"]
    pub ier1: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x294 - Interrupt Enable Register - Set"]
    pub iers1: crate::Reg<iers::IERS_SPEC>,
    #[doc = "0x298 - Interrupt Enable Register - Clear"]
    pub ierc1: crate::Reg<ierc::IERC_SPEC>,
    #[doc = "0x29c - Interrupt Enable Register - Toggle"]
    pub iert1: crate::Reg<iert::IERT_SPEC>,
    #[doc = "0x2a0 - Interrupt Mode Register 0"]
    pub imr01: crate::Reg<imr0::IMR0_SPEC>,
    #[doc = "0x2a4 - Interrupt Mode Register 0 - Set"]
    pub imr0s1: crate::Reg<imr0s::IMR0S_SPEC>,
    #[doc = "0x2a8 - Interrupt Mode Register 0 - Clear"]
    pub imr0c1: crate::Reg<imr0c::IMR0C_SPEC>,
    #[doc = "0x2ac - Interrupt Mode Register 0 - Toggle"]
    pub imr0t1: crate::Reg<imr0t::IMR0T_SPEC>,
    #[doc = "0x2b0 - Interrupt Mode Register 1"]
    pub imr11: crate::Reg<imr1::IMR1_SPEC>,
    #[doc = "0x2b4 - Interrupt Mode Register 1 - Set"]
    pub imr1s1: crate::Reg<imr1s::IMR1S_SPEC>,
    #[doc = "0x2b8 - Interrupt Mode Register 1 - Clear"]
    pub imr1c1: crate::Reg<imr1c::IMR1C_SPEC>,
    #[doc = "0x2bc - Interrupt Mode Register 1 - Toggle"]
    pub imr1t1: crate::Reg<imr1t::IMR1T_SPEC>,
    #[doc = "0x2c0 - Glitch Filter Enable Register"]
    pub gfer1: crate::Reg<gfer::GFER_SPEC>,
    #[doc = "0x2c4 - Glitch Filter Enable Register - Set"]
    pub gfers1: crate::Reg<gfers::GFERS_SPEC>,
    #[doc = "0x2c8 - Glitch Filter Enable Register - Clear"]
    pub gferc1: crate::Reg<gferc::GFERC_SPEC>,
    #[doc = "0x2cc - Glitch Filter Enable Register - Toggle"]
    pub gfert1: crate::Reg<gfert::GFERT_SPEC>,
    #[doc = "0x2d0 - Interrupt Flag Register"]
    pub ifr1: crate::Reg<ifr::IFR_SPEC>,
    _reserved133: [u8; 0x04],
    #[doc = "0x2d8 - Interrupt Flag Register - Clear"]
    pub ifrc1: crate::Reg<ifrc::IFRC_SPEC>,
    _reserved134: [u8; 0x04],
    #[doc = "0x2e0 - Open Drain Mode Register"]
    pub odmer1: crate::Reg<odmer::ODMER_SPEC>,
    #[doc = "0x2e4 - Open Drain Mode Register - Set"]
    pub odmers1: crate::Reg<odmers::ODMERS_SPEC>,
    #[doc = "0x2e8 - Open Drain Mode Register - Clear"]
    pub odmerc1: crate::Reg<odmerc::ODMERC_SPEC>,
    #[doc = "0x2ec - Open Drain Mode Register - Toggle"]
    pub odmert1: crate::Reg<odmert::ODMERT_SPEC>,
    _reserved138: [u8; 0x10],
    #[doc = "0x300 - Output Driving Capability Register 0"]
    pub odcr01: crate::Reg<odcr0::ODCR0_SPEC>,
    #[doc = "0x304 - Output Driving Capability Register 0 - Set"]
    pub odcr0s1: crate::Reg<odcr0s::ODCR0S_SPEC>,
    #[doc = "0x308 - Output Driving Capability Register 0 - Clear"]
    pub odcr0c1: crate::Reg<odcr0c::ODCR0C_SPEC>,
    #[doc = "0x30c - Output Driving Capability Register 0 - Toggle"]
    pub odcr0t1: crate::Reg<odcr0t::ODCR0T_SPEC>,
    #[doc = "0x310 - Output Driving Capability Register 1"]
    pub odcr11: crate::Reg<odcr1::ODCR1_SPEC>,
    #[doc = "0x314 - Output Driving Capability Register 1 - Set"]
    pub odcr1s1: crate::Reg<odcr1s::ODCR1S_SPEC>,
    #[doc = "0x318 - Output Driving Capability Register 1 - Clear"]
    pub odcr1c1: crate::Reg<odcr1c::ODCR1C_SPEC>,
    #[doc = "0x31c - Output Driving Capability Register 1 - Toggle"]
    pub odcr1t1: crate::Reg<odcr1t::ODCR1T_SPEC>,
    _reserved146: [u8; 0x10],
    #[doc = "0x330 - Output Slew Rate Register 0"]
    pub osrr01: crate::Reg<osrr0::OSRR0_SPEC>,
    #[doc = "0x334 - Output Slew Rate Register 0 - Set"]
    pub osrr0s1: crate::Reg<osrr0s::OSRR0S_SPEC>,
    #[doc = "0x338 - Output Slew Rate Register 0 - Clear"]
    pub osrr0c1: crate::Reg<osrr0c::OSRR0C_SPEC>,
    #[doc = "0x33c - Output Slew Rate Register 0 - Toggle"]
    pub osrr0t1: crate::Reg<osrr0t::OSRR0T_SPEC>,
    _reserved150: [u8; 0x20],
    #[doc = "0x360 - Schmitt Trigger Enable Register"]
    pub ster1: crate::Reg<ster::STER_SPEC>,
    #[doc = "0x364 - Schmitt Trigger Enable Register - Set"]
    pub sters1: crate::Reg<sters::STERS_SPEC>,
    #[doc = "0x368 - Schmitt Trigger Enable Register - Clear"]
    pub sterc1: crate::Reg<sterc::STERC_SPEC>,
    #[doc = "0x36c - Schmitt Trigger Enable Register - Toggle"]
    pub stert1: crate::Reg<stert::STERT_SPEC>,
    _reserved154: [u8; 0x10],
    #[doc = "0x380 - Event Enable Register"]
    pub ever1: crate::Reg<ever::EVER_SPEC>,
    #[doc = "0x384 - Event Enable Register - Set"]
    pub evers1: crate::Reg<evers::EVERS_SPEC>,
    #[doc = "0x388 - Event Enable Register - Clear"]
    pub everc1: crate::Reg<everc::EVERC_SPEC>,
    #[doc = "0x38c - Event Enable Register - Toggle"]
    pub evert1: crate::Reg<evert::EVERT_SPEC>,
    _reserved158: [u8; 0x10],
    #[doc = "0x3a0 - Lock Register"]
    pub lock1: crate::Reg<lock::LOCK_SPEC>,
    #[doc = "0x3a4 - Lock Register - Set"]
    pub locks1: crate::Reg<locks::LOCKS_SPEC>,
    #[doc = "0x3a8 - Lock Register - Clear"]
    pub lockc1: crate::Reg<lockc::LOCKC_SPEC>,
    #[doc = "0x3ac - Lock Register - Toggle"]
    pub lockt1: crate::Reg<lockt::LOCKT_SPEC>,
    _reserved162: [u8; 0x30],
    #[doc = "0x3e0 - Unlock Register"]
    pub unlock1: crate::Reg<unlock::UNLOCK_SPEC>,
    #[doc = "0x3e4 - Access Status Register"]
    pub asr1: crate::Reg<asr::ASR_SPEC>,
    _reserved164: [u8; 0x10],
    #[doc = "0x3f8 - Parameter Register"]
    pub parameter1: crate::Reg<parameter::PARAMETER_SPEC>,
    #[doc = "0x3fc - Version Register"]
    pub version1: crate::Reg<version::VERSION_SPEC>,
    #[doc = "0x400 - GPIO Enable Register"]
    pub gper2: crate::Reg<gper::GPER_SPEC>,
    #[doc = "0x404 - GPIO Enable Register - Set"]
    pub gpers2: crate::Reg<gpers::GPERS_SPEC>,
    #[doc = "0x408 - GPIO Enable Register - Clear"]
    pub gperc2: crate::Reg<gperc::GPERC_SPEC>,
    #[doc = "0x40c - GPIO Enable Register - Toggle"]
    pub gpert2: crate::Reg<gpert::GPERT_SPEC>,
    #[doc = "0x410 - Peripheral Mux Register 0"]
    pub pmr02: crate::Reg<pmr0::PMR0_SPEC>,
    #[doc = "0x414 - Peripheral Mux Register 0 - Set"]
    pub pmr0s2: crate::Reg<pmr0s::PMR0S_SPEC>,
    #[doc = "0x418 - Peripheral Mux Register 0 - Clear"]
    pub pmr0c2: crate::Reg<pmr0c::PMR0C_SPEC>,
    #[doc = "0x41c - Peripheral Mux Register 0 - Toggle"]
    pub pmr0t2: crate::Reg<pmr0t::PMR0T_SPEC>,
    #[doc = "0x420 - Peripheral Mux Register 1"]
    pub pmr12: crate::Reg<pmr1::PMR1_SPEC>,
    #[doc = "0x424 - Peripheral Mux Register 1 - Set"]
    pub pmr1s2: crate::Reg<pmr1s::PMR1S_SPEC>,
    #[doc = "0x428 - Peripheral Mux Register 1 - Clear"]
    pub pmr1c2: crate::Reg<pmr1c::PMR1C_SPEC>,
    #[doc = "0x42c - Peripheral Mux Register 1 - Toggle"]
    pub pmr1t2: crate::Reg<pmr1t::PMR1T_SPEC>,
    #[doc = "0x430 - Peripheral Mux Register 2"]
    pub pmr22: crate::Reg<pmr2::PMR2_SPEC>,
    #[doc = "0x434 - Peripheral Mux Register 2 - Set"]
    pub pmr2s2: crate::Reg<pmr2s::PMR2S_SPEC>,
    #[doc = "0x438 - Peripheral Mux Register 2 - Clear"]
    pub pmr2c2: crate::Reg<pmr2c::PMR2C_SPEC>,
    #[doc = "0x43c - Peripheral Mux Register 2 - Toggle"]
    pub pmr2t2: crate::Reg<pmr2t::PMR2T_SPEC>,
    #[doc = "0x440 - Output Driver Enable Register"]
    pub oder2: crate::Reg<oder::ODER_SPEC>,
    #[doc = "0x444 - Output Driver Enable Register - Set"]
    pub oders2: crate::Reg<oders::ODERS_SPEC>,
    #[doc = "0x448 - Output Driver Enable Register - Clear"]
    pub oderc2: crate::Reg<oderc::ODERC_SPEC>,
    #[doc = "0x44c - Output Driver Enable Register - Toggle"]
    pub odert2: crate::Reg<odert::ODERT_SPEC>,
    #[doc = "0x450 - Output Value Register"]
    pub ovr2: crate::Reg<ovr::OVR_SPEC>,
    #[doc = "0x454 - Output Value Register - Set"]
    pub ovrs2: crate::Reg<ovrs::OVRS_SPEC>,
    #[doc = "0x458 - Output Value Register - Clear"]
    pub ovrc2: crate::Reg<ovrc::OVRC_SPEC>,
    #[doc = "0x45c - Output Value Register - Toggle"]
    pub ovrt2: crate::Reg<ovrt::OVRT_SPEC>,
    #[doc = "0x460 - Pin Value Register"]
    pub pvr2: crate::Reg<pvr::PVR_SPEC>,
    _reserved191: [u8; 0x0c],
    #[doc = "0x470 - Pull-up Enable Register"]
    pub puer2: crate::Reg<puer::PUER_SPEC>,
    #[doc = "0x474 - Pull-up Enable Register - Set"]
    pub puers2: crate::Reg<puers::PUERS_SPEC>,
    #[doc = "0x478 - Pull-up Enable Register - Clear"]
    pub puerc2: crate::Reg<puerc::PUERC_SPEC>,
    #[doc = "0x47c - Pull-up Enable Register - Toggle"]
    pub puert2: crate::Reg<puert::PUERT_SPEC>,
    #[doc = "0x480 - Pull-down Enable Register"]
    pub pder2: crate::Reg<pder::PDER_SPEC>,
    #[doc = "0x484 - Pull-down Enable Register - Set"]
    pub pders2: crate::Reg<pders::PDERS_SPEC>,
    #[doc = "0x488 - Pull-down Enable Register - Clear"]
    pub pderc2: crate::Reg<pderc::PDERC_SPEC>,
    #[doc = "0x48c - Pull-down Enable Register - Toggle"]
    pub pdert2: crate::Reg<pdert::PDERT_SPEC>,
    #[doc = "0x490 - Interrupt Enable Register"]
    pub ier2: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x494 - Interrupt Enable Register - Set"]
    pub iers2: crate::Reg<iers::IERS_SPEC>,
    #[doc = "0x498 - Interrupt Enable Register - Clear"]
    pub ierc2: crate::Reg<ierc::IERC_SPEC>,
    #[doc = "0x49c - Interrupt Enable Register - Toggle"]
    pub iert2: crate::Reg<iert::IERT_SPEC>,
    #[doc = "0x4a0 - Interrupt Mode Register 0"]
    pub imr02: crate::Reg<imr0::IMR0_SPEC>,
    #[doc = "0x4a4 - Interrupt Mode Register 0 - Set"]
    pub imr0s2: crate::Reg<imr0s::IMR0S_SPEC>,
    #[doc = "0x4a8 - Interrupt Mode Register 0 - Clear"]
    pub imr0c2: crate::Reg<imr0c::IMR0C_SPEC>,
    #[doc = "0x4ac - Interrupt Mode Register 0 - Toggle"]
    pub imr0t2: crate::Reg<imr0t::IMR0T_SPEC>,
    #[doc = "0x4b0 - Interrupt Mode Register 1"]
    pub imr12: crate::Reg<imr1::IMR1_SPEC>,
    #[doc = "0x4b4 - Interrupt Mode Register 1 - Set"]
    pub imr1s2: crate::Reg<imr1s::IMR1S_SPEC>,
    #[doc = "0x4b8 - Interrupt Mode Register 1 - Clear"]
    pub imr1c2: crate::Reg<imr1c::IMR1C_SPEC>,
    #[doc = "0x4bc - Interrupt Mode Register 1 - Toggle"]
    pub imr1t2: crate::Reg<imr1t::IMR1T_SPEC>,
    #[doc = "0x4c0 - Glitch Filter Enable Register"]
    pub gfer2: crate::Reg<gfer::GFER_SPEC>,
    #[doc = "0x4c4 - Glitch Filter Enable Register - Set"]
    pub gfers2: crate::Reg<gfers::GFERS_SPEC>,
    #[doc = "0x4c8 - Glitch Filter Enable Register - Clear"]
    pub gferc2: crate::Reg<gferc::GFERC_SPEC>,
    #[doc = "0x4cc - Glitch Filter Enable Register - Toggle"]
    pub gfert2: crate::Reg<gfert::GFERT_SPEC>,
    #[doc = "0x4d0 - Interrupt Flag Register"]
    pub ifr2: crate::Reg<ifr::IFR_SPEC>,
    _reserved216: [u8; 0x04],
    #[doc = "0x4d8 - Interrupt Flag Register - Clear"]
    pub ifrc2: crate::Reg<ifrc::IFRC_SPEC>,
    _reserved217: [u8; 0x04],
    #[doc = "0x4e0 - Open Drain Mode Register"]
    pub odmer2: crate::Reg<odmer::ODMER_SPEC>,
    #[doc = "0x4e4 - Open Drain Mode Register - Set"]
    pub odmers2: crate::Reg<odmers::ODMERS_SPEC>,
    #[doc = "0x4e8 - Open Drain Mode Register - Clear"]
    pub odmerc2: crate::Reg<odmerc::ODMERC_SPEC>,
    #[doc = "0x4ec - Open Drain Mode Register - Toggle"]
    pub odmert2: crate::Reg<odmert::ODMERT_SPEC>,
    _reserved221: [u8; 0x10],
    #[doc = "0x500 - Output Driving Capability Register 0"]
    pub odcr02: crate::Reg<odcr0::ODCR0_SPEC>,
    #[doc = "0x504 - Output Driving Capability Register 0 - Set"]
    pub odcr0s2: crate::Reg<odcr0s::ODCR0S_SPEC>,
    #[doc = "0x508 - Output Driving Capability Register 0 - Clear"]
    pub odcr0c2: crate::Reg<odcr0c::ODCR0C_SPEC>,
    #[doc = "0x50c - Output Driving Capability Register 0 - Toggle"]
    pub odcr0t2: crate::Reg<odcr0t::ODCR0T_SPEC>,
    #[doc = "0x510 - Output Driving Capability Register 1"]
    pub odcr12: crate::Reg<odcr1::ODCR1_SPEC>,
    #[doc = "0x514 - Output Driving Capability Register 1 - Set"]
    pub odcr1s2: crate::Reg<odcr1s::ODCR1S_SPEC>,
    #[doc = "0x518 - Output Driving Capability Register 1 - Clear"]
    pub odcr1c2: crate::Reg<odcr1c::ODCR1C_SPEC>,
    #[doc = "0x51c - Output Driving Capability Register 1 - Toggle"]
    pub odcr1t2: crate::Reg<odcr1t::ODCR1T_SPEC>,
    _reserved229: [u8; 0x10],
    #[doc = "0x530 - Output Slew Rate Register 0"]
    pub osrr02: crate::Reg<osrr0::OSRR0_SPEC>,
    #[doc = "0x534 - Output Slew Rate Register 0 - Set"]
    pub osrr0s2: crate::Reg<osrr0s::OSRR0S_SPEC>,
    #[doc = "0x538 - Output Slew Rate Register 0 - Clear"]
    pub osrr0c2: crate::Reg<osrr0c::OSRR0C_SPEC>,
    #[doc = "0x53c - Output Slew Rate Register 0 - Toggle"]
    pub osrr0t2: crate::Reg<osrr0t::OSRR0T_SPEC>,
    _reserved233: [u8; 0x20],
    #[doc = "0x560 - Schmitt Trigger Enable Register"]
    pub ster2: crate::Reg<ster::STER_SPEC>,
    #[doc = "0x564 - Schmitt Trigger Enable Register - Set"]
    pub sters2: crate::Reg<sters::STERS_SPEC>,
    #[doc = "0x568 - Schmitt Trigger Enable Register - Clear"]
    pub sterc2: crate::Reg<sterc::STERC_SPEC>,
    #[doc = "0x56c - Schmitt Trigger Enable Register - Toggle"]
    pub stert2: crate::Reg<stert::STERT_SPEC>,
    _reserved237: [u8; 0x10],
    #[doc = "0x580 - Event Enable Register"]
    pub ever2: crate::Reg<ever::EVER_SPEC>,
    #[doc = "0x584 - Event Enable Register - Set"]
    pub evers2: crate::Reg<evers::EVERS_SPEC>,
    #[doc = "0x588 - Event Enable Register - Clear"]
    pub everc2: crate::Reg<everc::EVERC_SPEC>,
    #[doc = "0x58c - Event Enable Register - Toggle"]
    pub evert2: crate::Reg<evert::EVERT_SPEC>,
    _reserved241: [u8; 0x10],
    #[doc = "0x5a0 - Lock Register"]
    pub lock2: crate::Reg<lock::LOCK_SPEC>,
    #[doc = "0x5a4 - Lock Register - Set"]
    pub locks2: crate::Reg<locks::LOCKS_SPEC>,
    #[doc = "0x5a8 - Lock Register - Clear"]
    pub lockc2: crate::Reg<lockc::LOCKC_SPEC>,
    #[doc = "0x5ac - Lock Register - Toggle"]
    pub lockt2: crate::Reg<lockt::LOCKT_SPEC>,
    _reserved245: [u8; 0x30],
    #[doc = "0x5e0 - Unlock Register"]
    pub unlock2: crate::Reg<unlock::UNLOCK_SPEC>,
    #[doc = "0x5e4 - Access Status Register"]
    pub asr2: crate::Reg<asr::ASR_SPEC>,
    _reserved247: [u8; 0x10],
    #[doc = "0x5f8 - Parameter Register"]
    pub parameter2: crate::Reg<parameter::PARAMETER_SPEC>,
    #[doc = "0x5fc - Version Register"]
    pub version2: crate::Reg<version::VERSION_SPEC>,
}
#[doc = "ASR register accessor: an alias for `Reg<ASR_SPEC>`"]
pub type ASR = crate::Reg<asr::ASR_SPEC>;
#[doc = "Access Status Register"]
pub mod asr;
#[doc = "EVERC register accessor: an alias for `Reg<EVERC_SPEC>`"]
pub type EVERC = crate::Reg<everc::EVERC_SPEC>;
#[doc = "Event Enable Register - Clear"]
pub mod everc;
#[doc = "EVERS register accessor: an alias for `Reg<EVERS_SPEC>`"]
pub type EVERS = crate::Reg<evers::EVERS_SPEC>;
#[doc = "Event Enable Register - Set"]
pub mod evers;
#[doc = "EVERT register accessor: an alias for `Reg<EVERT_SPEC>`"]
pub type EVERT = crate::Reg<evert::EVERT_SPEC>;
#[doc = "Event Enable Register - Toggle"]
pub mod evert;
#[doc = "EVER register accessor: an alias for `Reg<EVER_SPEC>`"]
pub type EVER = crate::Reg<ever::EVER_SPEC>;
#[doc = "Event Enable Register"]
pub mod ever;
#[doc = "GFERC register accessor: an alias for `Reg<GFERC_SPEC>`"]
pub type GFERC = crate::Reg<gferc::GFERC_SPEC>;
#[doc = "Glitch Filter Enable Register - Clear"]
pub mod gferc;
#[doc = "GFERS register accessor: an alias for `Reg<GFERS_SPEC>`"]
pub type GFERS = crate::Reg<gfers::GFERS_SPEC>;
#[doc = "Glitch Filter Enable Register - Set"]
pub mod gfers;
#[doc = "GFERT register accessor: an alias for `Reg<GFERT_SPEC>`"]
pub type GFERT = crate::Reg<gfert::GFERT_SPEC>;
#[doc = "Glitch Filter Enable Register - Toggle"]
pub mod gfert;
#[doc = "GFER register accessor: an alias for `Reg<GFER_SPEC>`"]
pub type GFER = crate::Reg<gfer::GFER_SPEC>;
#[doc = "Glitch Filter Enable Register"]
pub mod gfer;
#[doc = "GPERC register accessor: an alias for `Reg<GPERC_SPEC>`"]
pub type GPERC = crate::Reg<gperc::GPERC_SPEC>;
#[doc = "GPIO Enable Register - Clear"]
pub mod gperc;
#[doc = "GPERS register accessor: an alias for `Reg<GPERS_SPEC>`"]
pub type GPERS = crate::Reg<gpers::GPERS_SPEC>;
#[doc = "GPIO Enable Register - Set"]
pub mod gpers;
#[doc = "GPERT register accessor: an alias for `Reg<GPERT_SPEC>`"]
pub type GPERT = crate::Reg<gpert::GPERT_SPEC>;
#[doc = "GPIO Enable Register - Toggle"]
pub mod gpert;
#[doc = "GPER register accessor: an alias for `Reg<GPER_SPEC>`"]
pub type GPER = crate::Reg<gper::GPER_SPEC>;
#[doc = "GPIO Enable Register"]
pub mod gper;
#[doc = "IERC register accessor: an alias for `Reg<IERC_SPEC>`"]
pub type IERC = crate::Reg<ierc::IERC_SPEC>;
#[doc = "Interrupt Enable Register - Clear"]
pub mod ierc;
#[doc = "IERS register accessor: an alias for `Reg<IERS_SPEC>`"]
pub type IERS = crate::Reg<iers::IERS_SPEC>;
#[doc = "Interrupt Enable Register - Set"]
pub mod iers;
#[doc = "IERT register accessor: an alias for `Reg<IERT_SPEC>`"]
pub type IERT = crate::Reg<iert::IERT_SPEC>;
#[doc = "Interrupt Enable Register - Toggle"]
pub mod iert;
#[doc = "IER register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "IFRC register accessor: an alias for `Reg<IFRC_SPEC>`"]
pub type IFRC = crate::Reg<ifrc::IFRC_SPEC>;
#[doc = "Interrupt Flag Register - Clear"]
pub mod ifrc;
#[doc = "IFR register accessor: an alias for `Reg<IFR_SPEC>`"]
pub type IFR = crate::Reg<ifr::IFR_SPEC>;
#[doc = "Interrupt Flag Register"]
pub mod ifr;
#[doc = "IMR0C register accessor: an alias for `Reg<IMR0C_SPEC>`"]
pub type IMR0C = crate::Reg<imr0c::IMR0C_SPEC>;
#[doc = "Interrupt Mode Register 0 - Clear"]
pub mod imr0c;
#[doc = "IMR0S register accessor: an alias for `Reg<IMR0S_SPEC>`"]
pub type IMR0S = crate::Reg<imr0s::IMR0S_SPEC>;
#[doc = "Interrupt Mode Register 0 - Set"]
pub mod imr0s;
#[doc = "IMR0T register accessor: an alias for `Reg<IMR0T_SPEC>`"]
pub type IMR0T = crate::Reg<imr0t::IMR0T_SPEC>;
#[doc = "Interrupt Mode Register 0 - Toggle"]
pub mod imr0t;
#[doc = "IMR1C register accessor: an alias for `Reg<IMR1C_SPEC>`"]
pub type IMR1C = crate::Reg<imr1c::IMR1C_SPEC>;
#[doc = "Interrupt Mode Register 1 - Clear"]
pub mod imr1c;
#[doc = "IMR1S register accessor: an alias for `Reg<IMR1S_SPEC>`"]
pub type IMR1S = crate::Reg<imr1s::IMR1S_SPEC>;
#[doc = "Interrupt Mode Register 1 - Set"]
pub mod imr1s;
#[doc = "IMR1T register accessor: an alias for `Reg<IMR1T_SPEC>`"]
pub type IMR1T = crate::Reg<imr1t::IMR1T_SPEC>;
#[doc = "Interrupt Mode Register 1 - Toggle"]
pub mod imr1t;
#[doc = "IMR0 register accessor: an alias for `Reg<IMR0_SPEC>`"]
pub type IMR0 = crate::Reg<imr0::IMR0_SPEC>;
#[doc = "Interrupt Mode Register 0"]
pub mod imr0;
#[doc = "IMR1 register accessor: an alias for `Reg<IMR1_SPEC>`"]
pub type IMR1 = crate::Reg<imr1::IMR1_SPEC>;
#[doc = "Interrupt Mode Register 1"]
pub mod imr1;
#[doc = "LOCKC register accessor: an alias for `Reg<LOCKC_SPEC>`"]
pub type LOCKC = crate::Reg<lockc::LOCKC_SPEC>;
#[doc = "Lock Register - Clear"]
pub mod lockc;
#[doc = "LOCKS register accessor: an alias for `Reg<LOCKS_SPEC>`"]
pub type LOCKS = crate::Reg<locks::LOCKS_SPEC>;
#[doc = "Lock Register - Set"]
pub mod locks;
#[doc = "LOCKT register accessor: an alias for `Reg<LOCKT_SPEC>`"]
pub type LOCKT = crate::Reg<lockt::LOCKT_SPEC>;
#[doc = "Lock Register - Toggle"]
pub mod lockt;
#[doc = "LOCK register accessor: an alias for `Reg<LOCK_SPEC>`"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "Lock Register"]
pub mod lock;
#[doc = "ODCR0C register accessor: an alias for `Reg<ODCR0C_SPEC>`"]
pub type ODCR0C = crate::Reg<odcr0c::ODCR0C_SPEC>;
#[doc = "Output Driving Capability Register 0 - Clear"]
pub mod odcr0c;
#[doc = "ODCR0S register accessor: an alias for `Reg<ODCR0S_SPEC>`"]
pub type ODCR0S = crate::Reg<odcr0s::ODCR0S_SPEC>;
#[doc = "Output Driving Capability Register 0 - Set"]
pub mod odcr0s;
#[doc = "ODCR0T register accessor: an alias for `Reg<ODCR0T_SPEC>`"]
pub type ODCR0T = crate::Reg<odcr0t::ODCR0T_SPEC>;
#[doc = "Output Driving Capability Register 0 - Toggle"]
pub mod odcr0t;
#[doc = "ODCR1C register accessor: an alias for `Reg<ODCR1C_SPEC>`"]
pub type ODCR1C = crate::Reg<odcr1c::ODCR1C_SPEC>;
#[doc = "Output Driving Capability Register 1 - Clear"]
pub mod odcr1c;
#[doc = "ODCR1S register accessor: an alias for `Reg<ODCR1S_SPEC>`"]
pub type ODCR1S = crate::Reg<odcr1s::ODCR1S_SPEC>;
#[doc = "Output Driving Capability Register 1 - Set"]
pub mod odcr1s;
#[doc = "ODCR1T register accessor: an alias for `Reg<ODCR1T_SPEC>`"]
pub type ODCR1T = crate::Reg<odcr1t::ODCR1T_SPEC>;
#[doc = "Output Driving Capability Register 1 - Toggle"]
pub mod odcr1t;
#[doc = "ODCR0 register accessor: an alias for `Reg<ODCR0_SPEC>`"]
pub type ODCR0 = crate::Reg<odcr0::ODCR0_SPEC>;
#[doc = "Output Driving Capability Register 0"]
pub mod odcr0;
#[doc = "ODCR1 register accessor: an alias for `Reg<ODCR1_SPEC>`"]
pub type ODCR1 = crate::Reg<odcr1::ODCR1_SPEC>;
#[doc = "Output Driving Capability Register 1"]
pub mod odcr1;
#[doc = "ODERC register accessor: an alias for `Reg<ODERC_SPEC>`"]
pub type ODERC = crate::Reg<oderc::ODERC_SPEC>;
#[doc = "Output Driver Enable Register - Clear"]
pub mod oderc;
#[doc = "ODERS register accessor: an alias for `Reg<ODERS_SPEC>`"]
pub type ODERS = crate::Reg<oders::ODERS_SPEC>;
#[doc = "Output Driver Enable Register - Set"]
pub mod oders;
#[doc = "ODERT register accessor: an alias for `Reg<ODERT_SPEC>`"]
pub type ODERT = crate::Reg<odert::ODERT_SPEC>;
#[doc = "Output Driver Enable Register - Toggle"]
pub mod odert;
#[doc = "ODER register accessor: an alias for `Reg<ODER_SPEC>`"]
pub type ODER = crate::Reg<oder::ODER_SPEC>;
#[doc = "Output Driver Enable Register"]
pub mod oder;
#[doc = "ODMERC register accessor: an alias for `Reg<ODMERC_SPEC>`"]
pub type ODMERC = crate::Reg<odmerc::ODMERC_SPEC>;
#[doc = "Open Drain Mode Register - Clear"]
pub mod odmerc;
#[doc = "ODMERS register accessor: an alias for `Reg<ODMERS_SPEC>`"]
pub type ODMERS = crate::Reg<odmers::ODMERS_SPEC>;
#[doc = "Open Drain Mode Register - Set"]
pub mod odmers;
#[doc = "ODMERT register accessor: an alias for `Reg<ODMERT_SPEC>`"]
pub type ODMERT = crate::Reg<odmert::ODMERT_SPEC>;
#[doc = "Open Drain Mode Register - Toggle"]
pub mod odmert;
#[doc = "ODMER register accessor: an alias for `Reg<ODMER_SPEC>`"]
pub type ODMER = crate::Reg<odmer::ODMER_SPEC>;
#[doc = "Open Drain Mode Register"]
pub mod odmer;
#[doc = "OSRR0C register accessor: an alias for `Reg<OSRR0C_SPEC>`"]
pub type OSRR0C = crate::Reg<osrr0c::OSRR0C_SPEC>;
#[doc = "Output Slew Rate Register 0 - Clear"]
pub mod osrr0c;
#[doc = "OSRR0S register accessor: an alias for `Reg<OSRR0S_SPEC>`"]
pub type OSRR0S = crate::Reg<osrr0s::OSRR0S_SPEC>;
#[doc = "Output Slew Rate Register 0 - Set"]
pub mod osrr0s;
#[doc = "OSRR0T register accessor: an alias for `Reg<OSRR0T_SPEC>`"]
pub type OSRR0T = crate::Reg<osrr0t::OSRR0T_SPEC>;
#[doc = "Output Slew Rate Register 0 - Toggle"]
pub mod osrr0t;
#[doc = "OSRR0 register accessor: an alias for `Reg<OSRR0_SPEC>`"]
pub type OSRR0 = crate::Reg<osrr0::OSRR0_SPEC>;
#[doc = "Output Slew Rate Register 0"]
pub mod osrr0;
#[doc = "OVRC register accessor: an alias for `Reg<OVRC_SPEC>`"]
pub type OVRC = crate::Reg<ovrc::OVRC_SPEC>;
#[doc = "Output Value Register - Clear"]
pub mod ovrc;
#[doc = "OVRS register accessor: an alias for `Reg<OVRS_SPEC>`"]
pub type OVRS = crate::Reg<ovrs::OVRS_SPEC>;
#[doc = "Output Value Register - Set"]
pub mod ovrs;
#[doc = "OVRT register accessor: an alias for `Reg<OVRT_SPEC>`"]
pub type OVRT = crate::Reg<ovrt::OVRT_SPEC>;
#[doc = "Output Value Register - Toggle"]
pub mod ovrt;
#[doc = "OVR register accessor: an alias for `Reg<OVR_SPEC>`"]
pub type OVR = crate::Reg<ovr::OVR_SPEC>;
#[doc = "Output Value Register"]
pub mod ovr;
#[doc = "PARAMETER register accessor: an alias for `Reg<PARAMETER_SPEC>`"]
pub type PARAMETER = crate::Reg<parameter::PARAMETER_SPEC>;
#[doc = "Parameter Register"]
pub mod parameter;
#[doc = "PDERC register accessor: an alias for `Reg<PDERC_SPEC>`"]
pub type PDERC = crate::Reg<pderc::PDERC_SPEC>;
#[doc = "Pull-down Enable Register - Clear"]
pub mod pderc;
#[doc = "PDERS register accessor: an alias for `Reg<PDERS_SPEC>`"]
pub type PDERS = crate::Reg<pders::PDERS_SPEC>;
#[doc = "Pull-down Enable Register - Set"]
pub mod pders;
#[doc = "PDERT register accessor: an alias for `Reg<PDERT_SPEC>`"]
pub type PDERT = crate::Reg<pdert::PDERT_SPEC>;
#[doc = "Pull-down Enable Register - Toggle"]
pub mod pdert;
#[doc = "PDER register accessor: an alias for `Reg<PDER_SPEC>`"]
pub type PDER = crate::Reg<pder::PDER_SPEC>;
#[doc = "Pull-down Enable Register"]
pub mod pder;
#[doc = "PMR0C register accessor: an alias for `Reg<PMR0C_SPEC>`"]
pub type PMR0C = crate::Reg<pmr0c::PMR0C_SPEC>;
#[doc = "Peripheral Mux Register 0 - Clear"]
pub mod pmr0c;
#[doc = "PMR0S register accessor: an alias for `Reg<PMR0S_SPEC>`"]
pub type PMR0S = crate::Reg<pmr0s::PMR0S_SPEC>;
#[doc = "Peripheral Mux Register 0 - Set"]
pub mod pmr0s;
#[doc = "PMR0T register accessor: an alias for `Reg<PMR0T_SPEC>`"]
pub type PMR0T = crate::Reg<pmr0t::PMR0T_SPEC>;
#[doc = "Peripheral Mux Register 0 - Toggle"]
pub mod pmr0t;
#[doc = "PMR1C register accessor: an alias for `Reg<PMR1C_SPEC>`"]
pub type PMR1C = crate::Reg<pmr1c::PMR1C_SPEC>;
#[doc = "Peripheral Mux Register 1 - Clear"]
pub mod pmr1c;
#[doc = "PMR1S register accessor: an alias for `Reg<PMR1S_SPEC>`"]
pub type PMR1S = crate::Reg<pmr1s::PMR1S_SPEC>;
#[doc = "Peripheral Mux Register 1 - Set"]
pub mod pmr1s;
#[doc = "PMR1T register accessor: an alias for `Reg<PMR1T_SPEC>`"]
pub type PMR1T = crate::Reg<pmr1t::PMR1T_SPEC>;
#[doc = "Peripheral Mux Register 1 - Toggle"]
pub mod pmr1t;
#[doc = "PMR2C register accessor: an alias for `Reg<PMR2C_SPEC>`"]
pub type PMR2C = crate::Reg<pmr2c::PMR2C_SPEC>;
#[doc = "Peripheral Mux Register 2 - Clear"]
pub mod pmr2c;
#[doc = "PMR2S register accessor: an alias for `Reg<PMR2S_SPEC>`"]
pub type PMR2S = crate::Reg<pmr2s::PMR2S_SPEC>;
#[doc = "Peripheral Mux Register 2 - Set"]
pub mod pmr2s;
#[doc = "PMR2T register accessor: an alias for `Reg<PMR2T_SPEC>`"]
pub type PMR2T = crate::Reg<pmr2t::PMR2T_SPEC>;
#[doc = "Peripheral Mux Register 2 - Toggle"]
pub mod pmr2t;
#[doc = "PMR0 register accessor: an alias for `Reg<PMR0_SPEC>`"]
pub type PMR0 = crate::Reg<pmr0::PMR0_SPEC>;
#[doc = "Peripheral Mux Register 0"]
pub mod pmr0;
#[doc = "PMR1 register accessor: an alias for `Reg<PMR1_SPEC>`"]
pub type PMR1 = crate::Reg<pmr1::PMR1_SPEC>;
#[doc = "Peripheral Mux Register 1"]
pub mod pmr1;
#[doc = "PMR2 register accessor: an alias for `Reg<PMR2_SPEC>`"]
pub type PMR2 = crate::Reg<pmr2::PMR2_SPEC>;
#[doc = "Peripheral Mux Register 2"]
pub mod pmr2;
#[doc = "PUERC register accessor: an alias for `Reg<PUERC_SPEC>`"]
pub type PUERC = crate::Reg<puerc::PUERC_SPEC>;
#[doc = "Pull-up Enable Register - Clear"]
pub mod puerc;
#[doc = "PUERS register accessor: an alias for `Reg<PUERS_SPEC>`"]
pub type PUERS = crate::Reg<puers::PUERS_SPEC>;
#[doc = "Pull-up Enable Register - Set"]
pub mod puers;
#[doc = "PUERT register accessor: an alias for `Reg<PUERT_SPEC>`"]
pub type PUERT = crate::Reg<puert::PUERT_SPEC>;
#[doc = "Pull-up Enable Register - Toggle"]
pub mod puert;
#[doc = "PUER register accessor: an alias for `Reg<PUER_SPEC>`"]
pub type PUER = crate::Reg<puer::PUER_SPEC>;
#[doc = "Pull-up Enable Register"]
pub mod puer;
#[doc = "PVR register accessor: an alias for `Reg<PVR_SPEC>`"]
pub type PVR = crate::Reg<pvr::PVR_SPEC>;
#[doc = "Pin Value Register"]
pub mod pvr;
#[doc = "STERC register accessor: an alias for `Reg<STERC_SPEC>`"]
pub type STERC = crate::Reg<sterc::STERC_SPEC>;
#[doc = "Schmitt Trigger Enable Register - Clear"]
pub mod sterc;
#[doc = "STERS register accessor: an alias for `Reg<STERS_SPEC>`"]
pub type STERS = crate::Reg<sters::STERS_SPEC>;
#[doc = "Schmitt Trigger Enable Register - Set"]
pub mod sters;
#[doc = "STERT register accessor: an alias for `Reg<STERT_SPEC>`"]
pub type STERT = crate::Reg<stert::STERT_SPEC>;
#[doc = "Schmitt Trigger Enable Register - Toggle"]
pub mod stert;
#[doc = "STER register accessor: an alias for `Reg<STER_SPEC>`"]
pub type STER = crate::Reg<ster::STER_SPEC>;
#[doc = "Schmitt Trigger Enable Register"]
pub mod ster;
#[doc = "UNLOCK register accessor: an alias for `Reg<UNLOCK_SPEC>`"]
pub type UNLOCK = crate::Reg<unlock::UNLOCK_SPEC>;
#[doc = "Unlock Register"]
pub mod unlock;
#[doc = "VERSION register accessor: an alias for `Reg<VERSION_SPEC>`"]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = "Version Register"]
pub mod version;
