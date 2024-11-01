use compile::Compile;
use qemu::Qemu;
use serde::{Deserialize, Serialize};

use crate::{project::Arch, uboot::UbootConfig};

pub mod compile;
pub mod qemu;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectConfig {
    pub compile: Compile,
    pub qemu: Qemu,
    pub uboot: Option<UbootConfig>,
}

impl ProjectConfig {
    pub fn new(arch: Arch) -> Self {
        Self {
            compile: Compile::default(),
            qemu: Qemu::new_default(arch),
            uboot: None,
        }
    }
}
