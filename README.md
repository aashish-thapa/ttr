ttr: The Terminal Typing Trainer

ttr is a simple, fast, and feature-rich typing trainer for the terminal. It's built in Rust for high performance and offers a seamless, non-flickering typing experience directly in your command line.

Whether you want to practice with a specific text or just want a quick, randomized session, ttr has you covered.
✨ Features

    Blazing Fast: Built with Rust, it provides a smooth, low-latency typing experience.

    Dual Mode: Practice with text from any file or generate a random typing session.

    Real-time Feedback: Color-coded text shows your typing accuracy instantly.

    Detailed Stats: Get a summary of your performance, including time elapsed, WPM, and accuracy.

🚀 Installation

ttr is easy to install if you have the Rust toolchain installed.

    Clone the repository:

    git clone https://github.com/iamaashishthapa/ttr.git
    cd ttr

    Build the project in release mode:

    cargo build --release

    Install the executable:

    sudo mv target/release/ttr /usr/local/bin/

    This command moves the ttr binary to a directory in your system's PATH, making it accessible from anywhere.

💡 Usage

Run ttr from your terminal with or without an argument.
1. Random Typing Session

Simply run the command without a file path. ttr will automatically generate a random set of common English words for you to practice with.

ttr

2. Practice with a Specific File

Provide the path to any .txt file you want to practice with.

ttr path/to/your/file.txt

3. Help and Version

Get more information about the command options.

ttr --help
ttr --version

🤝 Contributing

We welcome contributions! If you have an idea for a new feature or find a bug, please feel free to open an issue or submit a pull request.

    Fork the repository.

    Create your feature branch (git checkout -b feature/AmazingFeature).

    Commit your changes (git commit -m 'feat: Add amazing feature').

    Push to the branch (git push origin feature/AmazingFeature).

    Open a Pull Request.

📄 License

This project is licensed under the MIT License. See the LICENSE file for details.
🙏 Credits

    Created by iamaashishthapa.
