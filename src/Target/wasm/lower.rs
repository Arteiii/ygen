mod mov;
mod math;
mod cmove;
mod stack;
mod ret;
mod br;
mod cmp;
mod cast;
mod call;
mod switch;

use crate::{CodeGen::{MCInstr, MachineInstr}, Optimizations::Optimize, Target::CallConv};

pub(crate) fn wasm_lower_instr(sink: &mut Vec<super::asm::WasmMCInstr>, instr: MachineInstr) {
    match instr.mnemonic.to_owned() {
        crate::CodeGen::MachineMnemonic::Move => mov::wasm_lower_mov(sink, &instr),
        crate::CodeGen::MachineMnemonic::Add => math::wasm_lower_add(sink, &instr),
        crate::CodeGen::MachineMnemonic::And => math::wasm_lower_and(sink, &instr),
        crate::CodeGen::MachineMnemonic::Div => math::wasm_lower_div(sink, &instr),
        crate::CodeGen::MachineMnemonic::Mul => math::wasm_lower_mul(sink, &instr),
        crate::CodeGen::MachineMnemonic::Or => math::wasm_lower_or(sink, &instr),
        crate::CodeGen::MachineMnemonic::Sub => math::wasm_lower_sub(sink, &instr),
        crate::CodeGen::MachineMnemonic::Xor => math::wasm_lower_xor(sink, &instr),
        crate::CodeGen::MachineMnemonic::Rem => math::wasm_lower_rem(sink, &instr),
        crate::CodeGen::MachineMnemonic::Neg => math::wasm_lower_neg(sink, &instr),
        crate::CodeGen::MachineMnemonic::Shl => math::wasm_lower_shl(sink, &instr),
        crate::CodeGen::MachineMnemonic::Shr => math::wasm_lower_shr(sink, &instr),
        crate::CodeGen::MachineMnemonic::FMove => mov::wasm_lower_mov(sink, &instr),
        crate::CodeGen::MachineMnemonic::FAdd => math::wasm_lower_add(sink, &instr),
        crate::CodeGen::MachineMnemonic::FAnd => math::wasm_lower_and(sink, &instr),
        crate::CodeGen::MachineMnemonic::FDiv => math::wasm_lower_div(sink, &instr),
        crate::CodeGen::MachineMnemonic::FMul => math::wasm_lower_mul(sink, &instr),
        crate::CodeGen::MachineMnemonic::FOr => math::wasm_lower_or(sink, &instr),
        crate::CodeGen::MachineMnemonic::FSub => math::wasm_lower_sub(sink, &instr),
        crate::CodeGen::MachineMnemonic::FXor => math::wasm_lower_xor(sink, &instr),
        crate::CodeGen::MachineMnemonic::FRem => math::wasm_lower_rem(sink, &instr),
        crate::CodeGen::MachineMnemonic::FNeg => math::wasm_lower_neg(sink, &instr),
        crate::CodeGen::MachineMnemonic::FShl => math::wasm_lower_shl(sink, &instr),
        crate::CodeGen::MachineMnemonic::FShr => math::wasm_lower_shr(sink, &instr),
        crate::CodeGen::MachineMnemonic::FCompare(cmp_mode) => cmp::wasm_lower_cmp(sink, &instr, cmp_mode),
        crate::CodeGen::MachineMnemonic::FCast(start_ty) => cast::wasm_lower_cast(sink, &instr, start_ty),
        crate::CodeGen::MachineMnemonic::BrCond(iftrue, iffalse) => br::wasm_lower_brcond(sink, &instr, iftrue, iffalse),
        crate::CodeGen::MachineMnemonic::Compare(cmp_mode) => cmp::wasm_lower_cmp(sink, &instr, cmp_mode),
        crate::CodeGen::MachineMnemonic::Zext(start_ty) => cast::wasm_lower_cast(sink, &instr, start_ty),
        crate::CodeGen::MachineMnemonic::Downcast(start_ty) => cast::wasm_lower_cast(sink, &instr, start_ty),
        crate::CodeGen::MachineMnemonic::Call(func) => call::wasm_lower_call(sink, &instr, func),
        crate::CodeGen::MachineMnemonic::Br(block) => br::wasm_lower_br(sink, &instr, block),
        crate::CodeGen::MachineMnemonic::Return => ret::wasm_lower_return(sink, &instr),
        crate::CodeGen::MachineMnemonic::AdressLoad(constant) => stack::wasm_lower_adress_load(sink, &instr, constant),
        crate::CodeGen::MachineMnemonic::StackAlloc => stack::wasm_lower_alloc(sink, &instr),
        crate::CodeGen::MachineMnemonic::Store => stack::wasm_lower_store(sink, &instr),
        crate::CodeGen::MachineMnemonic::Load => stack::wasm_lower_load(sink, &instr),
        crate::CodeGen::MachineMnemonic::Prolog => {},
        crate::CodeGen::MachineMnemonic::Epilog => {},
        crate::CodeGen::MachineMnemonic::Push => panic!("illegal instruction for "),
        crate::CodeGen::MachineMnemonic::PushCleanup => {},
        crate::CodeGen::MachineMnemonic::CallStackPrepare => {},
        crate::CodeGen::MachineMnemonic::CallStackRedo => {},
        crate::CodeGen::MachineMnemonic::AdrMove => mov::wasm_lower_mov(sink, &instr),
        crate::CodeGen::MachineMnemonic::Switch(cases) => switch::wasm_lower_switch(sink, &instr, cases),
        crate::CodeGen::MachineMnemonic::MovIfZero => cmove::wasm_lower_cmove(sink, &instr),
        crate::CodeGen::MachineMnemonic::MovIfNotZero => cmove::wasm_lower_cmovne(sink, &instr),
    }
}

/// The function used for lowering general `MachineInstr` into `MCInstr`
pub(crate) fn wasm_lower(_: CallConv, instrs: Vec<MachineInstr>) -> Vec<Box<dyn MCInstr>> {
    let mut out = Vec::new();

    for instr in instrs {
        wasm_lower_instr(&mut out, instr.clone());
    }

    out.optimize();

    let mut mc_instrs = vec![];

    for instr in out {
        mc_instrs.push( instr.into() );
    }

    mc_instrs
}