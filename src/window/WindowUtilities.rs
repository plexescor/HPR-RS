
#[cfg(target_os = "windows")]
pub fn runSystemCommand_UNSAFE(command: &String) -> String
{
    let execution = std::process::Command::new("cmd")
        .arg("/C")
        .arg(command)
        .output()
        .expect("failed to execute process");

    if execution.status.code() != Some(0)
    {
        return String::new(); 
    }
    
    String::from_utf8_lossy(&execution.stdout).into_owned()
}

#[cfg(target_os = "linux")]
pub fn runSystemCommand_UNSAFE(command: &String) -> String
{
    let execution = std::process::Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("failed to execute process");

    if execution.status.code() != Some(0)
    {
        return String::new(); 
    }
    
    String::from_utf8_lossy(&execution.stdout).into_owned()
}