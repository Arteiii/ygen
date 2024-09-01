use crate::CodeGen::{MCInstr, MachineInstr, MachineMnemonic};
use crate::Optimizations::Optimize;

use super::{instr::{MemOp, Mnemonic, Operand, X64MCInstr}, x64Reg};

fn x64_lower_instr(sink: &mut Vec<X64MCInstr>, instr: MachineInstr) {
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
        MachineMnemonic::Call(to) => x64_lower_call(sink, &instr, to),
        MachineMnemonic::Return => x64_lower_return(sink, &instr),
        MachineMnemonic::AdressLoad(to) => x64_lower_adr_load(sink, &instr, to),
    }
}

/// The function used for lowering general `MachineInstr` into `MCInstr`
pub(crate) fn x64_lower(instrs: Vec<MachineInstr>) -> Vec<Box<dyn MCInstr>> {
    let mut out = vec![
        X64MCInstr::with0(Mnemonic::StartOptimization)
    ];

    for instr in instrs {
        x64_lower_instr(&mut out, instr);
    }

    out.optimize();

    let mut mc_instrs = vec![];

    for instr in out {
        mc_instrs.push( instr.into() );
    }

    mc_instrs
}

fn x64_lower_add(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr) {
    let op1 = instr.operands.get(0).expect("expected a first operand");
    let op2 = instr.operands.get(1).expect("expected a second operand");
    let out = instr.out.expect("expected a output operand");

    let op1 = match op1 {
        crate::CodeGen::MachineOperand::Imm(i) => Operand::Imm(*i),
        crate::CodeGen::MachineOperand::Reg(reg) => match reg {
            crate::CodeGen::Reg::x64(x64) => Operand::Reg(*x64),
        },
    };

    
    let op2 = match op2 {
        crate::CodeGen::MachineOperand::Imm(i) => Operand::Imm(*i),
        crate::CodeGen::MachineOperand::Reg(reg) => match reg {
            crate::CodeGen::Reg::x64(x64) => Operand::Reg(*x64),
        },
    };
    
    let out = match out {
        crate::CodeGen::MachineOperand::Imm(i) => Operand::Imm(i),
        crate::CodeGen::MachineOperand::Reg(reg) => match reg {
            crate::CodeGen::Reg::x64(x64) => Operand::Reg(x64),
        },
    };

    sink.push( X64MCInstr::with2(Mnemonic::Mov, out.clone(), op1).into() );
    sink.push( X64MCInstr::with2(Mnemonic::Add, out, op2).into() );

}

fn x64_lower_div(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr) {
    todo!()
}
fn x64_lower_and(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr) {
    let op1 = instr.operands.get(0).expect("expected a first operand");
    let op2 = instr.operands.get(1).expect("expected a second operand");
    let out = instr.out.expect("expected a output operand");

    let op1 = match op1 {
        crate::CodeGen::MachineOperand::Imm(i) => Operand::Imm(*i),
        crate::CodeGen::MachineOperand::Reg(reg) => match reg {
            crate::CodeGen::Reg::x64(x64) => Operand::Reg(*x64),
        },
    };

    
    let op2 = match op2 {
        crate::CodeGen::MachineOperand::Imm(i) => Operand::Imm(*i),
        crate::CodeGen::MachineOperand::Reg(reg) => match reg {
            crate::CodeGen::Reg::x64(x64) => Operand::Reg(*x64),
        },
    };
    
    let out = match out {
        crate::CodeGen::MachineOperand::Imm(i) => Operand::Imm(i),
        crate::CodeGen::MachineOperand::Reg(reg) => match reg {
            crate::CodeGen::Reg::x64(x64) => Operand::Reg(x64),
        },
    };

    sink.push( X64MCInstr::with2(Mnemonic::Mov, out.clone(), op1).into() );
    sink.push( X64MCInstr::with2(Mnemonic::And, out, op2).into() );
}
fn x64_lower_move(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr) {
    
    let op1 = instr.operands.get(0).expect("expected a first operand");
    let out = instr.out.expect("expected a output operand");

    let op1 = match op1 {
        crate::CodeGen::MachineOperand::Imm(i) => Operand::Imm(*i),
        crate::CodeGen::MachineOperand::Reg(reg) => match reg {
            crate::CodeGen::Reg::x64(x64) => Operand::Reg(*x64),
        },
    };
    
    let out = match out {
        crate::CodeGen::MachineOperand::Imm(i) => Operand::Imm(i),
        crate::CodeGen::MachineOperand::Reg(reg) => match reg {
            crate::CodeGen::Reg::x64(x64) => Operand::Reg(x64),
        },
    };

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
    };

    
    let op2 = match op2 {
        crate::CodeGen::MachineOperand::Imm(i) => Operand::Imm(*i),
        crate::CodeGen::MachineOperand::Reg(reg) => match reg {
            crate::CodeGen::Reg::x64(x64) => Operand::Reg(*x64),
        },
    };
    
    let out = match out {
        crate::CodeGen::MachineOperand::Imm(i) => Operand::Imm(i),
        crate::CodeGen::MachineOperand::Reg(reg) => match reg {
            crate::CodeGen::Reg::x64(x64) => Operand::Reg(x64),
        },
    };

    let mnemonic = if instr.meta.signed() {
        Mnemonic::Imul
    } else {
        Mnemonic::Mul
    };

    sink.push( X64MCInstr::with1(Mnemonic::Push, Operand::Reg(x64Reg::Rdx)).into() );
    sink.push( X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rax.sub_ty(instr.meta)), op1).into() );
        
    if let Operand::Imm(_) = op2 {
        sink.push( X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rbx), op2).into() );
        sink.push( X64MCInstr::with1(mnemonic, Operand::Reg(x64Reg::Rbx)).into() );
    }
    else {
        sink.push( X64MCInstr::with1(mnemonic, op2).into() );
    }

    sink.push( X64MCInstr::with2(Mnemonic::Mov, out, Operand::Reg(x64Reg::Rax.sub_ty(instr.meta))).into() );
    sink.push( X64MCInstr::with1(Mnemonic::Pop, Operand::Reg(x64Reg::Rdx)).into() );
}

