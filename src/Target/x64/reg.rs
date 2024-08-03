use std::fmt::Display;

use super::Reg;

/// A x64 register
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum x64Reg {
    Rax, Eax, Ax, Al,
    Rbx, Ebx, Bx, Bl,
    Rcx, Ecx, Cx, Cl,
    Rdx, Edx, Dx, Dl,
    Rsi, Esi, Si, Sil,
    Rdi, Edi, Di, Dil,

    Rsp, Esp, Sp, Spl,
    Rbp, Ebp, Bp, Bpl,

    R8, R8d, R8w, R8b,
    R9, R9d, R9w, R9b,
    R10, R10d, R10w, R10b,
    R11, R11d, R11w, R11b,
    R12, R12d, R12w, R12b,
    R13, R13d, R13w, R13b,
    R14, R14d, R14w, R14b,
    R15, R15d, R15w, R15b,
}

impl x64Reg {
    /// Parses the string to an register (Returns none if it's invalid)
    pub fn parse(string: String) -> Option<Self> {
        use x64Reg::*;
        match string.to_ascii_lowercase().as_str() {
            "rax" => Some(Rax), "eax" => Some(Eax), "ax" => Some(Ax), "al" => Some(Al),
            "rbx" => Some(Rbx), "ebx" => Some(Ebx), "bx" => Some(Bx), "bl" => Some(Bl),
            "rcx" => Some(Rcx), "ecx" => Some(Ecx), "cx" => Some(Cx), "cl" => Some(Cl),
            "rdx" => Some(Rdx), "edx" => Some(Edx), "dx" => Some(Dx), "dl" => Some(Dl),
            "rsi" => Some(Rsi), "esi" => Some(Esi), "si" => Some(Si), "sil" => Some(Sil),
            "rdi" => Some(Rdi), "edi" => Some(Edi), "di" => Some(Di), "dil" => Some(Dil),

            "rsp" => Some(Rsp), "esp" => Some(Esp), "sp" => Some(Sp), "spl" => Some(Spl),
            "rbp" => Some(Rbp), "ebp" => Some(Ebp), "bp" => Some(Bp), "bpl" => Some(Bpl),

            "r8" => Some(R8), "r8d" => Some(R8d), "r8w" => Some(R8w), "r8b" => Some(R8b),
            "r9" => Some(R9), "r9d" => Some(R9d), "r9w" => Some(R9w), "r9b" => Some(R9b),
            "r10" => Some(R10), "r10d" => Some(R10d), "r10w" => Some(R10w), "r10b" => Some(R10b),
            "r11" => Some(R11), "r11d" => Some(R11d), "r11w" => Some(R11w), "r11b" => Some(R11b),
            "r12" => Some(R12), "r12d" => Some(R12d), "r12w" => Some(R12w), "r12b" => Some(R12b),
            "r13" => Some(R13), "r13d" => Some(R13d), "r13w" => Some(R13w), "r13b" => Some(R13b),
            "r14" => Some(R14), "r14d" => Some(R14d), "r14w" => Some(R14w), "r14b" => Some(R14b),
            "r15" => Some(R15), "r15d" => Some(R15d), "r15w" => Some(R15w), "r15b" => Some(R15b),
            
            _ => None,
        }
    }

    /// Returns if the reg is in the extendet region (r8->r15)
    pub fn extended(&self) -> bool {
        use x64Reg::*;
        match self {
            R8 | R8d | R8w | R8b |
            R9 | R9d | R9w | R9b |
            R10 | R10d | R10w | R10b |
            R11 | R11d | R11w | R11b |
            R12 | R12d | R12w | R12b |
            R13 | R13d | R13w | R13b |
            R14 | R14d | R14w | R14b |
            R15 | R15d | R15w | R15b  => true,
            _ => false,
        }
    }
}

impl Display for x64Reg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

impl Reg for x64Reg {
    fn sub64(&self) -> String {
        use x64Reg::*;
        match self {
            Rax | Eax | Ax | Al => "rax",
            Rbx | Ebx | Bx | Bl => "rbx",
            Rcx | Ecx | Cx | Cl => "rcx",
            Rdx | Edx | Dx | Dl => "rdx",
            Rsi | Esi | Si | Sil => "rsi",
            Rdi | Edi | Di | Dil => "rdi",

            Rsp | Esp | Sp | Spl => "rsp",
            Rbp | Ebp | Bp | Bpl => "rbp",
        
            R8 | R8d | R8w | R8b => "r8",
            R9 | R9d | R9w | R9b => "r9",
            R10 | R10d | R10w | R10b => "r10",
            R11 | R11d | R11w | R11b => "r11",
            R12 | R12d | R12w | R12b => "r12",
            R13 | R13d | R13w | R13b => "r13",
            R14 | R14d | R14w | R14b => "r14",
            R15 | R15d | R15w | R15b => "r15",
        }.to_string()
    }

    fn sub32(&self) -> String {
        use x64Reg::*;
        match self {
            Rax | Eax | Ax | Al => "eax",
            Rbx | Ebx | Bx | Bl => "ebx",
            Rcx | Ecx | Cx | Cl => "ecx",
            Rdx | Edx | Dx | Dl => "edx",
            Rsi | Esi | Si | Sil => "esi",
            Rdi | Edi | Di | Dil => "edi",

            Rsp | Esp | Sp | Spl => "esp",
            Rbp | Ebp | Bp | Bpl => "ebp",
        
            R8 | R8d | R8w | R8b => "r8d",
            R9 | R9d | R9w | R9b => "r9d",
            R10 | R10d | R10w | R10b => "r10d",
            R11 | R11d | R11w | R11b => "r11d",
            R12 | R12d | R12w | R12b => "r12d",
            R13 | R13d | R13w | R13b => "r13d",
            R14 | R14d | R14w | R14b => "r14d",
            R15 | R15d | R15w | R15b => "r15d",
        }.to_string()
    }

