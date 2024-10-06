use crate::prelude::*;
use super::RegAllocPrep;
use crate::CodeGen::reg_alloc::RegAlloc;

impl RegAllocPrep<Neg<Var, Var>> for RegAlloc {
    fn prep(&mut self, node: &Neg<Var, Var>) {
        let location = self.alloc_rv(node.inner2.ty);
        self.vars.insert(node.inner2.name.to_owned(), location);
        self.var_types.insert(node.inner2.name.to_owned(), node.inner2.ty);
    }
}