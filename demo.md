# 🎯 Feature Demo

This file demonstrates all the markdown features supported by the Rust Markdown Server.

## Text Formatting

### Bold and Italic
- This is **bold text**
- This is *italic text*
- This is ***bold and italic***

### Inline Code
Use `cargo run` to start the server, or check the `main.rs` file for implementation details.

## Headers

This demonstrates different header levels:

# Header Level 1
## Header Level 2  
### Header Level 3

## Lists

### Unordered Lists
- First item
- Second item
- Third item
  - Nested item (manual spacing)
  - Another nested item

### Using Asterisks
* Item one
* Item two
* Item three

## Code Blocks

### Rust Code
```rust
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server running on http://localhost:7878");
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream),
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
}
```

### JavaScript Code
```javascript
function fetchMarkdown(filename) {
    return fetch(`/${filename}`)
        .then(response => response.text())
        .then(html => {
            document.body.innerHTML = html;
        })
        .catch(error => console.error('Error:', error));
}
```

### Python Code
```python
import requests

def get_markdown_page(server_url, filename):
    response = requests.get(f"{server_url}/{filename}")
    if response.status_code == 200:
        return response.text
    else:
        raise Exception(f"Failed to fetch {filename}")

# Example usage
html_content = get_markdown_page("http://localhost:7878", "demo.md")
print(html_content)
```

### Bash/Shell
```bash
# Start the server
cargo run

# Test with curl
curl http://localhost:7878/demo.md

# Create a new markdown file
echo "# Hello World" > hello.md
```

## Links

### External Links
- [Rust Official Website](https://www.rust-lang.org/)
- [Cargo Documentation](https://doc.rust-lang.org/cargo/)
- [Markdown Guide](https://www.markdownguide.org/)

### Internal Links (if you have these files)
- [Homepage](index.md)
- [About Page](about.md)
- [Back to Directory Listing](list)

## Horizontal Rule

---

## Special Characters Test

This tests HTML escaping:

- Less than: < 
- Greater than: >
- Ampersand: &
- Quotes: "double" and 'single'

## Mixed Content Example

Here's a real-world example combining multiple features:

## 🚀 Quick Start Guide

To get started with this **Rust Markdown Server**:

1. Clone the repository
2. Run `cargo build` to compile
3. Execute `cargo run` to start
4. Open your browser to `http://localhost:7878`

### Configuration Options

You can customize the server by modifying these settings in `main.rs`:

```rust
// Change the port
let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

// Adjust thread pool size  
let pool = ThreadPool::new(10);
```

For more information, visit the [project repository](https://github.com/yourusername/rust-markdown-server).

---

*This demo file showcases the markdown parsing capabilities of our custom Rust implementation.*
