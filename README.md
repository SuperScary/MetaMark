# **MetaMark (`.mmk`) - Structured Markup & Collaboration**

## **📖 Introduction to MetaMark (`.mmk`)**
MetaMark (`.mmk`) is a **structured, human-readable markup language** designed for **version control, security, and collaboration**. It extends Markdown with built-in **commit tracking, encryption, annotations, real-time collaboration, and export functionality**.

MetaMark is built for speed, security, and scalability, offering both a **CLI tool (`mmk-cli`)** and a **collaboration server (`mmkd`)** for self-hosted or cloud-based document editing.

---

## **📁 Project Modules Overview**
### **1️⃣ Core (`metamark-core/`)**
Responsible for **parsing, syntax handling, and security** features of `.mmk` files.
- **`lexer.rs`** – Tokenizes `.mmk` syntax.
- **`parser.rs`** – Parses MetaMark syntax into an **Abstract Syntax Tree (AST)**.
- **`ast.rs`** – Defines the AST structure for structured document handling.
- **`security.rs`** – Handles **AES-256 encryption, digital signatures, and document integrity checks**.
- **`encryption.rs`** – Provides **file and inline content encryption**.

---
### **2️⃣ Command-Line Interface (`mmk-cli/`)**
The command-line tool for working with `.mmk` files.
- **`main.rs`** – Entry point for CLI execution.
- **`commands.rs`** – Defines CLI commands for `mmk`.
- **`export.rs`** – Exports `.mmk` files to **PDF, HTML, JSON, and DOCX**.
- **`commit.rs`** – Handles **version tracking** and `.mmklog` commit history.
- **`diff.rs`** – Compares file versions and highlights changes.

Example CLI Usage:
```bash
mmk commit -m "Updated security section"
mmk rollback --to 1
mmk export --format pdf
```

---
### **3️⃣ Collaboration Server (`mmkd/`)**
Handles **real-time document collaboration and versioning**.
- **`mmkd.rs`** – Core **server engine**.
- **`auth.rs`** – Handles **user authentication (OAuth 2.0, JWT)**.
- **`collaboration.rs`** – Manages **real-time WebSocket sync and CRDT conflict resolution**.
- **`webapi.rs`** – Provides **REST API endpoints for `.mmk` file management**.

Run the server:
```bash
mmkd --host 0.0.0.0 --port 8080
```

---
### **4️⃣ GUI Editor (`mmk-editor/`)**
A **cross-platform graphical editor** for `.mmk` documents.
- **`tauri/`** – Desktop UI wrapper.
- **`src/`** – Source code for the editor.
- **`components/`** – UI components for **structured text formatting, encryption, and version control**.

Start the editor:
```bash
npm run tauri dev
```

---
### **5️⃣ Tests (`tests/`)**
Unit tests for **parsing, security, CLI commands, and server interactions**.
- **`test_parser.rs`** – Tests `.mmk` syntax parsing.
- **`test_security.rs`** – Validates **encryption and digital signature integrity**.
- **`test_collaboration.rs`** – Ensures **multi-user live editing stability**.

Run tests:
```bash
cargo test
```

---

## **🌍 Features of MetaMark (`.mmk`)**
✅ **Structured Markup:** Markdown-like syntax with annotations, collapsible sections, and structured objects.
✅ **Commit Tracking:** Built-in `.mmklog` version control for inline changes, rollback, and diffs.
✅ **Real-Time Collaboration:** WebSocket-based live editing with role-based permissions.
✅ **Security-First Design:** AES-256 encryption, digital signing, and JWT authentication.
✅ **Multi-Format Export:** Convert `.mmk` files to PDF, HTML, JSON, and DOCX.
✅ **Self-Hosted or Cloud:** Run `mmkd` on your own server or use MetaMark's cloud service.

---

## **🚀 Getting Started**
### **🔹 Install MetaMark CLI**
```bash
npm install -g mmk-cli
```

### **🔹 Create a New `.mmk` File**
```bash
echo "# Welcome to MetaMark" > document.mmk
```

### **🔹 Commit & Track Changes**
```bash
mmk commit -m "Initial document setup"
mmk diff --latest
```

### **🔹 Start a Real-Time Collaboration Session**
```bash
mmkd --host 0.0.0.0 --port 8080
```

---

## **👥 Contributing**
Pull requests are welcome! Fork the repo and submit changes.

```bash
git clone https://github.com/SuperScary/MetaMark.git
cd metamark
npm install
npm run build
```

---

## **📜 License**
MetaMark is licensed under the **MIT License**.

```text
MIT License
Copyright (c) 2025
```

---

🚀 **MetaMark is the next evolution in structured document management.** Try it today! ✨

