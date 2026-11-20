# SigmaOS-activemq Component Wiki

## Overview
SigmaOS-activemq provides Apache ActiveMQ compatibility and message queue services for the SigmaOS operating system. It demonstrates the messaging subsystem and enterprise integration patterns within the SigmaOS ecosystem, following the principles of clear abstraction and deterministic message initialization.

## Principles
- Single Responsibility
- Messaging Abstraction
- Fail-Fast on Messaging Initialization Errors
- Deterministic Messaging Setup
- Clear Abstraction Over Messaging APIs
- Composition over Inheritance for messaging extensions

## Architecture
SigmaOS-activemq follows a messaging pattern:
- **Messaging Core**: Provides message queue operations, topic publishing, and subscription management
- **Broker Integration**: Supports ActiveMQ broker connectivity and failover configurations
- **Message Format**: Supports various message formats (JSON, XML, bytes) and serialization
- **Error Handling**: Fail-fast on invalid messaging configurations or unsupported broker types

## API
The component provides the following public APIs:
- `messaging::queue::create(name)` - Create a new message queue
- `messaging::queue::send(name, message)` - Send a message to a queue
- `messaging::queue::receive(name)` - Receive a message from a queue
- `messaging::topic::publish(name, message)` - Publish a message to a topic
- `messaging::subscription::subscribe(name, callback)` - Subscribe to a topic with a callback
- Error types for messaging failure scenarios or unsupported broker types

## Test Structure (Arrange-Act-ASSERT)
All tests follow the AAA pattern:
1. **Arrange**: Set up test environment with specific messaging configuration
2. **Act**: Execute the code under test (message send, queue creation)
3. **Assert**: Verify the expected messaging results

Example:
```rust
#[test]
fn test_messaging_queue_send_aaa() {
    // Arrange
    let queue_name = "test-queue";
    let message = Message::new("test payload");
    
    // Act
    let result = messaging::queue::send(queue_name, message);
    
    // Assert
    assert!(result.is_ok());
    assert!(messaging::queue::receive(queue_name).is_some());
}
```

## Development
- Follows Rust best practices with `#![forbid(unsafe_code)]`
- Messaging initialization is best-effort; gracefully degrades on unsupported configurations
- Comprehensive error types for all messaging failure scenarios
- Unit tests provided for queue operations and topic publishing
- Integration tests for full messaging system with actual ActiveMQ broker

## TODO
- Add support for additional messaging protocols (STOMP, AMQP, Kafka)
- Expand broker integration capabilities (cluster mode, persistence, security)
- Add message format support (JSON, XML, Protobuf, serialization extensions)
- Create integration tests with actual ActiveMQ broker instances
- Implement message scheduling and delayed delivery features