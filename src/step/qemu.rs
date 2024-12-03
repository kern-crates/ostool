use std::{
    collections::HashMap,
    ffi::{OsStr, OsString},
    fs,
    path::PathBuf,
    process::{exit, Command},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use colored::Colorize;

use crate::{
    project::{Arch, Project},
    shell::Shell,
    QemuArgs,
};

use super::Step;

pub struct Qemu {
    args: QemuArgs,
    is_check_test: bool,
    machine: String,
    kernel: PathBuf,
    cmd: Command,
}

impl Qemu {
    pub fn new_boxed(cli: QemuArgs, is_check_test: bool) -> Box<dyn Step> {
        Box::new(Self {
            args: cli,
            is_check_test,
            machine: "virt".to_string(),
            kernel: PathBuf::new(),
            cmd: Command::new("ls"),
        })
    }

    fn cmd_windows_env(&mut self) {
        let env = self.cmd.get_envs().collect::<HashMap<_, _>>();
        let mut mysys2_root = PathBuf::from("C:\\msys64");
        if let Some(p) = std::env::var_os("MSYS2_ROOT") {
            mysys2_root = PathBuf::from(p);
        }

        let mut path = env
            .get(OsStr::new("PATH"))
            .unwrap_or(&None)
            .unwrap_or_default()
            .to_os_string();
        if !path.is_empty() {
            path.push(";");
        }

        let ucrt64 = mysys2_root.join("ucrt64/bin");

        if ucrt64.join("qemu-system-x86_64.exe").exists() {
            path.push(OsString::from(ucrt64));
        }

        let mingw64 = mysys2_root.join("mingw64/bin");

        if mingw64.join("qemu-system-x86_64.exe").exists() {
            path.push(mingw64);
        }

        self.cmd.env("PATH", path);
    }
}

impl Step for Qemu {
    fn run(&mut self, project: &mut Project) -> anyhow::Result<()> {
        self.cmd = project.shell(project.arch.unwrap().qemu_program());

        if matches!(project.arch, Some(Arch::X86_64)) {
            self.machine = "q35".to_string();
            self.kernel = project.elf_path.clone().unwrap();
        } else {
            self.kernel = project.bin_path.clone().unwrap();
        }

        if let Some(m) = project.config_ref().qemu.machine.as_ref() {
            self.machine = m.to_string();
        }

        if self.args.dtb {
            let _ = fs::remove_file("target/qemu.dtb");
            self.machine = format!("{},dumpdtb=target/qemu.dtb", self.machine);
        }

        self.kernel = fs::canonicalize(&self.kernel).unwrap();

        #[cfg(target_os = "windows")]
        self.cmd_windows_env();

        if !project.config_ref().qemu.graphic {
            self.cmd.arg("-nographic");
        }
        self.cmd.args(["-machine", &self.machine]);

        let more_args = project
            .config_ref()
            .qemu
            .args
            .split(" ")
            .map(|o| o.trim())
            .filter(|o| !o.is_empty())
            .collect::<Vec<_>>();

        if !more_args.is_empty() {
            self.cmd.args(more_args);
        }

        if self.args.debug {
            self.cmd.args(["-s", "-S"]);
        }

        if let Some(cpu) = &project.config_ref().qemu.cpu {
            self.cmd.arg("-cpu");
            self.cmd.arg(cpu);
        }
        self.cmd.arg("-kernel");
        self.cmd.arg(&self.kernel);

        if self.is_check_test {
            let is_ok = Arc::new(AtomicBool::new(false));
            let is_ok2 = is_ok.clone();
            self.cmd
                .exec_with_lines(project.is_print_cmd, move |line| {
                    if line.contains("All tests passed") {
                        is_ok2.store(true, Ordering::SeqCst);
                    }
                    Ok(())
                })
                .unwrap();
            if !is_ok.load(Ordering::SeqCst) {
                println!("{}", "Test failed!".red());
                exit(1);
            }
        } else {
            self.cmd
                .exec(project.is_print_cmd)
                .expect("run qemu failed!");
        }
        Ok(())
    }
}