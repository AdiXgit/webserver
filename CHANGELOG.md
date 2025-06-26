# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned
- Enhanced markdown features (tables, strikethrough, task lists)
- Syntax highlighting for code blocks
- File upload web interface
- Live reload functionality
- HTTPS support
- Docker containerization

## [0.1.0] - 2025-06-26

### Added
- Initial release of Rust Markdown Server
- Multi-threaded web server with custom thread pool
- Real-time markdown to HTML conversion
- Support for basic markdown features:
  - Headers (H1, H2, H3)
  - Bold and italic text
  - Inline code and code blocks
  - Unordered lists
  - Links
- Beautiful CSS styling with responsive design
- Directory listing functionality
- 404 error handling
- HTML character escaping for security
- Graceful server shutdown
- Comprehensive documentation
- MIT license

### Technical Features
- HTTP/1.1 compliant server
- Producer-consumer pattern with mpsc channels
- Memory-safe implementation with Rust ownership
- Configurable thread pool size
- Efficient file I/O operations
- Zero-copy string processing where possible

### Documentation
- Comprehensive README with usage examples
- Feature demonstration file (demo.md)
- Architecture documentation
- Performance benchmarks
- Contributing guidelines
- MIT license

### Development
- GitHub Actions CI/CD pipeline
- Cross-platform build support (Linux, Windows, macOS)
- Code formatting and linting checks
- Security audit integration
- Cargo.toml with proper metadata
