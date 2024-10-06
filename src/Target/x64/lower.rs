use crate::prelude::CmpMode;
use crate::CodeGen::{MCInstr, MachineCallingConvention, MachineInstr, MachineMnemonic, MachineOperand};
use crate::Optimizations::Optimize;
use crate::Target::CallConv;
use crate::IR::{BlockId, Type};

use super::{instr::{MemOp, Mnemonic, Operand, X64MCInstr}, x64Reg};

macro_rules! x64_stack {
    ($off:expr) => {
        Operand::Mem(x64Reg::Rbp - $off)
    };
}

pub(crate) fn x64_lower_instr(conv: CallConv, sink: &mut Vec<X64MCInstr>, instr: MachineInstr) {
    match &instr.mnemonic {
        MachineMnemonic::Move => x64_lower_move(sink, &instr),
        MachineMnemonic::Add => x64_lower_add(sink, &instr),
        MachineMnemonic::And => x64_lower_and(sink, &instr),
        MachineMnemonic::Div => x64_lower_div(sink, &instr),
        MachineMnemonic::Mul => x64_lower_mul(sink, &instr),
        MachineMnemonic::Or => x64_lower_or(sink, &instr),
        MachineMnemonic::Sub => x64_lower_sub(sink, &instr),
        MachineMnemonic::Xor => x64_lower_xor(sink, &instr),
        MachineMnemonic::Zext => x64_lower_zext(sink, &instr),
        MachineMnemonic::Downcast => x64_lower_downcast(sink, &instr),
        MachineMnemonic::Call(to) => x64_lower_call(conv, sink, &instr, to),
        MachineMnemonic::Return => x64_lower_return(sink, &instr),
        MachineMnemonic::AdressLoad(to) => x64_lower_adr_load(sink, &instr, to),
        MachineMnemonic::Br(to) => x64_lower_br(sink, &instr, to),
        MachineMnemonic::BrCond(iftrue, iffalse) => x64_lower_cond_br(sink, &instr, iftrue, iffalse),
        MachineMnemonic::Compare(mode) => x64_lower_cmp(sink, &instr, mode),
        MachineMnemonic::Prolog => x64_lower_prolog(sink, &instr),
        MachineMnemonic::Epilog => x64_lower_epilog(sink, &instr),
        MachineMnemonic::StackAlloc => x64_lower_salloc(sink, &instr),
        MachineMnemonic::Store => x64_lower_store(sink, &instr),
        MachineMnemonic::Load => x64_lower_load(sink, &instr),
        MachineMnemonic::Push => x64_lower_push(sink, &instr),
        MachineMnemonic::PushCleanup => x64_lower_push_cleanup(sink, &instr),
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
        MachineMnemonic::AdrMove => x64_lower_adrm(sink, &instr),
        MachineMnemonic::Switch(cases) => x64_lower_switch(sink, &instr, cases),
        MachineMnemonic::Neg => x64_lower_neg(sink, &instr),
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

fn x64_lower_div(_sink: &mut Vec<X64MCInstr>, _instr: &MachineInstr) {
    todo!()
}
fn x64_lower_move(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr) {
    let op1 = instr.operands.get(0).expect("expected a first operand");
    let out = instr.out.expect("expected a output operand");

    let op1 = match op1 {
        crate::CodeGen::MachineOperand::Imm(i) => Operand::Imm(*i),
        crate::CodeGen::MachineOperand::Reg(reg) => match reg {
            crate::CodeGen::Reg::x64(x64) => Operand::Reg(*x64),
        },
        crate::CodeGen::MachineOperand::Stack(off) => x64_stack!((*off as u32)),
    };
    
    let out = match out {
        crate::CodeGen::MachineOperand::Imm(i) => Operand::Imm(i),
        crate::CodeGen::MachineOperand::Reg(reg) => match reg {
            crate::CodeGen::Reg::x64(x64) => Operand::Reg(x64),
        },
        crate::CodeGen::MachineOperand::Stack(off) => x64_stack!((off as u32)),
    };

    if let Operand::Mem(_) = out {
        if let Operand::Reg(_) = op1 {} else {
            sink.push( X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rax), op1) );
            sink.push( X64MCInstr::with2(Mnemonic::Mov, out, Operand::Reg(x64Reg::Rax)) );
            return;
        }
    }

    sink.push( X64MCInstr::with2(Mnemonic::Mov, out, op1).into() );
}
fn x64_lower_mul(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr) {
    let op1 = instr.operands.get(0).expect("expected a first operand");
    let op2 = instr.operands.get(1).expect("expected a second operand");
    let out = instr.out.expect("expected a output operand");

    let op1 = match op1 {
        crate::CodeGen::MachineOperand::Imm(i) => Operand::Imm(*i),
        crate::CodeGen::MachineOperand::Reg(reg) => match reg {
            crate::CodeGen::Reg::x64(x64) => Operand::Reg(*x64),
        },
        crate::CodeGen::MachineOperand::Stack(off) => x64_stack!((*off as u32)),
    };

    
    let op2 = match op2 {
        crate::CodeGen::MachineOperand::Imm(i) => Operand::Imm(*i),
        crate::CodeGen::MachineOperand::Reg(reg) => match reg {
            crate::CodeGen::Reg::x64(x64) => Operand::Reg(*x64),
        },
        crate::CodeGen::MachineOperand::Stack(off) => x64_stack!((*off as u32)),
    };
    
    let out = match out {
        crate::CodeGen::MachineOperand::Imm(i) => Operand::Imm(i),
        crate::CodeGen::MachineOperand::Reg(reg) => match reg {
            crate::CodeGen::Reg::x64(x64) => Operand::Reg(x64),
        },
        crate::CodeGen::MachineOperand::Stack(off) => x64_stack!((off as u32)),
    };

    let mnemonic = if instr.meta.signed() {
        Mnemonic::Imul
    } else {
        Mnemonic::Mul
    };

    if out != Operand::Reg(x64Reg::Rdx.sub_ty(instr.meta)) {
        sink.push( X64MCInstr::with1(Mnemonic::Push, Operand::Reg(x64Reg::Rdx)).into() );
    }

    // MUL node here:
    // mov rax, op1
    // mul/imul op2
    // mov out, rax
    // RDX = is upper slice which will just get destroyed

    let rax = || Operand::Reg(x64Reg::Rax.sub_ty(instr.meta));

    sink.push(X64MCInstr::with2(Mnemonic::Mov, rax(), op1));
    
    // mul/imul only accept r/m
    if let Operand::Imm(_) = op2 {
        sink.push(X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rbx.sub_ty(instr.meta)), op2));
        sink.push(X64MCInstr::with1(mnemonic, Operand::Reg(x64Reg::Rbx.sub_ty(instr.meta))));
    } else if let Operand::Mem(_) = op2 {
        sink.push(X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rbx.sub_ty(instr.meta)), op2));
        sink.push(X64MCInstr::with1(mnemonic, Operand::Reg(x64Reg::Rbx.sub_ty(instr.meta))));
    } else {
        sink.push(X64MCInstr::with1(mnemonic, op2));
    }

    sink.push(X64MCInstr::with2(Mnemonic::Mov, out.to_owned(), rax()));

    if out != Operand::Reg(x64Reg::Rdx.sub_ty(instr.meta)) {
        sink.push( X64MCInstr::with1(Mnemonic::Pop, Operand::Reg(x64Reg::Rdx)).into() );
    }
}
fn x64_lower_zext(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr) {
    
    let op1 = instr.operands.get(0).expect("expected a first operand");
    let op2 = instr.operands.get(0).expect("expected a secound operand");
    let out = instr.out.expect("expected a output operand");

    let mut movxz = false;

    let op1 = match op1 {
        crate::CodeGen::MachineOperand::Imm(i) => Operand::Imm(*i),
        crate::CodeGen::MachineOperand::Reg(reg) => match reg {
            crate::CodeGen::Reg::x64(x64) => Operand::Reg(*x64),
        },
        crate::CodeGen::MachineOperand::Stack(off) => x64_stack!((*off as u32)),
    };

    let op2 = match op2 {
        crate::CodeGen::MachineOperand::Imm(i) => Operand::Imm(*i),
        crate::CodeGen::MachineOperand::Reg(reg) => match reg {
            crate::CodeGen::Reg::x64(x64) => Operand::Reg(*x64),
        },
        crate::CodeGen::MachineOperand::Stack(off) => x64_stack!((*off as u32)),
    };
    
    let out = match out {
        crate::CodeGen::MachineOperand::Imm(i) => Operand::Imm(i),
        crate::CodeGen::MachineOperand::Reg(reg) => match reg {
            crate::CodeGen::Reg::x64(x64) => Operand::Reg(x64),
        },
        crate::CodeGen::MachineOperand::Stack(off) => x64_stack!((off as u32)),
    };

    if let Operand::Reg(op1) = op1 {
        if let Operand::Reg(op2) = op2 {
            if (op1.is_gr16() | op1.is_gr8()) && (op2.is_gr32() | op2.is_gr64()) { // movxz allowes a gr8/16 zext into gr32/64
                movxz = true;
            }
        }
    }

    if movxz {
        let tmp = Operand::Reg(x64Reg::Rax.sub_ty(instr.meta));

        sink.push(X64MCInstr::with2(Mnemonic::Mov, tmp.clone(), op1));
        sink.push(X64MCInstr::with2(Mnemonic::Movzx, tmp.clone(), op2));
        sink.push(X64MCInstr::with2(Mnemonic::Mov, out, tmp));
    } else {
        let tmp = Operand::Reg(x64Reg::Rax.sub_ty(instr.meta));

        if op1 == out {
            sink.push(X64MCInstr::with2(Mnemonic::Mov, op1,  op2));
        } else {
            sink.push(X64MCInstr::with2(Mnemonic::Mov, tmp.clone(), op1));
            sink.push(X64MCInstr::with2(Mnemonic::Mov, tmp.clone(), op2));
            sink.push(X64MCInstr::with2(Mnemonic::Mov, out, tmp));
        }
    }

}
fn x64_lower_downcast(_sink: &mut Vec<X64MCInstr>, _instr: &MachineInstr) {
    todo!()
}
fn x64_lower_call(conv: CallConv, sink: &mut Vec<X64MCInstr>, _: &MachineInstr, target: &String) {   
    let func = target;

    if conv.reset_eax() {
        sink.push( X64MCInstr::with2(Mnemonic::Xor, Operand::Reg(x64Reg::Eax), Operand::Reg(x64Reg::Eax)) );
    }

    sink.push( X64MCInstr::with1(Mnemonic::Call, Operand::Imm(0)).into() );
    sink.push( X64MCInstr::with1(Mnemonic::Link, Operand::LinkDestination(func.to_string(), -4)).into() );
}
fn x64_lower_return(sink: &mut Vec<X64MCInstr>, _: &MachineInstr) {
    sink.push( X64MCInstr::with0(Mnemonic::Ret).into() )
}
fn x64_lower_adr_load(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr, symbol: &String) {
    let out = instr.out.expect("expected a output operand");

    let out = match out {
        crate::CodeGen::MachineOperand::Imm(i) => Operand::Imm(i),
        crate::CodeGen::MachineOperand::Reg(reg) => match reg {
            crate::CodeGen::Reg::x64(x64) => Operand::Reg(x64),
        },
        crate::CodeGen::MachineOperand::Stack(off) => x64_stack!((off as u32)),
    };

    sink.push(
        X64MCInstr::with2(Mnemonic::Lea, Operand::Reg(x64Reg::Rax), Operand::Mem(MemOp { base: None, index: None, scale: 1, displ: 1, rip: true })).into()
    );
    sink.push(
        X64MCInstr::with1(Mnemonic::Link, Operand::LinkDestination(symbol.to_string(), -4)).into()
    );
    sink.push(
        X64MCInstr::with2(Mnemonic::Mov, out, Operand::Reg(x64Reg::Rax)).into()
    );
}
fn x64_lower_br(sink: &mut Vec<X64MCInstr>, _: &MachineInstr, symbol: &String) {
    let target = Operand::BlockLinkDestination(symbol.to_owned(), -4);

    sink.push(
        X64MCInstr::with1(Mnemonic::Jmp, Operand::Imm(0))
    );


    sink.push(
        X64MCInstr::with1(Mnemonic::Link, target)
    );
}
fn x64_lower_cond_br(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr, iftrue: &String, iffalse: &String) {
    let src = instr.operands.get(0).expect("expected valid src operand at 1. place");
    let value = instr.operands.get(1).expect("expected valid value to compare at 1. place");

    let src = match src {
        crate::CodeGen::MachineOperand::Imm(_) => unreachable!(),
        crate::CodeGen::MachineOperand::Reg(reg) => match *reg {
            crate::CodeGen::Reg::x64(x64) => Operand::Reg(x64),
        },
        crate::CodeGen::MachineOperand::Stack(off) => x64_stack!((*off as u32)),
    };

    let value = match value {
        crate::CodeGen::MachineOperand::Imm(imm) => Operand::Imm(*imm),
        crate::CodeGen::MachineOperand::Reg(reg) => match *reg {
            crate::CodeGen::Reg::x64(x64) => Operand::Reg(x64),
        },
        crate::CodeGen::MachineOperand::Stack(off) => x64_stack!((*off as u32)),
    };

    if let Operand::Mem(_) = src {
        sink.push(X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rax), src));
        sink.push(X64MCInstr::with2(Mnemonic::Cmp, Operand::Reg(x64Reg::Rax), value));
    } else {
        sink.push(X64MCInstr::with2(Mnemonic::Cmp, src, value));
    }
    sink.push(X64MCInstr::with1(Mnemonic::Jne, Operand::Imm(0)));
    sink.push(X64MCInstr::with1(Mnemonic::Link, Operand::BlockLinkDestination(iftrue.to_owned(), -4))); // not 0
    sink.push(X64MCInstr::with1(Mnemonic::Jmp, Operand::Imm(0)));
    sink.push(X64MCInstr::with1(Mnemonic::Link, Operand::BlockLinkDestination(iffalse.to_owned(), -4))); // is 0
}
fn x64_lower_cmp(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr, mode: &CmpMode) {
    let ls = instr.operands.get(0).expect("expected valid src operand at 1. place");
    let rs = instr.operands.get(1).expect("expected valid value to compare at 2. place");

    let out = instr.out.expect("expected output");

    let out = match out {
        crate::CodeGen::MachineOperand::Reg(reg) => match reg {
            crate::CodeGen::Reg::x64(x64) => Operand::Reg(x64),
        },
        MachineOperand::Imm(imm) => Operand::Imm(imm),
        MachineOperand::Stack(stack) => x64_stack!(stack as u32),
    };

    let mut ls = match ls {
        crate::CodeGen::MachineOperand::Imm(_) => unreachable!(),
        crate::CodeGen::MachineOperand::Reg(reg) => match *reg {
            crate::CodeGen::Reg::x64(x64) => Operand::Reg(x64),
        },
        crate::CodeGen::MachineOperand::Stack(off) => x64_stack!((*off as u32)),
    };

    let mut rs = match rs {
        crate::CodeGen::MachineOperand::Imm(imm) => Operand::Imm(*imm),
        crate::CodeGen::MachineOperand::Reg(reg) => match *reg {
            crate::CodeGen::Reg::x64(x64) => Operand::Reg(x64),
        },
        crate::CodeGen::MachineOperand::Stack(off) => x64_stack!((*off as u32)),
    };

    if let Operand::Imm(_) = ls {
        let tmp = ls;
        ls = rs;
        rs = tmp;
    }

    if let Operand::Mem(_) = ls {
        if ls == out {
            sink.push(X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rbx), ls.clone()));
            sink.push(X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rax), Operand::Imm(0)));
            sink.push(X64MCInstr::with2(Mnemonic::Mov, ls, Operand::Reg(x64Reg::Rax)));
            sink.push(X64MCInstr::with2(Mnemonic::Cmp, Operand::Reg(x64Reg::Rbx), rs));
        } else if rs == out {
            sink.push(X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rbx), rs.clone()));
            sink.push(X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rax), Operand::Imm(0)));
            sink.push(X64MCInstr::with2(Mnemonic::Mov, rs, Operand::Reg(x64Reg::Rax)));
            sink.push(X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rax), ls));
            sink.push(X64MCInstr::with2(Mnemonic::Cmp, Operand::Reg(x64Reg::Rax), Operand::Reg(x64Reg::Rbx)));
        } else {
            if let Operand::Reg(_) = out {
                sink.push(X64MCInstr::with2(Mnemonic::Mov, out.clone(), Operand::Imm(0)));
            } else {
                sink.push(X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rax), Operand::Imm(0)));
                sink.push(X64MCInstr::with2(Mnemonic::Mov, out.clone(), Operand::Reg(x64Reg::Rax)));
            }
            sink.push(X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rax), ls));
            sink.push(X64MCInstr::with2(Mnemonic::Cmp, Operand::Reg(x64Reg::Rax), rs));
        }
    } else {
        if ls == out {
            sink.push(X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rax), ls.clone()));
            sink.push(X64MCInstr::with2(Mnemonic::Mov, ls, Operand::Imm(0)));
            sink.push(X64MCInstr::with2(Mnemonic::Cmp, Operand::Reg(x64Reg::Rax), rs));
        } else if rs == out {
            sink.push(X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rax), rs.clone()));
            sink.push(X64MCInstr::with2(Mnemonic::Mov, rs, Operand::Imm(0)));
            sink.push(X64MCInstr::with2(Mnemonic::Cmp, ls, Operand::Reg(x64Reg::Rax)));
        } else {
            if let Operand::Reg(_) = out {
                sink.push(X64MCInstr::with2(Mnemonic::Mov, out.clone(), Operand::Imm(0)));
            } else {
                sink.push(X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rax), Operand::Imm(0)));
                sink.push(X64MCInstr::with2(Mnemonic::Mov, out.clone(), Operand::Reg(x64Reg::Rax)));
            }
            sink.push(X64MCInstr::with2(Mnemonic::Cmp, ls, rs));
        }
    }

    let mne = match mode {
        CmpMode::Eqal => Mnemonic::Sete,
        CmpMode::NotEqal => Mnemonic::Setne,
        CmpMode::GreaterThan => Mnemonic::Setg,
        CmpMode::LessThan => Mnemonic::Setl,
        CmpMode::GreaterThanOrEqual => Mnemonic::Setge,
        CmpMode::LessThanOrEqual => Mnemonic::Setle,
    };

    sink.push( X64MCInstr::with1(mne, out) );
}

