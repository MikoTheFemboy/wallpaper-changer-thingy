use std::{io::{self, BufWriter ,Write, stdout}, path::PathBuf, os::unix::{net::UnixStream}};
use crossterm::{cursor, execute, terminal::{Clear, ClearType, window_size}};
use serde_json::{json, value};

// Draw logic
pub fn draw(files: &[PathBuf], crt_idx:usize) -> io::Result<()>{
    let mut out = BufWriter::new(stdout());
    print!("\x1B[2J\x1B[3J\x1B[H"); // actually clears the screen, for some reason Clear(ClearType::All) doesn't work on alacritty... 
    execute!(out, cursor::MoveTo(0,0), Clear(ClearType::All))?; // idk though, i might just be dumb as hell

    let mut buffer = String::new();
    buffer.push_str("Miko's Wallpaper Changer <3\r\n\n");

    for(i, path) in files.iter().enumerate(){
        let name = path.file_name().unwrap().to_string_lossy();
        if i == crt_idx {
            buffer.push_str(&format!("  >>> [{}] <<<\r\n", name));
        }
        else {
            buffer.push_str(&format!(" {}\r\n", name));
        }
    }

    buffer.push_str("\r\n");

    write!(out, "{}", buffer)?;
    out.flush()?;
    Ok(())
}

pub fn send_command(socket_path:&str, args:Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = UnixStream::connect(socket_path)?;
    let cmd = json!({ "command": args }).to_string() + "\n";
    stream.write_all(cmd.as_bytes())?;
    Ok(())
}