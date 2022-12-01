# Replace Roblox Death Sound
Rewritten in Rust!

## How it works:
1. Checks your Start Menu folder for the Roblox Player shortcut
2. Uses that shortcut to get to the main Roblox Launcher directory
3. Finds the 'ouch.ogg' (death sound) file and replaces it with the file in the script's directory

# DISCLAIMER:
### Only supports Windows 10 or 11, but you can probably configure it to work for your OS. Linux may require a whole rewrite in order to work. (Due to differences in the filesystem and whatnot)

## Notes:
- Requires Rust/Cargo to compile.
- Be careful with pre-compiled junk!!!!!!
- Make sure the formatting is correct! The new sound must be called 'ouch' with the file extension of 'ogg'. 
- Make sure the new sound is also in the same working directory as the executable file.
