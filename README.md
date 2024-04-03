WORK IN PROGRESS

This repo holds, currently, a binary for testing out an implementation of the AMQ 0-9-1 protocol from scratch in Rust.

TODO:

- [x] Move to Bincode
- [x] Add Properties - Some weirdness with table only accepting long string as it's value, but it's there.
- [ ] Reply-to structure
- [ ] Abstract Connection to another Struct
- [ ] Sort out Channels
- [ ] Sort out Tokio implementation
- [ ] Convert to Lib
- [ ] Start building out User API
- [ ] Logging
- [ ] Split pub/sub clients? Might have a lot of code reuse (although this will be mitigated by abstracting connection)
- [ ] Builder API for clients
- [ ] Implement more functionality - Exchanges + Transactions etc.
- [ ] Tests and Documentation for Public API
- [ ] Profit? 

