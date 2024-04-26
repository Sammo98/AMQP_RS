WORK IN PROGRESS

This repo holds, currently, a binary for testing out an implementation of the AMQ 0-9-1 protocol from scratch in Rust.

TODO:

- [x] Move to Bincode
- [x] Add Properties - Some weirdness with table only accepting long string as it's value, but it's there.
- [x] Convert to Lib
- [x] Sort out Tokio implementation - Have moved to channels which is going to be much nicer to deal with
- [x] Split pub/sub clients? Might have a lot of code reuse (although this will be mitigated by abstracting connection)
- [x] Work out how channel numbers are picked
- [x] Bundle Header and Ids - Remove frame end?
- [x] Abstract Connection out - This should interact with the TcpAdapter

- [ ] Internal API improvements
  - [x] Impl default for properties with useful information.
  - [x] Bits? Is there a better way to handle this? (A:bool, B, ... )  -> Bits(vec![0u8, ...])?
  - [x] impl From for encde types
  - [x] Make all reserved constant 0_u16
  - [x] Channel Handling - non-multiplexed for now
  - [ ] Auto generate consumer tag - how to do without uuid
  - [ ] Table Builder
  - [ ] Error handling - impl From or thiserror 
  - [ ] Tests
  - [ ] Do this last - macro for implementing shared behaviour between pub and sub to avoid code duplication

- [x] Add all Frames
  - [x] Connection
  - [x] Channel
  - [x] Queue
  - [x] Exchange
  - [x] Basic
  - [x] Transaction

- [ ] Test all Frames
  - [ ] Connection
  - [ ] Channel
  - [ ] Queue
  - [ ] Exchange
  - [ ] Basic
  - [ ] Transaction

- [ ] Start building out User API
  - [ ] Builder for Connection
  - [ ] Builder for Publisher
  - [ ] Builder for Consumer
  - [ ] Add property like access for bit fields and table fields
  - [ ] Handle multi-queue consumer with queue:ConsumerTaskSender map
  - [ ] Reply-to structure
  - [ ] Ensure both high level api and lower level api for granular control is implemented.
  - [ ] Logging
  - [ ] Tests
  - [ ] Documentation
  - [ ] Ensure pub/private access is correct

- [ ] Try to figure out why the fuck we're not picking up messages which are already on the queue after basic.consume
- [ ] Tests and Documentation for Public API
- [ ] CI
- [ ] Profit? 

### Future Features:

- [ ] SSL