fn x64_lower_or(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr) {
    let op1 = instr.operands.get(0).expect("expected a first operand");
    let op2 = instr.operands.get(1).expect("expected a second operand");
    let out = instr.out.expect("expected a output operand");

    let op1 = match op1 {
        crate::CodeGen::MachineOperand::Imm(i) => Operand::Imm(*i),
        crate::CodeGen::MachineOperand::Reg(reg) => match reg {
            crate::CodeGen::Reg::x64(x64) => Operand::Reg(*x64),
        },
    };

    
    let op2 = match op2 {
        crate::CodeGen::MachineOperand::Imm(i) => Operand::Imm(*i),
        crate::CodeGen::MachineOperand::Reg(reg) => match reg {
            crate::CodeGen::Reg::x64(x64) => Operand::Reg(*x64),
        },
    };
    
    let out = match out {
        crate::CodeGen::MachineOperand::Imm(i) => Operand::Imm(i),
        crate::CodeGen::MachineOperand::Reg(reg) => match reg {
            crate::CodeGen::Reg::x64(x64) => Operand::Reg(x64),
        },
    };

    sink.push( X64MCInstr::with2(Mnemonic::Mov, out.clone(), op1).into() );
    sink.push( X64MCInstr::with2(Mnemonic::Or, out, op2).into() );
}
fn x64_lower_sub(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr) {
    let op1 = instr.operands.get(0).expect("expected a first operand");
    let op2 = instr.operands.get(1).expect("expected a second operand");
    let out = instr.out.expect("expected a output operand");

    let op1 = match op1 {
        crate::CodeGen::MachineOperand::Imm(i) => Operand::Imm(*i),
        crate::CodeGen::MachineOperand::Reg(reg) => match reg {
            crate::CodeGen::Reg::x64(x64) => Operand::Reg(*x64),
        },
    };

    
    let op2 = match op2 {
        crate::CodeGen::MachineOperand::Imm(i) => Operand::Imm(*i),
        crate::CodeGen::MachineOperand::Reg(reg) => match reg {
            crate::CodeGen::Reg::x64(x64) => Operand::Reg(*x64),
        },
    };
    
    let out = match out {
        crate::CodeGen::MachineOperand::Imm(i) => Operand::Imm(i),
        crate::CodeGen::MachineOperand::Reg(reg) => match reg {
            crate::CodeGen::Reg::x64(x64) => Operand::Reg(x64),
        },
    };

    sink.push( X64MCInstr::with2(Mnemonic::Mov, out.clone(), op1).into() );
    sink.push( X64MCInstr::with2(Mnemonic::Sub, out, op2).into() );
}
fn x64_lower_xor(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr) {
    let op1 = instr.operands.get(0).expect("expected a first operand");
    let op2 = instr.operands.get(1).expect("expected a second operand");
    let out = instr.out.expect("expected a output operand");

    let op1 = match op1 {
        crate::CodeGen::MachineOperand::Imm(i) => Operand::Imm(*i),
        crate::CodeGen::MachineOperand::Reg(reg) => match reg {
            crate::CodeGen::Reg::x64(x64) => Operand::Reg(*x64),
        },
    };

    
    let op2 = match op2 {
        crate::CodeGen::MachineOperand::Imm(i) => Operand::Imm(*i),
        crate::CodeGen::MachineOperand::Reg(reg) => match reg {
            crate::CodeGen::Reg::x64(x64) => Operand::Reg(*x64),
        },
    };
    
    let out = match out {
        crate::CodeGen::MachineOperand::Imm(i) => Operand::Imm(i),
        crate::CodeGen::MachineOperand::Reg(reg) => match reg {
            crate::CodeGen::Reg::x64(x64) => Operand::Reg(x64),
        },
    };

    sink.push( X64MCInstr::with2(Mnemonic::Mov, out.clone(), op1).into() );
    sink.push( X64MCInstr::with2(Mnemonic::Xor, out, op2).into() );
}
fn x64_lower_zext(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr) {
    todo!()
}
fn x64_lower_downcast(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr) {
    todo!()
}
fn x64_lower_call(sink: &mut Vec<X64MCInstr>, _: &MachineInstr, target: &String) {   
    let func = target;

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