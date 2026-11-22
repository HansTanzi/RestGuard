//! System integration when running as an MSIX package (Microsoft Store version).
//! Writes to the registry Run key are virtualized inside the package and have no effect, so autostart uses the StartupTask declared in the manifest

use std::sync::OnceLock;
use windows::{
    core::HSTRING,
    ApplicationModel::{
        Activation::ActivationKind, AppInstance, StartupTask, StartupTaskState,
    },
    Win32::{
        Foundation::APPMODEL_ERROR_NO_PACKAGE, Storage::Packaging::Appx::GetCurrentPackageFullName,
    },
};

/// Matches the TaskId of uap5:StartupTask in msix/AppxManifest.xml
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

/// Whether this launch is the StartupTask's at login rather than the user's
pub fn launched_by_startup_task() -> bool {
    AppInstance::GetActivatedEventArgs()
        .and_then(|args| args.Kind())
        .is_ok_and(|kind| kind == ActivationKind::StartupTask)
}

pub fn set_startup(on: bool) -> windows::core::Result<()> {
    let task = startup_task()?;
    if on {
        // If the user disabled it in Task Manager it stays DisabledByUser; only the user can re-enable it in system settings
        task.RequestEnableAsync()?.join()?;
    } else {
        task.Disable()?;
    }
    Ok(())
}
