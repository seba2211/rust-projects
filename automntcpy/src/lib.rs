use std::env;
use std::time::{Instant, Duration};
use std::thread;
use std::path;
use std::io;
use std::io::Write;

extern crate fs_extra;

/* On WSL, if getting errors on compilation:
* - apt-get install pkg-config
* - apt-get install libudev-dev 
*/
extern crate block_utils;

#[cfg(test)]
mod unit_test;

pub struct Config {
    filename: path::PathBuf,
    dev_mnt_point: path::PathBuf,
    should_search_mnt_point: bool,
    mountpoint_candidates: Vec<path::PathBuf>,
}

// Assuming the mountpoints are as described in the rules for automounting
const BASE_MOUNTPOINT_KEYWORD1: &'static str= "media"; 
const BASE_MOUNTPOINT_KEYWORD2: &'static str= "usb"; 
const MAX_NUMBER_OF_MOUNTPOINTS: usize = 4; // from usb 1, to usb 4
const SEARCH_DEVICE_TIMEOUT_S: u64 = 60;

impl Config {
    // create new element
    /// Takes env::Args elements and returns a Config element
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next(); // skip the first element, program name

        let filename = match args.next() {
            Some(arg) => path::PathBuf::new().join(arg),
            None => return Err("No filename provided"),
        };

        let mut should_search_mnt_point = false;
        let dev_mnt_point: path::PathBuf = match args.next() {
            Some(arg) => {
                path::PathBuf::new().join(arg)
            },
            None => {
                should_search_mnt_point = true;
                path::Path::new("/").join(BASE_MOUNTPOINT_KEYWORD1).join(BASE_MOUNTPOINT_KEYWORD2)
            },
        };

        // Generate mountpoints candidates -> this works only on Linux!
        let mountpoint_candidates_iter = (0..MAX_NUMBER_OF_MOUNTPOINTS).map(|elem| path::Path::new("/").join(BASE_MOUNTPOINT_KEYWORD1).join(BASE_MOUNTPOINT_KEYWORD2.to_owned() + &(elem + 1).to_string()));  // media/usb1,..., media/usbN

        Ok(Config{filename: filename,
                dev_mnt_point: dev_mnt_point,
                should_search_mnt_point: should_search_mnt_point,
                mountpoint_candidates: mountpoint_candidates_iter.collect::<Vec<path::PathBuf>>(),
            })
    }

    // check configuration arguments
    pub fn check_arguments(&self) -> Result<(), String> {
        // display mountpoint candidates
        // self.mountpoint_candidates.iter().for_each(|mp| println!("{}", mp));

        // check if filename exists
        if self.filename.exists() {
            return Ok(());
        } else {
            return Err(String::from("Filename does not exist"));
        }
        // check if mountpoint exists ? TODO

    }
}

