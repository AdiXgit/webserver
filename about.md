# About This Server

## Technical Details

This markdown server is built with:

- **Language**: Rust
- **Architecture**: Multi-threaded with custom thread pool
- **Protocol**: HTTP/1.1
- **Features**: Markdown parsing, HTML generation, CSS styling

## Server Architecture

```rust
struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}
```

The server uses a **producer-consumer pattern** with:
- Main thread accepts connections
- Worker threads process requests
- Channel-based job distribution

## Supported Markdown Features

### Headers
- H1, H2, H3 support
- Automatic styling with borders

### Text Formatting
- **Bold text** with `**text**`
- *Italic text* with `*text*`
- `Inline code` with backticks

### Code Blocks
Fenced code blocks with language support:

```python
def hello_world():
    print("Hello from Python!")
```

```javascript
function helloWorld() {
    console.log("Hello from JavaScript!");
}
```

### Lists
- Bullet point lists
- Nested list support
- Automatic HTML conversion

### Links
- [External links](https://www.rust-lang.org)
- [Internal links](index.md)

## Performance

The multi-threaded architecture allows:
- Concurrent request handling
- Scalable performance
- Efficient resource utilization
