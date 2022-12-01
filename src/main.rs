use lnk::{self, ShellLink};
use std::{path::Path};
use whoami::username;

fn main() {
    let shortcut_base = format!("C:\\Users\\{}\\AppData\\Roaming\\Microsoft\\Windows\\Start Menu\\Programs\\Roblox", username());
    let shortcut_name = "Roblox Player.lnk";
    let shortcut = format!("{}\\{}", shortcut_base, shortcut_name);

    let shell = lnk::ShellLink::open( Path::new(&shortcut) ).expect("Couldn't create ShellLink");

    let executable_base = format!("C:\\Users\\{}\\AppData\\", username());
    let executable_path = get_executable_path(&shell);
    let executable = format!("{}{}", executable_base, executable_path);

    let executable_parent = Path::new(&executable).parent().unwrap().to_str().expect("Failed to get the executable's parent");

    let old_sound = format!("{}\\content\\sounds\\ouch.ogg", executable_parent);
    let new_sound = format!("{}{}", 
        std::env::current_dir().unwrap().to_str().expect("Failed to get current directory"), 
        "\\ouch.ogg"
    );

    println!("\nTHIS FILE WILL BE DELETED: {}\nTHIS FILE WILL REPLACE IT: {}", old_sound, new_sound);
    println!("\nAre you sure you want to replace this file? This action cannot be undone.\n[Y/N]");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read the user's input");

    if input.trim().to_uppercase() == String::from("Y") {
        println!("\nReplacing death sound, please wait...");
        std::fs::copy(new_sound, old_sound).expect("Couldn't replace the death sound");
        println!("Successful!")
    } else {
        println!("Aborted!");
    }

    println!("\nPress 'Enter' to continue.");
    std::io::stdin().read_line(&mut input).unwrap();
}

fn get_executable_path(shell: &ShellLink) -> String { // for readability y'know
    lnk::ShellLink::relative_path(&shell).clone().unwrap().replace("..\\", "")
}
