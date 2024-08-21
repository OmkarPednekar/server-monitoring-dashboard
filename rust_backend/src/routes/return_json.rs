use axum::Json;
use serde_json::{ json, Value };
use serde::{ Serialize };
use sysinfo::{ System, Components };
pub async fn return_some_json() -> Json<Value> {
    // #[derive(Serialize)]
    let mut sys = System::new_all();
    sys.refresh_all();
    let mut procs: Vec<(String, String, (u64, u64))> = Vec::new();
    for (pid, process) in sys.processes() {
        procs.push((
            pid.to_string(),
            process.name().to_string_lossy().to_string(),
            (process.disk_usage().total_written_bytes, process.disk_usage().total_read_bytes),
        ));
    }

    let system_name = System::name();

    let kernel_version = System::kernel_version();

    let os_version = System::os_version();

    let host_name = System::host_name();

    let components = Components::new_with_refreshed_list();
    for component in &components {
        dbg!("{} {}°C", component.label(), component.temperature());
    }
    let json =
        json!({
        "system_name":system_name,
        "kernel_version":kernel_version,
        "os_version":os_version,
        "host_name":host_name,
        "total_memory": sys.total_memory(),
        "used_memory": sys.used_memory(),
        "total_swap": sys.total_swap(),
        "used_swap": sys.used_swap(),
        "processes":procs,
    });
    dbg!(&components);
    Json(json)
}
