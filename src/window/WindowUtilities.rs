use std::process::Command;

pub fn runSystemCommand_UNSAFE(command: String) -> String
{
    #[cfg(target_os = "windows")]
    let output = Command::new("cmd")
        .args(["/C", &command])
        .output();

    #[cfg(not(target_os = "windows"))]
    let output = Command::new("sh")
        .args(["-c", &command])
        .output();

    match output
    {
        Ok(output) => String::from_utf8_lossy(&output.stdout).to_string(),
        Err(_) => "".to_string(),
    }
}