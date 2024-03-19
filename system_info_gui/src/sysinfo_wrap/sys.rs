use sysinfo::System;

use super::{cpu::Cpu, memory::Memory};

const UNKONWN:&str="Unkonwn";

pub struct SysInfo{
    system:System,
}

impl SysInfo {
    pub fn new()->Self{
        let mut system = System::new_all();

        system.refresh_all();
        Self { 
            system:system
        }
    }


}


impl SysInfo {
    pub fn get_name(&self)->String{
        System::name().unwrap_or(UNKONWN.to_string())
    }

    pub fn get_kernel_version(&self)->String{
        System::kernel_version().unwrap_or(UNKONWN.to_string())
    }
    
    pub fn get_os_version(&self)->String{
        System::os_version().unwrap_or(UNKONWN.to_string())
    }

    pub fn get_host_name(&self)->String{
        System::host_name().unwrap_or(UNKONWN.to_string())
    }
}

impl Memory for SysInfo {
    fn get_total_memory(&self)->u64 {
        self.system.total_memory()
    }

    fn get_used_memory(&self)->u64 {
        self.system.used_memory()
    }
}

impl Cpu for SysInfo {
    fn refresh_cpu(&mut self) {
        self.system.refresh_cpu();
    }

    fn get_cpu_usage(&self)->Vec<f32> {
        self.system.cpus().into_iter().map(|cpu| cpu.cpu_usage()).collect()
    }
}