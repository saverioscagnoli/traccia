use std::sync::{LazyLock, RwLock};

use crate::Error;

pub enum Hook {
    BeforeLog(Box<dyn Fn() -> ()>),
    AfterLog(Box<dyn Fn() -> ()>),
}

unsafe impl Send for Hook {}
unsafe impl Sync for Hook {}

pub struct HookSystem {
    before_log: Vec<Box<dyn Fn() -> ()>>,
    after_log: Vec<Box<dyn Fn() -> ()>>,
}

impl HookSystem {
    pub const fn new() -> Self {
        Self {
            before_log: vec![],
            after_log: vec![],
        }
    }

    pub fn trigger_before_log(&self) {
        for hook in &self.before_log {
            hook();
        }
    }

    pub fn trigger_after_log(&self) {
        for hook in &self.after_log {
            hook();
        }
    }
}

unsafe impl Send for HookSystem {}
unsafe impl Sync for HookSystem {}

pub static HOOK_SYSTEM: LazyLock<RwLock<HookSystem>> =
    LazyLock::new(|| RwLock::new(HookSystem::new()));

pub fn hook_system() -> &'static RwLock<HookSystem> {
    &HOOK_SYSTEM
}

pub fn set_hook(hook: Hook) -> Result<(), Error> {
    let mut hooks = HOOK_SYSTEM.write()?;

    match hook {
        Hook::BeforeLog(hook) => hooks.before_log.push(hook),
        Hook::AfterLog(hook) => hooks.after_log.push(hook),
    }

    Ok(())
}
