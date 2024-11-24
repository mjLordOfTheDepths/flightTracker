# flightTracker
## Description
This is a piece of flight tracking software created in the Rust programming language.
I came up with the idea whilst working in taxi dispatch, as the dispatch software we were using would allow users to attach flight numbers to bookings, but it would not give the user any alerts or updates in relation to the flight.
This led to one or two issues wherein flights that landed early would not be noticed, and no taxi would be dispatched until either a) the customer rang in, or b) the scheduled arrival time.
This piece of software hopes to solve this problem by issuing regular updates regarding flight status.
## How to use
### In order to use this software, you must have Rust installed on your system and a valid Aviation Stack API Key.
1. Clone the repo 
    ```bash
   git clone https://github.com/mjLordOfTheDepths/flightTracker.git
   cd flightTracker
3. Go to src, main.rs, and add your api key to the api_key variable. Additionally, alter the variable "t" to suit your preferences. By default, the value is set to 5 [minutes].
4. Compile the software 
      ```
    cargo build
    cargo run 
6. Type the flight ICAO number into the provided text box.
7. Press "fetch".
8. Have fun. [Mandatory]
## Technologies used
### Main Technologies.
- Rust
- Aviation Stack API
### Rust Crates
- reqwest
- serde
- tokio
## Example I/O
![flightTracker](https://github.com/user-attachments/assets/eca0ce25-830a-41ed-b32e-57a7a9b97984)
