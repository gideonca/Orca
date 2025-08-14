# Introduction
  - [x] Bind to a port
  - [ ] Respond to PING
  - [ ] Respond to multiple PINGs
  - [ ] Handle concurrent clients
  - [ ] Implement the ECHO command
  - [ ] Implement the SET & GET commands
  - [ ] Expiry

# Lists
  - [ ] Create a list
  - [ ] Append an element
  - [ ] Append multiple elements
  - [ ] List elements (positive indexes)
  - [ ] List elements (negative indexes)
  - [ ] Prepend elements
  - [ ] Query list length
  - [ ] Remove an element
  - [ ] Remove multiple elements
  - [ ] Blocking retrieval
  - [ ] Blocking retrieval with timeout

# Streams
  - [ ] The TYPE command
  - [ ] Create a stream
  - [ ] Validating entry IDs
  - [ ] Partially auto-generated IDs
  - [ ] Fully auto-generated IDs
  - [ ] Query entries from stream
  - [ ] Query with -
  - [ ] Query with +
  - [ ] Query single stream using XREAD
  - [ ] Query multiple streams using XREAD
  - [ ] Blocking reads
  - [ ] Blocking reads without timeout
  - [ ] Blocking reads using $

# Transactions
  - [ ] The INCR command (1/3)
  - [ ] The INCR command (2/3)
  - [ ] The INCR command (3/3)
  - [ ] The MULTI command
  - [ ] The EXEC command
  - [ ] Empty transaction
  - [ ] Queueing commands
  - [ ] Executing a transaction
  - [ ] The DISCARD command
  - [ ] Failures within transactions
  - [ ] Multiple transactions

# Replication
  - [ ] Configure listening port
  - [ ] The INFO command
  - [ ] The INFO command on a replica
  - [ ] Initial replication ID and offset
  - [ ] Send handshake (1/3)
  - [ ] Send handshake (2/3)
  - [ ] Send handshake (3/3)
  - [ ] Receive handshake (1/2)
  - [ ] Receive handshake (2/2)
  - [ ] Empty RDB transfer
  - [ ] Single-replica propagation
  - [ ] Multi-replica propagation
  - [ ] Command processing
  - [ ] ACKs with no commands
  - [ ] ACKs with commands
  - [ ] WAIT with no replicas
  - [ ] WAIT with no commands
  - [ ] WAIT with multiple commands

# RDB Persistence
  - [ ] RDB file config
  - [ ] Read a key
  - [ ] Read a string value
  - [ ] Read multiple keys
  - [ ] Read multiple string values
  - [ ] Read value with expiry

# Pub/Sub
  - [ ] Subscribe to a channel
  - [ ] Subscribe to multiple channels
  - [ ] Enter subscribed mode
  - [ ] PING in subscribed mode
  - [ ] Publish a message
  - [ ] Deliver messages
  - [ ] Unsubscribe

# Sorted Sets
  - [ ] Create a sorted set
  - [ ] Add members
  - [ ] Retrieve member rank
  - [ ] List sorted set members
  - [ ] ZRANGE with negative indexes
  - [ ] Count sorted set members
  - [ ] Retrieve member score
  - [ ] Remove a member