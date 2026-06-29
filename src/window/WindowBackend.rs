use crate::window::WindowUtilities::runSystemCommand_UNSAFE;
use std::sync::OnceLock;

static BACKENDS: OnceLock<Vec<WindowBackend>> = OnceLock::new();

struct WindowBackend
{
    name: String, //because extensions can also at runtime

    //personal notes; its just fucking normal
    matchesEnvironment: Box<dyn Fn(String) -> bool + Send + Sync>,
    initialize: Box<dyn Fn() + Send + Sync>,
    isUsable: Box<dyn Fn() -> bool + Send + Sync>,
    getCurrentWindow: Box<dyn Fn() -> String + Send + Sync>,
    getCurrentTitle: Box<dyn Fn() -> String + Send + Sync>,
    getCurrentPid: Box<dyn Fn() -> String + Send + Sync>    
}

impl WindowBackend {
    fn registerBuiltInBackends()
    {
        let mut tempVec = Vec::new();

        let hyprlandBackend = WindowBackend 
        {
            name: String::from("Hyprland"),

            matchesEnvironment: Box::new(|env| -> bool
            {
                env == "Hyprland" || env == "hyprland"
            }),

            initialize: Box::new (|| 
            {
                let u = 3; //Just to shut up intellisense
            }),

            isUsable: Box::new(|| -> bool
            {
                let test = String::from("hyprctl monitors -j");
                let result = runSystemCommand_UNSAFE(&test);
                return result.contains("id"); //will fail if no monitors
            }),

            getCurrentWindow: Box::new(|| -> String
            {
                let cmd = String::from("hyprctl activewindow -j");
                let json = runSystemCommand_UNSAFE(&cmd);

                let key = r#""class":"#;
                let key_pos = if let Some(pos) = json.find(key) { pos } else { return String::new(); };
                let quote_start = if let Some(pos) = json[key_pos + key.len()..].find('"') { pos + key_pos + key.len() } else { return String::new(); };
                let quote_end = if let Some(pos) = json[quote_start + 1..].find('"') { pos + quote_start + 1 } else { return String::new(); };

                return json[quote_start + 1..quote_end].to_string();
            }),

            getCurrentTitle: Box::new(|| -> String
            {
                let cmd = String::from("hyprctl activewindow -j");
                let json = runSystemCommand_UNSAFE(&cmd);

                let key = r#""title":"#;
                let key_pos = if let Some(pos) = json.find(key) { pos } else { return String::new(); };
                let quote_start = if let Some(pos) = json[key_pos + key.len()..].find('"') { pos + key_pos + key.len() } else { return String::new(); };
                let quote_end = if let Some(pos) = json[quote_start + 1..].find('"') { pos + quote_start + 1 } else { return String::new(); };

                return json[quote_start + 1..quote_end].to_string();
            }),

            getCurrentPid: Box::new(|| -> String
            {
                let cmd = String::from("hyprctl activewindow -j");
                let json = runSystemCommand_UNSAFE(&cmd);

                let key = r#""pid":"#;
                let key_pos = if let Some(pos) = json.find(key) { pos } else { return String::new(); };
                let quote_start = if let Some(pos) = json[key_pos + key.len()..].find('"') { pos + key_pos + key.len() } else { return String::new(); };
                let quote_end = if let Some(pos) = json[quote_start + 1..].find('"') { pos + quote_start + 1 } else { return String::new(); };

                return json[quote_start + 1..quote_end].to_string();
            })
        };

        tempVec.push(hyprlandBackend);
        let _ = BACKENDS.set(tempVec);
    }
}