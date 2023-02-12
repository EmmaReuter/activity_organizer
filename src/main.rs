use rand::seq::SliceRandom;
use notify_rust::Notification;
use std::{ thread, time };
use std::fs::File;
use std::io::BufReader;
use rodio::{ Decoder, OutputStream, source::Source };

fn sleep(x: u64) {
    thread::sleep(time::Duration::from_secs(60 * x));
    if x != 0 {
        Notification::new().summary("5 min left").body("5 minutes left").show().unwrap();
        println!("five minutes left!");
    }
    thread::sleep(time::Duration::from_secs(60 * 5));
    Notification::new().summary("Done").body("Finished!").show().unwrap();
    ding();
}

fn ding() {
    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open("ding-sound-effect_2.mp3").unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    // Play the sound directly on the device
    stream_handle.play_raw(source.convert_samples()).unwrap();
    std::thread::sleep(std::time::Duration::from_secs(5));
}

fn main() {
    let school = vec!["English", "Compilers", "Research", "Anki"];
    let mut hacking = vec!["Blog", "HTB", "HTB Academy", "THM", "Bug Bounties", "Malware course"];
    let hacking1 = hacking.clone();
    let mut general = vec!["Yoga", "Medidation", "Voice training"];

    let mut activities = Vec::new();
    activities.append(&mut school.clone());
    activities.append(&mut school.clone());
    activities.append(&mut school.clone());
    activities.append(&mut hacking);
    activities.append(&mut general);

    let mut line = String::new();
    println!("Chores y/n");
    std::io::stdin().read_line(&mut line).unwrap();
    let choice;
    if line.eq("y\n") {
        println!("woof");
        std::io::stdin().read_line(&mut line).unwrap();
        choice = hacking1.choose(&mut rand::thread_rng()).unwrap();
    } else {
        choice = activities.choose(&mut rand::thread_rng()).unwrap();
    }

    println!("Work on {}", choice);
    sleep(20);
    println!("Your capable of growing through work");
    std::io::stdin().read_line(&mut line).unwrap();
    if rand::random() {
        println!("Twitter");
        sleep(0);
    }

    println!("5 minutes");
    sleep(0)
}