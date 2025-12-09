use crate::{LogLevel, target::TargetId};
use std::sync::{LazyLock, RwLock};

type HookCallback = Box<dyn Fn(LogLevel, &TargetId) + Send + Sync>;

/// Represents a hook that can be set to trigger
/// at specific points in the logging process.
///
/// Hooks can be used to perform custom actions, for example,
/// for performing side effects, such as clearing the console
/// or sending notifications.
///
/// Hooks are executed in the order they are added.
pub enum Hook {
    /// Hook that is called before a log message is written.
    BeforeLog(HookCallback),
    /// Hook that is called after a log message is written.
    /// This is useful for post-processing or additional actions.
    AfterLog(HookCallback),
}

pub struct HookSystem {
    before_log_hooks: Vec<Hook>,
    after_log_hooks: Vec<Hook>,
}

impl HookSystem {
    pub fn new() -> Self {
        Self {
            before_log_hooks: Vec::new(),
            after_log_hooks: Vec::new(),
        }
    }

    /// Splits the hooks into two separate vectors
    pub fn add_hook(&mut self, hook: Hook) {
        match hook {
            Hook::BeforeLog(_) => self.before_log_hooks.push(hook),
            Hook::AfterLog(_) => self.after_log_hooks.push(hook),
        }
    }

    /// Triggers all before log hooks
    /// with the provided log level and target ID.
    pub fn trigger_before_log(&self, level: LogLevel, target_id: &TargetId) {
        for hook in &self.before_log_hooks {
            if let Hook::BeforeLog(callback) = hook {
                callback(level, target_id);
            }
        }
    }

    /// Triggers all after log hooks
    /// with the provided log level and target ID.
    pub fn trigger_after_log(&self, level: LogLevel, target_id: &TargetId) {
        for hook in &self.after_log_hooks {
            if let Hook::AfterLog(callback) = hook {
                callback(level, target_id);
            }
        }
    }
}

static HOOK_SYSTEM: LazyLock<RwLock<HookSystem>> = LazyLock::new(|| RwLock::new(HookSystem::new()));

pub fn hook_system() -> &'static RwLock<HookSystem> {
    &HOOK_SYSTEM
}

/// Sets a hook to be called at the specified point in the logging process.
///
/// Multiple hooks can be set.
///
/// IMPORTANT: do not use the macros defined in this crate in `BeforeLog` or `AfterLog` hooks.
/// This will lead to an infinite loop, as the logging macros trigger the hooks
/// themselves.
pub fn set_hook(hook: Hook) {
    if let Ok(mut hook_system) = HOOK_SYSTEM.write() {
        hook_system.add_hook(hook);
    } else {
        eprintln!("Failed to acquire write lock on hook system. Hook not set.");
    }
}
