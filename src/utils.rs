/*
    Unspecific functions
*/
use crossterm::{execute, terminal, cursor};
use std::io::{stdout, Write};

/// Clears the terminal screen
pub fn clear() {
    execute!(stdout(), terminal::Clear(terminal::ClearType::All)).expect("Error trying to clear the screen");
    execute!(stdout(), cursor::MoveTo(0,0)).expect("Error at moving");
    stdout().flush().expect("Error at flush to stdout");
}
//pub fn clear() {
//    print!("\x1B[2J\x1B[1;1H");
//}

/// Returns the cursor to the top-left of the screen
pub fn refresh() {
    let mut out = stdout();
    out.flush().expect("Error at flusing to stdout");
    execute!(out, cursor::MoveTo(0,0)).expect("Error at moving");
    out.flush().expect("Error at flusing to stdout");
}
//pub fn refresh() {
//    print!("\x1B[H");
//}

/// Delays any thread action
pub fn sleep(n: f64) {
    use std::thread;
    use std::time::Duration;

    thread::sleep(Duration::from_secs_f64(n));
}

//pub fn rand_num(lowest: usize, highest: usize) -> usize {
//    use rand::Rng;
//
//    if lowest > highest {
//        panic!("Invalid range for randomization: {}..{}", lowest, highest);
//    }
//    if lowest == highest {
//        return lowest;
//    }
//    let mut rng = rand::thread_rng();
//    rng.gen_range(lowest..=highest)
//}

pub(crate) fn log(data: String) {
    use std::fs::OpenOptions;
    use std::io::Write;
    use std::thread::{self, JoinHandle};

    let callback: JoinHandle<std::io::Result<()>> = thread::spawn(move || {
        if let Ok(mut file) = OpenOptions::new()
            .append(true)
            .create(true)
            .open(r"C:\Users\gmend\Rust\lab\spaceinvaders\logs.txt")
        {
            let _ = writeln!(file, "{}", data);
        }
        Ok(())
    });
    let _ = callback.join();
}
