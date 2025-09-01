use std::{collections::HashMap, fs::File, io::Write, path::Path};

use object::{
    write::{Object, SectionId, Symbol, SymbolId, SymbolSection}, Architecture, BinaryFormat, Endianness, SectionKind, SymbolFlags, SymbolKind, SymbolScope
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
    pub fn create_section(&mut self, name: String, kind: SectionKind) -> Section {
        let id = self
            .elf
            .add_section(vec![], name.as_str().as_bytes().to_vec(), kind);

        let section = Section {
                name: name.clone(),
                kind,
                id,
                tvalue: 0,
                symbol_table: HashMap::new(),
            };
        
        self.sections.insert(
            name.clone(),
            section.clone()            
        );

        section
    }
    pub fn write_section(&mut self, id: SectionId, content: &[u8], align: u64) {
        self.elf.section_mut(id).append_data(content, align);
    }
    pub fn search_section(&self, name: String) -> &Section {
        self.sections.get(&name).expect("Invalid section")
    }
    pub fn create_symbol(&mut self, ) {}
    pub fn reallocate(&mut self) {}
    pub fn write(&self, path: &Path) {
        let mut file = File::create(path).unwrap();
        let content = self.elf.write().unwrap();
        file.write_all(&content).unwrap();
    }
}


#[derive(Clone)]
pub struct Section {
    pub name: String,
    pub kind: SectionKind,
    pub id: SectionId,
    pub tvalue: u64,
    pub symbol_table: HashMap<String, SymbolId>,
}

impl Section {
    pub fn create_symbol(&mut self, name: String, kind: SymbolKind, content: &[u8], align: u64, elf: &mut Elf) {
        elf.elf.add_symbol(Symbol {
            name: name.as_bytes().to_vec(),
            value: self.tvalue,
            size: content.len() as u64,
            kind,
            scope: SymbolScope::Linkage,
            weak: false,
            section: SymbolSection::Section(self.id),
            flags: SymbolFlags::Elf { st_info: 0x12, st_other: 0x0 }
        });

        self.tvalue += content.len() as u64;

        elf.write_section(self.id, content, align);
    }
} 