    fn sub16(&self) -> String {
        use x64Reg::*;
        match self {
            Rax | Eax | Ax | Al => "ax",
            Rbx | Ebx | Bx | Bl => "bx",
            Rcx | Ecx | Cx | Cl => "cx",
            Rdx | Edx | Dx | Dl => "dx",
            Rsi | Esi | Si | Sil => "si",
            Rdi | Edi | Di | Dil => "di",

            Rsp | Esp | Sp | Spl => "sp",
            Rbp | Ebp | Bp | Bpl => "bp",
        
            R8 | R8d | R8w | R8b => "r8w",
            R9 | R9d | R9w | R9b => "r9w",
            R10 | R10d | R10w | R10b => "r10w",
            R11 | R11d | R11w | R11b => "r11w",
            R12 | R12d | R12w | R12b => "r12w",
            R13 | R13d | R13w | R13b => "r13w",
            R14 | R14d | R14w | R14b => "r14w",
            R15 | R15d | R15w | R15b => "r15w",
        }.to_string()
    }

    fn sub8(&self) -> String {
        use x64Reg::*;
        match self {
            Rax | Eax | Ax | Al => "ax",
            Rbx | Ebx | Bx | Bl => "bx",
            Rcx | Ecx | Cx | Cl => "cx",
            Rdx | Edx | Dx | Dl => "dx",
            Rsi | Esi | Si | Sil => "sil",
            Rdi | Edi | Di | Dil => "dil",

            Rsp | Esp | Sp | Spl => "spl",
            Rbp | Ebp | Bp | Bpl => "bpl",
        
            R8 | R8d | R8w | R8b => "r8b",
            R9 | R9d | R9w | R9b => "r9b",
            R10 | R10d | R10w | R10b => "r10b",
            R11 | R11d | R11w | R11b => "r11b",
            R12 | R12d | R12w | R12b => "r12b",
            R13 | R13d | R13w | R13b => "r13b",
            R14 | R14d | R14w | R14b => "r14b",
            R15 | R15d | R15w | R15b => "r15b",
        }.to_string()
    }

    fn boxed(&self) -> Box<dyn Reg> {
        Box::from(*self)
    }
    
    fn from(&self, string: String) -> Box<dyn Reg> {
        x64Reg::parse(string).expect("need valid register").boxed()
    }
    
    fn is_gr64(&self) -> bool {
        use x64Reg::*;
        match self {
            Rax | Rbx | Rcx | Rdx | Rsi | Rdi |
            Rsp | Rbp | R8 | R9 | R10 | R11 |
            R12 | R13 | R14 | R15 => true,

            _ => false,
        }
    }
    
    fn is_gr32(&self) -> bool {
        use x64Reg::*;
        match self {
            Eax | Ebx | Ecx | Edx | Esi | Edi |
            Esp | Ebp | R8d | R9d | R10d | R11d |
            R12d | R13d | R14d | R15d => true,

            _ => false,
        }
    }
    
    fn is_gr16(&self) -> bool {
        use x64Reg::*;
        match self {
            Ax | Bx | Cx | Dx | Si | Di |
            Sp | Bp | R8w | R9w | R10w | R11w |
            R12w | R13w | R14w | R15w => true,

            _ => false,
        }
    }
    
    fn is_gr8(&self) -> bool {
        use x64Reg::*;
        match self {
            Al | Bl | Cl | Dl | Sil | Dil |
            Spl | Bpl | R8b | R9b | R10b |
            R11b | R12b | R13b | R14b | R15b => true,

            _ => false,
        }
    }

    fn enc(&self) -> u8 {
        match self {
            x64Reg::Rax | x64Reg::Eax | x64Reg::Ax | x64Reg::Al => 0,
            x64Reg::Rcx | x64Reg::Ecx | x64Reg::Cx | x64Reg::Cl => 1,
            x64Reg::Rdx | x64Reg::Edx | x64Reg::Dx | x64Reg::Dl => 2,
            x64Reg::Rbx | x64Reg::Ebx | x64Reg::Bx | x64Reg::Bl => 3,
            x64Reg::Rsi | x64Reg::Esi | x64Reg::Si | x64Reg::Sil => 6,
            x64Reg::Rbp | x64Reg::Ebp | x64Reg::Bp | x64Reg::Bpl => 5,
            x64Reg::Rsp | x64Reg::Esp | x64Reg::Sp | x64Reg::Spl => 4,
            x64Reg::Rdi | x64Reg::Edi | x64Reg::Di | x64Reg::Dil => 7,

            // this here use a prefix
            x64Reg::R8 | x64Reg::R8d | x64Reg::R8w | x64Reg::R8b => 0,
            x64Reg::R9 | x64Reg::R9d | x64Reg::R9w | x64Reg::R9b => 1,
            x64Reg::R10 | x64Reg::R10d | x64Reg::R10w | x64Reg::R10b => 2,
            x64Reg::R11 | x64Reg::R11d | x64Reg::R11w | x64Reg::R11b => 3,
            x64Reg::R12 | x64Reg::R12d | x64Reg::R12w | x64Reg::R12b => 4,
            x64Reg::R13 | x64Reg::R13d | x64Reg::R13w | x64Reg::R13b => 5,
            x64Reg::R14 | x64Reg::R14d | x64Reg::R14w | x64Reg::R14b => 6,
            x64Reg::R15 | x64Reg::R15d | x64Reg::R15w | x64Reg::R15b => 7,
        }
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
