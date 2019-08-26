#![feature(proc_macro_span)]
#![recursion_limit = "128"]

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, LitStr};

#[proc_macro]
pub fn secret_string(tokens: TokenStream) -> TokenStream {
  use rand::{thread_rng, Rng};

  let input = parse_macro_input!(tokens as LitStr);

  let value = input.value();
  let len = value.as_bytes().len();
  let key: Vec<u8> = {
    let mut rng = thread_rng();
    let mut bytes = Vec::with_capacity(len);
    for _ in 0..len {
      bytes.push(rng.gen());
    }
    bytes
  };
  let obf_bytes: Vec<u8> = value
    .as_bytes()
    .into_iter()
    .enumerate()
    .map(|(i, b)| *b ^ key[i])
    .collect();

  let expanded = if len == 0 {
    quote! {
      "".to_owned()
    }
  } else {
    quote! {
      {
        let key = [
          #(#key),*
        ];
        let obf_bytes = [
          #(#obf_bytes),*
        ];
        let mut bytes: [u8; #len] = [0; #len];
        for i in 0..#len {
          bytes[i] = obf_bytes[i] ^ key[i];
        }
        unsafe { ::std::str::from_utf8_unchecked(&bytes as &[u8]) }.to_owned()
      }
    }
  };
  /* else {
    quote! {
      {
        let mut key = Vec::with_capacity(#len);
        #(
          key.push(#key);
        )*
        let mut obf_bytes = Vec::with_capacity(#len);
        #(
          obf_bytes.push(#obf_bytes);
        )*
        let mut bytes = Vec::with_capacity(#len);
        for i in 0..#len {
          bytes.push(obf_bytes[i] ^ key[i]);
        }
        unsafe { ::std::str::from_utf8_unchecked(&bytes as &[u8]) }.to_owned()
      }
    }
  };
  */

  TokenStream::from(expanded)
}

#[proc_macro]
pub fn secret_string_from_file(tokens: TokenStream) -> TokenStream {
  use std::env::current_dir;
  use std::fs::read_to_string;
  use std::path::PathBuf;
  use std::str::FromStr;

  use rand::{thread_rng, Rng};

  let input = parse_macro_input!(tokens as LitStr);
  let project_root = current_dir().unwrap();
  let path = PathBuf::from_str(&input.value()).unwrap();

  let content = read_to_string(project_root.join(path)).unwrap();

  let len = content.as_bytes().len();
  let key: Vec<u8> = {
    let mut rng = thread_rng();
    let mut bytes = Vec::with_capacity(len);
    for _ in 0..len {
      bytes.push(rng.gen());
    }
    bytes
  };
  let obf_bytes: Vec<u8> = content
    .as_bytes()
    .into_iter()
    .enumerate()
    .map(|(i, b)| *b ^ key[i])
    .collect();

  let expanded = if len == 0 {
    quote! {
      "".to_owned()
    }
  } else {
    quote! {
      {
        let key = [
          #(#key),*
        ];
        let obf_bytes = [
          #(#obf_bytes),*
        ];
        let mut bytes: [u8; #len] = [0; #len];
        for i in 0..#len {
          bytes[i] = obf_bytes[i] ^ key[i];
        }
        unsafe { ::std::str::from_utf8_unchecked(&bytes as &[u8]) }.to_owned()
      }
    }
  };
  /*
  else {
    quote! {
      {
        let mut key = Vec::with_capacity(#len);
        #(
          key.push(#key);
        )*
        let mut obf_bytes = Vec::with_capacity(#len);
        #(
          obf_bytes.push(#obf_bytes);
        )*
        let mut bytes = Vec::with_capacity(#len);
        for i in 0..#len {
          bytes.push(obf_bytes[i] ^ key[i]);
        }
        unsafe { ::std::str::from_utf8_unchecked(&bytes as &[u8]) }.to_owned()
      }
    }
  };
  */

  TokenStream::from(expanded)
}

#[proc_macro_derive(ParseClientValue)]
pub fn derive_parse_client_value(tokens: TokenStream) -> TokenStream {
  use syn::{Data, Fields};
  let input = parse_macro_input!(tokens as DeriveInput);
  let ident = input.ident;
  let ident_str = ident.to_string();

  let expanded = match input.data {
    Data::Struct(ref data) => {
      let inits: Vec<_> = match data.fields {
        Fields::Named(ref fields) => fields
          .named
          .iter()
          .enumerate()
          .map(|(i, f)| {
            let f_ident = &f.ident;
            let f_len = fields.named.len();
            quote! {
              #f_ident: ::bridge_value::ParseClientValue::parse_client_value(
                vec.get(#i).ok_or_else(|| {
                  ::bridge_value::ParseClientValueError::Message(
                    format!("{} has {} fields, but the array length is {}.",
                      #ident_str, #f_len, vec.len()
                    )
                  )
                })?
              )?
            }
          })
          .collect(),
        _ => panic!("Named struct fields expected."),
      };

      quote! {
        impl ::bridge_value::ParseClientValue for #ident {
          fn parse_client_value(value: &::bridge_value::Value) -> Result<Self, ::bridge_value::ParseClientValueError> {
            if let Some(vec) = value.as_array() {
              Ok(#ident {
                #(#inits,)*
              })
            } else {
              Err(::bridge_value::ParseClientValueError::TypeMismatch(stringify!(#ident), value.clone()))
            }
          }
        }
      }
    }
    Data::Enum(ref data) => {
      let arms: Vec<_> = data
        .variants
        .iter()
        .enumerate()
        .map(|(i, v)| {
          let i = i as u64;
          if v.discriminant.is_some() {
            panic!("Variant discriminant not supported.");
          }
          if let Fields::Unit = v.fields {
            let v_ident = &v.ident;
            quote! {
              #i => Ok(#ident::#v_ident)
            }
          } else {
            panic!("Unit enum variant expected.");
          }
        })
        .collect();
      let arms_len = arms.len();
      quote! {
        impl ::bridge_value::ParseClientValue for #ident {
          fn parse_client_value(value: &::bridge_value::Value) -> Result<Self, ::bridge_value::ParseClientValueError> {
            if let Some(v) = value.as_u64() {
              match v {
                #(#arms,)*
                i => Err(::bridge_value::ParseClientValueError::Message(
                  format!("{} has {} variants, but the value is {}.",
                    #ident_str, #arms_len, i
                  )
                ))
              }
            } else {
              Err(::bridge_value::ParseClientValueError::TypeMismatch(stringify!(#ident), value.clone()))
            }
          }
        }
      }
    }
    _ => panic!("Struct or Enum expected."),
  };
  TokenStream::from(expanded)
}
