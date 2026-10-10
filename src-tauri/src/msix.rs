//! 以 MSIX 包（微软商店版）运行时的系统集成。
//! 包内写注册表 Run 键会被虚拟化而不生效，开机自启要改用清单里声明的 StartupTask

use std::sync::OnceLock;
use windows::{
    core::HSTRING,
    ApplicationModel::{StartupTask, StartupTaskState},
    Win32::{
        Foundation::APPMODEL_ERROR_NO_PACKAGE, Storage::Packaging::Appx::GetCurrentPackageFullName,
    },
};

/// 与 msix/AppxManifest.xml 中 uap5:StartupTask 的 TaskId 一致
const STARTUP_TASK_ID: &str = "RestGuardStartup";

pub fn is_packaged() -> bool {
    static PACKAGED: OnceLock<bool> = OnceLock::new();
    *PACKAGED.get_or_init(|| {
        let mut len = 0;
        unsafe { GetCurrentPackageFullName(&mut len, None) != APPMODEL_ERROR_NO_PACKAGE }
    })
}

fn startup_task() -> windows::core::Result<StartupTask> {
    StartupTask::GetAsync(&HSTRING::from(STARTUP_TASK_ID))?.join()
}

pub fn startup_enabled() -> bool {
    startup_task()
        .and_then(|task| task.State())
        .is_ok_and(|state| {
            state == StartupTaskState::Enabled || state == StartupTaskState::EnabledByPolicy
        })
}

pub fn set_startup(on: bool) -> windows::core::Result<()> {
    let task = startup_task()?;
    if on {
        // 用户在任务管理器里禁用过时保持 DisabledByUser，只能由用户在系统设置里重新开启
        task.RequestEnableAsync()?.join()?;
    } else {
        task.Disable()?;
    }
    Ok(())
}