macro_rules! LowerSimpleMath {
    ($func:ident, $mnemonic:expr) => {
        fn $func(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr) {       
            let op1 = instr.operands.get(0).expect("expected a first operand");
            let op2 = instr.operands.get(1).expect("expected a second operand");
            let out = instr.out.expect("expected a output operand");

            let op1 = match op1 {
                crate::CodeGen::MachineOperand::Imm(i) => Operand::Imm(*i),
                crate::CodeGen::MachineOperand::Reg(reg) => match reg {
                    crate::CodeGen::Reg::x64(x64) => Operand::Reg(*x64),
                },
                crate::CodeGen::MachineOperand::Stack(off) => x64_stack!((*off as u32)),
            };

            
            let op2 = match op2 {
                crate::CodeGen::MachineOperand::Imm(i) => Operand::Imm(*i),
                crate::CodeGen::MachineOperand::Reg(reg) => match reg {
                    crate::CodeGen::Reg::x64(x64) => Operand::Reg(*x64),
                },
                crate::CodeGen::MachineOperand::Stack(off) => x64_stack!((*off as u32)),
            };
            
            let out = match out {
                crate::CodeGen::MachineOperand::Imm(i) => Operand::Imm(i),
                crate::CodeGen::MachineOperand::Reg(reg) => match reg {
                    crate::CodeGen::Reg::x64(x64) => Operand::Reg(x64),
                },
                crate::CodeGen::MachineOperand::Stack(off) => x64_stack!((off as u32)),
            };

            let tmp = || Operand::Reg(x64Reg::Rax.sub_ty(instr.meta));

            sink.push( X64MCInstr::with2(Mnemonic::Mov, tmp(), op1).into() );
            sink.push( X64MCInstr::with2($mnemonic, tmp(), op2).into() );
            sink.push( X64MCInstr::with2(Mnemonic::Mov, out, tmp()).into() );
        }
    };
}

