
use deno_ast::{
  ParsedSource,
  swc::ast::{
    Stmt,
    Decl,
    Program,
    ModuleItem
  }
};

pub enum AstDepth {
  Deep,
  Shallow
}

pub trait AstExt {
  fn depth(&self)-> AstDepth;
  fn decl_items(&self)-> Vec<&Decl>;
}

impl AstExt for ParsedSource {
  fn depth(&self)-> AstDepth {
    todo!()
  }

  fn decl_items(&self)-> Vec<&Decl> {
    match self.program_ref() {
      Program::Script(script)=> {
        script.body
        .iter()
        .flat_map(stmt_filter_decl_items)
        .collect::<Vec<_>>()
      },
      Program::Module(module)=> {
        module.body
        .iter()
        .flat_map(mod_item_filter_stmts)
        .flat_map(stmt_filter_decl_items)
        .collect::<Vec<_>>()
      }
    }
  }
}


#[inline]
fn stmt_filter_decl_items(stmt: &Stmt)-> Option<&Decl> {
  match stmt {
    Stmt::Decl(decl)=> Some(decl),
    _=> None
  }
}

#[inline]
fn mod_item_filter_stmts(mod_item: &ModuleItem)-> Option<&Stmt> {
  match mod_item {
    ModuleItem::Stmt(stmt)=> Some(stmt),
    _=> None
  }
}





