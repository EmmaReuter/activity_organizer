use rand::seq::SliceRandom;
use notify_rust::Notification;
use std::{ thread, time };
use std::fs::File;
use std::io::BufReader;
use rodio::{ Decoder, OutputStream, source::Source };
use rusqlite::{ Connection };

#[derive(Debug)]
struct Todo {
    id: u64,
    todo: String,
    date: String,
}

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


fn insert_rows() {
    let conn = Connection::open("todos.db").unwrap();
    conn.execute(
        "INSERT INTO todos (todo,date) values (?1, datetime('now','localtime'))",
        &["Brush Teeth"] // empty list of parameters.
    ).unwrap();
    conn.execute(
        "INSERT INTO todos (todo,date) values (?1, datetime('now','localtime'))",
        &["Make Bed"] // empty list of parameters.
    ).unwrap();
    conn.execute(
        "INSERT INTO todos (todo,date) values (?1, datetime('now','localtime'))",
        &["Voice Training"] // empty list of parameters.
    ).unwrap();
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
    //insertRows();

   

    let conn = Connection::open("todos.db").unwrap();
    let mut stmt = conn.prepare("select * from todos;").unwrap();
    let todos = stmt
        .query_map([], |row| {
            Ok(Todo {
                id: row.get(0)?,
                todo: row.get(1)?,
                date: row.get(2)?,
            })
        })
        .unwrap();

    let school = vec!["English", "Compilers", "Research", "Anki"];
    let mut hacking = vec![
        "Blog",
        "HTB",
        "HTB Academy",
        "THM",
        "Bug Bounties",
        "Malware course",
        "pwn.college"
    ];
    let mut general = vec!["Yoga", "Medidation", "Duilingo"];

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
    if line.eq("m\n") {
        insert_rows();
        return
    }
    let b = line.eq("y\n");
    if b {
        for todo in todos {
            let t = todo.unwrap();
            println!("{}. {}", t.id, t.todo);
        }
        let mut intline = String::new();
        std::io::stdin().read_line(&mut intline).unwrap();
        let trimmed = intline.trim();
        println!("{:?}", trimmed);
        let mut delete = 0;
        match trimmed.parse::<u32>() {
            Ok(i) => {
                delete = i;
            }
            Err(..) => println!("This was not integer"),
        }
        conn.execute("DELETE FROM todos where id = ?1", (delete,)).unwrap();
    } else {
        choice = activities.choose(&mut rand::thread_rng()).unwrap();
        println!("Work on {}", choice);
        sleep(20);
    }

    println!("Your capable of growing through work");
    std::io::stdin().read_line(&mut line).unwrap();
    if rand::random() {
        println!("Twitter");
        sleep(0);
    }
    println!("5 minutes");
    sleep(0)
}