LowerSimpleMath!(x64_lower_add, Mnemonic::Add);
LowerSimpleMath!(x64_lower_and, Mnemonic::And);
LowerSimpleMath!(x64_lower_or, Mnemonic::Or);
LowerSimpleMath!(x64_lower_sub, Mnemonic::Sub);
LowerSimpleMath!(x64_lower_xor, Mnemonic::Xor);

fn x64_lower_prolog(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr) {
    //sink.push( X64MCInstr::with0(Mnemonic::Endbr64) );
    sink.push( X64MCInstr::with1(Mnemonic::Push, Operand::Reg(x64Reg::Rbp) ) );
    sink.push( X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rbp), Operand::Reg(x64Reg::Rsp)  ) );
    if let Some(op0) = instr.operands.get(0) {
        let op0 = match op0 {
            crate::CodeGen::MachineOperand::Imm(imm) => Operand::Imm(*imm),
            crate::CodeGen::MachineOperand::Reg(reg) => {
                match reg {
                    crate::CodeGen::Reg::x64(x64) => Operand::Reg(*x64),
                }
            },
            crate::CodeGen::MachineOperand::Stack(off) => x64_stack!(*off as u32),
        };

        sink.push( X64MCInstr::with2(Mnemonic::Sub, Operand::Reg(x64Reg::Rbp),  op0) );
    }
}

