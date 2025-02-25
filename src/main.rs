use winproclist;

fn main() -> Result<(), String> {
    let mut win_proc_list = winproclist::get().map_err(|e| e.to_string())?;

    for proc in win_proc_list.proc_list.iter() {
        println!("--------------------------------------------------");
        println!("ImageName: {}", proc.image_name);
        println!("PID: {}", proc.unique_process_id);
        println!("HandleCount: {}", proc.handle_count);
        println!("SessionId: {}", proc.session_id);
        println!("PeakVirtualSize: {}", proc.peak_virtual_size);
        println!("VirtualSize: {}", proc.virtual_size);
        println!("PeakWorkingSetSize: {}", proc.peak_working_set_size);
        println!("QuotaPagedPoolUsage: {}", proc.quota_paged_pool_usage);
        println!("QuotaNonPagedPoolUsage: {}", proc.quota_non_paged_pool_usage);
        println!("PagefileUsage: {}", proc.pagefile_usage);
        println!("PeakPagefileUsage: {}", proc.peak_pagefile_usage);
        println!("PrivatePageCount: {}", proc.private_page_count);
        println!("--------------------------------------------------");
    }
    Ok(())
}
