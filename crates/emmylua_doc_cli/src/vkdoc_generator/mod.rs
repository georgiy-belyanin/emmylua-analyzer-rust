pub mod generator;
mod init_tl;
mod markdown_types;
mod mixin_copy;
mod render;

use std::path::PathBuf;

use crate::OutputDestination;
use emmylua_code_analysis::EmmyLuaAnalysis;
use generator::{
    generate_global_markdown, generate_index, generate_module_markdown, generate_type_markdown,
};
use markdown_types::MkdocsIndex;

pub fn generate_vkdoc(
    analysis: &EmmyLuaAnalysis,
    output: OutputDestination,
    override_template: Option<PathBuf>,
    site_name: Option<String>,
    mixin: Option<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    let OutputDestination::File(output) = output else {
        return Err("Output must be a path when using markdown format".into());
    };

    let docs_dir = output.join("docs");
    let types_out = docs_dir.join("types");
    /*
    if !types_out.exists() {
        log::info!("Creating types directory: {:?}", types_out);
        std::fs::create_dir_all(&types_out)?;
    } else {
        log::info!("Clearing types directory: {:?}", types_out);
        std::fs::remove_dir_all(&types_out)?;
        std::fs::create_dir_all(&types_out)?;
    }
    */

    let module_out = docs_dir.join("modules");
    /*
    if !module_out.exists() {
        log::info!("Creating modules directory: {:?}", module_out);
        std::fs::create_dir_all(&module_out)?;
    } else {
        log::info!("Clearing modules directory: {:?}", module_out);
        std::fs::remove_dir_all(&module_out)?;
        std::fs::create_dir_all(&module_out)?;
    }
    */

    let global_out = docs_dir.join("globals");
    /*
    if !global_out.exists() {
        log::info!("Creating globals directory: {:?}", global_out);
        std::fs::create_dir_all(&global_out)?;
    } else {
        log::info!("Clearing globals directory: {:?}", global_out);
        std::fs::remove_dir_all(&global_out)?;
        std::fs::create_dir_all(&global_out)?;
    }
    */

    let tl = init_tl::init_tl(override_template).ok_or("Failed to initialize TL")?;
    let mut mkdocs_index = MkdocsIndex::default();
    if let Some(site_name) = site_name {
        mkdocs_index.site_name = site_name;
    }

    let db = analysis.compilation.get_db();
    let type_index = db.get_type_index();
    let types = type_index.get_all_types();
    for type_decl in types {
        generate_type_markdown(db, &tl, type_decl, &types_out, &mut mkdocs_index);
    }

    let module_index = db.get_module_index();
    let modules = module_index.get_module_infos();
    for module in modules {
        generate_module_markdown(db, &tl, module, &module_out, &mut mkdocs_index);
    }

    let global_index = db.get_global_index();
    let globals = global_index.get_all_global_decl_ids();
    for global_decl_id in globals {
        generate_global_markdown(db, &tl, &global_decl_id, &global_out, &mut mkdocs_index);
    }

    // generate_index(&tl, &mut mkdocs_index, &output);

    use crate::vkdoc_generator::generator::vkdoc;
    mkdocs_index.iter_modules().for_each(|(namespace, module)| {
        let render_text = module
            .modules
            .iter()
            .chain(module.types.iter())
            .chain(module.globals.iter())
            .fold("".to_string(), |acc, s| format!("{}\n{}", acc, s));

        vkdoc::generate_vkdoc_entry(
            &PathBuf::from(escape_type_name(&namespace).replace(".", "/").as_str()),
            vkdoc::VkdocEntry {
                name: namespace.clone(),
                description: "no description provided".to_string(),
                content: render_text,
            },
            &vkdocs_rs::Vkdoc::new(&output).unwrap(),
        );
    });

    if let Some(mixin) = mixin {
        mixin_copy::mixin_copy(&output, mixin);
    }

    eprintln!("Documentation markdown exported to {:?}", docs_dir);

    Ok(())
}

fn escape_type_name(name: &str) -> String {
    name.chars()
        .map(|c| {
            // Windows Invalid Characters
            if "<>:\"/\\|?*".contains(c) { '_' } else { c }
        })
        .collect()
}