fn x64_lower_epilog(sink: &mut Vec<X64MCInstr>, _: &MachineInstr) {
    sink.push( X64MCInstr::with1(Mnemonic::Pop, Operand::Reg(x64Reg::Rbp) ) );
}

fn x64_lower_salloc(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr) {
    let out = instr.out.expect("stack allocations need outputs");
    let offset = instr.operands.get(0).expect("stack allocations need one operand");

    let offset = match offset {
        MachineOperand::Imm(imm) => *imm,
        _ => panic!("stack allocations require one operand of type imm")
    };

    let out = match out {
        crate::CodeGen::MachineOperand::Imm(i) => Operand::Imm(i),
        crate::CodeGen::MachineOperand::Reg(reg) => match reg {
            crate::CodeGen::Reg::x64(x64) => Operand::Reg(x64),
        },
        crate::CodeGen::MachineOperand::Stack(off) => x64_stack!((off as u32)),
    };

    if let Operand::Mem(_) = out {
        let tmp = || Operand::Reg( x64Reg::Rax );

        sink.push(
            X64MCInstr::with2(Mnemonic::Lea, tmp(), x64_stack!(offset as u32))
        );
        sink.push(
            X64MCInstr::with2(Mnemonic::Mov, out, tmp())
        )
    } else {
        sink.push(
            X64MCInstr::with2(Mnemonic::Lea, out, x64_stack!(offset as u32))
        )
    }
}

