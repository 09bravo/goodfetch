use include_dir::{include_dir, Dir};
static LOGOS_PATH: Dir = include_dir!("src/logos");
use std::fs;
fn main() {
    let os_release = fs::read_to_string("/etc/os-release")
        .expect("/etc/os-release Was not found (in other words goodfetch can't know what distro you are using)");
    let pretty_name = os_release.lines().nth(1).unwrap_or("NULL");
    let distro = pretty_name.trim_start_matches("PRETTY_NAME=").trim_matches('"'); 
    let candidate = LOGOS_PATH.get_file(format!("{}.txt", distro)).or_else(|| LOGOS_PATH.get_file("tux.txt")).unwrap();
    let logo = candidate.contents_utf8().unwrap();
    println!("{logo}");
}
