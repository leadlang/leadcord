declare message

fn on ->$msg
  $dsc: *import leadcord

  $ctx: $dsc::message::separate ->&$msg
  
  print $msg $ctx
*end
