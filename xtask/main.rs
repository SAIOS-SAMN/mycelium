// AI-GENERATED CONTENT — see DISCLAIMER.md.
mod lint;

fn main() {
    match lint::run() {
        Ok(()) => {}
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    }
}
