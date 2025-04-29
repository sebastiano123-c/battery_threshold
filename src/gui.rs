use super::battery::Battery;
use gtk::glib::ffi::GString;
use gtk::glib::GString;
use gtk::{glib, Application, ApplicationWindow, Box, Button, Entry, Label};
use gtk::{prelude::*, StringObject};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Gui {
    bat: Rc<RefCell<Battery>>,
}

impl Gui {
    pub fn new() -> Self {
        let battery = Battery::new();
        Self {
            bat: Rc::new(RefCell::new(battery)),
        }
    }

    pub fn build_ui(&self, app: &gtk::Application) {
        // clone battery
        let bat = self.bat.clone();

        // get charging threshold
        bat.borrow_mut().get_bat_start_threshold();
        bat.borrow_mut().get_bat_end_threshold();

        // create window
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
        let battery_name_label = gtk::Label::new(Some("Battery name: "));
        let battery_name_dropdown_menu = gtk::DropDown::from_strings(&["pippo", "marco"]);
        // gtk::DropDown::from_strings(&self.bat.borrow().get_bat_name_strings());
        line0.append(&battery_name_label);
        line0.append(&battery_name_dropdown_menu);
        row.append(&line0);
        println!(
            "{:?}",
            battery_name_dropdown_menu
                .selected_item()
                .unwrap()
                .property_value("string")
                .to_value()
        );

        // whenever bat name is choosen in the combobox
        battery_name_dropdown_menu.connect_selected_item_notify(glib::clone!(
            // #[weak]
            // bat,
            move |dropdown| {
                if let Some(selected_item) = dropdown.selected_item() {
                    // Get the string representation of the selected item
                    println!("{:?}", selected_item.property_value("string"));
                }
            }
        ));

        // "Start at" text
        let line1 = Box::new(gtk::Orientation::Horizontal, 5);
        let start_label = Label::new(Some("Start charging at: "));
        let start_at = Entry::new();
        start_at.set_text(&bat.borrow().bat_start_thrs);
        line1.append(&start_label);
        line1.append(&start_at);
        row.append(&line1);

        // whenever start at changes
        start_at.connect_changed(glib::clone!(
            #[strong]
            bat,
            move |b| {
                let gstring = GString::from(b.text()); // Example GString
                match Self::gstring_to_u8(gstring) {
                    Ok(value) => bat.borrow_mut().bat_start_thrs = value.to_string(),
                    Err(err) => println!("Error: {}", err),
                }
            }
        ));

        // "End at" text
        let line2 = Box::new(gtk::Orientation::Horizontal, 5);
        let end_label = Label::new(Some("Stop charging at: "));
        let end_at = Entry::new();
        end_at.set_text(&bat.borrow().bat_end_thrs);
        line2.append(&end_label);
        line2.append(&end_at);
        row.append(&line2);

        // whenever end at changes
        end_at.connect_changed(glib::clone!(
            #[weak]
            bat,
            move |b| {
                let gstring = GString::from(b.text()); // Example GString
                match Self::gstring_to_u8(gstring) {
                    Ok(value) => bat.borrow_mut().bat_end_thrs = value.to_string(),
                    Err(err) => println!("Error: {}", err),
                }
            }
        ));

        // Set button
        let button = Button::with_label("Update values");
        button.connect_clicked(glib::clone!(
            #[weak]
            bat,
            move |_| {
                // Get the battery name
                bat.borrow().set_bat_start_threshold();
                bat.borrow().set_bat_end_threshold();
            }
        ));
        row.append(&button);

        // Add widgets
        window.set_child(Some(&row));

        window.present();
    }

    fn gstring_to_u8(gstring: GString) -> Result<u8, String> {
        // Convert GString to &str
        let str_value = gstring.as_str();

        // Parse the string to u8
        match str_value.parse::<u8>() {
            Ok(value) => Ok(value),
            Err(_) => Err(format!("Failed to convert '{}' to u8", str_value)),
        }
    }
}
