use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

trait Animation {
    fn animate(&self, running: &AtomicBool);
}

struct Dna {
    pause: Duration,
    rows: [&'static str; 18],
}

impl Animation for Dna {
    fn animate(&self, running: &AtomicBool) {
        let mut row_index = 0;
        let rows_len = self.rows.len();
        while running.load(Ordering::SeqCst) {
            row_index = (row_index + 1) % rows_len;

            let row = if row_index == 0 || row_index == 9 {
                self.rows[row_index].to_owned()
            } else {
                let n = match rand::random::<u8>() % 4 {
                    0 => ('A', 'T'),
                    1 => ('T', 'A'),
                    2 => ('C', 'G'),
                    3 => ('G', 'C'),
                    _ => unreachable!(),
                };
                self.rows[row_index]
                    .replace("{}", &n.0.to_string())
                    .replace("{}", &n.1.to_string())
            };
            println!("{}", row);
            thread::sleep(self.pause);
        }
    }
}

struct App {
    animation: Box<dyn Animation>,
}

impl App {
    fn new(animation: Box<dyn Animation>) -> Self {
        Self { animation }
    }

    fn run(&self, running: Arc<AtomicBool>) {
        println!("DNA Animation");
        println!("Press Ctrl-C to quit...");

        let r = running.clone();
        ctrlc::set_handler(move || {
            r.store(false, Ordering::SeqCst);
        })
        .expect("Error setting Ctrl-C handler");

        self.animation.animate(&running);
    }
}

fn main() {
    println!("DNA Animation");
    println!("Press Ctrl-C to quit...");

    let dna = Dna {
        pause: Duration::from_millis(150),
        rows: [
            "         ##",
            "        #{}-{}#",
            "       #{}---{}#",
            "      #{}-----{}#",
            "     #{}------{}#",
            "    #{}------{}#",
            "    #{}-----{}#",
            "     #{}---{}#",
            "     #{}-{}#",
            "      ##",
            "     #{}-{}#",
            "     #{}---{}#",
            "    #{}-----{}#",
            "    #{}------{}#",
            "     #{}------{}#",
            "      #{}-----{}#",
            "       #{}---{}#",
            "        #{}-{}#",
        ],
    };

    let running = Arc::new(AtomicBool::new(true));

    let app = App::new(Box::new(dna));
    app.run(running);
}