fn x64_lower_store(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr) {
    let ptr = instr.out.expect("stack stores need a output");
    let value = instr.operands.get(0).expect("stack stores need one operand");

    let ptr = match ptr {
        MachineOperand::Imm(imm) => Operand::Imm(imm),
        MachineOperand::Reg(reg) => match reg {
            crate::CodeGen::Reg::x64(x64) => Operand::Reg(x64),
        },
        MachineOperand::Stack(off) => x64_stack!(off as u32),
    };
    
    let value = match value {
        MachineOperand::Imm(imm) => Operand::Imm(*imm),
        MachineOperand::Reg(reg) => match reg {
            crate::CodeGen::Reg::x64(x64) => Operand::Reg(*x64),
        },
        MachineOperand::Stack(off) => x64_stack!(*off as u32),
    };

    if let Operand::Reg(ptr) = ptr {
        if let Operand::Reg(_) = value {
            sink.push( 
                X64MCInstr::with2(Mnemonic::Mov, Operand::Mem(MemOp {
                    base: Some(ptr),
                    index: None,
                    scale: 1,
                    displ: 0,
                    rip: false,
                }), value)
            )
        } else {
            sink.push(
                X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rax), value)
            );
            sink.push( 
                X64MCInstr::with2(Mnemonic::Mov, Operand::Mem(MemOp {
                    base: Some(ptr),
                    index: None,
                    scale: 1,
                    displ: 0,
                    rip: false,
                }), Operand::Reg(x64Reg::Rax))
            )
        }
    } else {
        sink.push( 
            X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rax), value)
        );
    
        sink.push( 
            X64MCInstr::with2(Mnemonic::Mov, ptr, Operand::Reg(x64Reg::Rax))
        );
    }

}

