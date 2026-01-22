pub mod vm;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn run_simulation(code: &str, syntax: String, input: Vec<i64>) -> Result<vm::VmState, String> {
    let syntax_enum = match syntax.as_str() {
        "Intel" => vm::Syntax::Intel,
        "Att" => vm::Syntax::Att,
        _ => return Err("Invalid syntax type".to_string()),
    };

    let program = vm::parse_program(code, syntax_enum)?;
    let mut vm = vm::VM::new(program);
    
    // ABI: Input 1 -> RDI, Input 2 -> RSI
    if input.len() > 0 { vm.set_register(vm::Register::RDI, input[0]); }
    if input.len() > 1 { vm.set_register(vm::Register::RSI, input[1]); }
    if input.len() > 2 { vm.set_register(vm::Register::RDX, input[2]); }
    
    // Run until finished or max 1000 steps
    let mut steps = 0;
    while vm.step() {
        steps += 1;
        if steps > 1000 {
            return Err("Time Limit Exceeded".to_string());
        }
    }
    
    Ok(vm.get_state())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
    .plugin(tauri_plugin_opener::init())
    .invoke_handler(tauri::generate_handler![greet, run_simulation])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
