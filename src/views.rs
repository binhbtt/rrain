use crate::api;

use chrono;
use textplots::{Chart, Plot, ColorPlot, Shape, LabelBuilder, LabelFormat};



pub fn view_minutely(response: &api::OneCallResponse) {

    let iter_minutely = response.minutely.iter();

    for minute in iter_minutely {
        println!("{}: {} mm", convert_unix_to_local(minute.dt), minute.precipitation);
    }
}

pub fn plot_minutely(response: &api::OneCallResponse) {
    let iter_minutely = response.minutely.iter();

    // Create an array of points to plot. Cannot be vector
    let mut points = [(0.0, 0.0); 62];

    // Iterate over the minutely data
    for (i, minute) in iter_minutely.enumerate() {
        // Add the point to the array
        points[i] = (i as f32, minute.precipitation as f32);
    }





    // Plot the data using textplots
    Chart::new(140, 30, 0.0, points.len() as f32)
        .linecolorplot(&Shape::Bars(&points),
            rgb::RGB {
                r: 10, 
                g: 100,
                b: 200,
            },)
        .x_label_format(LabelFormat::Custom(Box::new(|x| format!("{}mins", x as u64))))
        .y_label_format(LabelFormat::Custom(Box::new(|y| format!("{:.1}mm", y))))
        .display();

}

fn convert_unix_to_local(unix: u64) -> String {
    // Create a NaiveDateTime from the timestamp
    let naive = chrono::NaiveDateTime::from_timestamp(unix as i64, 0);
    
    // Create a normal DateTime from the NaiveDateTime
    let datetime: chrono::DateTime<chrono::Utc> = chrono::DateTime::from_utc(naive, chrono::Utc);

    // Format the datetime how you want
    let newdate = datetime.format("%Y-%m-%d %H:%M:%S").to_string();

    // Return the string
    newdate
}