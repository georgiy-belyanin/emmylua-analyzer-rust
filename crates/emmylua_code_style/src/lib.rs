mod test;

use emmylua_parser::{LuaAst, LuaParser, ParserConfig};
use styles::LuaCodeStyle;

mod format;
mod style_ruler;
mod styles;

pub fn reformat_lua_code(code: &str, styles: &LuaCodeStyle) -> String {
    let tree = LuaParser::parse(code, ParserConfig::default());

    let mut formatter = format::LuaFormatter::new(LuaAst::LuaChunk(tree.get_chunk_node()));
    style_ruler::apply_styles(&mut formatter, styles);
    let formatted_text = formatter.get_formatted_text();
    formatted_text
}

pub fn reformat_node(node: &LuaAst, styles: &LuaCodeStyle) -> String {
    let mut formatter = format::LuaFormatter::new(node.clone());
    style_ruler::apply_styles(&mut formatter, styles);
    let formatted_text = formatter.get_formatted_text();
    formatted_text
}
