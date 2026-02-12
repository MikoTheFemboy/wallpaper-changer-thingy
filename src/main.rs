<<<<<<< HEAD
use std::{env::{self, args}, fs, os::unix::process::CommandExt, process::{Command, Stdio, exit}, path::PathBuf};
use dirs;
use libc::name_t;
use users::get_effective_uid;
use rsbash::{rash, rashf};
use crossterm::{event::{self, Event, KeyCode, KeyEvent, KeyModifiers}, terminal::{disable_raw_mode, enable_raw_mode}};

mod helper;

// Invalid Argument
=======
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

>>>>>>> 2c3fef901187d3b80d65311f6434d0bc740ad81c
fn invalid_arg(){
    static HELP: &str = "Change Wallpaper
    USAGE:
    cw [arg1] [optional]
    VID     = only search and play videos       : Supported files gif, mp4, mkv, webm
    IMG     = only search and display images    : Supported files png, jpg, jpeg
    MUTE    = OPTIONAL mute video
<<<<<<< HEAD
    FIT     = OPTIONAL fit wallpaper to screen
=======
>>>>>>> 2c3fef901187d3b80d65311f6434d0bc740ad81c

    Keybinds:
    Q/ESC   =   Quit
    K       =   Kill mpvpaper
    R       =   Refresh
    M       =   Toggle mute
    f/b     =   Seek forward 5 seconds (or whatever you changed it to)
    F/B     =   Seek forward 1 seconds (or whatever you changed it to)
    S       =   Start swww daemon
    UP/DOWN =   Seek available
    ENTER   =   Select as wallpaper

Made by Miko みこ <3";
    
    println!("{}", HELP);
    exit(1);
}

<<<<<<< HEAD
// I'll just keep these four as is. Until i can have a good enough reason to make a config file.
// But tbh, i'd rather keep it simple enough to not need a config file.
static SEEK_VALUE:u32 = 5;
static SEEK_VALUE_PRECISE:f32 = 0.5;

static MPV_SOCKET_PATH:&str = "/tmp/mpv-socket";
static WALLPAPER_DIR:&str = "Pictures/wallpapers";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Checks if the user tries to run this program as root
    if get_effective_uid() == 0 {
        eprintln!("PLEASE DO NOT RUN THIS PROGRAM AS ROOT!");
        exit(1);
    }

    // Gets user arguments
    let args: Vec<String> = env::args().collect();
    
    // Video/image argument logic
    let mut fit: bool = false;
    let video: bool = if args.len() > 1 {
        match args[1].as_str() {
            "IMG" => {if args.len() > 2 && args[2] == "FIT" {
                    fit = true;
                }
                else if args.len() > 2 {
=======
static SEEK_VALUE:u32 = 5; // Change these
static SEEK_VALUE_PRECISE:u32 = 1; // Two if you want idc

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
>>>>>>> 2c3fef901187d3b80d65311f6434d0bc740ad81c
                    invalid_arg();
                    unreachable!()
                }
                false
            },
            "VID" => {
                true
            },
<<<<<<< HEAD

            _ => {invalid_arg(); unreachable!()}
        }
    } 
=======
            _ => {invalid_arg(); unreachable!() }
        }
    }
>>>>>>> 2c3fef901187d3b80d65311f6434d0bc740ad81c
    else {
        invalid_arg();
        unreachable!()
    };

    let mut mute: bool = false;
    if video && args.len() > 2 && args[2] == "MUTE" {
        mute = true;
    }
<<<<<<< HEAD
    else if args.len() > 2 {
        invalid_arg();
    }

    // kind of self-explanatory
    let wallpaper_dir = dirs::home_dir().unwrap().join(WALLPAPER_DIR);

=======
    else if video && args.len() > 2{
        invalid_arg();
    }

    let wallpaper_dir = dirs::home_dir().unwrap().join("Pictures/wallpapers");
    
>>>>>>> 2c3fef901187d3b80d65311f6434d0bc740ad81c
    let scan_files = || -> Vec<PathBuf> {
        fs::read_dir(&wallpaper_dir)
            .map(|rd| {
                rd.filter_map(|entry| entry.ok().map(|e| e.path()))
                    .filter(|path| {
                        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("").to_lowercase();
                        if video {
                            matches!(ext.as_str(), "gif" | "mp4" | "mkv" | "webm")
                        } else {
                            matches!(ext.as_str(), "png" | "jpg" | "jpeg")
                        }
                    })
                    .collect()
            })
            .unwrap_or_else(|_| Vec::new())
    };

<<<<<<< HEAD
    // Sort A-Z
    let mut files = scan_files();
    files.sort_by_key(|p| {
        p.file_name().and_then(|n|{
            n.to_str().map(|s| s.to_ascii_lowercase())
        })
    });

    enable_raw_mode()?;

    let mut crt_idx = 0; // Current index

    let _ = helper::draw(&files, crt_idx);
    loop {
        if files.is_empty() {
            println!("No wallpapers found in {:?} \r\n", wallpaper_dir);
            println!("Press R to refresh \r\n");
        }
        else {
            println!("Wallpaper directory at {:?} \r\n", wallpaper_dir);
        }

        //let selected = files[crt_idx].to_str().unwrap();
        let selected = files.get(crt_idx);
=======
    let mut files = scan_files();

    enable_raw_mode()?;
    let mut stdout = stdout();
    let mut crt_idx= 0;

    loop {
        execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0))?;
        println!("Miko's swww & mpvpaper wallpaper changer <3\r\n");

        if files.is_empty(){
        println!("No wallpapers found in {:?} \r\n", wallpaper_dir);
        println!("Press R to refresh\r\n");
        }
        else{
            println!("Wallpaper directory at: {:?}\r\n", wallpaper_dir);
        }

        for(i, path) in files.iter().enumerate(){
            let name = path.file_name().unwrap().to_string_lossy();
            if i == crt_idx {
                println!(" > [{}]\r\n", name);
            }
            else {
                println!("   {}\r\n", name);
            }
        }
