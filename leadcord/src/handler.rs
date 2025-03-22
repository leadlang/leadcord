use interpreter::{
  tokio::sync::mpsc::UnboundedSender,
  types::{AnyWrapper, BufValue, AppliesEq, RawRTValue},
};
use serenity::{
  all::{
    ChannelId, Context, EventHandler, GuildId, Interaction, Message, MessageId, MessageUpdateEvent,
    Ready, ResumedEvent,
  },
  async_trait,
};
use std::future::Future;
use std::pin::Pin;

use crate::onready::OnReady;

macro_rules! generates {
  ($($x:ident),*) => {
    #[derive(Debug, Default)]
    pub(crate) struct Handler {
      $(
        pub(crate) $x: Option<UnboundedSender<BufValue>>
      ),*
    }

    unsafe impl Send for Handler {}
    unsafe impl Sync for Handler {}
  };
}

macro_rules! methods {
  (m $($x:ident, $s:ident, $($y:ty),+);*) => {
    $(
      methods! { $x, $s, $($y),* }
    )*
  };

  ($x:ident, $s:ident, $a:ty) => {
    fn $x<'life0, 'async_trait>(&'life0 self, ctx: Context, a: $a)
    -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
      where
      'life0: 'async_trait,
      Self: 'async_trait,
    {
      Box::pin(async move {
        if let Some(x) = &self.$x {
          let val = BufValue::RuntimeRaw("runtime", AppliesEq(RawRTValue::RT(Box::new(
            $s::new(ctx, a)
          ))));

          let _ = x.send(val);
        }
      })
    }
  };
  ($x:ident, $s:ident, $a:ty, $b:ty) => {
    fn $x<'life0, 'async_trait>(&'life0 self, ctx: Context, a: $a, b: $b)
    -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
      where
      'life0: 'async_trait,
      Self: 'async_trait,
    {
      Box::pin(async move {
        if let Some(x) = &self.$x {
          let val = BufValue::RuntimeRaw("runtime", AppliesEq(RawRTValue::RT(Box::new(
            $s::new(ctx, a, b)
          ))));

          let _ = x.send(val);
        }
      })
    }
  };
  ($x:ident, $s:ident, $a:ty, $b:ty, $c:ty) => {
    fn $x<'life0, 'async_trait>(&'life0 self, ctx: Context, a: $a, b: $b, c: $c)
    -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
      where
      'life0: 'async_trait,
      Self: 'async_trait,
    {
      Box::pin(async move {
        if let Some(x) = &self.$x {
          let val = BufValue::RuntimeRaw("runtime", AppliesEq(RawRTValue::RT(Box::new(
            $s::new(ctx, a, b, c)
          ))));

          let _ = x.send(val);
        }
      })
    }
  };
  ($x:ident, $s:ident, $a:ty, $b:ty, $c:ty, $d:ty) => {
    fn $x<'life0, 'async_trait>(&'life0 self, ctx: Context, a: $a, b: $b, c: $c, d: $d)
    -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
      where
      'life0: 'async_trait,
      Self: 'async_trait,
    {
      Box::pin(async move {
        if let Some(x) = &self.$x {
          let val = BufValue::RuntimeRaw("runtime", AppliesEq(RawRTValue::RT(Box::new(
            $s::new(ctx, a, b, c, d)
          ))));

          let _ = x.send(val);
        }
      })
    }
  };
}

generates! {
  onmessage,
  message_delete,
  message_delete_bulk,
  message_update,
  interaction_create,
  ready,
  resume,
  shards_ready
}

#[async_trait]
impl EventHandler for Handler {
  async fn message(&self, ctx: Context, msg: Message) {
    if let Some(x) = &self.onmessage {
      let _ = x.send(BufValue::Runtime(AnyWrapper(Box::new((ctx, msg)))));
    }
  }

  methods! {
    m
    ready, OnReady, Ready
  }
}

//     resume, ResumedEvent;
//     shards_ready, u32;
//     message_delete, ChannelId, MessageId, Option<GuildId>;
//     message_delete_bulk, ChannelId, Vec<MessageId>, Option<GuildId>;
//     message_update, Option<Message>, Option<Message>, MessageUpdateEvent;
//     interaction_create, Interaction
