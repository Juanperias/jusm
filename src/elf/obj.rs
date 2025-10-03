use std::{collections::HashMap, fs::File, io::Write, path::Path};

use object::{
    Architecture, BinaryFormat, Endianness, SectionKind, SymbolFlags, SymbolKind, SymbolScope,
    write::{Object, Relocation, SectionId, Symbol, SymbolId, SymbolSection},
};
use thiserror::Error;

use crate::parser::ast::{SymbolInfo, Visibility};

pub struct Elf<'a> {
    pub elf: Object<'a>,
    pub values: HashMap<SectionId, u64>,
    pub symbols: HashMap<String, (SymbolId, SectionId)>
}

pub struct CreateSectionInfo {
    pub name: &'static str,
    pub content: &'static [u8],
    pub section_id: SectionId,
    pub symbol_kind: SymbolKind,
    pub symbol_info: &'static SymbolInfo,
    pub align: u64,
}

#[derive(Error, Debug)]
pub enum ElfError {
    #[error("{0}")]
    ObjectWriteError(#[from] object::write::Error),

    #[error("Symbol {0} not exist")]
    SymbolNotExists(String),
}

impl<'a> Elf<'a> {
    pub fn new() -> Self {
        Self {
            elf: Object::new(BinaryFormat::Elf, Architecture::Riscv64, Endianness::Little),
            values: HashMap::new(),
            symbols: HashMap::new(),
        }
    }
    pub fn create_section(&mut self, name: &str, kind: SectionKind) -> SectionId {
        let id = self.elf.add_section(vec![], name.as_bytes().to_vec(), kind);

        self.values.insert(id, 0);

        id
    }
    pub fn write_section(&mut self, id: SectionId, content: &[u8], align: u64) {
        self.elf.section_mut(id).append_data(content, align);
    }
    pub fn get_symbol(&self, name: &String) -> Result<&(SymbolId, SectionId), ElfError> {
       match self.symbols.get(name) {
            Some(v) => Ok(v),
            None => Err(ElfError::SymbolNotExists(name.clone())),
       } 
    }
    pub fn create_symbol(&mut self, info: CreateSectionInfo) -> SymbolId {
        let id = self.elf.add_symbol(Symbol {
            name: info.name.as_bytes().to_vec(),
            value: *self
                .values
                .get(&info.section_id)
                .expect("Section not found"),
            size: info.content.len() as u64,
            weak: info.symbol_info.weak,
            kind: info.symbol_kind,
            scope: SymbolScope::Linkage,
            section: SymbolSection::Section(info.section_id),
            flags: SymbolFlags::Elf {
                st_info: match info.symbol_info.visibility {
                    Visibility::Local => 0x10,
                    Visibility::Global => 0x12,
                },
                st_other: 0x0,
            },
        });

        self.write_section(info.section_id, info.content, info.align);

        self.symbols.insert(info.name.to_string(), (id, info.section_id));

        id
    }

    pub fn create_reallocation(&mut self, id: SectionId, relocation: Relocation) -> Result<(), ElfError> {
        self.elf.add_relocation(id,  relocation)?;

        Ok(())
    }
    pub fn write(&self, path: &Path) {
        let mut file = File::create(path).unwrap();
        let content = self.elf.write().unwrap();
        file.write_all(&content).unwrap();
    }
}

// new
// create_section -> SectionId
// create_symbol -> SymbolId
//

/*
pub struct Elf<'a> {
    pub sections: HashMap<String, Section>,
    pub symbols: HashMap<String, (SymbolId, u64)>,
    pub elf: Object<'a>,
}

impl<'a> Elf<'a> {
    pub fn new() -> Self {
        Self {
            sections: HashMap::new(),
            elf: Object::new(BinaryFormat::Elf, Architecture::Riscv64, Endianness::Little),
            symbols: HashMap::new(),
        }
    }
    pub fn create_section(&mut self, name: String, kind: SectionKind) -> &mut Section {
        let id = self
            .elf
            .add_section(vec![], name.as_str().as_bytes().to_vec(), kind);

        self.sections.insert(
            name.clone(),
            Section {
                name: name.clone(),
                kind,
                id,
                tvalue: 0,
                symbol_table: HashMap::new(),
            },
        );

        self.sections.get_mut(&name).unwrap()
    }
    pub fn write_section(&mut self, id: SectionId, content: &[u8], align: u64) {
        self.elf.section_mut(id).append_data(content, align);
    }
    pub fn search_section(&mut self, name: String) -> &mut Section {
        self.sections.get_mut(&name).expect("Invalid section")
    }
    pub fn create_symbol(
        &mut self,
        section_name: String,
        name: String,
        kind: SymbolKind,
        content: &[u8],
        align: u64,
        symbol_info: &SymbolInfo,
    ) {
        let (section_id, tvalue) = {
            let section = self.search_section(section_name.clone());
            (section.id, section.tvalue)
        };

        if self.symbols.contains_key(&name) {
            return;
        }

        let id = self.elf.add_symbol(Symbol {
            name: name.as_bytes().to_vec(),
            value: tvalue,
            size: content.len() as u64,
            kind,
            scope: match symbol_info.visibility {
                Visibility::Local => SymbolScope::Compilation,
                Visibility::Global => SymbolScope::Linkage,
            },
            weak: symbol_info.weak,
            section: SymbolSection::Section(section_id),
            flags: SymbolFlags::Elf {
                st_info: match symbol_info.visibility {
                    Visibility::Local => 0x10,
                    Visibility::Global => 0x12,
                },
                st_other: 0x0,
            },
        });

        self.symbols.insert(name, (id, tvalue));

        {
            let section = self.search_section(section_name);
            section.tvalue += content.len() as u64;
        }

        self.write_section(section_id, content, align);
    }
    pub fn get_symbol_id(&self, name: String) -> (SymbolId, u64) {
        *self.symbols.get(&name).expect("Symbol not found")
    }
    pub fn reallocate(
        &mut self,
        section_id: SectionId,
        symbol_id: SymbolId,
        offset: u64,
        addend: i64,
        kind: u32,
    ) {
        self.elf
            .add_relocation(
                section_id,
                Relocation {
                    flags: object::RelocationFlags::Elf { r_type: kind },
                    symbol: symbol_id,
                    offset,
                    addend,
                },
            )
            .unwrap();
    }
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
*/
