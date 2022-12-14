use lnk::{self, ShellLink};
use std::path::Path;
use std::env as script; // for readability similar to Roblox's Luau
use whoami::username;

fn get_executable_path(shell: ShellLink) -> String {
    let relative = lnk::ShellLink::relative_path(&shell).clone().unwrap().replace("..\\", "");
    let pathstr = format!("C:\\Users\\{}\\AppData\\{}", username(), relative);
    return String::from(pathstr);
} // Used to shorten the relative path provided by the ShellLink to get the (not quite absolute) path of the executable

fn main() {
    let shortcut = format!("C:\\Users\\{}\\AppData\\Roaming\\Microsoft\\Windows\\Start Menu\\Programs\\Roblox\\Roblox Player.lnk", username());
    // The path to the Roblox Player shortcut

    let shell = lnk::ShellLink::open( Path::new(&shortcut) ).expect("Couldn't create ShellLink");

    let executable = get_executable_path(shell);
    let executable_parent = Path::new(&executable).parent()    .unwrap().to_str().expect("Failed to get the executable's parent");
    // Gets the executable and its parent

    let old_sound = format!("{}\\content\\sounds\\ouch.ogg", executable_parent); // Completes the path to the current Roblox death sound
    let new_sound = format!("{}\\ouch.ogg", script::current_dir().expect("Failed to get current directory").to_str().unwrap()); // Completes the path to the new Roblox death sound

    println!("\nTHIS FILE WILL BE DELETED: {}\nTHIS FILE WILL REPLACE IT: {}", old_sound, new_sound);
    println!("\nAre you sure you want to replace this file? This action cannot be undone.\n[Y/N]");
    
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read the user's input"); // Gives the user an input prompt

    if input.trim().to_uppercase() == String::from("Y") {
        println!("\nReplacing death sound, please wait...");
        std::fs::copy(new_sound, old_sound).expect("Couldn't replace the death sound"); // Replaces the current sound
        println!("Successful!")
    } else {
        println!("Aborted!");
    }

    println!("\nPress 'Enter' to continue.");
    std::io::stdin().read_line(&mut input).unwrap();
}