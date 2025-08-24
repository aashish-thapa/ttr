
use std::env;
use std::fs;
use std::io::{self, Write};
use std::time::Instant;

use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyCode, KeyEventKind},
    execute, queue,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn print_usage(program_name: &str) {
    eprintln!("A simple terminal-based typing trainer.");
    eprintln!();
    eprintln!("Usage: {} <path_to_text_file>", program_name);
    eprintln!();
    eprintln!("Options:");
    eprintln!("  -h, --help     Print this help message and exit.");
    eprintln!("  --version      Print version information and exit.");
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let program_name = &args[0];

    // Check for help or version flags
    if args.len() > 1 {
        let arg = &args[1];
        if arg == "-h" || arg == "--help" {
            print_usage(program_name);
            return Ok(());
        } else if arg == "--version" {
            eprintln!("ttr version {}", VERSION);
            return Ok(());
        }
    }

    // Check if a file path was provided
    if args.len() < 2 {
        print_usage(program_name);
        return Ok(());
    }
    let file_path = &args[1];

    // Read the content of the file into a vector of strings (lines).
    let content_lines: Vec<String> = match fs::read_to_string(file_path) {
        Ok(s) => s.lines().map(|s| s.to_string()).collect(),
        Err(e) => {
            eprintln!("Error reading file '{}': {}", file_path, e);
            return Ok(());
        }
    };

    // --- Terminal Setup ---
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    let mut user_input = String::new();
    let mut current_line_index = 0;
    let start_time = Instant::now();
    let mut correct_chars_total = 0;
    let mut total_chars_total = 0;
    
    // Main typing loop, iterates through each line of the file
    while current_line_index < content_lines.len() {
        let source_line = &content_lines[current_line_index];

        // --- Render the screen ---
        queue!(stdout, terminal::Clear(terminal::ClearType::All), MoveTo(0, 0))?;
        
        // Display the source text
        for (line_index, line) in content_lines.iter().enumerate() {
            let row = line_index as u16;
            queue!(stdout, MoveTo(0, row))?;
            
            if line_index == current_line_index {
                // This is the active line. Show real-time feedback.
                let mut visual_col = 0;
                for (i, source_char) in line.chars().enumerate() {
                    let user_char_opt = user_input.chars().nth(i);
                    let char_to_print = source_char;
                    
                    if let Some(user_char) = user_char_opt {
                        if user_char == source_char {
                            queue!(stdout, SetForegroundColor(Color::Green))?;
                        } else {
                            queue!(stdout, SetForegroundColor(Color::Red))?;
                        }
                    } else {
                        queue!(stdout, ResetColor)?;
                    }
                    
                    if source_char == '\t' {
                        let spaces_to_add = 8 - (visual_col % 8);
                        for _ in 0..spaces_to_add {
                            queue!(stdout, Print(' '))?;
                        }
                        visual_col += spaces_to_add;
                    } else {
                        queue!(stdout, Print(char_to_print))?;
                        visual_col += 1;
                    }
                }
            } else {
                queue!(stdout, ResetColor, Print(line))?;
            }
        }
        
        let mut visual_cursor_pos = 0;
        for c in user_input.chars() {
            if c == '\t' {
                visual_cursor_pos += 8 - (visual_cursor_pos % 8);
            } else {
                visual_cursor_pos += 1;
            }
        }

        let col = visual_cursor_pos as u16;
        let row = current_line_index as u16;
        queue!(stdout, MoveTo(col, row))?;
        
        stdout.flush()?;

        // --- Handle user input ---
        if let Event::Key(key_event) = event::read()? {
            if key_event.kind == KeyEventKind::Press {
                match key_event.code {
                    KeyCode::Esc => break,
                    KeyCode::Tab => {
                        if let Some(source_char) = source_line.chars().nth(user_input.len()) {
                            if source_char == '\t' {
                                user_input.push('\t');
                            }
                        }
                    }
                    KeyCode::Enter => {
                        let correct_on_line = user_input.chars().zip(source_line.chars())
                            .filter(|(u, s)| u == s)
                            .count();
                        correct_chars_total += correct_on_line;
                        total_chars_total += source_line.len();
                        
                        current_line_index += 1;
                        user_input.clear();
                    }
                    KeyCode::Backspace => {
                        user_input.pop();
                    }
                    KeyCode::Char(c) => {
                        user_input.push(c);
                    }
                    _ => {}
                }
            }
        }
    }

    // --- Terminal Cleanup ---
    execute!(stdout, LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    // --- Display Final Stats ---
    let elapsed = start_time.elapsed().as_secs_f64();
    let accuracy = if total_chars_total > 0 {
        (correct_chars_total as f64 / total_chars_total as f64) * 100.0
    } else {
        0.0
    };
    let wpm = if elapsed > 0.0 {
        (correct_chars_total as f64 / 5.0) / (elapsed / 60.0)
    } else {
        0.0
    };

    println!("\n--- Typing Session Complete ---");
    println!("Time Elapsed: {:.2} seconds", elapsed);
    println!("Typing Speed: {:.2} WPM", wpm);
    println!("Accuracy: {:.2}%", accuracy);
    println!("-----------------------------");

    Ok(())
}
