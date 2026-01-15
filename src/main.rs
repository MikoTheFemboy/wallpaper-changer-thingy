use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    cursor,
};
use rsbash::rashf;
use std::{fs, os::unix::process::CommandExt, process::{Command, Stdio, exit}};
use std::io::{stdout};
use std::path::PathBuf;
use dirs;
use std::env;

fn invalid_arg(){
    static HELP: &str = "Change Wallpaper
    USAGE:
    cw [arg1] [optional]
    VID     = only search and play videos       : Supported files gif, mp4, mkv, webm
    IMG     = only search and display images    : Supported files png, jpg, jpeg
    MUTE    = OPTIONAL mute video

    Keybinds:
    M       =   Toggle mute
    K       =   Kill mpvpaper
    Q/ESC   =   Quit
    S       =   Start swww daemon
    UP/DOWN =   Seek available
    ENTER   =   Select as wallpaper

Made by Miko みこ <3";
    
    println!("{}", HELP);
    exit(1);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let mut fit: bool = false;
    let video: bool = if args.len() > 1{
        match args[1].as_str(){
            "IMG" => {
                if args.len() > 2 && args[2] == "FIT"{
                    fit = true;
                }
                else if args.len() > 2{
                    invalid_arg();
                    unreachable!()
                }
                false
            },
            "VID" => {
                true
            },
            _ => {invalid_arg(); unreachable!() }
        }
    }
    else {
        invalid_arg();
        unreachable!()
    };

    let mut mute: bool = false;
    if video && args.len() > 2 && args[2] == "MUTE" {
        mute = true;
    }
    else if video && args.len() > 2{
        invalid_arg();
    }

    let wallpaper_dir = dirs::home_dir().unwrap().join("Pictures/wallpapers");
    
    let files: Vec<PathBuf> = fs::read_dir(&wallpaper_dir)?.filter_map(|entry|entry.ok().map(|e| e.path()))
    .filter(|path|{
        let ext = path.extension().and_then(|s|s.to_str()).unwrap_or("");
        if video{
            matches!(ext, "gif" | "mp4" | "mkv" | "webm")
        }
        else{
            matches!(ext, "png" | "jpg" | "jpeg")
        }
    }).collect();

    if files.is_empty(){
        println!("No wallpapers found in {:?}", wallpaper_dir);
        return Ok(())
    }

    enable_raw_mode()?;
    let mut stdout = stdout();
    let mut crt_idx= 0;

    loop {
        execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0))?;
        println!("Miko's swww & mpvpaper wallpaper changer <3\r\n");
        println!("Wallpaper directory at: {:?}\r\n", wallpaper_dir);

        for(i, path) in files.iter().enumerate(){
            let name = path.file_name().unwrap().to_string_lossy();
            if i == crt_idx {
                println!(" > [{}]\r\n", name);
            }
            else {
                println!("   {}\r\n", name);
            }
        }

        if let Event::Key(key_event) = event::read()?{
            match key_event.code{
                KeyCode::Char('s') => {let _ = Command::new("swww-daemon").process_group(0).stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null()).spawn();},
                KeyCode::Up => {
                    if crt_idx > 0 {crt_idx -= 1;}
                },
                KeyCode::Down => {
                    if crt_idx < files.len() - 1 {crt_idx += 1;}
                },
                KeyCode::Enter => {
                    let selected = files[crt_idx].to_str().unwrap();
                    if !video && fit == true{
                        let _ = rashf!("swww img '{}' --resize fit", selected);
                    }
                    else if !video{
                        let _ = rashf!("swww img '{}'", selected);
                    }
                    else {
                        let _ = rashf!("pkill mpvpaper");
                        if !mute{
                            let _ = Command::new("nohup").arg("mpvpaper").arg("ALL").arg("-o").arg("--loop-file=inf input-ipc-server=/tmp/mpv-socket").arg(selected).arg("&").arg("disown").spawn();
                        }
                        else{
                            let _ = Command::new("nohup").arg("mpvpaper").arg("ALL").arg("-o").arg("--loop-file=inf no-audio input-ipc-server=/tmp/mpv-socket").arg(selected).arg("&").arg("disown").spawn();
                        }
                    }
                }
                KeyCode::Char('m') => {let _ = rashf!("echo 'cycle mute' | socat - /tmp/mpv-socket");},
                KeyCode::Char('k') => {let _ = rashf!("pkill mpvpaper");},
                KeyCode::Char('q') | KeyCode::Esc => break,
                _ => {}
            }
        }
    }
    disable_raw_mode()?;
    println!("goodbye! <3");
    Ok(())
}
