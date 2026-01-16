pub fn get_day_file_name(days_before: i64) -> String {
    (chrono::offset::Local::now() - chrono::Duration::days(days_before))
        .format("%Y-%m-%d.txt")
        .to_string()
}