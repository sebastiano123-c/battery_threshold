// *****************************************************************
//
//  File name         : main.rs
//  Description       : The current program will look into:
//                         /sys/class/power_supply/$battery_name/charge_control_start_threshold
//                         /sys/class/power_supply/$battery_name/charge_control_end_threshold
//                       and finds, firstly, if they exists and, secondly, the current values settings.
//                       That files are filled with a integer value representing the battery percentage at which,
//                       respectively, the battery starts and stops charging.
//
//  Revision History  :
//  Date        Author      Comments
//  -----------------------------------------------------------------
//  2024/08/29  Seb         All setup
//
// *****************************************************************

// use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Box, Button, Entry, Label};

// For file reader
use std::fs;
use std::process::Command;

fn main() -> glib::ExitCode {
    // Define the battery name.
    // If not sure what it is, look into folder '/sys/class/power_supply/'
    let battery_name: &str = "BAT0";

    // Define the file names strings
    let start_file_name = format!(
        "/sys/class/power_supply/{}/charge_control_start_threshold",
        &battery_name
    );

    // Define the file names strings
    let end_file_name = format!(
        "/sys/class/power_supply/{}/charge_control_end_threshold",
        &battery_name
    );

    // Lets firstly find current start charging value
    let mut start_value_int: String =
        fs::read_to_string(start_file_name).expect("\nfile {start_file} wasn't found.\n");

    // Lets firstly find current stop charging value
    let mut end_value_int: String =
        fs::read_to_string(end_file_name).expect("\nfile {end_file} wasn't found.\n");

    // Remove the '\n' char at the end
    start_value_int.pop().unwrap().to_string();
    end_value_int.pop().unwrap().to_string();

    // Print values
    println!(
        "\n ACTUAL {} CONFIG:\n Battery start charging at: {} Battery stop charging at: {}",
        &battery_name, &start_value_int, &end_value_int
    );

    // Application GTK
    let application = Application::builder()
        .application_id("com.example.battery_threshold")
        .build();

    // Activate GTK app
    application.connect_activate(move |app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Battery threshold changer")
            .default_width(300)
            .default_height(70)
            .build();

        // Define a row
        let row = Box::new(gtk::Orientation::Vertical, 5);

        // Define the battery name
        let line0 = Box::new(gtk::Orientation::Horizontal, 5);
        let battery_name_label = Label::new(Some("Battery name: "));
        let battery_name_entry = Entry::new();
        battery_name_entry.set_text(&battery_name);
        line0.append(&battery_name_label);
        line0.append(&battery_name_entry);
        row.append(&line0);

        // "Start at" text
        let line1 = Box::new(gtk::Orientation::Horizontal, 5);
        let start_label = Label::new(Some("Start charging at: "));
        let start_at = Entry::new();
        line1.append(&start_label);
        line1.append(&start_at);
        row.append(&line1);

        // "End at" text
        let line2 = Box::new(gtk::Orientation::Horizontal, 5);
        let end_label = Label::new(Some("Stop charging at: "));
        let end_at = Entry::new();
        line2.append(&end_label);
        line2.append(&end_at);
        row.append(&line2);

        // Suggestion
        let line3 = Box::new(gtk::Orientation::Horizontal, 5);
        let suggestion_label = Label::new(Some("If not sure about the battery name (e.g. BAT0), look into folder '/sys/class/power_supply/'"));
        line3.append(&suggestion_label);
        row.append(&line3);

        // Add text
        start_at.set_text(&start_value_int);
        end_at.set_text(&end_value_int);

        // Set button
        let button = Button::with_label("Update values");
        button.connect_clicked(move |_| {
            // Get the battery name
            let bat_name = battery_name_entry.text();

            // -----
            // Set start charging threshold
            // -----
            let mut cmd = format!(
                "echo {} | sudo tee /sys/class/power_supply/{}/charge_control_start_threshold",
                start_at.text(),
                &bat_name
            );

            // Print command
            println!("{}", cmd);

            // Execute command
            let output1 = Command::new("bash")
                .args(&["-c", &cmd])
                .output()
                .expect("Failed to get random");
            let mut content = String::from_utf8(output1.stdout).unwrap();

            println!("Stdout: {}", content);

            // -----
            // Set end charging threshold
            // -----
            cmd = format!(
                "echo {} | sudo tee /sys/class/power_supply/{}/charge_control_end_threshold",
                end_at.text(),
                &bat_name
            );
            println!("{}", cmd);

            let output2 = Command::new("bash")
                .args(&["-c", &cmd])
                .output()
                .expect("Failed to get random");
            content = String::from_utf8(output2.stdout).unwrap();

            println!("Stdout: {}", content);
        });
        row.append(&button);

        // Add widgets
        window.set_child(Some(&row));

        // window.present();
        window.show();
    });

    application.run()
}
