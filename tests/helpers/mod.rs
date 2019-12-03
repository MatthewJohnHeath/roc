extern crate bumpalo;

use self::bumpalo::Bump;
use roc::can;
use roc::can::expr::Expr;
use roc::can::problem::Problem;
use roc::can::symbol::Symbol;
use roc::can::Output;
use roc::collections::{ImMap, MutMap, SendSet};
use roc::ident::Ident;
use roc::parse;
use roc::parse::ast::{self, Attempting};
use roc::parse::blankspace::space0_before;
use roc::parse::parser::{loc, Fail, Parser, State};
use roc::region::{Located, Region};
use roc::subs::{VarStore, Variable};
use roc::types::{Expected, Type};
use std::hash::Hash;
use std::path::{Path, PathBuf};

#[allow(dead_code)]
pub fn parse_with<'a>(arena: &'a Bump, input: &'a str) -> Result<ast::Expr<'a>, Fail> {
    parse_loc_with(arena, input).map(|loc_expr| loc_expr.value)
}

#[allow(dead_code)]
pub fn parse_loc_with<'a>(arena: &'a Bump, input: &'a str) -> Result<Located<ast::Expr<'a>>, Fail> {
    let state = State::new(&input, Attempting::Module);
    let parser = space0_before(loc(parse::expr(0)), 0);
    let answer = parser.parse(&arena, state);

    answer
        .map(|(loc_expr, _)| loc_expr)
        .map_err(|(fail, _)| fail)
}

#[allow(dead_code)]
pub fn can_expr(expr_str: &str) -> (Expr, Output, Vec<Problem>, VarStore, Variable) {
    can_expr_with(&Bump::new(), "blah", expr_str, &ImMap::default())
}

#[allow(dead_code)]
pub fn uniq_expr(
    expr_str: &str,
) -> (
    roc::uniqueness::Output,
    Output,
    Vec<Problem>,
    VarStore,
    Variable,
    VarStore,
    Variable,
) {
    uniq_expr_with(&Bump::new(), "blah", expr_str, &ImMap::default())
}

#[allow(dead_code)]
pub fn uniq_expr_with(
    arena: &Bump,
    name: &str,
    expr_str: &str,
    declared_idents: &ImMap<Ident, (Symbol, Region)>,
) -> (
    roc::uniqueness::Output,
    Output,
    Vec<Problem>,
    VarStore,
    Variable,
    VarStore,
    Variable,
) {
    let loc_expr = parse_loc_with(&arena, expr_str).unwrap_or_else(|_| {
        panic!(
            "can_expr_with() got a parse error when attempting to canonicalize:\n\n{:?}",
            expr_str
        )
    });

    let var_store1 = VarStore::new();
    let variable = var_store1.fresh();
    let expected = Expected::NoExpectation(Type::Variable(variable));
    let home = "Test";
    let (loc_expr, output, problems) = can::canonicalize_declaration(
        arena,
        &var_store1,
        home.into(),
        name.into(),
        Region::zero(),
        loc_expr,
        declared_idents,
        expected,
    );

    // double check
    let var_store2 = VarStore::new();

    let variable2 = var_store2.fresh();
    let expected2 = Expected::NoExpectation(Type::Variable(variable2));
    let output2 = roc::uniqueness::canonicalize_declaration(
        &var_store2,
        Region::zero(),
        loc_expr,
        declared_idents,
        expected2,
    );

    (
        output2, output, problems, var_store1, variable, var_store2, variable2,
    )
}

#[allow(dead_code)]
pub fn can_expr_with(
    arena: &Bump,
    name: &str,
    expr_str: &str,
    declared_idents: &ImMap<Ident, (Symbol, Region)>,
) -> (Expr, Output, Vec<Problem>, VarStore, Variable) {
    let loc_expr = parse_loc_with(&arena, expr_str).unwrap_or_else(|_| {
        panic!(
            "can_expr_with() got a parse error when attempting to canonicalize:\n\n{:?}",
            expr_str
        )
    });

    let var_store = VarStore::new();
    let variable = var_store.fresh();
    let expected = Expected::NoExpectation(Type::Variable(variable));
    let home = "Test";
    let (loc_expr, output, problems) = can::canonicalize_declaration(
        arena,
        &var_store,
        home.into(),
        name.into(),
        Region::zero(),
        loc_expr,
        declared_idents,
        expected,
    );

    (loc_expr.value, output, problems, var_store, variable)
}

#[allow(dead_code)]
pub fn mut_map_from_pairs<K, V, I>(pairs: I) -> MutMap<K, V>
where
    I: IntoIterator<Item = (K, V)>,
    K: Hash + Eq,
{
    let mut answer = MutMap::default();

    for (key, value) in pairs {
        answer.insert(key, value);
    }

    answer
}

#[allow(dead_code)]
pub fn im_map_from_pairs<K, V, I>(pairs: I) -> ImMap<K, V>
where
    I: IntoIterator<Item = (K, V)>,
    K: Hash + Eq + Clone,
    V: Clone,
{
    let mut answer = ImMap::default();

    for (key, value) in pairs {
        answer.insert(key, value);
    }

    answer
}

#[allow(dead_code)]
pub fn send_set_from<V, I>(elems: I) -> SendSet<V>
where
    I: IntoIterator<Item = V>,
    V: Hash + Eq + Clone,
{
    let mut answer = SendSet::default();

    for elem in elems {
        answer.insert(elem);
    }

    answer
}

#[allow(dead_code)]
pub fn fixtures_dir<'a>() -> PathBuf {
    Path::new("tests").join("fixtures").join("build")
}

#[allow(dead_code)]
pub fn builtins_dir<'a>() -> PathBuf {
    PathBuf::new().join("builtins")
}
