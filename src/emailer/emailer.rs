use std::process;

pub fn send_email(
    recipient: &str,
    sender: &str,
    subject: &str,
    message: &str,
) {
    let output = process::Command::new("bash").args([
        "-e",
        "/root/scripts/send_email.sh",
        recipient,
        sender,
        subject,
        message
    ]).output().unwrap();

}
