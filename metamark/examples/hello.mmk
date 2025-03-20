---
title: Hello MetaMark
authors: ["Example User"]
version: "1.0.0"
tags: ["example", "tutorial"]
created_at: "2024-03-20T13:00:00Z"
updated_at: "2024-03-20T13:00:00Z"
---

# Welcome to MetaMark

This is an example document showcasing MetaMark's features.

## Rich Text Formatting

You can use **bold**, *italic*, and `inline code` formatting.

## Code Blocks

Here's a Rust code example:

```rust
fn main() {
    println!("Hello, MetaMark!");
}
```

## Math Support

Inline math: \(E = mc^2\)

Block math:
\[
\int_{0}^{\infty} e^{-x^2} dx = \frac{\sqrt{\pi}}{2}
\]

## Diagrams

```mermaid
sequenceDiagram
    participant User
    participant MetaMark
    participant Server

    User->>MetaMark: Edit document
    MetaMark->>Server: Sync changes
    Server->>MetaMark: Broadcast updates
    MetaMark->>User: Show changes
```

## Lists

1. First item
2. Second item
   - Nested item
   - Another nested item
3. Third item

## Tables

| Feature | Status |
|---------|--------|
| Parsing | ✅ |
| Export  | ✅ |
| Sync    | ✅ |

## Links and References

Visit [MetaMark Documentation](https://metamark.docs) for more information. 