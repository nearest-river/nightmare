

use deno_ast::swc::ast::{
  Decl,
  VarDecl,
  UsingDecl,
  TsModuleName,
};



pub fn parse<D: AsRef<Decl>>(decl_items: impl AsRef<[D]>) {
  let items=decl_items.as_ref();
  let mut idents=Vec::<String>::new();

  for item in items {
    let item=item.as_ref();
    match item {
      Decl::Fn(f)=> idents.push(f.ident.to_string()),
      Decl::Class(class)=> idents.push(class.ident.to_string()),
      Decl::TsEnum(ts_enum)=> idents.push(ts_enum.id.to_string()),
      Decl::TsModule(module)=> idents.push(ts_mod_name_to_str(&module.id)),
      Decl::TsInterface(interface)=> idents.push(interface.id.to_string()),
      Decl::TsTypeAlias(ty_alias)=> idents.push(ty_alias.id.to_string()),
      Decl::Var(var)=> idents.extend(var_decl_extract_idents(var)),
      Decl::Using(using)=> idents.extend(using_decl_extract_idents(using)),
    };
  }
}

#[inline]
fn ts_mod_name_to_str(ts_mod_name: &TsModuleName)-> String {
  match ts_mod_name {
    TsModuleName::Str(str)=> str.value.to_string(),
    TsModuleName::Ident(ident)=> ident.to_string()
  }
}

#[inline]
fn var_decl_extract_idents(decl: &VarDecl)-> impl Iterator<Item=String> {
  decl.decls
  .iter()
  .flat_map(|decl| decl.name.as_ident())
  .map(|ident| ident.to_string())
}

fn using_decl_extract_idents(decl: &UsingDecl)-> impl Iterator<Item=String> {
  decl.decls
  .iter()
  .flat_map(|decl| decl.name.as_ident())
  .map(|ident| ident.to_string())
}













