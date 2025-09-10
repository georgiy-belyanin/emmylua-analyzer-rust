use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Doc {
    pub name: String,
    pub display: Option<String>,
    pub supers: Option<String>,
    pub namespace: Option<String>,
    pub fields: Option<Vec<MemberDoc>>,
    pub methods: Option<Vec<MemberDoc>>,
    pub property: Property,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemberDoc {
    pub name: String,
    pub display: String,
    pub property: Property,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Property {
    pub description: Option<String>,
    pub see: Option<String>,
    pub deprecated: Option<String>,
    pub other: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MkdocsIndex {
    pub site_name: String,
    pub types: Vec<IndexStruct>,
    pub modules: Vec<IndexStruct>,
    pub globals: Vec<IndexStruct>,

    vkdoc_modules: std::collections::HashMap<String, VkdocModule>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexStruct {
    pub name: String,
    pub file: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VkdocModule {
    pub modules: Vec<String>,
    pub types: Vec<String>,
    pub globals: Vec<String>,
}

pub enum VkdocKind {
    Type,
    Module,
    Global,
}

impl MkdocsIndex {
    pub fn insert_vkdoc(&mut self, namespace: String, kind: VkdocKind, body: String) {
        if !self.vkdoc_modules.contains_key(&namespace) {
            self.vkdoc_modules.insert(
                namespace.clone(),
                VkdocModule {
                    modules: vec![],
                    types: vec![],
                    globals: vec![],
                },
            );
        }

        let module = self.vkdoc_modules.get_mut(&namespace).unwrap();
        match kind {
            VkdocKind::Type => module.types.push(body),
            VkdocKind::Module => module.modules.push(body),
            VkdocKind::Global => module.globals.push(body),
        }
    }
    pub fn iter_modules(&self) -> impl std::iter::Iterator<Item = (&String, &VkdocModule)> {
        self.vkdoc_modules.iter()
    }
}
