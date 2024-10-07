use crate::CodeGen::{MCInstr, MachineCallingConvention, MachineInstr, MachineMnemonic, MachineOperand};
use crate::Optimizations::Optimize;
use crate::Target::CallConv;

mod adr;
mod br;
mod call;
mod cmp;
mod downcast;
mod mov;
mod math;
mod prolog;
mod push;
mod ret;
mod stack;
mod switch;
mod zext;

use super::{instr::{Mnemonic, Operand, X64MCInstr}, x64Reg};

macro_rules! x64_stack {
    ($off:expr) => {
        Operand::Mem(x64Reg::Rbp - $off)
    };
}

pub(crate) fn x64_lower_instr(conv: CallConv, sink: &mut Vec<X64MCInstr>, instr: MachineInstr) {
    match &instr.mnemonic {
        MachineMnemonic::Move =>                                          mov::x64_lower_move(sink, &instr),
        MachineMnemonic::Add =>                                           math::x64_lower_add(sink, &instr),
        MachineMnemonic::And =>                                           math::x64_lower_and(sink, &instr),
        MachineMnemonic::Div =>                                           math::x64_lower_div(sink, &instr),
        MachineMnemonic::Mul =>                                           math::x64_lower_mul(sink, &instr),
        MachineMnemonic::Or =>                                            math::x64_lower_or(sink, &instr),
        MachineMnemonic::Sub =>                                           math::x64_lower_sub(sink, &instr),
        MachineMnemonic::Xor =>                                           math::x64_lower_xor(sink, &instr),
        MachineMnemonic::Zext =>                                          zext::x64_lower_zext(sink, &instr),
        MachineMnemonic::Downcast =>                                      downcast::x64_lower_downcast(sink, &instr),
        MachineMnemonic::Call(to) =>                             call::x64_lower_call(conv, sink, &instr, to),
        MachineMnemonic::Return =>                                        ret::x64_lower_return(sink, &instr),
        MachineMnemonic::AdressLoad(to) =>                       adr::x64_lower_adr_load(sink, &instr, to),
        MachineMnemonic::Br(to) =>                               br::x64_lower_br(sink, &instr, to),
        MachineMnemonic::BrCond(iftrue, iffalse) =>     br::x64_lower_cond_br(sink, &instr, iftrue, iffalse),
        MachineMnemonic::Compare(mode) =>                       cmp::x64_lower_cmp(sink, &instr, mode),
        MachineMnemonic::Prolog =>                                        prolog::x64_lower_prolog(sink, &instr),
        MachineMnemonic::Epilog =>                                        prolog::x64_lower_epilog(sink, &instr),
        MachineMnemonic::StackAlloc =>                                    stack::x64_lower_salloc(sink, &instr),
        MachineMnemonic::Store =>                                         stack::x64_lower_store(sink, &instr),
        MachineMnemonic::Load =>                                          stack::x64_lower_load(sink, &instr),
        MachineMnemonic::Push =>                                          push::x64_lower_push(sink, &instr),
        MachineMnemonic::PushCleanup =>                                   push::x64_lower_push_cleanup(sink, &instr),
        MachineMnemonic::CallStackPrepare => {
            sink.push(X64MCInstr::with2(
                Mnemonic::Sub, Operand::Reg(x64Reg::Rsp), 
                Operand::Imm(
                    MachineCallingConvention {
                    call_conv: conv
                }.shadow(crate::Target::Arch::X86_64) - 8
            )));
        },MachineMnemonic::CallStackRedo => {
            sink.push(X64MCInstr::with2(
                Mnemonic::Add, Operand::Reg(x64Reg::Rsp), 
                Operand::Imm(
                    MachineCallingConvention {
                    call_conv: conv
                }.shadow(crate::Target::Arch::X86_64) - 8
            )));
        },
        MachineMnemonic::AdrMove =>                                      adr::x64_lower_adrm(sink, &instr),
        MachineMnemonic::Switch(cases) =>         switch::x64_lower_switch(sink, &instr, cases),
        MachineMnemonic::Neg =>                                          math::x64_lower_neg(sink, &instr),
    }
}

/// The function used for lowering general `MachineInstr` into `MCInstr`
pub(crate) fn x64_lower(conv: CallConv, instrs: Vec<MachineInstr>) -> Vec<Box<dyn MCInstr>> {
    let mut out = vec![
        X64MCInstr::with0(Mnemonic::StartOptimization)
    ];

    for instr in instrs {
        x64_lower_instr(conv, &mut out, instr);
    }

    out.optimize();

    let mut mc_instrs = vec![];

    for instr in out {
        mc_instrs.push( instr.into() );
    }

    mc_instrs
}

impl From<MachineOperand> for Operand {
    fn from(value: MachineOperand) -> Self {
        match value {
            MachineOperand::Stack(stack) => x64_stack!(stack as u32),
            MachineOperand::Imm(imm) => Operand::Imm(imm),
            MachineOperand::Reg(reg) => match reg {
                crate::CodeGen::Reg::x64(x64_reg) => Operand::Reg(x64_reg),
            },
        }
    }
}