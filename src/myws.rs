use actix::prelude::*;
use actix_web_actors::ws;
// use bytes::{Bytes, BytesMut};
use std::time::{Duration, Instant};

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// websocket connection is long running connection, it easier
/// to handle with an actor
pub struct MyWebSocket {
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    hb: Instant,
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }
}

/// Handler for `ws::Message`
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        // process websocket messages
        println!("WS: {:?}", msg);
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            // Ok(ws::Message::Text(text)) => ctx.text(text),
            // Ok(ws::Message::Text(text)) => ctx.text(self.handle_text(&text)),
            Ok(ws::Message::Text(_)) => ctx.binary(self.new_binary()),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            // Ok(ws::Message::Binary(bin)) => ctx.binary(self.handle_binary(bin.to_vec())),
            Ok(ws::Message::Close(_)) => {
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}

use super::pmodels;
impl MyWebSocket {
    pub fn new() -> Self {
        Self { hb: Instant::now() }
    }

    fn new_binary(&self) -> Vec<u8> {
        let ashirt = pmodels::create_large_shirt(String::from("hello color"));
        let v = pmodels::serialize_shirt(&ashirt);
        v
    }

    fn handle_binary(&self, _bin: Vec<u8>) -> Vec<u8> {
        let ashirt = pmodels::create_large_shirt(String::from("hello color"));
        let v = pmodels::serialize_shirt(&ashirt);
        v
    }

    // fn handle_text<'a>(&self, t: &'a str) -> &'a str {
    //     match t {
    //         "ok\n" => "great",
    //         "thanks\n" => "welcome",
    //         _ => t,
    //     }
    // }

    /// helper method that sends ping to client every second.
    ///
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");

                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }
}
