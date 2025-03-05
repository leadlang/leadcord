$discord: *import leadcord

$token: env TOKEN
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

# Start Client
$discord::client::run ->$token ->$intents ->$handlers