fn x64_lower_load(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr) {
    let out = instr.out.expect("stack stores need a output");
    let ptr = instr.operands.get(0).expect("stack stores need one operand");

    let ptr = match ptr {
        MachineOperand::Imm(imm) => Operand::Imm(*imm),
        MachineOperand::Reg(reg) => match reg {
            crate::CodeGen::Reg::x64(x64) => Operand::Reg(*x64),
        },
        MachineOperand::Stack(off) => x64_stack!(*off as u32),
    };
    
    let out = match out {
        MachineOperand::Imm(imm) => Operand::Imm(imm),
        MachineOperand::Reg(reg) => match reg {
            crate::CodeGen::Reg::x64(x64) => Operand::Reg(x64),
        },
        MachineOperand::Stack(off) => x64_stack!(off as u32),
    };

    if let Operand::Reg(ptr) = ptr {
        if let Operand::Reg(_) = out {
            sink.push( 
                X64MCInstr::with2(Mnemonic::Mov, out, Operand::Mem(MemOp {
                    base: Some(ptr),
                    index: None,
                    scale: 1,
                    displ: 0,
                    rip: false,
                }))
            )
        } else {
            sink.push( 
                X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rax.sub_ty(instr.meta)), Operand::Mem(MemOp {
                    base: Some(ptr),
                    index: None,
                    scale: 1,
                    displ: 0,
                    rip: false,
                }))
            );

            sink.push(
                X64MCInstr::with2(Mnemonic::Mov, out, Operand::Reg(x64Reg::Rax.sub_ty(instr.meta)))
            );
        }
    } else {
        sink.push( 
            X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rax.sub_ty(instr.meta)), ptr)
        );
    
        sink.push( 
            X64MCInstr::with2(Mnemonic::Mov, out, Operand::Reg(x64Reg::Rax.sub_ty(instr.meta)))
        );
    }

}

