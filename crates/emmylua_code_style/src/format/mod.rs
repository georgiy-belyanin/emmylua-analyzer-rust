mod syntax_node_change;

use std::collections::HashMap;

use emmylua_parser::{LuaAst, LuaAstNode, LuaSyntaxId};
use rowan::NodeOrToken;

use crate::format::syntax_node_change::TokenNodeChange;

#[allow(unused)]
#[derive(Debug)]
pub struct LuaFormatter {
    root: LuaAst,
    token_changes: HashMap<LuaSyntaxId, TokenNodeChange>,
}

#[allow(unused)]
impl LuaFormatter {
    pub fn new(root: LuaAst) -> Self {
        Self {
            root,
            token_changes: HashMap::new(),
        }
    }

    pub fn add_token_change(&mut self, syntax_id: LuaSyntaxId, change: TokenNodeChange) {
        self.token_changes.insert(syntax_id, change);
    }

    pub fn get_token_change(&self, syntax_id: &LuaSyntaxId) -> Option<&TokenNodeChange> {
        self.token_changes.get(syntax_id)
    }

    pub fn get_formatted_text(&self) -> String {
        let mut formatted_text = String::new();
        for node_or_token in self.root.syntax().descendants_with_tokens() {
            if let NodeOrToken::Token(token) = node_or_token {
                let syntax_id = LuaSyntaxId::from_token(&token);
                if let Some(change) = self.token_changes.get(&syntax_id) {
                    match change {
                        TokenNodeChange::Remove => continue,
                        TokenNodeChange::AddLeft(s) => {
                            formatted_text.push_str(s);
                            formatted_text.push_str(&token.text());
                        }
                        TokenNodeChange::AddRight(s) => {
                            formatted_text.push_str(&token.text());
                            formatted_text.push_str(s);
                        }
                        TokenNodeChange::ReplaceWith(s) => {
                            formatted_text.push_str(s);
                        }
                    }
                } else {
                    formatted_text.push_str(&token.text());
                }
            }
        }

        formatted_text
    }
}
