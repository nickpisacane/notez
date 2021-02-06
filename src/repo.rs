use std::io::Result;
use std::process::Command;

pub fn sync(root: &str) -> Result<()> {
    if !has_changed(root)? {
        return Ok(());
    }
    Command::new("git")
        .arg("add")
        .arg(".")
        .current_dir(root)
        .output()?;
    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("Auto notes commit")
        .current_dir(root)
        .output()?;
    Command::new("git")
        .arg("push")
        .arg("origin")
        .arg("master")
        .current_dir(root)
        .output()?;
    Ok(())
}

pub fn has_changed(path: &str) -> Result<bool> {
    let output = Command::new("git")
        .arg("status")
        .arg("--short")
        .current_dir(path)
        .output()?;
    Ok(output.stdout.len() > 0)
}
