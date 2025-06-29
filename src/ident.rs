

use std::{
  io,
  fs,
  rc::Rc,
  sync::LazyLock,
  collections::{
    HashMap,
    HashSet
  }
};

use deno_ast::swc::ast::{
  Decl,
  VarDecl,
  UsingDecl,
  TsModuleName,
};


static DATA_PATH: &str="assets/xd";
static CORPUS: LazyLock<Vec<Vec<String>>>=LazyLock::new(Vec::new);

fn open_vocab()-> io::Result<HashMap<String,usize>> {
  let raw_data=fs::read_to_string(DATA_PATH)?;
  let corpus=raw_data.lines()
  .map(tokenize_ident);
  let mut vocab=HashMap::<String,usize>::new();
  let mut idx=0usize;

  for tokens in corpus {
    for token in tokens {
      vocab.entry(token.to_owned())
      .or_insert_with(|| {
        let i=idx;
        idx+=1;
        i
      });
    }
  }

  Ok(vocab)
}

pub struct Vectorizer {
  pub vocab: HashMap<Rc<str>,usize>,
  pub ordered_tokens: Vec<Rc<str>>
}

impl Vectorizer {
  pub fn new<S: AsRef<str>>(common_idents: &[S])-> Self {
    let mut unique_tokens=HashSet::<Rc<str>>::new();
    for ident in common_idents {
      for token in tokenize_ident(ident.as_ref()) {
        unique_tokens.insert(token.into());
      }
    }

    let mut ordered_tokens=unique_tokens.into_iter()
    .collect::<Vec<_>>();
    ordered_tokens.sort_unstable();

    let vocab=ordered_tokens.iter()
    .enumerate()
    .map(|(idx,token)| (token.clone(),idx))
    .collect();

    Self {
      vocab,
      ordered_tokens
    }
  }

  pub fn vectorize(&self,ident: &str)-> Vec<f64> {
    let mut vec=vec![0.0;self.vocab.len()];
    for token in tokenize_ident(ident) {
      if let Some(&idx)=self.vocab.get(token.as_str()) {
        vec[idx]+=1.0;
      }
    }

    vec
  }
}

fn tokenize_ident(ident: &str)-> Vec<String> {
  let mut tokens=Vec::<String>::new();
  let mut current_token=String::new();

  for c in ident.chars() {
    match c {
      '_'|'-'=> {
        if !current_token.is_empty() {
          tokens.push(current_token.to_lowercase());
          current_token.clear();
        }
      },
      c if c.is_uppercase()=> {
        if !current_token.is_empty() {
          tokens.push(current_token.to_lowercase());
        }
        current_token=c.to_string();
      },
      c=> current_token.push(c)
    }
  }

  if !current_token.is_empty() {
    tokens.push(current_token.to_lowercase());
  }

  tokens
}


pub fn parse<D: AsRef<Decl>>(decl_items: impl AsRef<[D]>)-> Vec<String> {
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

  idents
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