fn get_mnt_point(config: &Config) -> Result<Vec<&path::PathBuf>, String> {

    if !config.should_search_mnt_point {  // check for a provided mountpoint
        println!("Looking for mounted devices at {}...", config.dev_mnt_point.display());

        match block_utils::is_mounted(&config.dev_mnt_point) {  // check if block 
            Ok(mp_mount_flag) => {
                if mp_mount_flag {
                    Ok(vec![&config.dev_mnt_point])
                } else {
                    Err(format!("provided mount point -> {}, NOT mounted", config.dev_mnt_point.display()))
                }
            },
            Err(e) => {
                Err(format!("provided mount point -> {}, NOT mounted, error: {}", config.dev_mnt_point.display(), e))
            },
        }
    } else {  // check for all standard mountpoints (based on automount rules -> see .py installation files)
        

        // Next are test functions and time measuring of different operations
        /*
        let mut start_t : Instant;
        let mut end_t : Instant;

        println!("Mounted candidates");
        config.mountpoint_candidates.iter().for_each(|mp| println!("{:?}", mp));
        
        println!("'mounted devices'");
        start_t = Instant::now();
        block_utils::get_mounted_devices().unwrap_or_else(|_| vec![]).iter().for_each(|dev| println!("{:?}", dev));
        end_t = Instant::now();
        println!("Duration: {} [ns]", (end_t - start_t).as_nanos());
        println!("----------------------");

        println!("'Mounted devices' on the candidate list (/media/usbX)");  // THIS DOES NOT UNWRAP THE RESULT VALUE INSIDE THE FUNCTION, SO IS USELESS
        start_t = Instant::now();
        config.mountpoint_candidates.iter().filter(|mountpoint_candidate| block_utils::is_mounted(mountpoint_candidate).is_ok()).for_each(|mp| println!("{:?}", mp));
        end_t = Instant::now();
        println!("Duration: {} [ns]", (end_t - start_t).as_nanos());
        println!("----------------------");
        
        // THIS WAS THE SOLUTION TO NOT INTERPRETING THE RESULT VALUE CONTAINED IN block_utils::is_mounted, ULTRA SLOW  -> REMEMBER TO UNWRAP THE VALUES!
        println!("'Mounted devices' on the candidate list (/media/usbX) that have an associated device -> the true mountpoints");
        start_t = Instant::now();
        config.mountpoint_candidates.iter()  // vector to iterator
                                    .filter(|mp| 
                                        block_utils::is_mounted(mp).is_ok())  // filter only the ' supposedly mounted' devices yielding an Ok() result
                                    .filter_map(|mp|
                                        block_utils::get_mount_device(mp).ok())  // get the associated device of them, if any (this yields those with an Ok() result)
                                    .filter(|mp| mp.is_some())  // The previous operation yields an Option, get only those which have 'Some'
                                    .map(|mp| mp.unwrap())  // Get the contained value from Some, it should be a Path representing the associated device, ex. /sda/sda1
                                    .filter_map(|mp| block_utils::get_mountpoint(mp).ok())  // For each associated device, get the mountpoints
                                    .filter(|mp| mp.is_some())  // the latter yields an Option, filter those which have 'Some'
                                    .map(|mp| mp.unwrap())  // Get the value from the Some, this is a Path, representing are the mountpoints related to a physically connected device
                                    .for_each(|mp| println!("{:?}", mp.display()));  // Print all the elements in the iterator
        end_t = Instant::now();
        println!("Duration: {} [ns]", (end_t - start_t).as_nanos());
        println!("----------------------");
        
        // THIS IS THE CORRECT SOLUTION IN THIS CASE
        println!("'Mounted candidates devices -> PROCESSING THE BOOLEAN contained in the result'");
        start_t = Instant::now();
        config.mountpoint_candidates.iter()  // vector to iterator
                                    .filter(|mp| block_utils::is_mounted(mp).unwrap_or_default() 
                                    )  // filter only the ' supposedly mounted' devices yielding an Ok() result, which unwrapped is true (Err is mapped to False, default case for bool)
                                    .for_each(|mp| println!("{:?}", mp));
        end_t = Instant::now();
        println!("Duration: {} [ns]", (end_t - start_t).as_nanos());
        println!("----------------------");
        
        */
        
        let mount_points : Vec<&path::PathBuf> = config.mountpoint_candidates.iter()  // vector to iterator
                                                       .filter(|mp| block_utils::is_mounted(mp).unwrap_or_default())
                                                       .collect::<Vec<&path::PathBuf>>();
                                                                                       
        if mount_points.len() > 0 {
            Ok(mount_points)
        } else {
            Err(String::from("no mountpoints found [automatic search]"))
        }
        
    }
}

fn cpy_file_to_src(config: &Config, mp: &str, options: &fs_extra::dir::CopyOptions) -> Result<u64, fs_extra::error::Error> {
    
    let handle = |process_info: fs_extra::TransitProcess| {
        //println!("Total bytes {}", process_info.total_bytes);
        if let Err(e) = print_cpy_progress(process_info.copied_bytes, process_info.total_bytes) { // if this yields Err, don't care
            eprintln!("io:Error on print sync -> {}", e);
        }  
        fs_extra::dir::TransitProcessResult::ContinueOrAbort
    };
    let mut from_paths : Vec<&path::PathBuf> = Vec::with_capacity(1);  // only 1 file allowed!
    from_paths.push(&config.filename);
    // copy filename to target/filename
    if !options.skip_exist {
        println!("Attempting to copy {} to {}...", config.filename.display(), mp);
    }
    let r = fs_extra::copy_items_with_progress(&from_paths, mp, options, handle)?;
    println!("");
    Ok(r)
}


