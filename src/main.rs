use std::{process, env, thread, time::Duration};

// show help, if wrong arguments are given
fn help() {
    println!("usage:
    pause for given time(seconds), or if no argument is given, pause forever");
    process::exit(0);
}

// trigger pause loop
// AFAIK rust does not provide default or optional func arguments
// so pause_duration is always set, even if forever loop is triggered. 
fn pause(duration: i64) {
    let one_second = Duration::new(1, 0);
 
    if duration < 0 {
        loop {
            thread::sleep(one_second);
        }
    } else {
        // in case with args
        let mut i = 0;
        println!("pause for {} seconds", duration);
        while i < duration {
            thread::sleep(one_second);
            i += 1;
        }
    }

}

// pause for given time in seconds or for forever
fn main() {
    // handle ctrl-c so we can break out of the loop
    ctrlc::set_handler(move || {
        println!("received Ctrl+C!");
        process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");
    
    // get all args
    let args: Vec<String> = env::args().collect();

    match args.len() {
        // no arguments passed, pause forever
        1 => {
            // -1 means forever, 0 is 0 seconds duration
            pause(-1);
        },
        // one argument passed, pause for given duration
        // except for empty string, which is also forever loop, since no arg is given
        2 => {
            // check for empty string and if empty call help()
            if args[1].len() == 0 {
                pause(-1);
            }
            let is_int = args[1].parse::<i64>();
            //

            // match for int64, if not trigger help
            match is_int {
              Ok(_ok) => {
                pause(args[1].parse().unwrap());
              }
              // if argument is given, but is not an int, show help message
              Err(_e) => {
                  help();
              }
            }  
        },
        // in case no cases match
        // we should never land here
        _ => {
            // show a help message
            help();
        }
    }
}