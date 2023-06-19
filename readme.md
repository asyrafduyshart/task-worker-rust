# Rust-Task-Worker

Rust-Task-Worker is a task processing server built in Rust that is capable of handling HTTP and Kafka tasks. It accepts HTTP POST requests containing tasks, then routes and processes them accordingly.

## Setup & Running

This project requires [Rust](https://www.rust-lang.org/tools/install) and [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed in your local environment. Once installed, you can proceed with the following steps:

1. Clone the repository
```bash
git clone https://github.com/user/rust-task-worker.git
cd rust-task-worker
```

2. Build the project
```bash
cargo build
```

3. Run the project
```bash
cargo run
```

## Environment Variables

The application uses the following environment variables:

* `LISTEN_IP`: The IP address that the server should listen on (default: `127.0.0.1`)
* `APP_PORT`: The port that the server should bind to (default: `8050`)
* `KAFKA_BROKER`: The address of the Kafka broker (no default)
* `KAFKA_KEY`: The key for the Kafka broker (no default)
* `KAFKA_SECRET`: The secret for the Kafka broker (no default)

## Available API Endpoints

- `POST /task/kafka` : Accepts tasks for Kafka
- `POST /task/http` : Accepts tasks for HTTP

### Sample Kafka Task Request

```json
{
  "topic": "test_topic",
  "key": "key1",
  "payload": "This is the message payload"
}
```

### Sample HTTP Task Request

```json
{
  "url": "http://example.com",
  "method": "POST",
  "payload": "This is the message payload",
  "headers": {
    "Content-Type": "application/json"
  },
  "timeout": 5
}
```

## Error Handling

Both the HTTP and Kafka handlers are designed to return HTTP responses with descriptive error messages in case of failures such as incorrect JSON input or request sending errors. Errors are also logged to the console for debugging purposes.

## Testing

You can test the application by sending HTTP POST requests to the server using a tool like curl or Postman.

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under the [MIT](https://choosealicense.com/licenses/mit/) License.
