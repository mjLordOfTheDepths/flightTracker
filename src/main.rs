use fltk::{app, button::Button, frame::Frame, input::Input, prelude::*, window::Window, text::{TextBuffer, TextDisplay}};
use reqwest::Error;
use serde::Deserialize;

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
    date: Option<String>,
}

#[derive(Debug, Deserialize)]
struct FlightInfo {
    number: String,
    iata: String,
    icao: String,
}

#[derive(Debug, Deserialize)]
struct LocationInfo {
    airport: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Initialize the FLTK application
    let app = app::App::default();

    // Create a window
    let mut wind = Window::new(100, 100, 600, 400, "Flight Info");

    // Create an input for the flight number 
    let mut iata_input = Input::new(100, 50, 200, 40, "Flight IATA:");

    let mut fetch_button = Button::new(320, 50, 100, 40, "Enter");

    // Create a text display for showing flight information
    let mut text_display = TextDisplay::new(50, 150, 500, 200, "");
    let mut text_buffer = TextBuffer::default();
    text_display.set_buffer(Some(text_buffer.clone()));

    wind.end();
    wind.show();

    // Button callback to fetch flight data
    fetch_button.set_callback(move |_| {
        let iata_code = iata_input.value();
        if !iata_code.is_empty() {
            let mut text_buffer = text_buffer.clone();
            tokio::spawn(async move {
                let api_key = "#"; // enter your Aviation Stack API Key
                let api_url = format!(
                    "http://api.aviationstack.com/v1/flights?access_key={}&flight_iata={}",
                    api_key, iata_code
                );

                // Make the request to the Aviationstack API
                match reqwest::get(&api_url).await {
                    Ok(response) => {
                        if response.status().is_success() {
                            match response.json::<FlightResponse>().await {
                                Ok(flight_data) => {
                                    let mut result = String::new();
                                    if let Some(flight) = flight_data.data.get(0) {
                                        result.push_str(&format!("Flight 1:\n"));
                                        result.push_str(&format!("Flight Number: {}\n", flight.flight.number));
                                        result.push_str(&format!("Flight IATA: {}\n", flight.flight.iata));
                                        result.push_str(&format!("Flight ICAO: {}\n", flight.flight.icao));
                                        result.push_str(&format!("Departure Airport: {}\n", flight.departure.airport.as_deref().unwrap_or("Unknown")));
                                        result.push_str(&format!("Arrival Airport: {}\n", flight.arrival.airport.as_deref().unwrap_or("Unknown")));
                                        // result.push_str(&format!("Date: {:?}\n", flight.date)); // Couldn't get Date to print properly in output
                                        result.push_str(&format!("Status: {}\n\n", flight.flight_status.as_deref().unwrap_or("Unknown")));
                                    }
                                    text_buffer.set_text(&result);
                                }
                                Err(err) => {
                                    text_buffer.set_text(&format!("Failed to parse flight data: {}", err));
                                }
                            }
                        } else {
                            text_buffer.set_text("Failed to fetch flight data");
                        }
                    }
                    Err(err) => {
                        text_buffer.set_text(&format!("Error: {}", err));
                    }
                }
            });
        } else {
            text_buffer.set_text("Please enter a valid IATA code");
        }
    });

    // Run the FLTK event loop
    app.run().unwrap();

    Ok(())
}
