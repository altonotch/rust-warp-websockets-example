Example Websocket Server
========================

Simple example showing how to implement a websocket server in Rust on top of Warp.

Why?
====

1st of all to see how it's done.

2nd - we needed something to simulate websockets server in order to write up the integration client,
which will be generic, asyncio (python) client that is capable of managing many open connections to many
different webservers.
If you have a similar need and don't want to write up your own, feel free to use ours or contact us, we are more than happy to consult :)

Whats not included
==================

PubSub is not included. If you're specifically looking for a pubsub example check out following links:

https://blog.logrocket.com/how-to-build-a-websocket-server-with-rust/ (`code <https://github.com/zupzup/warp-websockets-example/blob/master/src/handler.rs>`_
)

`warp example <https://github.com/seanmonstar/warp/blob/a584ca375f620316bbd92a1bc6683c69e02c24ca/examples/websockets_chat.rs>`_


If you really would like to see how to setup pubsub in the context of this project, open an issue and we will be happy to add this.


URLs
====

1. ``GET`` ``/health-check`` - pinging this endpoint will return ok message. Can be used to validate that the server is running.

2. ``/ws`` - websocket endpoint with no authorization.

    - Expected payload `{"kind": "some kind", "message": "some message"}`.
    - Result returned `{"status": "success", "response": "awesome message"}`.


3. ``/ws-private`` - websocket endpoint with api token authorization. Same payloads as in step 2.

    - Expected header `'Authorization: Token 6smtr8ke3s7yq63f3zug9z3th'`
