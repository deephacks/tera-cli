extern crate serde_json;
extern crate tera;
use serde_json::value::Value;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, Read};
use std::process;
use tera::{Context, Tera};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage: tera <TMPL> -- <JSON>");
        return;
    }
    let tmpl = &args[1];
    let mut stdinjson = String::new();
    io::stdin().read_to_string(&mut stdinjson).unwrap();
    // Read all the file tmpl into a variable (ignoring the result of the operation).
    match serde_json::from_str::<Value>(&stdinjson) {
        Ok(v) => {
            // copy json fields separatley instead of whole object
            // so every field can be accessed directly
            let mut context = Context::new();
            for (key, value) in v.as_object().unwrap() {
                context.insert(key, &value);
            }
            // autoscape is mostly for html templates, converting
            // special characters like ' to &#x27;
            let autoescape = false;
            // render template
            match Tera::one_off(&tmpl, &context, autoescape) {
                Ok(s) => print!("{}", s),
                Err(e) => {
                    // omit printing the actual error since it doesn't carry any help
                    // with troubleshooting why the template rendering failed, ie:
                    // Failed to render '__tera_one_off'
                    // eprint!("{}", e);

                    eprint!("tera: ");
                    let mut cause = e.source();
                    while let Some(e) = cause {
                        eprintln!("{}", e);
                        cause = e.source();
                    }
                    process::exit(1);
                }
            };
        }
        Err(error) => {
            println!("Error parsing json: {}", error);
        }
    }
}
