use chrono::prelude::*;
fn main() {
    let now_time = Local::now();
    println!("Time now is: {}", now_time);
    let start_point = Local.with_ymd_and_hms(2024, 10, 10, 12, 27, 0).unwrap();

    let diff = now_time.signed_duration_since(start_point);

    let days = diff.num_days();
    let hours = diff.num_hours();
    let minutes = diff.num_minutes();

    println!("Start time: {}", start_point);
    println!("Now: {}", now_time);
    println!("Days: {days}");
    println!("Hours: {hours}, Total hours: {}", hours % 24);
    println!("Minutes: {}, Total minutes: {}", minutes % 60, minutes);
}
