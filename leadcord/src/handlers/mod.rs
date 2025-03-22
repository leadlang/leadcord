use crate::Handler;
use interpreter::tokio::sync::mpsc::unbounded_channel;
use interpreter::types::{AppliesEq, BufValue};
use interpreter::{module, pkg_name};
use lead_lang_macros::{define, methods};

macro_rules! modifier {
  ($($x:ident, $y:ident, $desc:literal, $code:literal);*) => {
    $(
    #[define((
      desc: "Add method to handler",
      usage: [
        (
          desc: $desc,
          code: $code
        ),
      ],
      notes: None
    ))]
    fn $x(val: &mut BufValue) -> BufValue {
      let BufValue::Runtime(w) = val else {
        panic!("Cannot get Runtime");
      };

      let (tx, rx) = unbounded_channel::<BufValue>();
      let hwnd = w
        .0
        .downcast_mut::<Handler>()
        .expect("Cannot cast to Handler");

      hwnd.$y = Some(tx);

      BufValue::Listener(AppliesEq(rx))
    })*
  };
}

module! {
  Handlers,
  pkg_name! { "📦 LeadCord / Handles" }
  methods! {
    handle::onmessage=a,
    handle::interaction_create=b,
    handle::ready=c,
    handle::resume=d,
    handle::shards_ready=e,
    handle::message_delete=f,
    handle::message_delete_bulk=g,
    handle::message_update=h
  }
}

modifier! {
  a,
  onmessage,
  "Get listener to attach to onmessage",
  "$onmessage: handle::onmessage ->$handler";

  b,
  interaction_create,
  "Get listener to attach to interaction_create",
  "$interaction_create: handle::interaction_create ->$handler";

  c,
  ready,
  "Get listener to attach to ready",
  "$ready: handle::ready ->$handler";

  d,
  resume,
  "Get listener to attach to resume",
  "$resume: handle::resume ->$handler";

  e,
  shards_ready,
  "Get listener to attach to shards_ready",
  "$shards_ready: handle::shards_ready ->$handler";

  f,
  message_delete,
  "Get listener to attach to message_delete",
  "$message_delete: handle::message_delete ->$handler";

  g,
  message_delete_bulk,
  "Get listener to attach to message_delete_bulk",
  "$message_delete_bulk: handle::message_delete_bulk ->$handler";

  h,
  message_update,
  "Get listener to attach to message_update",
  "$message_update: handle::message_update ->$handler"
}
