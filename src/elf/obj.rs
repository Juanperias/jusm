use std::{collections::HashMap, fs::File, io::Write, path::Path};

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
    pub fn create_section(&mut self, name: String, kind: SectionKind) -> SectionId {
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

        id
    }
    pub fn write_section(&mut self, id: SectionId, content: &[u8], align: u64) {
        self.elf.section_mut(id).append_data(content, align);
    }
    pub fn search_section(&self, name: String) -> &Section {
        self.sections.get(&name).expect("Invalid section")
    }
    pub fn create_symbol(&mut self) {}
    pub fn reallocate(&mut self) {}
    pub fn write(&self, path: &Path) {
        let mut file = File::create(path).unwrap();
        let content = self.elf.write().unwrap();
        file.write_all(&content).unwrap();
    }
}

pub struct Section {
    pub name: String,
    pub kind: SectionKind,
    pub id: SectionId,
    pub tvalue: u64,
    pub symbol_table: HashMap<String, SymbolId>,
}
