pub fn kill_by_pid(pid: u32) {
    #[cfg(windows)]
    {
        if let Err(err) = win_kill_process_by_pid(pid) {
            println!("kill process failed, error: {}", err);
        }

        fn win_kill_process_by_pid(pid: u32) -> Result<(), String> {
            use windows::Win32::Foundation::{CloseHandle, HANDLE};
            use windows::Win32::System::Threading::{
                OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_TERMINATE, TerminateProcess,
            };

            unsafe {
                let desired_access = PROCESS_TERMINATE | PROCESS_QUERY_INFORMATION;

                let process_handle: HANDLE =
                    OpenProcess(desired_access, false, pid).map_err(|err| err.to_string())?;

                // Use a guard or defer equivalent for closing the handle
                // The HANDLE type doesn't implement Drop automatically in windows-rs for all scenarios,
                // so explicit closing is a good practice, or use an external crate like 'scopeguard'.

                TerminateProcess(process_handle, 1).map_err(|err| err.to_string())?;
                CloseHandle(process_handle).map_err(|err| err.to_string())?;
            };

            Ok(())
        }
    }

    #[cfg(not(windows))]
    {
        let mut sys = sysinfo::System::new();
        sys.refresh_all();

        let pid = sysinfo::Pid::from_u32(pid);

        if let Some(process) = sys.process(pid) {
            process.kill_and_wait().unwrap_or_default();
        }
    }
}
