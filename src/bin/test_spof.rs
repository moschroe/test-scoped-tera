use std::sync::Arc;
use tera::{Context, CtxThreadSafe, Function, FunctionRelaxed, Tera};

use rayon::prelude::*;

fn do_stuff(tera: &Tera, context: &Context<CtxThreadSafe>) -> String {
    tera.render("test", context).unwrap()
}

fn main() {
    dbg!("tera_test");
    let mut tera = Tera::default();
    tera.add_raw_template("test", "{{ name }}");
    let mut context = Context::new();
    context.insert("name", "toto");

    let r: Vec<String> = (0..10)
        .into_par_iter()
        .map(|_| do_stuff(&tera, &context))
        .collect();
    println!("{:?}", r);
}
