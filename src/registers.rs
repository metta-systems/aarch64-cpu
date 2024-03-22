//! Processor core registers

#![allow(unused_attributes)]

#[macro_use]
mod macros;

mod actlr_el1;
mod actlr_el2;
mod actlr_el3;
mod apdakeyhi_el1;
mod apdakeylo_el1;
mod apdbkeyhi_el1;
mod apdbkeylo_el1;
mod apgakeyhi_el1;
mod apgakeylo_el1;
mod apiakeyhi_el1;
mod apiakeylo_el1;
mod apibkeyhi_el1;
mod apibkeylo_el1;
mod ccsidr_el1;
mod clidr_el1;
mod cntfrq_el0;
mod cnthctl_el2;
mod cnthp_ctl_el2;
mod cntkctl_el1;
mod cntp_ctl_el0;
mod cntp_cval_el0;
mod cntp_tval_el0;
mod cntpct_el0;
mod cntpoff_el2;
mod cntv_ctl_el0;
mod cntv_cval_el0;
mod cntv_tval_el0;
mod cntvct_el0;
mod cntvoff_el2;
mod cpacr_el1;
mod cptr_el2;
mod csselr_el1;
mod currentel;
mod dacr32_el2;
mod daif;
mod dbgdtr_el0;
mod dbgdtrrx_el0;
mod dbgdtrtx_el0;
mod elr_el1;
mod elr_el2;
mod elr_el3;
mod esr_el1;
mod esr_el2;
mod esr_el3;
mod far_el1;
mod far_el2;
mod far_el3;
mod fp;
mod hcr_el2;
mod hpfar_el2;
mod icc_ctlr_el1;
mod icc_sre_el2;
mod ich_ap0r0_el2;
mod ich_ap0r1_el2;
mod ich_ap0r2_el2;
mod ich_ap0r3_el2;
mod ich_ap1r0_el2;
mod ich_ap1r1_el2;
mod ich_ap1r2_el2;
mod ich_ap1r3_el2;
mod ich_hcr_el2;
mod ich_lr0_el2;
mod ich_lr10_el2;
mod ich_lr11_el2;
mod ich_lr12_el2;
mod ich_lr13_el2;
mod ich_lr14_el2;
mod ich_lr15_el2;
mod ich_lr1_el2;
mod ich_lr2_el2;
mod ich_lr3_el2;
mod ich_lr4_el2;
mod ich_lr5_el2;
mod ich_lr6_el2;
mod ich_lr7_el2;
mod ich_lr8_el2;
mod ich_lr9_el2;
mod ich_misr_el2;
mod ich_vmcr_el2;
mod ich_vtr_el2;
mod id_aa64afr0_el1;
mod id_aa64afr1_el1;
mod id_aa64dfr0_el1;
mod id_aa64dfr1_el1;
mod id_aa64isar0_el1;
mod id_aa64isar1_el1;
mod id_aa64mmfr0_el1;
mod id_aa64mmfr1_el1;
mod id_aa64mmfr2_el1;
mod id_aa64pfr0_el1;
mod id_aa64pfr1_el1;
mod lr;
mod mair_el1;
mod mair_el2;
mod mdccsr_el0;
mod midr_el1;
mod mpidr_el1;
mod oslar_el1;
mod par_el1;
mod rvbar_el1;
mod rvbar_el2;
mod rvbar_el3;
mod scr_el3;
mod sctlr_el1;
mod sctlr_el2;
mod sctlr_el3;
mod sp;
mod sp_el0;
mod sp_el1;
mod spsel;
mod spsr_el1;
mod spsr_el2;
mod spsr_el3;
mod tcr_el1;
mod tcr_el2;
mod tpidr_el0;
mod tpidr_el1;
mod tpidr_el2;
mod tpidrro_el0;
mod ttbr0_el1;
mod ttbr0_el2;
mod ttbr1_el1;
mod vbar_el1;
mod vbar_el2;
mod vbar_el3;
mod vtcr_el2;
mod vttbr_el2;
mod vmpidr_el2;

