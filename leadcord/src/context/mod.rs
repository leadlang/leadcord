use interpreter::{error, module, pkg_name, types::{AnyWrapper, BufValue}};
use lead_lang_macros::{methods, define};
use serenity::all::{ActivityData, Context};

module! {
  Ctx,
  pkg_name! { "📦 LeadCord / Context" }
  methods! {
    activity::status=set_status,
    activity::set=set_activity,
    activity::reset=reset_activity,
    client::cache=cache,
    client::http=http
  }
}

macro_rules! prelude {
    ($x:ident, $file:ident) => {
      {
        let BufValue::Runtime(x) = $x else {
          error("Cannot cast as Runtime value", $file);
        };
    
        x.downcast_ref::<Context>().expect("Cannot cast to Context")
      }
    };
}

#[define((
  desc: "Set status of the Client",
  usage: [
    (
      desc: "Set Presence",
      code: "activity::status $context online/dnd/idle/invisible"
    ),
  ],
  notes: None,
  params: [
    r"\$[a-z]* (online|dnd|idle|invisible)"
  ]
))]
fn set_status(event: &BufValue, status: &str) {
  let x = prelude!(event, file);

  match status {
    "online" => x.online(),
    "dnd" => x.dnd(),
    "invisible" => x.invisible(),
    "idle" => x.idle(),
    _ => unreachable!()
  }
}

#[define((
  desc: "Reset activity",
  usage: [
    (
      desc: "Reset Presence",
      code: "activity::reset $context activity_type $data"
    ),
  ],
  notes: None
))]
fn reset_activity(event: &BufValue) {
  let x = prelude!(event, file);

  x.set_activity(None);
}

#[define((
  desc: "Set Activity of the client",
  usage: [
    (
      desc: "Set Activity",
      code: "activity::set $context activity_type $data"
    ),
  ],
  notes: Some(
    "This function does not move $text\nTypes of activity: `streaming`, `competiting`, `playing, `listening`, `watching`, `custom`\n"
  ),
  params: [
    r"\$[a-z0-9_]* (streaming|competiting|playing|listening|watching|custom) \$[a-z0-9_]*"
  ]
))]
fn set_activity(event: &BufValue, activity: &str, data: &BufValue) {
  let x = prelude!(event, file);

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
    _ => unreachable!("Unknown activity, provided")
  };

  x.set_activity(Some(data));
}

#[define((
  desc: "Return Client Cache",
  usage: [
    (
      desc: "This stores cache into $cache variable",
      code: "$cache: client::cache $ctx"
    ),
  ],
  notes: None
))]
fn cache(event: &BufValue) -> BufValue {
  let x = prelude!(event, file);

  BufValue::Runtime(AnyWrapper(Box::new(x.cache.clone())))
}

#[define((
  desc: "Returns HTTP method endpoints",
  usage: [
    (
      desc: "This returns you a list of http endpoints that you can call",
      code: "$http: client::http $ctx"
    ),
  ],
  notes: None
))]
fn http(event: &BufValue) -> BufValue {
  let x = prelude!(event, file);

  BufValue::Runtime(AnyWrapper(Box::new(x.http.clone())))
}