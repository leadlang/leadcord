use std::mem::replace;

use interpreter::{
  module, pkg_name,
  types::{AnyWrapper, BufValue},
};
use lead_lang_macros::{define, methods};
use serenity::all::{Context, Message};

module! {
  MessageReader,
  pkg_name! { "📦 LeadCord / Message #1" }
  methods! {
    message::separate=separate
  }
}

#[define((
  desc: "Remove Context from Message struct",
  usage: [
    (
      desc: "Allocating Intent",
      code: "$context: message::separate ->$event"
    ),
  ],
  notes: None
))]
fn separate(event: &mut BufValue) -> BufValue {
  let x = replace(event, BufValue::Bool(false));

  let BufValue::Runtime(x) = x else {
    panic!("Cannot cast as Runtime value");
  };

  let x = x
    .0
    .downcast::<(Context, Message)>()
    .expect("Cannot cast to Message");

  let context = x.0;
  let message = x.1;

  *event = BufValue::Runtime(AnyWrapper(Box::new(message)));

  BufValue::Runtime(AnyWrapper(Box::new(context)))
}
