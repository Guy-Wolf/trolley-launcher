use tauri::{generate_context, Builder};

mod games;
mod settings;

fn main() {
    Builder::default()
        .run(generate_context!())
        .expect("Error while running tauri application");
}
