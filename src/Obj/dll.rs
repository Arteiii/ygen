use std::{error::Error, fs::File};

use object::FileFlags;

use object::{pe, elf, macho};

use super::{ObjectBuilder, Decl, Linkage};
use crate::prelude::Triple;

/// The dll builder: used for creating shared libaries like .dlls, .so, .dylib
///
/// It can't have relocs (links) so it need to be already linked
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DllBuilder {
    wrapper: ObjectBuilder,
    triple: Triple
}

impl DllBuilder {
    /// Creates a new dll builder
    pub fn new(triple: Triple) -> Self {
        let mut obj = ObjectBuilder::new(triple);

        obj.flags = Some(match triple.bin {
            crate::Target::ObjFormat::Coff => FileFlags::Coff { characteristics: pe::IMAGE_FILE_DLL },
            crate::Target::ObjFormat::Elf => FileFlags::Elf { os_abi: elf::ELFOSABI_SYSV, abi_version: 0, e_flags: elf::ET_DYN as u32 },
            crate::Target::ObjFormat::MachO => FileFlags::MachO { flags: macho::MH_DYLIB },
            _ => panic!("unsuported object format: {:?} for shared libarys", triple.bin), // not really
        });

        Self {
            wrapper: obj,
            triple: triple,
        }
    }

    /// Adds one decl to the function
    pub fn decl(&mut self, decl: (&str, Decl, Linkage)) {
        self.wrapper.decls.push((decl.0.to_string(), decl.1, decl.2));
    }

    /// Defines a symbol
    pub fn define(&mut self, name: &str, data: Vec<u8>) {
        self.wrapper.defines.insert(name.to_string(), data);
    }

    /// Writes the symbols into the shared libary
    pub fn emit(&self, file: File) -> Result<(), Box<dyn Error>> {
        self.wrapper.emit(file, None)
    }
}