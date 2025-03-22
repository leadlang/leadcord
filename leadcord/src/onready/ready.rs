use interpreter::{
  rtval_name, runtime_value,
  types::{AppliesEq, BufValue, RawRTValue},
};
use lead_lang_macros::{define, runtime_value_methods};
use serenity::all::{Context, Ready};

use crate::context::Ctx;

runtime_value! {
  OnReady,
  {
    pub context: Context,
    pub ready: Ready
  },
  rtval_name! { "📦 LeadCord / Ready" }
  runtime_value_methods! {
    getContext=separate
  }
}

#[define((
  desc: "Remove Context from Ready RuntimeValue",
  usage: [
    (
      desc: "",
      code: "$ready::getContext"
    ),
  ],
  notes: Some("This can only be used **once**"),
  root: Some("OnReady")
))]
fn separate() -> BufValue {
  let context = me.context.take().expect("Cannot get context");

  BufValue::RuntimeRaw(
    "Context",
    AppliesEq(RawRTValue::RT(Box::new(Ctx::new(context)))),
  )
}
