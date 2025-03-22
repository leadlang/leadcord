use interpreter::{
  error, runtime_value,
  types::{AnyWrapper, BufValue},
};
use lead_lang_macros::{define, runtime_value_methods};
use serenity::all::{ActivityData, Context};

runtime_value! {
  Ctx,
  {
    pub ctx: Context
  },
  fn name(&self) -> &'static str {
    "📦 LeadCord / Context"
  }
  runtime_value_methods! {
    status::set=set_status,
    activity::set=set_activity,
    activity::reset=reset_activity,
    client::cache=cache,
    client::http=http
  }
}

#[define((
  desc: "Set status of the Client",
  usage: [
    (
      desc: "Set Presence",
      code: "$context::status::set online/dnd/idle/invisible"
    ),
  ],
  notes: None,
  params: [
    r"(online|dnd|idle|invisible)"
  ],
  root: Some("Ctx")
))]
fn set_status(status: &str) {
  // NOTE: Impossible to fail
  let x = me.ctx.as_ref().unwrap();

  match status {
    "online" => x.online(),
    "dnd" => x.dnd(),
    "invisible" => x.invisible(),
    "idle" => x.idle(),
    _ => unreachable!(),
  }
}

#[define((
  desc: "Reset activity",
  usage: [
    (
      desc: "Reset Presence",
      code: "$context::status::reset"
    ),
  ],
  notes: None,
  root: Some("Ctx")
))]
fn reset_activity() {
  // NOTE: Impossible to fail
  let x = me.ctx.as_ref().unwrap();

  x.set_activity(None);
}

#[define((
  desc: "Set Activity of the client",
  usage: [
    (
      desc: "Set Activity",
      code: "$context::activity::set activity_type ->$data"
    ),
  ],
  notes: Some(
    "**activity_type**: `streaming`, `competiting`, `playing, `listening`, `watching`, `custom`\n"
  ),
  params: [
    r"(streaming|competiting|playing|listening|watching|custom) ->\$[a-z0-9_]*"
  ],
  root: Some("Ctx")
))]
fn set_activity(activity: &str, data: BufValue) {
  // NOTE: Impossible to fail
  let x = me.ctx.as_ref().unwrap();

  if activity == "streaming" {
    let BufValue::Array(vect) = &data else {
      error("Expected Array when activity is streaming", file);
    };

    let [name, url] = &vect[..] else {
      error("Expected Array with exactly two items [name, url]", file);
    };

    let BufValue::Str(name) = &name else {
      error("name must be STRING", file);
    };

    let BufValue::Str(url) = &url else {
      error("url must be STRING", file);
    };

    let data = ActivityData::streaming(name, url).expect("URL parsing failed");

    x.set_activity(Some(data));

    return;
  }

  let BufValue::Str(name) = data else {
    error("Activity data must be STRING", file);
  };

  let data: ActivityData = match activity {
    "competiting" => ActivityData::competing(name),
    "playing" => ActivityData::playing(name),
    "listening" => ActivityData::listening(name),
    "watching" => ActivityData::watching(name),
    "custom" => ActivityData::custom(name),
    _ => unreachable!("Unknown activity, provided"),
  };

  x.set_activity(Some(data));
}

#[define((
  desc: "Return Client Cache",
  usage: [
    (
      desc: "This stores cache into $cache variable",
      code: "$context::client::cache"
    ),
  ],
  notes: None,
  root: Some("Ctx")
))]
fn cache() -> BufValue {
  // NOTE: Impossible to fail
  let x = me.ctx.as_ref().unwrap();

  BufValue::Runtime(AnyWrapper(Box::new(x.cache.clone())))
}

#[define((
  desc: "Returns HTTP method endpoints",
  usage: [
    (
      desc: "This returns you a list of http endpoints that you can call",
      code: "$http: $context::client::http"
    ),
  ],
  notes: None,
  root: Some("Ctx")
))]
fn http() -> BufValue {
  // NOTE: Impossible to fail
  let x = me.ctx.as_ref().unwrap();

  BufValue::Runtime(AnyWrapper(Box::new(x.http.clone())))
}