pub fn run(config: &Config) -> Result<(), String> {
    
    if let Err(e) = config.check_arguments() {
        return Err(e);
    }

    // search for mounting point, wait if there are not (timeout)
    let instant_time_reference = Instant::now();
    let mut timeout :bool = false;
    let mut mount_points : Vec<&path::PathBuf> = Vec::with_capacity(MAX_NUMBER_OF_MOUNTPOINTS);
    println!("Insert your USB device");
    while !timeout {  // get mount points until timeout
        match get_mnt_point(config) {
            Ok(mps) => {  // if ok, assign on
                let number_mp = mps.len();
                if (number_mp > 0) && (number_mp < MAX_NUMBER_OF_MOUNTPOINTS) {
                    print!("Valid mountpoints found:");
                    mount_points = mps;
                    mount_points.iter().for_each(|mp| print!(" {} ", mp.display()));
                    println!("");
                    break;  // break the loop in this case
                } else if number_mp > MAX_NUMBER_OF_MOUNTPOINTS{
                    eprintln!("Mountpoint length is higher than the maximum allowed: {}", MAX_NUMBER_OF_MOUNTPOINTS);
                } else {  // otherwise the value is 0 (could not be negative)
                    eprintln!("Mountpoint length is 0");
                }
            },
            Err(_err) => {
                // eprintln!("Error: {}", _err);
                thread::sleep(Duration::from_millis(100));  // sleep
                if instant_time_reference.elapsed().as_secs() > SEARCH_DEVICE_TIMEOUT_S {timeout = true}
            }
        }
    }
    if timeout {return Err(String::from("Timeout!"))}  // evaluate timeout


    // Copy file
    if mount_points.len() > 0 {
        for mp in mount_points {

            let mp_str = mp.to_str().unwrap_or_default();  // default case creates an empty str
            
            if mp_str.is_empty() {
                eprint!("mp is not a valid UTF-8 sequence");
                continue;
            }

            println!("Destination folder: {}", mp_str);
            // Check if file exists -> overwrite?
            let target_destination : path::PathBuf = mp.join(&config.filename);
            println!("Target destination: {}", target_destination.display());
            let mut cpy_options = fs_extra::dir::CopyOptions::new(); // Initialize default values for CopyOptions
            if target_destination.exists() {
                // ask user if it is needed to overwrite
                match handle_user_yes_no(&format!("Filename already exists ({}), overwrite?", target_destination.display())) {
                    Ok (flag) => { 
                        if flag { // overwrite
                            cpy_options.overwrite  = true;
                        } else {
                            cpy_options.overwrite  = false;
                            cpy_options.skip_exist = true;
                        }
                    },
                    Err(e) => {eprintln!("Error getting user Y/n: {}", e)}   
                }
            }
            
            // copy file (with the options provided)
            if let Err(e) = cpy_file_to_src(config, mp_str, &cpy_options) {
                eprintln!("Copy Error: {}", e);
            } else {
                if cpy_options.skip_exist {println!("Copy operations skipped")}
                else {println!("Copy finished with success ({} -> {})", config.filename.display(), target_destination.display());}
            }
            
            // At the time, only 1 filename could be copied to each mp -> unmount
            if let Err(e) = block_utils::unmount_device(mp) {
                eprintln!("Unmount Error: {}", e);
            } else {
                println!("Device successfully unmounted. Extract your USB device");
            }
            
        }
    }  
    Ok(())
}

fn handle_user_yes_no(msg: &str) -> Result<bool, io::Error> {
    print!("{} [Y/n]: ", msg);
    io::stdout().flush()?;  // print sync
    let mut user_input = String::new();
    let stdin = io::stdin();
    stdin.lock();
    stdin.read_line(&mut user_input)?;
    Ok(yes_no(&user_input))
}

fn yes_no(user_input_str: &str) -> bool {
    match user_input_str.to_lowercase().trim() {
        "yes" => true,
        "y" => true,
        _ => false
    }
}

fn print_cpy_progress(copied_bytes: u64, total_bytes: u64) -> Result<(), io::Error> {
    let cpy_progress = (copied_bytes * 100)/total_bytes;
    match cpy_progress {
        0..= 9 =>   print!("\r=>          [{}%] ({}/{})", cpy_progress, copied_bytes, total_bytes),
        10..= 19 => print!("\r==>         [{}%] ({}/{})", cpy_progress, copied_bytes, total_bytes),
        20..= 29 => print!("\r===>        [{}%] ({}/{})", cpy_progress, copied_bytes, total_bytes),
        30..= 39 => print!("\r====>       [{}%] ({}/{})", cpy_progress, copied_bytes, total_bytes),
        40..= 49 => print!("\r=====>      [{}%] ({}/{})", cpy_progress, copied_bytes, total_bytes),
        50..= 59 => print!("\r======>     [{}%] ({}/{})", cpy_progress, copied_bytes, total_bytes),
        60..= 69 => print!("\r=======>    [{}%] ({}/{})", cpy_progress, copied_bytes, total_bytes),
        70..= 79 => print!("\r========>   [{}%] ({}/{})", cpy_progress, copied_bytes, total_bytes),
        80..= 89 => print!("\r=========>  [{}%] ({}/{})", cpy_progress, copied_bytes, total_bytes),
        90..= 99 => print!("\r==========> [{}%] ({}/{})", cpy_progress, copied_bytes, total_bytes),
        100 =>      print!("\r===========>[{}%] ({}/{})", cpy_progress, copied_bytes, total_bytes),
        _ => print!("\r"),
    }
    io::stdout().flush()?; // print sync
    Ok(())
}