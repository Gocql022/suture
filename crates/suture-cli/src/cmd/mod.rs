pub mod add;
pub mod apply;
pub mod archive;
pub mod audit;
pub mod batch;
pub mod bisect;
pub mod blame;
pub mod branch;
pub mod checkout;
pub mod cherry_pick;
pub mod classification;
pub mod clean;
pub mod clone;
pub mod commit;
pub mod config;
pub mod describe;
pub mod diff;
pub mod doctor;
pub mod drivers;
pub mod export;
pub mod fetch;
pub mod fsck;
pub mod gc;
pub mod git;
pub mod grep;
pub mod hook;
pub mod hub;
pub mod ignore;
pub mod init;
pub mod key;
pub mod lfs;
pub mod log;
pub mod ls_remote;
pub mod merge;
pub mod merge_file;
pub mod mv;
pub mod notes;
pub mod pull;
pub mod push;
pub mod rebase;
pub mod reflog;
pub mod remote;
pub mod repack;
pub mod repo_size;
pub mod report;
pub mod reset;
pub mod restore;
pub mod rev_parse;
pub mod revert;
pub mod rm;
pub mod rollback;
pub mod shortlog;
pub mod show;
pub mod squash;
pub mod stash;
pub mod status;
pub mod sync;
pub mod tag;
pub mod timeline;
pub mod tui;
pub mod undo;
pub mod verify;
pub mod version;
pub mod worktree;

pub fn user_error(ctx: &str, e: impl std::fmt::Display) -> Box<dyn std::error::Error> {
    Box::new(std::io::Error::other(format!("{ctx}: {e}")))
}

/// 测试共享的 CWD 串行锁：部分测试会 `set_current_dir` 改变进程全局工作目录，
/// 与并行测试互相干扰（workspace Cargo.toml 注释即提到
/// "CWD-dependent tests in suture-cli use a mutex guard"）。
/// 所有会修改 CWD 的测试必须在开头获取此锁。
#[cfg(test)]
pub(crate) static TEST_CWD_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
