# Battery threshold 
<p aling="center">
Want to stop the battery charging at 80% on your Linux laptop?
Are you using a tiling window manager like i3WM or SwayWM and you don't know how to do it?
You are in the right place!
</p> 

## Requirements
<p align="center">
  <img src="https://img.shields.io/badge/OS-Linux-blue" /> 
  <img src="https://img.shields.io/badge/Rust->=1.80.1-green" />
</p>

## Description
A Rust program to help setting the battery threshold of your PC with Linux installed.

The current program will look into:
    `/sys/class/power_supply/$battery_name/charge_control_start_threshold`
    `/sys/class/power_supply/$battery_name/charge_control_end_threshold`
and finds, firstly, if they exists and, secondly, the current values settings.
That files are filled with a integer value representing the battery percentage at which,
respectively, the battery starts and stops charging.

## Installation
Clone this repo where you prefer (for instance under `~/Documents/dev/rust/`)
```{sh}
mkdir -r ~/Documents/dev/rust/ && cd ~/Documents/dev/rust/
git clone https://github.com/sebastiano123-c/battery_threshold.git
cd battery_threshold/
cargo build
cargo run
cargo install --path .
```
The latter command will install the executable in the $CARGO bin directory, so you can execute where you want.
After shutting down the terminal and creating a new one, test the program by typing
```{sh}
battery_threshold
```

**Important notes**:
 1. Not every laptop can change the battery threshold, especially old PCs charge until they reach 100% capacity. This means that this program may not work on your PC;
 2. Verify that the battery name, __e.g.__ `BAT0`, is under the `/sys/class/power_supply` folder (or directory I never learned what's the name in linux).

## Tests
Tested with the following linux distributions: 
 - [x] Fedora 40 Workstation;
     - [x] SwayWM;
     - [x] i3WM;

## TODO
 - [ ] add the possibility to set sudo privileges with gui and not in terminal;

## License 
MIT
