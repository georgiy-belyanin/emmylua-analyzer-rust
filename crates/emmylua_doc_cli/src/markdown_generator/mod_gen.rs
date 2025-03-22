use std::path::Path;

use emmylua_code_analysis::{
    humanize_type, DbIndex, FileId, LuaMemberKey, LuaMemberOwner, LuaPropertyOwnerId, LuaType, LuaMemberId,
    ModuleInfo, RenderLevel,
};
use emmylua_parser::VisibilityKind;
use tera::{Context, Tera};
use uuid;

use crate::markdown_generator::{escape_type_name, IndexStruct, MemberDisplay};
use serde_json;

use super::{
    render::{render_const_type, render_function_type, render_function_name},
    MkdocsIndex,
};

pub fn generate_module_markdown(
    db: &DbIndex,
    tl: &Tera,
    module: &ModuleInfo,
    output: &Path,
    mkdocs_index: &mut MkdocsIndex,
) -> Option<()> {
    check_filter(db, module.file_id)?;

    let mut context = tera::Context::new();
    context.insert("module_name", &module.full_module_name);
    let property_owner_id = module.property_owner_id.clone();
    if let Some(property_owner_id) = property_owner_id {
        if let Some(property) = db.get_property_index().get_property(&property_owner_id) {
            let description = property
                .description
                .clone()
                .unwrap_or("".to_string().into())
                .to_string();
            context.insert("description", &description);
        }
    }

    let export_typ = module.export_type.clone()?;
    match &export_typ {
        LuaType::Def(type_id) => {
            let member_owner = LuaMemberOwner::Type(type_id.clone());
            let type_simple_name = type_id.get_simple_name();
            generate_member_owner_module(db, member_owner, type_simple_name, &mut context);
        }
        LuaType::TableConst(t) => {
            let member_owner = LuaMemberOwner::Element(t.clone());
            generate_member_owner_module(db, member_owner, &module.full_module_name, &mut context);
        }
        LuaType::Instance(i) => {
            let member_owner = LuaMemberOwner::Element(i.get_range().clone());
            generate_member_owner_module(db, member_owner, &module.full_module_name, &mut context);
        }
        _ => {}
    }

    let render_text = match tl.render("lua_module_template.tl", &context) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("Failed to render template: {}", e);
            return None;
        }
    };

    let dir_name = format!("{}", escape_type_name(&module.full_module_name).replace(".", "-"));
    let outpath = output.join(dir_name.clone());
    match std::fs::create_dir_all(outpath) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Failed to create dir: {}", e);
            return None;
        }
    }
    let meta_file_name = format!("{}/{}.meta.json", dir_name, escape_type_name(&module.full_module_name).replace(".", "-"));
    let file_name = format!("{}/{}.md", dir_name, escape_type_name(&module.full_module_name).replace(".", "-"));
    mkdocs_index.modules.push(IndexStruct {
        name: module.full_module_name.clone(),
        file: format!("{}", dir_name.clone()),
    });

    let outpath = output.join(file_name);
    println!("output module file: {}", outpath.display());
    match std::fs::write(outpath, render_text) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Failed to write file: {}", e);
            return None;
        }
    }

    let render_text = serde_json::json!({
        "title": &module.full_module_name,
        "metaTitle": &module.full_module_name,
        "sectionTitle": &module.full_module_name,
        "pageDescription": &module.full_module_name,
        "shortDescription": &module.full_module_name,
        "weight": 1,
        "uuid": uuid::Uuid::new_v4().to_string(),
        "createdAt": "2024-05-27T10:57:28.000Z",
        "updatedAt": "2024-05-27T10:57:28.000Z"
    });

    let outpath = output.join(meta_file_name);
    println!("meta output module file: {}", outpath.display());
    match std::fs::write(outpath, render_text.to_string()) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Failed to write file: {}", e);
            return None;
        }
    }
    Some(())
}

fn check_filter(db: &DbIndex, file_id: FileId) -> Option<()> {
    let module = db.get_module_index().get_module(file_id)?;
    if module.workspace_id.is_main() {
        return Some(());
    }

    None
}

pub fn generate_member_owner_module(
    db: &DbIndex,
    member_owner: LuaMemberOwner,
    owner_name: &str,
    context: &mut Context,
) -> Option<()> {
    let members = db.get_member_index().get_sorted_members(&member_owner);
    let mut method_members: Vec<MemberDisplay> = Vec::new();
    let mut field_members: Vec<MemberDisplay> = Vec::new();

    if let Some(members) = members {
        for member in members {
            let member_typ = member.get_decl_type();
            let member_id = member.get_id();
            let member_property_id = LuaPropertyOwnerId::Member(member_id.clone());
            let member_property = db.get_property_index().get_property(&member_property_id);
            if let Some(member_property) = member_property {
                if member_property.visibility.unwrap_or(VisibilityKind::Public)
                    != VisibilityKind::Public
                {
                    continue;
                }
            }

            let description = if let Some(member_property) = member_property {
                let des = member_property
                    .description
                    .clone()
                    .unwrap_or("".to_string().into())
                    .to_string();

                if let Some(see) = &member_property.see_content {
                    format!("{}\n See: {}\n", des, see)
                } else {
                    des
                }
            } else {
                "".to_string()
            };

            let member_key = member.get_key();
            let name = match member_key {
                LuaMemberKey::Name(name) => name,
                _ => continue,
            };

            let title_name = format!("{}.{}()", owner_name, name);
            if member_typ.is_function() {
                let func_name = format!("{}.{}", owner_name, name);
                let display_name = render_function_name(db, &member_typ, &func_name);
                let display = render_function_type(db, &member_typ, &func_name, false);
                method_members.push(MemberDisplay {
                    name: display_name,
                    display,
                    description,
                });
            } else if member_typ.is_const() {
                let display = render_const_type(db, &member_typ);
                field_members.push(MemberDisplay {
                    name: title_name,
                    display: format!("```lua\n{}.{}: {}\n```\n", owner_name, name, display),
                    description,
                });
            } else {
                let typ_display = humanize_type(db, &member_typ, RenderLevel::Detailed);
                field_members.push(MemberDisplay {
                    name: title_name,
                    display: format!("```lua\n{}.{} : {}\n```\n", owner_name, name, typ_display),
                    description,
                });
            }
        }
    }
    if !method_members.is_empty() {
        context.insert("methods", &method_members);
    }

    if !field_members.is_empty() {
        context.insert("fields", &field_members);
    }

    Some(())
}
