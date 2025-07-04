// Bring in all necessary GTK trait extensions (like `.set_label()`, `.connect_toggled()` etc.)
use gtk::prelude::*;

// Import core GTK widgets and components
use gtk::{Application, ApplicationWindow, Box, CssProvider, Label, Orientation, ToggleButton};

// For shared mutable state inside closures
use std::cell::RefCell;
use std::rc::Rc;

// Async runtime and D-Bus types
use tokio::runtime::Runtime;
use zbus::zvariant::{OwnedValue, Value}; // Types needed to pass data over D-Bus
use zbus::Proxy; // Used to call D-Bus methods via a proxy

// Function to toggle Bluetooth on/off via D-Bus
fn toggle_bluetooth(powered: bool) {
    // Create a blocking Tokio runtime to run async D-Bus code inside a sync function
    let rt = Runtime::new().expect("Failed to create Tokio runtime");

    // Use the runtime to execute an async block
    rt.block_on(async move {
        // Connect to the system D-Bus (not session/user bus)
        match zbus::Connection::system().await {
            Ok(conn) => {
                // Create a proxy object targeting the Bluetooth adapter interface
                let proxy = Proxy::new(
                    &conn,
                    "org.bluez", // D-Bus service name
                    "/org/bluez/hci0", // Path to the Bluetooth adapter (first adapter)
                    "org.freedesktop.DBus.Properties", // Interface used to set/get properties
                )
                .await
                .unwrap(); // Panic if proxy creation fails

                // Convert the boolean into a D-Bus-compatible value
                let value = OwnedValue::from(powered);

                // Call the `Set` method to change the `Powered` property on the adapter
                let result = proxy
                    .call_method(
                        "Set",
                        &("org.bluez.Adapter1", "Powered", Value::from(value)),
                    )
                    .await;

                // Handle response from D-Bus
                match result {
                    Ok(_) => {
                        println!(
                            "Bluetooth {}",
                            if powered { "enabled" } else { "disabled" }
                        );
                    }
                    Err(e) => eprintln!("D-Bus error: {}", e), // Print error if D-Bus call fails
                }
            }
            Err(e) => eprintln!("Failed to connect to D-Bus: {}", e), // Connection failed
        }
    });
}

fn main() {
    // Initialize GTK application with a unique application ID
    let app = Application::builder()
        .application_id("org.abhi.bluetooth") // Must be unique, like a domain
        .build();

    // Set up UI components when the app is launched
    app.connect_activate(|app| {
        // === CSS Styling ===
        let css = "
            window {
                background-color: #1e1e2e; // Dark background
            }
            label {
                color: #cdd6f4;            // Light blue text
                font-size: 20px;
            }
            button {
                background: #89b4fa;       // Soft blue
                color: #1e1e2e;            // Dark text
                font-weight: bold;
                border-radius: 10px;
                padding: 10px;
            }
        ";

        // Create and load CSS into GTK
        let provider = CssProvider::new();
        provider.load_from_data(css).expect("Failed to load CSS");

        // Apply the CSS globally to the application
        gtk::style_context_add_provider_for_display(
            &gtk::gdk::Display::default().unwrap(),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        // === UI Setup ===

        // Create a label widget for the window
        let label = Label::new(Some("Bluetooth Controller"));

        // Create a toggle button with default label
        let toggle = ToggleButton::with_label("Turn ON Bluetooth");
        toggle.set_active(false); // Initially OFF

        // Shared state using Rc<RefCell<T>> for closure access
        let toggle_ref = Rc::new(RefCell::new(false));

        // Define behavior when button is toggled
        toggle.connect_toggled({
            let toggle_ref = toggle_ref.clone();
            move |btn| {
                let is_on = btn.is_active(); // Check current toggle state

                // Change button label based on new state
                btn.set_label(if is_on { "Turn OFF Bluetooth" } else { "Turn ON Bluetooth" });

                // Update shared state
                *toggle_ref.borrow_mut() = is_on;

                // Call our function to control Bluetooth
                toggle_bluetooth(is_on);
            }
        });

        // Create a vertical box layout container with spacing
        let vbox = Box::new(Orientation::Vertical, 20);
        vbox.set_margin_top(30);
        vbox.set_margin_bottom(30);
        vbox.set_margin_start(30);
        vbox.set_margin_end(30);

        // Add label and button to layout
        vbox.append(&label);
        vbox.append(&toggle);

        // Create the main window with properties and layout
        let window = ApplicationWindow::builder()
            .application(app)              // Bind to this GTK app
            .title("Rust Bluetooth Toggle") // Window title
            .default_width(400)
            .default_height(200)
            .child(&vbox)                  // Add our layout to the window
            .build();

        // Show the window
        window.show();
    });

    // Run the GTK event loop
    app.run();
}
