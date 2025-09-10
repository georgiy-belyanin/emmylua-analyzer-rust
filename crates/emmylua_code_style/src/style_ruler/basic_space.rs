use crate::{format::LuaFormatter, styles::LuaCodeStyle};

use super::StyleRuler;

pub struct BasicSpaceRuler;

impl StyleRuler for BasicSpaceRuler {
    fn apply_style(_: &mut LuaFormatter, _: &LuaCodeStyle) {}
}
