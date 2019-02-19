#![feature(proc_macro_span)]

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

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
  } else if len < 100 {
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
  } else {
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
  } else if len < 100 {
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
  } else {
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

  TokenStream::from(expanded)
}
