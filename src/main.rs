use include_dir::{include_dir, Dir};
use rustix::{system::uname};
use users::{get_current_uid, get_user_by_uid};
mod flags;
mod colors;
static LOGOS_PATH: Dir = include_dir!("src/logos");
use std::{fs,process};

use crate::colors::Color;
fn main() {
    flags::flags();
    let os_release = match fs::read_to_string("/etc/os-release") {
        Ok(content) => content,
        Err(_) => String::from("Tux"),
        };
    let pretty_name = os_release.lines().find(|line| line.contains("PRETTY_NAME=")).unwrap_or("NULL");
    let distro = pretty_name.trim_start_matches("PRETTY_NAME=").trim_matches('"'); 
    let candidate = LOGOS_PATH.get_file(distro).or_else(|| LOGOS_PATH.get_file("Tux")).and_then(|f| f.contents_utf8()).unwrap();
    let logo = candidate.lines().skip(1).collect::<Vec<&str>>().join("\n");
    let logo_color = candidate.lines().next(); 
    let logo = match logo_color {
        Some("light cyan") => logo.bright_cyan(),
        _ => logo,
    };
    // TODO: Change this to gethostname insted (when it gets implemented, see https://docs.rs/rustix/latest/rustix/not_implemented/quite_yet/fn.gethostname.html) and by there own words "For now, use rustix::process::uname().nodename() instead."
    let uname = uname();
    let hostname = uname.nodename().to_string_lossy();
    let user = get_user_by_uid(get_current_uid()).unwrap();
    let username = user.name().to_string_lossy();
    let kernel_version = uname.release().to_string_lossy();
    let motherboard = match fs::read_to_string("/sys/class/dmi/id/board_name") {
        Ok(content) => content,
        Err(_) => String::from("NULL"),
    };
    let seconds = rustix::system::sysinfo().uptime;
    let minutes = (seconds % 3600) / 60;
    let hours = seconds / 3600;
    println!("{logo}");
    println!("{username}&{hostname}");
    println!("OS: {distro}");
    print!("Motherboard: {motherboard}");
    println!("Kernel: {kernel_version}");
    if hours > 0 {
        println!("Uptime: {hours} Hours, {minutes} Minutes");
    } else {
        println!("Uptime: {minutes} Minutes");
    }
   // println!("{:>50}", "◌".bright_black());
}
