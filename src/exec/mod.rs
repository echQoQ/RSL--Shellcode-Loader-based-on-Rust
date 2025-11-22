// CreateThread + WaitForSingleObject 方式
#[cfg(feature = "run_create_thread")]
mod create_thread;
#[cfg(feature = "run_create_thread")]
pub use crate::exec::create_thread::exec;

// EnumUILanguagesW 回调方式
#[cfg(feature = "run_enum_ui")]
mod enum_ui;
#[cfg(feature = "run_enum_ui")]
pub use crate::exec::enum_ui::exec;

// GDI 家族变种注入
#[cfg(feature = "run_gdi_families")]
mod gdi_families;
#[cfg(feature = "run_gdi_families")]
pub use crate::exec::gdi_families::exec;
