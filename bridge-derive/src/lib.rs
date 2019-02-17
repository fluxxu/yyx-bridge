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

  let expanded = quote! {
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
  };

  TokenStream::from(expanded)
}