fn x64_lower_push(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr) {
    let input = instr.operands.get(0).expect("push needs an operand");

    sink.push(
        X64MCInstr::with1(Mnemonic::Push, match input {
            MachineOperand::Imm(imm) => Operand::Imm(*imm),
            MachineOperand::Reg(reg) => {
                match *reg {
                    crate::CodeGen::Reg::x64(x64) => Operand::Reg(x64),
                }
            },
            MachineOperand::Stack(off) => x64_stack!(*off as u32),
        })
    );

    sink.push(X64MCInstr::with2(Mnemonic::Sub, Operand::Reg(x64Reg::Rsp), Operand::Imm(8))); // for 16 byte alignment
}

fn x64_lower_push_cleanup(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr) {
    sink.push( X64MCInstr::with1(Mnemonic::Pop, Operand::Reg(x64Reg::Rax.sub_ty(instr.meta))));
    sink.push(X64MCInstr::with2(Mnemonic::Add, Operand::Reg(x64Reg::Rsp), Operand::Imm(8))); // for 16 byte alignment
}

fn x64_lower_adrm(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr) {
    let op = instr.operands.get(0).expect("expected adrm expectes one operand");
    let out = instr.out.expect("expected adrm expectes one operand");

    let op = match op {
        MachineOperand::Stack(stack) => x64_stack!(*stack as u32),
        MachineOperand::Imm(imm) => Operand::Imm(*imm),
        MachineOperand::Reg(reg) => match *reg {
            crate::CodeGen::Reg::x64(x64_reg) => Operand::Reg(x64_reg),
        },
    };

    let out = match out {
        MachineOperand::Imm(imm) => Operand::Imm(imm),
        MachineOperand::Reg(reg) => match reg {
            crate::CodeGen::Reg::x64(x64) => Operand::Reg(x64),
        },
        MachineOperand::Stack(stack) => x64_stack!(stack as u32),
    };

    if let Operand::Reg(_) = out {
        if let Operand::Mem(_) = op {
            sink.push(X64MCInstr::with2(Mnemonic::Lea, out, op));
        } else {
            sink.push(X64MCInstr::with2(Mnemonic::Mov, out, op));
        }
    } else {
        if let Operand::Mem(_) = op {
            sink.push(X64MCInstr::with2(Mnemonic::Lea, Operand::Reg(x64Reg::Rax), op));
        } else {
            sink.push(X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rax), op));
        }
        sink.push(X64MCInstr::with2(Mnemonic::Mov, out, Operand::Reg(x64Reg::Rax)));
    }
}

