use moxy_ast::Ident;
use moxy_template::expand;

#[allow(unused)]
#[expand]
fn root(#[parse] name: Ident) {}

#[test]
fn should_expand_syntax() {}
