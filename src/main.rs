use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Box, CssProvider, Label, Orientation, ToggleButton,
};
use std::cell::RefCell;
use std::rc::Rc;

// D-Bus (zbus) and async runtime
use tokio::runtime::Runtime;
use zbus::zvariant::{OwnedValue, Value};
use zbus::Proxy;

fn toggle_bluetooth(powered: bool) {
    let rt = Runtime::new().expect("Failed to create Tokio runtime");

    rt.block_on(async move {
        match zbus::Connection::system().await {
            Ok(conn) => {
                let proxy = Proxy::new(
                    &conn,
                    "org.bluez",
                    "/org/bluez/hci0",
                    "org.freedesktop.DBus.Properties",
                )
                .await
                .unwrap();

                let value = OwnedValue::from(powered);

                let result = proxy
                    .call_method(
                        "Set",
                        &("org.bluez.Adapter1", "Powered", Value::from(value)),
                    )
                    .await;

                match result {
                    Ok(_) => {
                        println!(
                            "Bluetooth {}",
                            if powered { "enabled" } else { "disabled" }
                        );
                    }
                    Err(e) => eprintln!("D-Bus error: {}", e),
                }
            }
            Err(e) => eprintln!("Failed to connect to D-Bus: {}", e),
        }
    });
}

fn main() {
    let app = Application::builder()
        .application_id("org.abhi.bluetooth")
        .build();

    app.connect_activate(|app| {
        // --- CSS Theme ---
        let css = "
            window {
                background-color: #1e1e2e;
            }
            label {
                color: #cdd6f4;
                font-size: 20px;
            }
            button {
                background: #89b4fa;
                color: #1e1e2e;
                font-weight: bold;
                border-radius: 10px;
                padding: 10px;
            }
        ";

        let provider = CssProvider::new();
        provider.load_from_data(css);

        gtk::style_context_add_provider_for_display(
            &gtk::gdk::Display::default().unwrap(),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        // --- UI Elements ---
        let label = Label::new(Some("Bluetooth Controller"));
        let toggle = ToggleButton::with_label("Turn ON Bluetooth");
        toggle.set_active(false);

        let toggle_ref = Rc::new(RefCell::new(false));

        toggle.connect_toggled({
            let toggle_ref = toggle_ref.clone();
            move |btn| {
                let is_on = btn.is_active();
                btn.set_label(if is_on {
                    "Turn OFF Bluetooth"
                } else {
                    "Turn ON Bluetooth"
                });

                *toggle_ref.borrow_mut() = is_on;
                toggle_bluetooth(is_on);
            }
        });

        let vbox = Box::new(Orientation::Vertical, 20);
        vbox.set_margin_top(30);
        vbox.set_margin_bottom(30);
        vbox.set_margin_start(30);
        vbox.set_margin_end(30);
        vbox.append(&label);
        vbox.append(&toggle);

        let window = ApplicationWindow::builder()
            .application(app)
            .title("Rust Bluetooth Toggle")
            .default_width(400)
            .default_height(200)
            .child(&vbox)
            .build();

        window.show();
    });

    app.run();
}
