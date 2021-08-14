use std::error::Error;
use std::str;
use std::process::{Command, ExitStatus};
use http::StatusCode;

#[derive(Debug)]
struct ShellResult {
    status: ExitStatus,
    stdout: String,
    stderr: String,
}

// Run a shell command and return a ShellResult
// ShellResult includes trimmed stdout and stderr as String and the ExitStatus
fn shell(cmd: &str) -> ShellResult {
    let output = Command::new("/bin/bash")
                 .arg("-c")
                 .arg(cmd)
                 .output()
                 .expect(format!("failed to execute {}", cmd).as_str());
    let stdout = String::from(str::from_utf8(output.stdout.as_slice()).unwrap().trim());
    let stderr = String::from(str::from_utf8(output.stderr.as_slice()).unwrap().trim());
    ShellResult { status: output.status, stdout, stderr }
}

fn its_down(host: &str) -> bool {
    let result: ShellResult;
    result = shell(format!("ping -q -c 1 {}", host).as_str());
    ! result.status.success()
}

fn netbox_maint_mode(host: &str) -> Result<bool,Box<dyn std::error::Error>> {
    log::debug!("maint mode check");
    let path = format!("{}{}{}", "https://", host, "/maint/");
    let status = reqwest::blocking::get(path)?.status();
    if status == StatusCode::OK {
        return Ok(true)
    }
    Ok(false)
}

#[macro_use]
extern crate lazy_static;

fn system_is_virtual() -> bool {
    lazy_static!{
        static ref HV: bool = shell("cat /proc/cpuinfo | grep hypervisor").status.success();
    }
    *HV
}

pub fn run(nbhost: &str) -> Result<(),Box<dyn Error>> {
    println!("running...");
    if its_down(nbhost) {
        log::debug!("{} is down", nbhost);
        return Ok(())
    }
    log::debug!("{} is up!", nbhost);

    // Check if Netbox is in maintenance mode
    if let Ok(true) = netbox_maint_mode(nbhost) {
        log::debug!("netbox is in maintenance mode. k thx bye.");
        return Ok(())
    }
    log::debug!("netbox is not in maintenance mode.");

    let result = shell("hostname -f");
    if ! result.status.success() {
        return Err("failed to get hostname".into())
    }

    log::debug!("fqdn is: {}", result.stdout);

    if system_is_virtual() {
        log::debug!("vm");
    } else {
        log::debug!("phys");
    }

    Ok(())
}
