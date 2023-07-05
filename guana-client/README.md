<h2 align="center"><img align="center" src="https://github.com/vrmiguel/vrmiguel/assets/36349314/5170cc9d-e6bf-4e47-a7c0-e02c8778b8ec" height="70px" />  guana-client </h2>

The Guana Client is a tool designed to interact with the Guana server, facilitating the evaluation of User-Agents.

The library allows for easy integration of the Guana Client into larger Rust projects, providing a convenient way to interact with the server. The binary executable, on the other hand, enables interactive querying of the gRPC server with multiple User-Agents, making it simple to evaluate and process User-Agent data.

## Binary Usage

The guana-client executable has two modes: non-interactive and interactive.

```
Usage: guana-client [<user_agents...>] [--address <address>]

A fast gRPC server to filter User-Agent strings.

Positional Arguments:
  user_agents       user agents to check on the gRPC server. If none,
                    interactive mode will be enabled.

Options:
  --address         the server address to connect to
  --help            display usage information
```

### Non-interactive mode

Given a list of user-agent strings, `guana-client` will print their corresponding decisions in order, newline-terminated.

```bash
$ guana-client --address http://localhost:50051 \
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.0 Safari/605.1.15" \
    "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/114.0"
BLOCK
ALLOW
```

### Interactive mode

If run without positional arguments, `guana-client` will wait for user agents to be inserted through `stdin`:

```bash
$ guana-client --address http://localhost:50051
>> Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.0 Safari/605.1.15
> User-Agent Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.0 Safari/605.1.15: BLOCK
>> Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/114.0
> User-Agent Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/114.0: ALLOW
>> 
Received Ctrl+C. Exiting
```

## Library use

The crate exports `GuanaClient`, a wrapper over the auto-generated code from Tonic.
One example of its usage:

```rust
use guana_client::{GuanaClient, Decision};

let mut client = GuanaClient::connect(address).await?;
assert_eq!(
    client.analyze_user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/114.0").await?,
    Decision::Allow
);

assert_eq!(
    client.analyze_user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 Safari/605.1.15").await?,
    Decision::Deny
);
```