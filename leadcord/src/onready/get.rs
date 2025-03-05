use std::mem::replace;

use interpreter::{module, pkg_name, types::{AnyWrapper, BufValue}};
use lead_lang_macros::{methods, define};
use serenity::all::{Context, Ready};

module! {
  ReadyReader,
  pkg_name! { "📦 LeadCord / Ready #1" }
  methods! {
    ready::separate=separate
  }
}

#[define((
  desc: "Remove Context from Ready struct",
  usage: [
    (
      desc: "Allocating Intent",
      code: "$context: ready::separate ->$event"
    ),
  ],
  notes: None
))]
fn separate(event: &mut BufValue) -> BufValue {
  let x = replace(event, BufValue::Bool(false));
  
  let BufValue::Runtime(x) = x else {
    panic!("Cannot cast as Runtime value");
  };

  let x = x.0.downcast::<(Context, Ready)>().expect("Cannot cast to Ready event");

  let context = x.0;
  let message = x.1;

  *event = BufValue::Runtime(AnyWrapper(Box::new(message)));

  BufValue::Runtime(AnyWrapper(Box::new(context)))
}