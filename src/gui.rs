use super::battery::Battery;
use glib;
use glib::GString;
use gtk::prelude::*;
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
        let window = gtk::ApplicationWindow::builder()
            .application(app)
            .title("Battery threshold changer")
            .default_width(300)
            .default_height(70)
            .build();

        // Define a row
        let row = gtk::Box::new(gtk::Orientation::Vertical, 5);

        // Define the battery name
        let line0 = gtk::Box::new(gtk::Orientation::Horizontal, 5);
        let battery_name_label = gtk::Label::new(Some("Battery name: "));
        // let battery_name_dropdown_menu = gtk::DropDown::from_strings(&["pippo", "marco"]);
        let battery_name_dropdown_menu =
            gtk::DropDown::from_strings(&self.bat.borrow().get_bat_name_strings());
        line0.append(&battery_name_label);
        line0.append(&battery_name_dropdown_menu);
        row.append(&line0);

        // whenever bat name is choosen in the combobox
        battery_name_dropdown_menu.connect_selected_item_notify(glib::clone!(
            #[weak]
            bat,
            move |dropdown| {
                if let Some(selected_item) = dropdown.selected_item() {
                    // Get the string representation of the selected item
                    let value = selected_item.property_value("string");
                    // Attempt to extract the string from the Value
                    let result: Result<String, String> = value
                        .get::<String>()
                        .map_err(|_| "Failed to get String from Value".to_string());

                    match result {
                        Ok(rust_string) => bat.borrow_mut().change_bat_name(rust_string),
                        Err(e) => println!("Error: {}", e),
                    }
                }
            }
        ));

        // Add switch
        let line = gtk::Box::new(gtk::Orientation::Horizontal, 5);
        let page_break = gtk::Label::new(Some(&"Show battery status"));
        line.append(&page_break);
        let toggle_switch = gtk::Switch::new();
        line.append(&toggle_switch);
        row.append(&line);

        // battery status
        let bat_status = gtk::Label::new(Some(&bat.borrow().get_bat_property("uevent")));
        bat_status.set_visible(false);
        bat_status.set_halign(gtk::Align::Start);
        row.append(&bat_status);

        // Connect the switch's state change signal
        toggle_switch.connect_state_flags_changed(glib::clone!(
            #[weak]
            bat_status,
            move |switch, _| {
                if switch.is_active() {
                    bat_status.set_visible(true); // Show the label
                } else {
                    bat_status.set_visible(false); // Hide the label
                }
            }
        ));

        // "Start at" text
        let line1 = gtk::Box::new(gtk::Orientation::Horizontal, 5);
        let start_label = gtk::Label::new(Some("Start charging at: "));
        let start_at = gtk::Entry::new();
        start_at.set_text(&bat.borrow().bat_start_thrs.to_string());
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
                    Ok(value) => bat.borrow_mut().bat_start_thrs = value,
                    Err(err) => println!("Error: {}", err),
                }
            }
        ));

        // "End at" text
        let line2 = gtk::Box::new(gtk::Orientation::Horizontal, 5);
        let end_label = gtk::Label::new(Some("Stop charging at: "));
        let end_at = gtk::Entry::new();
        end_at.set_text(&bat.borrow().bat_end_thrs.to_string());
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
                    Ok(value) => bat.borrow_mut().bat_end_thrs = value,
                    Err(err) => println!("Error: {}", err),
                }
            }
        ));

        // Set button
        let button = gtk::Button::with_label("Update values");
        button.connect_clicked(glib::clone!(
            #[weak]
            bat,
            move |_| {
                bat.borrow().set_new_bat_threshold();
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
