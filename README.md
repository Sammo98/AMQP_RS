WORK IN PROGRESS

This repo holds, currently, a binary for testing out an implementation of the AMQ 0-9-1 protocol from scratch in Rust.

TODO:

- [x] Move to Bincode
- [x] Add Properties - Some weirdness with table only accepting long string as it's value, but it's there.
- [x] Convert to Lib
- [ ] Abstract Connection out - partially done, need to do more
- [ ] Split pub/sub clients? Might have a lot of code reuse (although this will be mitigated by abstracting connection)
- [ ] Sort out Channels - Okay so when you call channel open, the channel id is what you send in the header. The channel number is 0 for all frames which are global to the connection and 1-65535 for frames that refer to specific channels. This should be an auto managed thing or optionally managed by the user.
- [ ] Start building out User API
- [ ] Error handling
- [ ] Table Builder and Hashmap functionality for access. Have this as a struct field but ignore on endec 
- [ ] Builder API for clients
- [ ] Implement more functionality - Exchanges + Transactions etc.
- [ ] Reply-to structure
- [ ] Ensure both high level api and lower level api for granular control is implemented.
- [ ] Logging
- [ ] Sort out Tokio implementation
- [ ] Tests and Documentation for Public API
- [ ] CI
- [ ] Profit? 