>>>>>>> 2c3fef901187d3b80d65311f6434d0bc740ad81c

        if let Event::Key(key_event) = event::read()?{
            match key_event.code{
                KeyCode::Char('s') => {let _ = Command::new("swww-daemon").process_group(0).stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null()).spawn();},
<<<<<<< HEAD
                KeyCode::Char('r') => {
                    files = scan_files(); 
                    files.sort_by_key(|p| {
                        p.file_name().and_then(|n|{
                            n.to_str().map(|s| s.to_ascii_lowercase())
                        })
                    });
                    crt_idx = 0; 
                },
                KeyCode::Char('q') | KeyCode::Esc => break,

                KeyCode::Char('m') => {let _ = helper::send_command(MPV_SOCKET_PATH, vec!["cycle", "mute"]);},
                KeyCode::Char('f') => {let _ = helper::send_command(MPV_SOCKET_PATH, vec!["seek", &SEEK_VALUE.to_string(), "relative"]);},
                KeyCode::Char('b') => {
                    let seek = format!("--{}", SEEK_VALUE);
                    let _ = helper::send_command(MPV_SOCKET_PATH, vec!["seek", &seek, "relative"]);
                },
                KeyCode::Char('F') => {let _ = helper::send_command(MPV_SOCKET_PATH, vec!["seek", &SEEK_VALUE_PRECISE.to_string(), "relative"]);},
                KeyCode::Char('B') => {
                    let seek = format!("--{}", SEEK_VALUE_PRECISE);
                    let _ = helper::send_command(MPV_SOCKET_PATH, vec!["seek", &seek, "relative"]);
                },
                KeyCode::Char('p') => {let _ = helper::send_command(MPV_SOCKET_PATH, vec!["cycle", "pause"]);}
                KeyCode::Char('k') => {let _ = rashf!("pkill mpvpaper");},

                KeyCode::Char('c') => {
                    if key_event.modifiers.contains(KeyModifiers::CONTROL){
                        println!("Keyboard Interrupt received\r\n");
                        break;
                    }
                },

                KeyCode::Up => {
                    if !files.is_empty() && crt_idx > 0 {
                        crt_idx -= 1;
                    }
                },
                KeyCode::Down => {
                    if !files.is_empty() && crt_idx + 1 < files.len() {
                        crt_idx += 1;
                    }
                },

                KeyCode::Enter => {
                    if let Some(path) = selected {
                        let selected = path.to_str().unwrap();
                        
                    if !video && fit {
                        let _ = rashf!("swww img '{}' --resize fit", selected);
                    }
                    else if !video {
=======
                KeyCode::Char('r') => {files = scan_files(); crt_idx = 0;},
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
>>>>>>> 2c3fef901187d3b80d65311f6434d0bc740ad81c
                        let _ = rashf!("swww img '{}'", selected);
                    }
                    else {
                        let _ = rashf!("pkill mpvpaper");
<<<<<<< HEAD

                        let mut child = Command::new("mpvpaper");
                        child.arg("ALL").arg(selected);
                        let mut opts = String::from(format!("--loop-file=inf --input-ipc-server={}", MPV_SOCKET_PATH));
                        if mute {
                            opts.push_str(" --mute=yes");
                        }
                        child.arg("-o").arg(opts);

                        unsafe {
                            child.pre_exec(|| {
                                if libc::setsid() == - 1 {
                                    return Err(std::io::Error::last_os_error());
                                }
                                Ok(())
                            });
                        }

                        child.stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null());

                        let _ = child.spawn();
                    }
                }
            },
                _ => {}
            }
        }
        let _ = helper::draw(&files, crt_idx);
    }

    disable_raw_mode()?;
    println!("have a nice day <3");
=======
                        if !mute{
                            let _ = Command::new("nohup").arg("mpvpaper").arg("ALL").arg("-o").arg("--loop-file=inf input-ipc-server=/tmp/mpv-socket").arg(selected).arg("&").arg("disown").spawn();
                        }
                        else{
                            let _ = Command::new("nohup").arg("mpvpaper").arg("ALL").arg("-o").arg("--loop-file=inf no-audio input-ipc-server=/tmp/mpv-socket").arg(selected).arg("&").arg("disown").spawn();
                        }
                    }
                }
                KeyCode::Char('m') => {let _ = rashf!("echo 'cycle mute' | socat - /tmp/mpv-socket");},
                KeyCode::Char('f') => {let _ = rashf!("echo 'seek {} relative' | socat - /tmp/mpv-socket", SEEK_VALUE);},
                KeyCode::Char('b') => {let _ = rashf!("echo 'seek -{} relative' | socat - /tmp/mpv-socket", SEEK_VALUE);},
                KeyCode::Char('F') => {let _ = rashf!("echo 'seek {} relative' | socat - /tmp/mpv-socket", SEEK_VALUE_PRECISE);},
                KeyCode::Char('B') => {let _ = rashf!("echo 'seek -{} relative' | socat - /tmp/mpv-socket", SEEK_VALUE_PRECISE);},
                KeyCode::Char('p') => {let _ = rashf!("echo 'cycle pause' | socat - /tmp/mpv-socket");}
                KeyCode::Char('k') => {let _ = rashf!("pkill mpvpaper");},
                KeyCode::Char('q') | KeyCode::Esc => break,
                _ => {}
            }
        }
    }
    disable_raw_mode()?;
    println!("goodbye! <3");
>>>>>>> 2c3fef901187d3b80d65311f6434d0bc740ad81c
    Ok(())
}
