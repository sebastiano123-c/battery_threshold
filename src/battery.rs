use gtk::prelude::*;
use std::process::Command;
use std::string::String;

#[derive(Debug)]
pub struct Battery {
    pub bat_list: Vec<String>,
    pub bat: String,
    pub bat_start_thrs: String,
    pub bat_end_thrs: String,
    pub bat_status: String,
    pub bat_capacity: String,
}

impl Battery {
    pub fn new() -> Self {
        // create battery list
        let mut battery_list: Vec<String> = Vec::new();
        Self::get_bat_list(&mut battery_list);

        // set default battery
        let mut bat_name: String = String::new();
        for bn in battery_list.iter() {
            bat_name = bn.to_string();
            break; // we only need the first name if any
        }

        Self {
            bat_list: battery_list,
            bat: bat_name,
            bat_start_thrs: "0".to_string(),
            bat_end_thrs: "0".to_string(),
            bat_status: "".to_string(),
            bat_capacity: "".to_string(),
        }
    }

    pub fn get_bat_name_strings(&self) -> Vec<&str> {
        self.bat_list.iter().map(|s| s.as_str()).collect()
    }

    pub fn set_bat_property(&self, property_name: &String, new_value: &String) {
        // -----
        // Set new property
        // -----
        let cmd = format!(
            "echo {} | sudo tee /sys/class/power_supply/{}/{}",
            &new_value, &self.bat, &property_name,
        );

        // Print command
        println!("{}", cmd);

        // Execute command
        let output1 = Command::new("bash")
            .args(&["-c", &cmd])
            .output()
            .expect("Failed to get random");
        let content = String::from_utf8(output1.stdout).unwrap();

        println!("Stdout: {}", content);
    }

    pub fn set_bat_start_threshold(&self) {
        // -----
        // Set start charging threshold
        // -----
        let cmd = format!(
            "echo {} | sudo tee /sys/class/power_supply/{}/charge_control_start_threshold",
            &self.bat_start_thrs, &self.bat,
        );

        // Print command
        println!("{}", cmd);

        // Execute command
        let output1 = Command::new("bash")
            .args(&["-c", &cmd])
            .output()
            .expect("Failed to get random");
        let content = String::from_utf8(output1.stdout).unwrap();

        println!("Stdout: {}", content);
    }

    pub fn set_bat_end_threshold(&self) {
        // -----
        // Set end charging threshold
        // -----
        let cmd = format!(
            "echo {} | sudo tee /sys/class/power_supply/{}/charge_control_end_threshold",
            &self.bat_end_thrs, &self.bat,
        );
        println!("{}", cmd);

        let output2 = Command::new("bash")
            .args(&["-c", &cmd])
            .output()
            .expect("Failed to get random");
        let content = String::from_utf8(output2.stdout).unwrap();

        println!("Stdout: {}", content);
    }

    pub fn change_bat_name(&mut self, name: String) {
        self.bat = name;
    }

    pub fn get_bat_property(&self, property_name: &str) -> String {
        // cmd get battery status
        let property_file = format!("/sys/class/power_supply/{}/{}", &self.bat, &property_name);
        let output = Command::new("cat")
            .arg(property_file)
            .output()
            .expect("Failed to execute command");

        let bat_property = std::str::from_utf8(&output.stdout)
            .expect("Invalid UTF-8 output")
            .to_string();
        println!("Battery {}: {}", property_name, bat_property);
        bat_property
    }

    pub fn get_bat_capacity(&mut self) {
        // cmd get battery status
        let status_file = format!("/sys/class/power_supply/{}/capacity", &self.bat);
        let output = Command::new("cat")
            .arg(status_file)
            .output()
            .expect("Failed to execute command");

        self.bat_capacity = std::str::from_utf8(&output.stdout)
            .expect("Invalid UTF-8 output")
            .to_string();
        println!("Battery status: {}", self.bat_capacity);
    }

    pub fn get_bat_status(&mut self) {
        // cmd get battery status
        let status_file = format!("/sys/class/power_supply/{}/status", &self.bat);
        let output = Command::new("cat")
            .arg(status_file)
            .output()
            .expect("Failed to execute command");

        self.bat_status = std::str::from_utf8(&output.stdout)
            .expect("Invalid UTF-8 output")
            .to_string();
        println!("Battery status: {}", self.bat_status);
    }

    pub fn get_bat_end_threshold(&mut self) {
        // Define the file names strings
        let end_file_name = format!(
            "/sys/class/power_supply/{}/charge_control_end_threshold",
            &self.bat
        );

        // Lets firstly find current end charging value
        let mut end_value_int: String =
            std::fs::read_to_string(end_file_name).expect("\nfile {end_file} wasn't found.\n");

        // Remove the '\n' char at the end
        end_value_int.pop().unwrap().to_string();

        println!("Actual charging end threshold: {}", end_value_int);

        self.bat_end_thrs = end_value_int;
    }

    pub fn get_bat_start_threshold(&mut self) {
        // Define the file names strings
        let start_file_name = format!(
            "/sys/class/power_supply/{}/charge_control_start_threshold",
            &self.bat
        );

        // Lets firstly find current start charging value
        let mut start_value_int: String =
            std::fs::read_to_string(start_file_name).expect("\nfile {start_file} wasn't found.\n");

        // Remove the '\n' char at the end
        start_value_int.pop().unwrap().to_string();
        println!("Actual charging start threshold: {}", start_value_int);

        self.bat_start_thrs = start_value_int;
    }

    fn get_bat_list(bat_list: &mut Vec<String>) {
        // For executing: ls /sys/class/power_supply/ | grep BAT
        // Execute the `ls /sys/class/power_supply/` command
        let output = Command::new("ls")
            .arg("/sys/class/power_supply/")
            .output()
            .expect("Failed to execute command");

        // Check if the command was successful
        if output.status.success() {
            // Convert the output to a string
            let output_str = std::str::from_utf8(&output.stdout).expect("Invalid UTF-8 output");

            // Clear the existing contents of the bat_list
            bat_list.clear();

            // Filter the output for lines containing "BAT" and extend the bat_list
            bat_list.extend(
                output_str
                    .lines()
                    .filter(|line| line.contains("BAT"))
                    .map(String::from), // Convert &str to String
            );

            // check if something is found
            if bat_list.iter().len() == 0 {
                println!("Error! No BAT found!");
                // TODO: close program
            }
        } else {
            // Handle the error case
            eprintln!(
                "Command executed with error: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }
}
