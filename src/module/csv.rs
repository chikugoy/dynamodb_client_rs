use csv;
use std::error::Error;
use std::fs::OpenOptions;

pub fn write_to_csv(process_type: &str, item_count: usize, execution_time: u128) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new().write(true).append(true).create(true).open("csv/results.csv")?;
    let mut wtr = csv::Writer::from_writer(file);

    wtr.write_record(&[process_type, &item_count.to_string(), &execution_time.to_string()])?;
    wtr.flush()?;
    Ok(())
}

pub fn write_to_csv_header(process_type: &str, item_count: &str, execution_time: &str) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new().write(true).truncate(true).create(true).open("csv/results.csv")?;
    let mut wtr = csv::Writer::from_writer(file);

    wtr.write_record(&[process_type, &item_count, &execution_time])?;
    wtr.flush()?;
    Ok(())
}