fn x64_lower_switch(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr, cases: &Vec<(Type, BlockId)>) {
    let var = *instr.operands.get(0).expect("switch expectes an variable to switch");
    let mut var = match var {
        MachineOperand::Imm(imm) => Operand::Imm(imm),
        MachineOperand::Reg(x64) => match x64 {
            crate::CodeGen::Reg::x64(x64_reg) => Operand::Reg(x64_reg),
        },
        MachineOperand::Stack(stack) => x64_stack!(stack as u32),
    };

    if let Operand::Mem(_) = var {
        sink.push(
            X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rax), var)
        );

        var = Operand::Reg(x64Reg::Rax);
    }

    for (case_type, block) in cases {
        sink.push(
            X64MCInstr::with2(Mnemonic::Cmp, var.clone(), Operand::Imm(case_type.val() as i64)),
        ); 
        sink.push(
            X64MCInstr::with1(Mnemonic::Je, Operand::Imm(0))
        );
        sink.push(
            X64MCInstr::with1(Mnemonic::Link, Operand::BlockLinkDestination(block.name.to_owned(), -4))
        );
    }
}

fn x64_lower_neg(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr) {
    let out = instr.out.expect("neg expectes output");
    let op = instr.operands.get(0).expect("neg expectes operand");

    let out: Operand = out.into();
    let op: Operand = (*op).into();

    if op == out {
        sink.push(X64MCInstr::with1(Mnemonic::Neg, out));
        return;
    }

    if let Operand::Mem(_) = op {
        if let Operand::Reg(_) = out {
            sink.push(X64MCInstr::with2(Mnemonic::Mov, out.clone(), op));
            sink.push(X64MCInstr::with1(Mnemonic::Neg, out));
        } else {
            sink.push(X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rax), op));
            sink.push(X64MCInstr::with2(Mnemonic::Mov, out.clone(), Operand::Reg(x64Reg::Rax)));
            sink.push(X64MCInstr::with1(Mnemonic::Neg, out));
        }
    } else {
        sink.push(X64MCInstr::with2(Mnemonic::Mov, out.clone(), op));
        sink.push(X64MCInstr::with1(Mnemonic::Neg, out));
    }
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