pub use actlr_el1::ACTLR_EL1;
pub use actlr_el2::ACTLR_EL2;
pub use actlr_el3::ACTLR_EL3;
pub use apdakeyhi_el1::APDAKEYHI_EL1;
pub use apdakeylo_el1::APDAKEYLO_EL1;
pub use apdbkeyhi_el1::APDBKEYHI_EL1;
pub use apdbkeylo_el1::APDBKEYLO_EL1;
pub use apgakeyhi_el1::APGAKEYHI_EL1;
pub use apgakeylo_el1::APGAKEYLO_EL1;
pub use apiakeyhi_el1::APIAKEYHI_EL1;
pub use apiakeylo_el1::APIAKEYLO_EL1;
pub use apibkeyhi_el1::APIBKEYHI_EL1;
pub use apibkeylo_el1::APIBKEYLO_EL1;
pub use ccsidr_el1::CCSIDR_EL1;
pub use clidr_el1::CLIDR_EL1;
pub use cntfrq_el0::CNTFRQ_EL0;
pub use cnthctl_el2::CNTHCTL_EL2;
pub use cnthp_ctl_el2::CNTHP_CTL_EL2;
pub use cntkctl_el1::CNTKCTL_EL1;
pub use cntp_ctl_el0::CNTP_CTL_EL0;
pub use cntp_cval_el0::CNTP_CVAL_EL0;
pub use cntp_tval_el0::CNTP_TVAL_EL0;
pub use cntpct_el0::CNTPCT_EL0;
pub use cntpoff_el2::CNTPOFF_EL2;
pub use cntv_ctl_el0::CNTV_CTL_EL0;
pub use cntv_cval_el0::CNTV_CVAL_EL0;
pub use cntv_tval_el0::CNTV_TVAL_EL0;
pub use cntvct_el0::CNTVCT_EL0;
pub use cntvoff_el2::CNTVOFF_EL2;
pub use cpacr_el1::CPACR_EL1;
pub use cptr_el2::CPTR_EL2;
pub use csselr_el1::CSSELR_EL1;
pub use currentel::CurrentEL;
pub use dacr32_el2::DACR32_EL2;
pub use daif::DAIF;
pub use dbgdtr_el0::DBGDTR_EL0;
pub use dbgdtrrx_el0::DBGDTRRX_EL0;
pub use dbgdtrtx_el0::DBGDTRTX_EL0;
pub use elr_el1::ELR_EL1;
pub use elr_el2::ELR_EL2;
pub use elr_el3::ELR_EL3;
pub use esr_el1::ESR_EL1;
pub use esr_el2::ESR_EL2;
pub use esr_el3::ESR_EL3;
pub use far_el1::FAR_EL1;
pub use far_el2::FAR_EL2;
pub use far_el3::FAR_EL3;
pub use fp::FP;
pub use hcr_el2::HCR_EL2;
pub use hpfar_el2::HPFAR_EL2;
pub use icc_ctlr_el1::ICC_CTLR_EL1;
pub use icc_sre_el2::ICC_SRE_EL2;
pub use ich_ap0r0_el2::ICH_AP0R0_EL2;
pub use ich_ap0r1_el2::ICH_AP0R1_EL2;
pub use ich_ap0r2_el2::ICH_AP0R2_EL2;
pub use ich_ap0r3_el2::ICH_AP0R3_EL2;
pub use ich_ap1r0_el2::ICH_AP1R0_EL2;
pub use ich_ap1r1_el2::ICH_AP1R1_EL2;
pub use ich_ap1r2_el2::ICH_AP1R2_EL2;
pub use ich_ap1r3_el2::ICH_AP1R3_EL2;
pub use ich_hcr_el2::ICH_HCR_EL2;
pub use ich_lr0_el2::ICH_LR0_EL2;
pub use ich_lr10_el2::ICH_LR10_EL2;
pub use ich_lr11_el2::ICH_LR11_EL2;
pub use ich_lr12_el2::ICH_LR12_EL2;
pub use ich_lr13_el2::ICH_LR13_EL2;
pub use ich_lr14_el2::ICH_LR14_EL2;
pub use ich_lr15_el2::ICH_LR15_EL2;
pub use ich_lr1_el2::ICH_LR1_EL2;
pub use ich_lr2_el2::ICH_LR2_EL2;
pub use ich_lr3_el2::ICH_LR3_EL2;
pub use ich_lr4_el2::ICH_LR4_EL2;
pub use ich_lr5_el2::ICH_LR5_EL2;
pub use ich_lr6_el2::ICH_LR6_EL2;
pub use ich_lr7_el2::ICH_LR7_EL2;
pub use ich_lr8_el2::ICH_LR8_EL2;
pub use ich_lr9_el2::ICH_LR9_EL2;
pub use ich_misr_el2::ICH_MISR_EL2;
pub use ich_vmcr_el2::ICH_VMCR_EL2;
pub use ich_vtr_el2::ICH_VTR_EL2;
pub use id_aa64afr0_el1::ID_AA64AFR0_EL1;
pub use id_aa64afr1_el1::ID_AA64AFR1_EL1;
pub use id_aa64dfr0_el1::ID_AA64DFR0_EL1;
pub use id_aa64dfr1_el1::ID_AA64DFR1_EL1;
pub use id_aa64isar0_el1::ID_AA64ISAR0_EL1;
pub use id_aa64isar1_el1::ID_AA64ISAR1_EL1;
pub use id_aa64mmfr0_el1::ID_AA64MMFR0_EL1;
pub use id_aa64mmfr1_el1::ID_AA64MMFR1_EL1;
pub use id_aa64mmfr2_el1::ID_AA64MMFR2_EL1;
pub use id_aa64pfr0_el1::ID_AA64PFR0_EL1;
pub use id_aa64pfr1_el1::ID_AA64PFR1_EL1;
pub use lr::LR;
pub use mair_el1::MAIR_EL1;
pub use mair_el2::MAIR_EL2;
pub use mdccsr_el0::MDCCSR_EL0;
pub use midr_el1::MIDR_EL1;
pub use mpidr_el1::MPIDR_EL1;
pub use oslar_el1::OSLAR_EL1;
pub use par_el1::PAR_EL1;
pub use rvbar_el1::RVBAR_EL1;
pub use rvbar_el2::RVBAR_EL2;
pub use rvbar_el3::RVBAR_EL3;
pub use scr_el3::SCR_EL3;
pub use sctlr_el1::SCTLR_EL1;
pub use sctlr_el2::SCTLR_EL2;
pub use sctlr_el3::SCTLR_EL3;
pub use sp::SP;
pub use sp_el0::SP_EL0;
pub use sp_el1::SP_EL1;
pub use spsel::SPSel;
pub use spsr_el1::SPSR_EL1;
pub use spsr_el2::SPSR_EL2;
pub use spsr_el3::SPSR_EL3;
pub use tcr_el1::TCR_EL1;
pub use tcr_el2::TCR_EL2;
pub use tpidr_el0::TPIDR_EL0;
pub use tpidr_el1::TPIDR_EL1;
pub use tpidr_el2::TPIDR_EL2;
pub use tpidrro_el0::TPIDRRO_EL0;
pub use ttbr0_el1::TTBR0_EL1;
pub use ttbr0_el2::TTBR0_EL2;
pub use ttbr1_el1::TTBR1_EL1;
pub use vbar_el1::VBAR_EL1;
pub use vbar_el2::VBAR_EL2;
pub use vbar_el3::VBAR_EL3;
pub use vtcr_el2::VTCR_EL2;
pub use vttbr_el2::VTTBR_EL2;
pub use vmpidr_el2::VMPIDR_EL2;

pub trait TTBR {}

impl TTBR for vttbr_el2::Reg{}
impl TTBR for ttbr0_el1::Reg{}
impl TTBR for ttbr1_el1::Reg{}
impl TTBR for ttbr0_el2::Reg{}

#[doc(inline)]
pub use tock_registers::interfaces::{ReadWriteable, Readable, Writeable};
