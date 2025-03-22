$discord: *import leadcord

$token: fmt "TOKEN"
$token: env $token
$token: unwrap ->$token
print $token

$intents: $discord::intent::all
$handlers: $discord::handler::new

# Register onMessage
$onmessage: *mod ./handlers/message
$h: $discord::handle::onmessage ->&$handlers
*listen ->$onmessage ->$h

# Register onready
$ready: *mod ./handlers/ready
$h: $discord::handle::ready ->&$handlers
*listen ->$ready ->$h

$start: fmt "Starting"
print $start

# Start Client
$discord::client::run ->$token ->$intents ->$handlers