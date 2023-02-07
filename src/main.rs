use rand::seq::SliceRandom;
use notify_rust::Notification;
use std::{thread, time};

fn slep() {
    thread::sleep(time::Duration::from_secs(60*20));
    Notification::new().
    summary("5 min left")
    .body("5 minutes left")
    .show()
    .unwrap();
    thread::sleep(time::Duration::from_secs(60*5));
    Notification::new().
    summary("Done")
    .body("Finished!")
    .show()
    .unwrap();

}

fn main() {
    let mut school = vec!["English","Compilers","Research",];
    let mut hacking = vec!["Blog","HTB","HTB Academy","THM","Bug Bounties","Malware course"];
    let mut general = vec!["Yoga","Medidation"];

    let mut activities = Vec::new();
    activities.append(&mut school);
    activities.append(&mut school);
    activities.append(&mut hacking);
    activities.append(&mut general);
    let choice = activities.choose(&mut rand::thread_rng()).unwrap();
    println!("Work on {}", choice);
    slep();

    
}
