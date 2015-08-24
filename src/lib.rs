#![feature(plugin_registrar)]
#![feature(rustc_private)]
#![feature(slice_patterns)]

extern crate rustc;
extern crate syntax;

use rustc::plugin::Registry;
use syntax::ast::Name;
use syntax::ast::{self, TokenTree, TtToken, UintTy, Lit_, LitIntType};
use syntax::codemap::Span;
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager};
use syntax::ext::build::AstBuilder;  // trait for expr_usize
use syntax::parse::token;

//use aster::AstBuilder;

fn layout_id(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree])
        -> Box<MacResult + 'static> {

    let text = match args {
        [TtToken(_, token::Ident(s, _))] => s.name.as_str().to_string(),
        _ => {
            cx.span_err(sp, "argument must be a single type");
            return DummyResult::any(sp);
        }
    };

    println!("{:?}", text);

    println!("{:?}", sp);

    /*let mut text = &*text;
    let mut total = 0;
    while !text.is_empty() {
        match NUMERALS.iter().find(|&&(rn, _)| text.starts_with(rn)) {
            Some(&(rn, val)) => {
                total += val;
                text = &text[rn.len()..];
            }
            None => {
                cx.span_err(sp, "invalid Roman numeral");
                return DummyResult::any(sp);
            }
        }
    }*/

    MacEager::expr(cx.expr_lit(sp, Lit_::LitInt(128, LitIntType::UnsignedIntLit(UintTy::TyU64))))
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("layout_id", layout_id);
}