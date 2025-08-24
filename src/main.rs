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
use rand::seq::SliceRandom;
use rand::thread_rng;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const WORD_LIST: &str = "the be to of and a in that have I it for not on with he as you do at this but his by from they we say her she or an will my one all would there their what so up out if about who get which go me when make can like time no just him know take people into year your good some could see other than then now look only come its over think also back after use two how our work first well way even new want because any these give day most us";
const LINE_LENGTH_WORDS: usize = 15; // Number of words per line for random mode
const RANDOM_LINES: usize = 5; // Total number of lines for random mode

fn generate_random_lines(num_lines: usize) -> Vec<String> {
    let words: Vec<&str> = WORD_LIST.split_whitespace().collect();
    let mut rng = thread_rng();
    let mut lines = Vec::new();

    for _ in 0..num_lines {
        let mut line_words: Vec<&str> = Vec::new();
        for _ in 0..LINE_LENGTH_WORDS {
            if let Some(word) = words.choose(&mut rng) {
                line_words.push(word);
            }
        }
        lines.push(line_words.join(" "));
    }
    lines
}

fn print_usage(program_name: &str) {
    eprintln!("A simple terminal-based typing trainer.");
    eprintln!();
    eprintln!("Usage:");
    eprintln!("  {} <path_to_text_file>  (Use a specific file)", program_name);
    eprintln!("  {}                     (Generate a random typing session)", program_name);
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

    let content_lines: Vec<String> = if args.len() < 2 {
        // No file path provided, switch to random mode
        generate_random_lines(RANDOM_LINES)
    } else {
        // File path provided, read from the file
        let file_path = &args[1];
        match fs::read_to_string(file_path) {
            Ok(s) => s.lines().map(|s| s.to_string()).collect(),
            Err(e) => {
                eprintln!("Error reading file '{}': {}", file_path, e);
                return Ok(());
            }
        }
    };
    
    if content_lines.is_empty() {
        eprintln!("The provided text source is empty. Please provide a file with content or run without arguments for a random session.");
        return Ok(());
    }

    // --- Terminal Setup ---
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    let mut user_inputs: Vec<String> = vec![String::new(); content_lines.len()];
    let mut current_line_index = 0;
    let start_time = Instant::now();
    let mut correct_chars_total = 0;
    let mut total_chars_total = 0;
    
    // Main typing loop, iterates through each line of the file
    while current_line_index < content_lines.len() {
        // --- Render the screen ---
        queue!(stdout, terminal::Clear(terminal::ClearType::All), MoveTo(0, 0))?;
        
        // Display the source text
        for (line_index, line) in content_lines.iter().enumerate() {
            let row = line_index as u16;
            queue!(stdout, MoveTo(0, row))?;

            let user_input_line = &user_inputs[line_index];
            
            if line_index == current_line_index {
                // This is the active line. Show real-time feedback.
                let mut visual_col = 0;
                for (i, source_char) in line.chars().enumerate() {
                    let user_char_opt = user_input_line.chars().nth(i);
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
                // This is a previous or future line. Render it with its final colors if complete.
                if line_index < current_line_index {
                    // Previous line, render with final colors
                    for (i, source_char) in line.chars().enumerate() {
                        let user_char_opt = user_input_line.chars().nth(i);
                        
                        if let Some(user_char) = user_char_opt {
                            if user_char == source_char {
                                queue!(stdout, SetForegroundColor(Color::Green))?;
                            } else {
                                queue!(stdout, SetForegroundColor(Color::Red))?;
                            }
                        } else {
                            queue!(stdout, ResetColor)?;
                        }
                        queue!(stdout, Print(source_char))?;
                    }
                    queue!(stdout, ResetColor, Print("\n"))?;
                } else {
                    // Future line, render normally
                    queue!(stdout, ResetColor, Print(line))?;
                }
            }
        }

        let current_user_input = &user_inputs[current_line_index];
        let mut visual_cursor_pos = 0;
        for c in current_user_input.chars() {
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
                        if let Some(source_char) = content_lines[current_line_index].chars().nth(current_user_input.len()) {
                            if source_char == '\t' {
                                user_inputs[current_line_index].push('\t');
                            }
                        }
                    }
                    KeyCode::Enter => {
                        let source_line = &content_lines[current_line_index];
                        let user_input_line = &user_inputs[current_line_index];

                        let correct_on_line = user_input_line.chars().zip(source_line.chars())
                            .filter(|(u, s)| u == s)
                            .count();
                        correct_chars_total += correct_on_line;
                        total_chars_total += source_line.len();
                        
                        current_line_index += 1;
                    }
                    KeyCode::Backspace => {
                        user_inputs[current_line_index].pop();
                    }
                    KeyCode::Char(c) => {
                        user_inputs[current_line_index].push(c);
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
