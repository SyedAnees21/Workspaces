#![recursion_limit = "128"]

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use quote::quote;

//ast stands for abstract_syntax_tree
//Ident is the name of struct, enum or function, it stands for Identifier

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    // eprintln!("{:#?}", ast);

    let name_of_struct = &ast.ident;
    let new_name = format!("{}Builder", name_of_struct);
    let new_ident = syn::Ident::new(&new_name, name_of_struct.span());

    let expanded_tokenstream = quote!{
        // struct #new_ident {

        // }
        
        impl #name_of_struct {
            pub fn builder() -> #new_ident {
                #new_ident {
                    executable: None,
                    args: None,
                    env: None,
                    current_dir: None,
                }
            }
        }
        
        pub struct #new_ident{
            executable: Option<String>,
            args: Option<Vec<String>>,
            env: Option<Vec<String>>,
            current_dir: Option<String>,
        }

        impl #new_ident {
            pub fn executable(&mut self, executable: String)  {
                self.executable = Some(executable);               
            }
            pub fn args(&mut self, args: Vec<String>)  {
                self.args = Some(args);
            }
            pub fn env(&mut self, env: Vec<String>) {
                self.env = Some(env);
            }
            pub fn current_dir(&mut self, current_dir: String) {
                self.current_dir = Some(current_dir);
            }
            pub fn build(&mut self) -> Result<#name_of_struct, Box<dyn std::error::Error>> {
                Ok(#name_of_struct{
                    executable: self.executable.clone().ok_or("executable is not set")?,
                    args: self.args.clone().ok_or("args is not set")?,
                    env: self.env.clone().ok_or("env is not set")?,
                    current_dir: self.current_dir.clone().ok_or("current_dir is not set")?,
                })
            }
        }
    };

    expanded_tokenstream.into()
}
