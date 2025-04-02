use std::io::Write;

use cidre::ax;
use clap::Parser;

const IDENT: usize = 4;

#[derive(Parser, Debug)]
struct Args {
    /// The pid to attach to
    pid: u32,
}

fn print_tree(elem: &ax::UiElement, depth: usize, prefix: &mut String, children_attr: &ax::Attr) {
    if prefix.len() < depth {
        prefix.push_str("                        ");
    }

    if let Ok(attrs) = elem.attrs() {
        for attr in attrs.iter() {
            if attr.equal(children_attr) {
                continue;
            }
            std::io::stdout()
                .write_all(prefix[0..depth].as_bytes())
                .unwrap();
            if let Ok(value) = elem.attr_value(attr) {
                println!("{}: {:?}", attr.to_string(), value);
            } else {
                println!("{}: <error>", attr.to_string());
            }
        }
    }

    if let Ok(children) = elem.children() {
        let depth = depth + IDENT;

        for child in children.iter() {
            print_tree(child, depth, prefix, children_attr);
        }
    }
}

fn main() {
    if !ax::is_process_trusted_with_prompt(true) {
        println!("Not trusted");
        return;
    }

    let args = Args::parse();
    let app = ax::UiElement::with_app_pid(args.pid as i32);

    let mut prefix = String::new();
    print_tree(&app, 0, &mut prefix, ax::attr::children());
}
