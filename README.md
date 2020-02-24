# omnisci-rs

A Rust client for connecting to [OmniSciDB](https://github.com/omnisci/omniscidb) via its RPC protocol of [Thrift](https://thrift.apache.org/). This package takes care of creating a client with the Thrift binary protocol and buffered transport, and exposes all of the OmniSci Thrift types and methods for use with it.

## Requirements

This client has been tested on these versions, and is expected to work with more recent versions unless otherwise noted:

* Rust 1.41+
* OmniSciDB 5.1+

It is also likely to work on earlier versions of OmniSciDB, but this is not officially supported.

## How to use

Add `omnisci` to your `Cargo.toml`:

```toml
[dependencies]
omnisci = "0.1.0"
```

Then create a client and connect (to the 'Backend TCP' port on the OmniSciDB instance):

```rust
use omnisci;

let mut client = omnisci::client::create("127.0.0.1:6274")?;

let user = "admin".to_string();
let passwd = "HyperInteractive".to_string();
let database = "omnisci".to_string();

let session = client.connect(user, passwd, database)?;

let query = "SELECT * FROM flights_donotmodify LIMIT 5".to_string();
let columnar = false;
let nonce = "1".to_string();
let first_n = 10000;
let at_most_n = -1;

let results = client.sql_execute(session, query, columnar, nonce, first_n, at_most_n);
```

See the examples folder for a more complete example.

## Contributing

### Building

This project is built with [Cargo](https://github.com/rust-lang/cargo), using `cargo build`. Each file in src/ except for lib.rs is automatically generated using Thrift, based on the the Thrift definitions stored at [omniscidb](https://github.com/omnisci/omniscidb) ([mapd.thrift](https://github.com/omnisci/omniscidb/blob/master/mapd.thrift), plus the .thrift files it references via include). To regenerate them, install Thrift 0.13.0 with your system's package manager, clone [omniscidb](https://github.com/omnisci/omniscidb) locally, and then run:

```
./generate_thrift_bindings.sh ../omniscidb
```

All source is also formatted with [rustfmt](https://github.com/rust-lang/rustfmt), via `cargo fmt`.

### Testing

The integration tests expect a local running instance of OmniSciDB on the default Backend TCP port: http://localhost:6274

If running natively (such as with the `./startomnisci` script), that port is accessible by default. If running with Docker, expose the Backend TCP port in addition to the default Frontend Web port using `-p 6274:6274`.

When ready, run `cargo test`.

## License

This project is licensed under the MIT license.
