WORK IN PROGRESS

This repo holds, currently, a binary for testing out an implementation of the AMQ 0-9-1 protocol from scratch in Rust.

TODO:

- [x] Move to Bincode
- [x] Add Properties - Some weirdness with table only accepting long string as it's value, but it's there.
- [x] Convert to Lib
- [x] Sort out Tokio implementation - Have moved to channels which is going to be much nicer to deal with
- [x] Split pub/sub clients? Might have a lot of code reuse (although this will be mitigated by abstracting connection)
- [x] Work out how channel numbers are picked
- [ ] Add all Frames
- [ ] Bundle Header and Ids - Remove frame end?
- [ ] Start building out User API
- [ ] Abstract Connection out - This should interact with the TcpAdapter
- [ ] Consumer might have multiple handlers for different queues. I need to route by consumer tag. Will need a map of queue_name:ConsumerTaskReceiver.
- [ ] Error handling - This error?
- [ ] Table Builder and Hashmap functionality for access. Have this as a struct field but ignore on endec 
- [ ] Impl default for properties with useful information.
- [ ] Builder API for clients
- [ ] Implement more functionality - Exchanges + Transactions etc.
- [ ] Reply-to structure
- [ ] Ensure both high level api and lower level api for granular control is implemented.
- [ ] Logging
- [ ] Try to figure out why the fuck we're not picking up messages which are already on the queue after basic.consume
- [ ] Tests and Documentation for Public API
- [ ] CI
- [ ] Profit? 


