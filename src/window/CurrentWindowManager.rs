use std::sync::atomic::AtomicBool;
use std::thread;

use crate::window::WindowUtilities::runSystemCommand_UNSAFE;
use crate::window::WindowBackend;

struct CurrentWindowManager {
    CurrentPlatform: String,
    window: String,
    previousWindow: String, 
    tab: String,
    project: String,
    running: AtomicBool,
    windowPollingThread: Option<std::thread::JoinHandle<()>>
}

impl CurrentWindowManager {

    //Constructor handles registerBuiltInBackends
    #[cfg(target_os = "linux")]
    fn new() -> Self 
    {
        WindowBackend::registerBuiltInBackends();
        let command: String = String::from("echo $XDG_CURRENT_DESKTOP");
        let platform: String = runSystemCommand_UNSAFE(&command);

        Self {
            CurrentPlatform: platform,
            window: String::new(),
            previousWindow: String::new(),
            tab: String::new(),
            project: String::new(),
            running: AtomicBool::new(true), 
            windowPollingThread: None, 
        }
    }

    #[cfg(target_os = "windows")]
    fn new() -> Self 
    {
        let platform = String::from("Windows");

        Self {
            CurrentPlatform: String::from("Windows"),
            window: String::new(),
            previousWindow: String::new(),
            tab: String::new(),
            project: String::new(),
            running: AtomicBool::new(true), 
            windowPollingThread: None, 
        }
    }
}