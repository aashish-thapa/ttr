# ttr: Practise Your Typing in Terminal

[![Build](https://img.shields.io/badge/build-passing-brightgreen.svg)](#)  
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)  
[![Rust](https://img.shields.io/badge/Rust-stable-orange.svg)](https://www.rust-lang.org)  
[![Repo](https://img.shields.io/badge/github-aashish--thapa%2Fttr-lightgrey.svg)](https://github.com/aashish-thapa/ttr)


`ttr` is a simple, fast, and feature-rich typing trainer for the terminal.  
It's built in **Rust** for high performance and offers a seamless, non-flickering typing experience directly in your command line.

Whether you want to practice with a specific text or just want a quick, randomized session, `ttr` has you covered.

---

## ✨ Features

- **Blazing Fast**: Built with Rust, it provides a smooth, low-latency typing experience.  
- **Dual Mode**: Practice with text from any file or generate a random typing session.  
- **Real-time Feedback**: Color-coded text shows your typing accuracy instantly.  
- **Detailed Stats**: Get a summary of your performance, including time elapsed, WPM, and accuracy.  

---

## 🚀 Installation

`ttr` is easy to install if you have the Rust toolchain installed.

1. **Clone the repository**:

   ```bash
   git clone https://github.com/aashish-thapa/ttr.git
   cd ttr
    ````

2. **Build the project in release mode**:

   ```bash
   cargo build --release
   ```

3. **Install the executable**:

   ```bash
   sudo mv target/release/ttr /usr/local/bin/
   ```

   This moves the `ttr` binary to a directory in your system's `PATH`, making it accessible from anywhere.

---

## 💡 Usage

Run `ttr` from your terminal with or without an argument.

### 1. Random Typing Session

Run without arguments to generate a random set of common English words:

```bash
ttr
```

### 2. Practice with a Specific File

Provide the path to any `.txt` file you want to practice with:

```bash
ttr path/to/your/file.txt
```

### 3. Help and Version

Get more information about options:

```bash
ttr --help
ttr --version
```

---

## 🤝 Contributing

We welcome contributions! If you have an idea for a new feature or find a bug, please open an issue or submit a pull request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'feat: Add amazing feature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

---

## 📄 License

This project is licensed under the **MIT License**.
See the [LICENSE](LICENSE) file for details.

---

## 🙏 Credits

Created by [**aashish-thapa**](https://github.com/aashish-thapa).

## Instruction is created using CHATGPT
