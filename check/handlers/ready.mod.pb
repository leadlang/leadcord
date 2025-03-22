declare ready

fn on ->$ready
  $s: fmt "I am online!"
  print $s

  $ctx: $ready::getContext
  $ctx::status::set dnd
  $ctx::activity::set watching ->$s

  drop ->$ctx
*end
