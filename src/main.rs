mod api;
mod views;

use std::io;

fn main() {
    println!("Weather App");

    // Get user input
    println!("Enter the city:");
    let mut city = String::new();
    io::stdin().read_line(&mut city)
        .expect("Failed to read input");

    let output = api::get_response(&city.trim());


    // Process the response
    match output {
        Ok(response) => {
            println!("API response received");
            println!("Rain in {}", city);
            views::plot_minutely(&response);
            views::view_minutely(&response);
        },
        Err(error) => println!("Error: {}", error),
    }


    println!("API response processed");
}