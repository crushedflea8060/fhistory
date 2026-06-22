use std::path::PathBuf;
use std::fs::File;
use std::time::UNIX_EPOCH;
fn main() {
    let current_date: DateTime = unix_to_datetime(File::create("temporary-foo-fhistory.txt").expect("Error creating file")
        .metadata().expect("Metadata test reading failed")
        .modified().expect("Failed to read date from metadata on temp file")
        .duration_since(UNIX_EPOCH).expect("Time went backwards")
        .as_secs() as i64);

    let root: PathBuf = PathBuf::from("/");
    let _ = std::fs::remove_file("temporary-foo-fhistory.txt");
    match check_edits(&root, &current_date){
        Ok(()) => println!("Finished"),
        Err(_e) => println!("Error finishing - some files may be missing - try running as root"), //ignore errors and read all files
    }
}

fn check_edits(path: &PathBuf, date: &DateTime) -> std::io::Result<()>
{
    if compare_strings(&path.to_string_lossy(),&["/sys".to_string(),"/proc".to_string(),"/run".to_string(), "/dev".to_string(), "/var/run".to_string(), "/mnt".to_string()]) 
    //convert to String, idk an easier implementation, make an issue if you know one. // this line also removes directories that have a tradeoff of being too big for little gain.
    {
       return Ok(());
    }
    let file = File::open(&path)?;
    let attributes = file.metadata()?;
    let last_modified: DateTime = unix_to_datetime(
        attributes.modified()?
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards").as_secs() as i64); //essentially just getting the pure unixtime to put into the function
    let mod_today = last_modified.get_date() == date.get_date(); //check if the metadata shows a modification date of today
        if attributes.is_dir() //ignore the extra indenting, this is in case of a checking feature being added since one was removed due to linux being strange.
        {
            let options = std::fs::read_dir(&path)?;
            for option in options {
                let option_path = match option {
                    Ok(entry) => entry.path(),
                    Err(_) => continue, // skip unreadable directory entries
                };
                if let Err(_e) = check_edits(&option_path, &date) {
                    // ignore errors for this entry
                    // the flow is like: check(data) > permission denied > continue
                    continue;
                }
            }
        }
        else if attributes.is_file() && mod_today
        {
            println!("{}", path.to_string_lossy());
        }
        else if attributes.is_symlink()
        {
            let _ = check_edits(&std::fs::canonicalize(&path)?, &date)?;
        }   
    Ok(())

}   


struct DateTime {
    year: i32,
    month: u32,
    day: u32,
}

impl DateTime {
    fn get_date(&self) -> String
    {
        format!("{}/{}/{}", self.day, self.month, self.year).to_string()
    }
}
fn unix_to_datetime(timestamp: i64) -> DateTime {
    // 1. Extract the time of day components
    let seconds_in_day = 86400;


    // 2. Shift epoch from 1970-01-01 to 0000-03-01 to simplify leap years
    let days_since_unix = timestamp.div_euclid(seconds_in_day);
    let days_from_hinnant_epoch = days_since_unix + 719468;

    // Compute the era (400-year cycle)
    let era = days_from_hinnant_epoch.div_euclid(146097);
    let day_of_era = days_from_hinnant_epoch.rem_euclid(146097);

    // Compute the year within the era
    let year_of_era = (day_of_era - day_of_era / 1460 + day_of_era / 36524 - day_of_era / 146097) / 365;
    let mut year = (year_of_era + era * 400) as i32;

    // Compute day of the year (starting in March)
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);

    // Compute month and day
    let month_internal = (5 * day_of_year + 2) / 153;
    let day = (day_of_year - (153 * month_internal + 2) / 5 + 1) as u32;
    let month = (month_internal + if month_internal < 10 { 3 } else { -9 }) as u32;

    // Adjust year back if the shifted month belongs to the previous calendar year
    if month <= 2 {
        year += 1;
    }

    DateTime { year, month, day }
}

fn compare_strings(comp: &str, blocked: &[String]) -> bool
{
    for directory in blocked.iter()
    {
        if comp == directory { return true };
    }
    return false;   
}
