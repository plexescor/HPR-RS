#![allow(non_snake_case)] //lol get reckt

mod window {
    pub mod CurrentWindowManager;
    pub mod WindowUtilities;
    pub mod WindowBackend;
}

slint::include_modules!();

fn main()
{
    MainWindow::new().unwrap().run().unwrap();
}