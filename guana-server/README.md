<h2 align="center"><img align="center" src="https://github.com/vrmiguel/vrmiguel/assets/36349314/5170cc9d-e6bf-4e47-a7c0-e02c8778b8ec" height="70px" />  guana-server </h2>

`guana-server` serves [`guana-service`](../guana-service) alongside a [reflection service](https://github.com/grpc/grpc/blob/master/doc/server-reflection.md), for increased discoverability.

When run, the server will bind to the given address or, if not supplied, default to `[::1]:50051`.

Clients, such as `guana-client`, will then be able to query the [`UserAgentAnalyzer`](../proto/user_agent_analyzer.proto) gRPC service.

## Usage

```
Usage: guana-server [--address <address>]

A fast gRPC server to filter User-Agent strings.

Options:
  --address         the address in which the service will be served
  --help            display usage information
```