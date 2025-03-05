#![allow(non_snake_case)]

use std::sync::LazyLock;

use handler::Handler;
use serenity::{Client, prelude::*};

use interpreter::{
  error, generate, module, pkg_name,
  types::{AnyWrapper, BufValue},
  tokio::runtime::{Runtime, Builder},
};
use lead_lang_macros::{define, methods};

mod handler;
mod handlers;

mod message;
mod context;

mod onready;

pub(crate) static RUNTIME: LazyLock<Runtime> = LazyLock::new(|| Builder::new_multi_thread().worker_threads(1).enable_all().build().unwrap());

module! {
    LeadCordClient,
    pkg_name! { "📦 LeadCord / Prelude" }
    methods! {
      client::run=run,

      handler::new=handler,

      intent::all=intents_all,
      intent::new=intents,
      intent::add=intent,
      intent::empty=intents_empty,
      intent::remove=remove,
      intent::guilds=GUILDS,
      intent::guild_members=GUILD_MEMBERS,
      intent::guild_moderation=GUILD_MODERATION,
      intent::guild_bans=GUILD_BANS,
      intent::guild_emojis_and_stickers=GUILD_EMOJIS_AND_STICKERS,
      intent::guild_integrations=GUILD_INTEGRATIONS,
      intent::guild_webhooks=GUILD_WEBHOOKS,
      intent::guild_invites=GUILD_INVITES,
      intent::guild_voice_states=GUILD_VOICE_STATES,
      intent::guild_presences=GUILD_PRESENCES,
      intent::guild_messages=GUILD_MESSAGES,
      intent::guild_message_reactions=GUILD_MESSAGE_REACTIONS,
      intent::guild_message_typing=GUILD_MESSAGE_TYPING,
      intent::direct_messages=DIRECT_MESSAGES,
      intent::direct_message_reactions=DIRECT_MESSAGE_REACTIONS,
      intent::direct_message_typing=DIRECT_MESSAGE_TYPING,
      intent::message_content=MESSAGE_CONTENT,
      intent::guild_scheduled_events=GUILD_SCHEDULED_EVENTS,
      intent::auto_moderation_configuration=AUTO_MODERATION_CONFIGURATION,
      intent::auto_moderation_execution=AUTO_MODERATION_EXECUTION,
      intent::guild_message_polls=GUILD_MESSAGE_POLLS,
      intent::direct_message_polls=DIRECT_MESSAGE_POLLS
    }
}

#[define((
  desc: "Construct a handler",
  usage: [
    (
      desc: "Allocating Handler",
      code: "$handler: handle::new"
    ),
  ],
  notes: None
))]
fn handler() -> BufValue {
  BufValue::Runtime(AnyWrapper(Box::new(Handler::default())))
}

#[define((
  desc: "Construct all Intents",
  usage: [
    (
      desc: "Allocating Intent",
      code: "$intent: intent::all"
    ),
  ],
  notes: None
))]
fn intents_all() -> BufValue {
  BufValue::U_Int(GatewayIntents::all().bits())
}

#[define((
  desc: "Construct Intent",
  usage: [
    (
      desc: "Allocating Intent",
      code: "$intent: intent::new"
    ),
  ],
  notes: None
))]
fn intents() -> BufValue {
  BufValue::U_Int(GatewayIntents::non_privileged().bits())
}

#[define((
  desc: "Construct Empty Intent List",
  usage: [
    (
      desc: "Allocating Intent",
      code: "$intent: intent::empty"
    ),
  ],
  notes: None
))]
fn intents_empty() -> BufValue {
  BufValue::U_Int(GatewayIntents::empty().bits())
}

#[define((
  desc: "Add Intent to the List",
  usage: [
    (
      desc: "Allocating Intent",
      code: "intent::add ->&$intents ->$intent"
    ),
  ],
  notes: None
))]
fn intent(intents: &mut BufValue, intent: BufValue) {
  let BufValue::U_Int(intents) = intents else {
    error("Expected intents as a number", file);
  };

  let BufValue::U_Int(intent) = intent else {
    error("Expected intent as a number", file);
  };

  *intents = *intents | intent;
}

#[define((
  desc: "Remove Intent from the List",
  usage: [
    (
      desc: "Allocating Intent",
      code: "intent::remove ->&$intents ->$intent"
    ),
  ],
  notes: None
))]
fn remove(intents: &mut BufValue, intent: BufValue) {
  let BufValue::U_Int(intents) = intents else {
    error("Expected intents as a number", file);
  };

  let BufValue::U_Int(intent) = intent else {
    error("Expected intent as a number", file);
  };

  *intents = *intents & !intent;
}

#[define((
    desc: "Create and Run a Discord Client",
    usage: [
      (
        desc: "Start a Client",
        code: "$discord::client::run ->$token ->$intents ->$handler"
      ),
    ],
    notes: None
))]
fn run(token: BufValue, intent: BufValue, handler: BufValue) {
  let BufValue::Str(token) = token else {
    error("Expected String as token", file);
  };

  let BufValue::U_Int(intents) = intent else {
    error("Expected Int as intents", file);
  };

  let BufValue::Runtime(handler) = handler else {
    error("Expected Runtime as handler", file);
  };

  let handler = handler.0.downcast::<Handler>().expect("Cannot get handler");

  let builder = Client::builder(
    token,
    GatewayIntents::from_bits(intents).expect("Unable to construct Intents"),
  ).event_handler(*handler);

  let f = async move {
    let mut d = builder.await.expect("Cannot get client");
    d.start_autosharded().await;
  };

  RUNTIME.block_on(f);
}

macro_rules! intents {
    ($($x:ident $y:expr);*) => {
      $(
        #[define((
          desc: "Get the intent",
          usage: [],
          notes: None
        ))]
        fn $x() -> BufValue {
          BufValue::U_Int($y)
        }
      )*
    };
}

intents! {
  GUILDS 1;
  GUILD_MEMBERS 1 << 1;
  GUILD_MODERATION 1 << 2;
  GUILD_BANS 1 << 2;
  GUILD_EMOJIS_AND_STICKERS 1 << 3;
  GUILD_INTEGRATIONS 1 << 4;
  GUILD_WEBHOOKS 1 << 5;
  GUILD_INVITES 1 << 6;
  GUILD_VOICE_STATES 1 << 7;
  GUILD_PRESENCES 1 << 8;
  GUILD_MESSAGES 1 << 9;
  GUILD_MESSAGE_REACTIONS 1 << 10;
  GUILD_MESSAGE_TYPING 1 << 11;
  DIRECT_MESSAGES 1 << 12;
  DIRECT_MESSAGE_REACTIONS 1 << 13;
  DIRECT_MESSAGE_TYPING 1 << 14;
  MESSAGE_CONTENT 1 << 15;
  GUILD_SCHEDULED_EVENTS 1 << 16;
  AUTO_MODERATION_CONFIGURATION 1 << 20;
  AUTO_MODERATION_EXECUTION 1 << 21;
  GUILD_MESSAGE_POLLS 1 << 24;
  DIRECT_MESSAGE_POLLS 1 << 25
}

use message::MessageReader;
use handlers::Handlers;
use onready::ReadyReader;
use context::Ctx;

generate! {
  LeadCordClient,
  MessageReader,
  Handlers,
  ReadyReader,
  Ctx
}
