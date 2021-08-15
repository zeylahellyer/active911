# active911

Unofficial library and CLI for accessing Active911's "API".

I wanted a feed to know what was going on when I saw a fire truck going by. The
city provides this information on a website, but it'd be cool to be notified in
real-time. This is a client library (and small accompanying CLI) to scrape the
JavaScript emitted by the website that includes a JSON string with recent fire
alarm information.

## Library

`active911` is a library with serde definitions of the API along with a function
to request an agency's recent alarms.

```rust
use active911::request;

let response = request::alarms(agency_key).await?;

for alarm in response.alarms {
    println!("alarm #{}: {}", alarm.id, alarm.description);
}
```

Active911 is not on crates.io at this time. You can install the library by
depending on the Git repository in your `Cargo.toml`:

```toml
[dependencies]
active911 = { default-features = false, git = "https://github.com/zeylahellyer/active911" }
```

## CLI

A light CLI is included that calls into the library and displays basic
information about recent alarms: the type of alarm, when it was, and where it
was.

```shell
ACTIVE911_KEY="put key here" active911-cli
REASON - 8.8 hr ago
  Location
REASON - 9.1 hr ago
  Location
REASON - 10 hr ago
  Location
REASON - 16.2 hr ago
  Location
REASON - 17.8 hr ago
  Location
```

To install it run:

```shell
$ cargo install --bin active911-cli --git "https://github.com/zeylahellyer/active911"
```

### License

This project is licensed under the ISC.
