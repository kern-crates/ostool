use std::{fmt, time::Duration};

use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use uboot_shell::UbootShell;

fn main() {
    println!("wait for uboot");

    // dd if=/dev/zero of=target/test.txt bs=1k count=258
    let file = "target/test.txt";

    let mut uboot = new_uboot();

    // let addr = uboot.env_int("kernel_addr_r").unwrap();
    let addr = 0x90000000;

    let pb = ProgressBar::new(100);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn fmt::Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));

    uboot
        .loady(addr, file, |r, a| {
            pb.set_length(a as _);
            pb.set_position(r as _);
        })
        .unwrap();
    pb.finish_with_message("upload done");
    println!("finish");

    uboot
        .loady(addr, file, |r, a| {
            pb.set_length(a as _);
            pb.set_position(r as _);
        })
        .unwrap();
    pb.finish_with_message("upload done");
    println!("finish2");
}

fn new_uboot() -> UbootShell {
    let port = "/dev/ttyUSB0";
    let baud = 115200;
    // let baud = 1500000;

    let rx = serialport::new(port, baud)
        .timeout(Duration::from_millis(3000))
        .open()
        .map_err(|e| format!("无法打开串口: {:?}", e))
        .unwrap();

    let tx = rx.try_clone().unwrap();

    UbootShell::new(tx, rx).unwrap()
}
