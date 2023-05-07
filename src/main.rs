use reqwest;
use std::thread;
use std::time::Duration;
use sysinfo::{ProcessExt, System, SystemExt};

struct ProcessItem {
    name: String,
    cpu: f32,
    memory: u64,
}

fn slack_alert() {
    let alert = "```test are this block code?```";
    let res = reqwest::blocking::Client::new()
        .post("__SLACK_WEBHOOK_URL__")
        .body("{\"text\":\"".to_owned() + alert + "\"}")
        .send()
        .unwrap();

    if res.status().is_success() {
        println!("notification success");
    }
}

fn get_top_process(sys: &mut System, cpus: usize, top_process: &mut Vec<ProcessItem>) {
    sys.refresh_all();

    for (_, process) in sys.processes() {
        let name = process.name();
        let cpu = process.cpu_usage() / cpus as f32;
        let memory = process.memory();

        if cpu < 0.1 {
            continue;
        }
        top_process.push(ProcessItem {
            name: name.to_string(),
            cpu,
            memory,
        });
    }

    top_process.sort_by(|a, b| b.cpu.partial_cmp(&a.cpu).unwrap());

    for val in top_process.iter() {
        println!(
            "{}\nCPU    : {:.2}%\nMemory : {}Bytes",
            val.name,
            val.cpu,
            val.memory
        );
        println!("------------------------------------");
    }
}

fn main() {
    let mut sys = System::new_all();
    sys.refresh_cpu();
    let cpus = sys.cpus().len();

    loop {
        let mut top_process: Vec<ProcessItem> = Vec::new();
        get_top_process(&mut sys, cpus, &mut top_process);
        println!("####################################");
        thread::sleep(Duration::from_secs(60));
    }
}
