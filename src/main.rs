use fltk::{app, button::Button, input::Input, prelude::*, window::Window, text::{TextBuffer, TextDisplay}, dialog::alert};use reqwest::{Error, Response};
use serde::Deserialize;
use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};

/*
A project with the intention of tracking flights via flight number,
intention to add a notification once the flight status changes

Using fltk for UI [I am **NOT** a UX designer ;_;]
reqwest for accessing the Aviation Stack API
Aviation Stack API for accessing flight data

Arc is being used to share data between asynchronous tasks
Mutex allows for these functions to allow for read/write between functions :3

Recurring fltk functions:
fltk::app::awake() = Ensure UI is updated
*/

#[derive(Debug, Deserialize)]
struct FlightResponse {
    data: Vec<FlightData>,
}

#[derive(Debug, Deserialize)]
struct FlightData {
    flight: FlightInfo,
    departure: LocationInfo,
    arrival: LocationInfo,
    flight_status: Option<String>,
}

#[derive(Debug, Deserialize)]
struct FlightInfo {
    number: String,
    iata: String,
}

#[derive(Debug, Deserialize)]
struct LocationInfo {
    airport: Option<String>,
}

// Print error
async fn error_message(response: Response, text_buffer: Arc<Mutex<TextBuffer>>) {
    let error_message = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
    
    let mut text_buffer = text_buffer.lock().unwrap();
    text_buffer.set_text(&format!("Failed to fetch flight data: {}", error_message));
    fltk::app::awake();  
}

// Get flight info
async fn flight_info(iata_code: String, text_buffer: Arc<Mutex<TextBuffer>>,previous_status: Arc<Mutex<Option<String>>>) -> Result<bool, Error> {
    let api_key = "#"; // enter API Key
    let api_url = format!(
        "http://api.aviationstack.com/v1/flights?access_key={}&flight_iata={}",
        api_key, iata_code
    );
    let mut landed = false;

    match reqwest::get(&api_url).await {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<FlightResponse>().await {
                    Ok(flight_data) => {
                        let mut result = String::new();
                        if let Some(flight) = flight_data.data.get(0) {
                            result.push_str(&format!("Flight Number: {}\n", flight.flight.number));
                            result.push_str(&format!("Flight IATA: {}\n", flight.flight.iata));
                            result.push_str(&format!("Departure: {}\n", flight.departure.airport.as_deref().unwrap_or("Unknown")));
                            result.push_str(&format!("Arrival: {}\n", flight.arrival.airport.as_deref().unwrap_or("Unknown")));
                            result.push_str(&format!("Status: {}\n", flight.flight_status.as_deref().unwrap_or("Unknown")));

                            // Checking if the status has updated :3
                            let mut previous_status = previous_status.lock().unwrap();
                            if &*previous_status != &flight.flight_status {
                                if let Some(status) = &flight.flight_status {
                                    alert(150, 100, &format!("Flight status changed to: {}", status));
                                }
                                *previous_status = flight.flight_status.clone();
                                landed = true;
                            }
                        }

                        let mut text_buffer = text_buffer.lock().unwrap();
                        text_buffer.set_text(&result);
                        fltk::app::awake();  // Refresh the UI
                    }
                    Err(err) => {
                        let mut text_buffer = text_buffer.lock().unwrap();
                        text_buffer.set_text(&format!("Failed to parse flight data: {}", err));
                        fltk::app::awake();  
                    }
                }
            } else {
                error_message(response, text_buffer).await;  
            }
        }
        Err(err) => {
            let mut text_buffer = text_buffer.lock().unwrap();
            text_buffer.set_text(&format!("Error: {}", err));
            fltk::app::awake();  
        }
    }

    Ok(landed)
}

// Update flight info periodically
async fn fetch_and_update_flight_info(iata_code: String, text_buffer: Arc<Mutex<TextBuffer>>, interval: Duration, previous_status: Arc<Mutex<Option<String>>>) {
    // First-time fetch
    flight_info(iata_code.clone(), text_buffer.clone(), previous_status.clone()).await.unwrap();

    // Periodic update loop
    loop {
        sleep(interval).await;  // Wait for the defined interval
        let landed = flight_info(iata_code.clone(), text_buffer.clone(), previous_status.clone()).await.unwrap();  // Fetch flight info again
        if landed == true {break;} // If the status changes, break out of the loop; effectivelta do {} while loop
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Initialize the FLTK application
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 600, 400, "Flight Info");
    let t = 5; // Time [in minutes] per refresh

    // Input field for the flight number [IATA]
    let mut iata_input = Input::new(100, 50, 200, 40, "Flight IATA:");
    let mut fetch_button = Button::new(320, 50, 100, 40, "Enter");

    // Text display for showing flight information
    let mut text_display = TextDisplay::new(50, 150, 500, 200, "");
    let text_buffer = Arc::new(Mutex::new(TextBuffer::default()));
    text_display.set_buffer(Some(text_buffer.lock().unwrap().clone()));
    let previous_status = Arc::new(Mutex::new(None));

    wind.end();
    wind.show();

    // Button callback to fetch and refresh flight info >_<
    let text_buffer_clone = text_buffer.clone();
    let previous_status_clone = previous_status.clone(); 
    fetch_button.set_callback(move |_| {
        let iata_code = iata_input.value();
        if !iata_code.is_empty() {
            let text_buffer = text_buffer_clone.clone();
            let iata_code_clone = iata_code.clone();
            let previous_status = previous_status_clone.clone();  

            // Spawn the periodic update task
            tokio::spawn(async move {
                let interval = Duration::from_secs(t * 60); // Set refresh interval 
                fetch_and_update_flight_info(iata_code_clone, text_buffer, interval, previous_status).await;
            });
        } else {
            let mut text_buffer = text_buffer_clone.lock().unwrap();
            text_buffer.set_text("Please enter a valid IATA code");
            fltk::app::awake();  
        }
    });

    // Run the FLTK event loop
    app.run().unwrap();

    Ok(())
}