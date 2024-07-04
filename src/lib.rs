use std::{collections::HashMap, error::Error};
use sysinfo::{System,Pid};

#[derive(Clone)]
pub struct PidTreeNode {
    pid:Pid,
    children:Vec<PidTreeNode>,
    level:u8
}

impl PidTreeNode{

    pub fn new(pid:Pid) -> Self{
        PidTreeNode{pid,children:vec![],level:0}
    }
    pub fn add_child(&mut self,child:PidTreeNode){
        self.children.push(child.clone())
    }
    pub fn increment_level(&mut self){
        self.level=self.level+1;
    }

    pub fn get_processes_name(&self,sys:&mut System) -> String{
        if let Some(process) = sys.process(self.pid) {
            return process.name().to_string();
        } 
        "".to_string()
    }
}

pub fn get_processes() -> Result<Vec<i32>,Box<dyn Error>>{
    let mut sys = System::new_all();
    sys.refresh_all();
    println!("=> system:");
    // RAM and swap information:
    println!("total memory: {} bytes  ", sys.total_memory());
    println!("used memory : {} bytes  ", sys.used_memory());
    println!("total swap  : {} bytes  ", sys.total_swap());
    println!("used swap   : {} bytes\n", sys.used_swap());

    // Display system information:
    println!("System name:             {:?}  ", System::name());
    println!("System kernel version:   {:?}  ", System::kernel_version());
    println!("System OS version:       {:?}  ", System::os_version());
    println!("System host name:        {:?}\n", System::host_name());

    // Number of CPUs:
    println!("NB CPUs: {}\n", sys.cpus().len());

    let mut node_map:HashMap<Pid,PidTreeNode> = HashMap::new();
    let mut pids: Vec<Pid> = sys.processes().keys().cloned().collect();
    let mut roots: Vec<Pid> = Vec::new();

    /// should return this...
    pids.sort();

    /// and move this to the main program...
    /// or make it so that I can traverse the roots vector...
    /// maybe add the process name to the PidTreeNode.
    for pid in pids {
        node_map.insert(pid, PidTreeNode::new(pid ));
        if let Some(process) = sys.process(pid) {
            if let Some(ppid) = process.parent() {
                if let Some(parent_node) = node_map.get_mut(&ppid){
                    let mut node = PidTreeNode::new(pid);
                    node.increment_level();
                    let proc_name = node.get_processes_name(&mut sys);                    
                    let level = node.level as usize;
                    parent_node.add_child(node);
                    println!("{}\u{251c} {pid} {}","  ".repeat(level),proc_name);
                }
            } else {
                let node = PidTreeNode::new(pid);
                let proc_name = node.get_processes_name(&mut sys);  
                println!("\u{2500} {pid} {proc_name}");
                roots.push(pid);
            }
        }
    }

    Ok(vec![1,2,3])
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proc_info(){
        let x = get_processes();
        match x {
            Ok(v) => println!("There are {:?} processes running",v.len()),
            Err(e) => eprintln!("error: {}",e)
        }
    }
}