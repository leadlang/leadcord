declare ready

fn on ->$event
  $s: fmt "I am online!"
  print $s

  $dsc: *import leadcord

  $ctx: $dsc::ready::separate ->&$event

  $dsc::activity::status $ctx dnd
  $dsc::activity::set $ctx playing $s
*end
