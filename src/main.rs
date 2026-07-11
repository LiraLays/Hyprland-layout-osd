use std::env;
use std::io::{BufRead, BufReader};
use std::os::unix::net::UnixStream;
use std::path::Path;
use std::process::Command;

fn main() -> anyhow::Result<()> {
    // 1. Finding Hypr socket with auto-repeat at start
    let mut stream = None;
    let mut attempts = 0;

    while stream.is_none() && attempts < 50 {
        if let Ok(signature) = env::var("HYPRLAND_INSTANCE_SIGNATURE") {
             // Checking path in XDG_RUNTIME_DIR, if he is empty - check in /tmp
            let xdg_runtime = env::var("XDG_RUNTIME_DIR").unwrap_or_else(|_| "/tmp".to_string());
            let mut socket_path = format!("{}/hypr/{}/.socket2.sock", xdg_runtime, signature);

            if !Path::new(&socket_path).exists() {
                socket_path = format!("/tmp/hypr/{}/.socket2.sock", signature);
            }

            if let Ok(s) = UnixStream::connect(&socket_path) {
                stream = Some(s);
                break;
            }
        }
        // If can't connect, wait 200 ms and try again
        std::thread::sleep(std::time::Duration::from_millis(200));
        attempts += 1;
    }
   
    let stream = stream.ok_or_else(|| anyhow::anyhow!("Couldn't wait for Hyprland socket after 500 attempts"))?;
    let reader = BufReader::new(stream);
    println!("Monitoring Hyprland layout started for all devices");

    // 2. Reading events stream by string
    for line in reader.lines() {
        if let Ok(l) = line {
            // Layout change event looks like: activelayout>>keyboard-name, Language Name
            if l.starts_with("activelayout>>") {
                let content = l.trim_start_matches("activelayout>>");
                if let Some((kbd, layout)) = content.split_once(',') {
                    // React only on changes on main keyboard

                    if kbd.contains("hotkeys") || kbd.contains("swayosd") || kbd.contains("button") {
                        continue;
                    }

                    // Formatting name (English US -> EN, Russian -> Ru)
                    let short_name = layout.chars().take(2).collect::<String>().to_uppercase();

                    // Add flags
                    let flag = if short_name == "EN" { "🇺🇸" } else { "🇷🇺" };
                    let display_text = format!("{} {}", flag, short_name);

                    println!("Hyprland layout for device {} changed to: {}", kbd, display_text);

                    // Calling SwayOSD
                    let _ = Command::new("swayosd-client")
                        .arg("--custom-message")
                        .arg(display_text)
                        .stdin(std::process::Stdio::null())
                        .stdout(std::process::Stdio::null())
                        .stderr(std::process::Stdio::null())
                        .spawn();
                }
            }
        }
    }
    Ok(())
}
