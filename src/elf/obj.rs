use std::collections::HashMap;

use object::{
    Architecture, BinaryFormat, Endianness, SectionKind,
    write::{Object, SectionId, SymbolId},
};

pub struct Elf<'a> {
    pub sections: HashMap<String, Section>,
    pub elf: Object<'a>,
}

impl<'a> Elf<'a> {
    pub fn new() -> Self {
        Self {
            sections: HashMap::new(),
            elf: Object::new(BinaryFormat::Elf, Architecture::Riscv64, Endianness::Little),
        }
    }
    pub fn create_section(&mut self, name: String, kind: SectionKind) {
        let id = self
            .elf
            .add_section(vec![], name.as_str().as_bytes().to_vec(), kind);

        self.sections.insert(
            name.clone(),
            Section {
                name: name,
                kind,
                id,
                tvalue: 0,
                symbol_table: HashMap::new(),
            },
        );
    }
}

pub struct Section {
    pub name: String,
    pub kind: SectionKind,
    pub id: SectionId,
    pub tvalue: u64,
    pub symbol_table: HashMap<String, SymbolId>,
}
