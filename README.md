# WinProcList Library

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
![crates.io](https://img.shields.io/badge/crates.io-v0.1.0-brightgreen.svg)

## Overview
WinProcList is a Rust library that utilizes Windows API to retrieve information about processes and threads within the system. This library provides two methods for obtaining process information: batch retrieval of process information and retrieval of specific process information.

## Methods for Retrieving Process Information

1. **Retrieving information of all processes and threads**
   - Use `winproclist::get()` to retrieve information on all processes and threads at once.
   - Within the retrieved `WinProcList`, it is possible to search by process name or PID.
   - Effective when you want to retrieve multiple process information at once.

2. **Retrieving only specific process information**
   - Use `get_proc_info_by_pid(pid: u32) -> Result<Option<ProcInfo>, WinProcListError>` to retrieve only the process information corresponding to the specified PID.
   - Effective when you want to retrieve information for a few processes while conserving memory usage.

## `ProcInfo` Structure
`ProcInfo` is a structure that holds detailed information about a process and has the following member variables.

```rust
pub struct ProcInfo {
    pub next_entry_offset: u32,
    pub image_name: String,
    pub unique_process_id: u32,
    pub handle_count: u32,
    pub session_id: u32,
    pub peak_virtual_size: usize,
    pub virtual_size: usize,
    pub peak_working_set_size: usize,
    pub quota_paged_pool_usage: usize,
    pub quota_non_paged_pool_usage: usize,
    pub pagefile_usage: usize,
    pub peak_pagefile_usage: usize,
    pub private_page_count: usize,
    pub number_of_threads: u32,
    pub working_set_private_size: LargeInteger,
    pub hard_fault_count: u32,
    pub number_of_threads_high_watermark: u32,
    pub cycle_time: u64,
    pub create_time: LargeInteger,
    pub user_time: LargeInteger,
    pub kernel_time: LargeInteger,
    pub base_priority: i32,
    pub inherited_from_unique_process_id: *mut c_void,
    pub unique_process_key: usize,
    pub page_fault_count: u32,
    pub working_set_size: usize,
    pub quota_peak_paged_pool_usage: usize,
    pub quota_peak_non_paged_pool_usage: usize,
    pub read_operation_count: LargeInteger,
    pub write_operation_count: LargeInteger,
    pub other_operation_count: LargeInteger,
    pub read_transfer_count: LargeInteger,
    pub write_transfer_count: LargeInteger,
    pub other_transfer_count: LargeInteger,
    pub threads: Vec<ThreadInfo>,
}
```

Each member variable corresponds to the member variables of the SYSTEM_PROCESS_INFORMATION structure in the ntapi crate.

## ThreadInfo Structure
Each ProcInfo holds thread information as a Vec of ThreadInfo.  
``ProcInfo.threads`` has a list of ThreadInfo structures that hold detailed information about a thread.

``` rust
pub struct ThreadInfo {
    pub kernel_time: LargeInteger,
    pub user_time: LargeInteger,
    pub create_time: LargeInteger,
    pub wait_time: u32,
    pub start_address: *mut c_void,
    pub priority: i32,
    pub base_priority: i32,
    pub context_switches: u32,
    pub thread_state: u32,
    pub wait_reason: u32,
    pub client_id: ClientID,
}
```

Each member variable corresponds to the member variables of the SYSTEM_THREAD_INFORMATION structure in the ntapi crate.

## Sample Code
### Example: Retrieving all process information
```rust
use winproclist::WinProcList;

fn main() {
    let proc_list = winproclist::get().expect("Failed to retrieve process list");
    for proc in proc_list.proc_list.iter() {
        println!("PID: {}, Name: {}", proc.unique_process_id, proc.image_name);
    }
}
```

### Example: Retrieving specific process information
```rust
use winproclist::get_proc_info_by_pid;

fn main() {
    let pid = 1234; // PID of the process to retrieve
    if let Ok(Some(proc_info)) = get_proc_info_by_pid(pid) {
        println!("Found process: {} (PID: {})", proc_info.image_name, proc_info.unique_process_id);
    } else {
        println!("Process with PID {} not found", pid);
    }
}
```


### Example: Searching for a process by name
```rust
use winproclist::WinProcList;

fn main() {
    let proc_list = winproclist::get().expect("Failed to retrieve process list");
    let process_name = "cargo.exe";
    println!("\nSearch by process name: {}", process_name);
    let procs = proc_list.search_by_name(process_name);
    if procs.is_empty() {
        println!("Process not found.");
    }
    else {
        for proc in procs.iter() {
            println!("PID: {}, Name: {}", proc.unique_process_id, proc.image_name);
        }
    }
}
```

### All sample code
Print all process and thread information, search by PID, search by process name, get PID by process name, get process name by PID, and get process info by PID.
```rust
use winproclist;
use std::fmt;

fn print_proc_header() {
    println!("{:<25} {:<10} {:<10} {:<10} {:<15} {:<15} {:<15} {:<10} {:<10}",
        "ImageName", "PID", "Handles", "SessionId", "VirtualSize", "PagefileUsage", "PrivatePages", "Priority", "Threads");
    println!("{:-<125}", "");
}

fn print_thread_header() {
    println!("    {:<10} {:<15} {:<15} {:<20} {:<10} {:<15} {:<10}", "TID", "KernelTime", "UserTime", "CreateTime", "WaitTime", "ContextSwitches", "Priority");
    println!("    {:-<121}", "");
}

fn print_proc_info(proc: &winproclist::ProcInfo) {
    print_proc_header();
    println!("{:<25} {:<10} {:<10} {:<10} {:<15} {:<15} {:<15} {:<10} {:<10}",
        proc.image_name,
        proc.unique_process_id,
        proc.handle_count,
        proc.session_id,
        proc.virtual_size,
        proc.pagefile_usage,
        proc.private_page_count,
        proc.base_priority,
        proc.number_of_threads
    );
    
    if !proc.threads.is_empty() {
        print_thread_header();
        for thread in &proc.threads {
            println!("    {:<10} {:<15} {:<15} {:<20} {:<10} {:<15} {:<10}",
                thread.client_id.unique_thread_id as u32,
                thread.kernel_time.to_u64(),
                thread.user_time.to_u64(),
                thread.create_time.to_u64(),
                thread.wait_time,
                thread.context_switches,
                thread.priority
            );
        }
    }
    println!("{:-<125}", "");
}

fn main() -> Result<(), String> {
    let win_proc_list = winproclist::get().map_err(|e| e.to_string())?;
    
    for proc in win_proc_list.proc_list.iter() {
        print_proc_info(proc);
    }

    println!("\n{:=<125}", "");
    
    let pid = std::process::id();
    println!("\nSearch by this process id: {}", pid);
    if let Some(proc) = win_proc_list.search_by_pid(pid) {
        print_proc_info(proc);
    } else {
        println!("Process not found.");
    }

    println!("\n{:=<125}", "");
    
    let name = std::env::current_exe().map_err(|e| e.to_string())?;
    let name = name.file_name().ok_or("Invalid file name.")?.to_str().ok_or("Invalid file name.")?;
    println!("\nSearch by process name: {}", name);
    let procs = win_proc_list.search_by_name(name);
    if procs.is_empty() {
        println!("Process not found.");
    }
    else {
        for proc in procs.iter() {
            print_proc_info(proc);
        }
    }

    println!("\n{:=<125}", "");
    
    println!("\nGet PID by process name: {}", name);
    if let Some(pids) = win_proc_list.get_pids_by_name(name) {
        for pid in pids.iter() {
            println!("PID: {}", pid);
        }
    } else {
        println!("Process not found.");
    }

    println!("\n{:=<125}", "");
    
    println!("\nGet process name by PID: {}", pid);
    if let Some(name) = win_proc_list.get_name_by_pid(pid) {
        println!("Process name: {}", name);
    } else {
        println!("Process not found.");
    }

    println!("\n{:=<125}", "");
    
    println!("\nGet process info by PID: {}", pid);
    if let Some(proc) = winproclist::get_proc_info_by_pid(pid).map_err(|e| e.to_string())? {
        print_proc_info(&proc);
    } else {
        println!("Process not found.");
    }
    
    Ok(())
}
```

## License
This library is provided under the MIT or Apache-2.0 license. Please refer to the LICENSE file for details.
