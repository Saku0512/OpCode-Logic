pub mod vm;
pub mod levels;

use serde::{Serialize};

#[derive(Serialize)]
struct SimulationResult {
    vm_state: vm::VmState,
    success: bool,
    message: String,
}

#[tauri::command]
fn get_levels() -> Vec<levels::Level> {
    levels::get_levels()
}

#[tauri::command]
fn run_simulation(code: &str, syntax: String, input: Vec<i64>, level_id: Option<String>) -> Result<SimulationResult, String> {
    let syntax_enum = match syntax.as_str() {
        "Intel" => vm::Syntax::Intel,
        "Att" => vm::Syntax::Att,
        _ => return Err("Invalid syntax type".to_string()),
    };

    let (program, labels) = vm::parse_program(code, syntax_enum.clone())?;
    
    // If level_id is provided, verify against ALL test cases
    if let Some(lid) = level_id {
        if let Some(level) = levels::get_level(&lid) {
            for (idx, (test_in, expected)) in level.test_cases.iter().enumerate() {
                let mut vm = vm::VM::new(program.clone(), labels.clone(), test_in.clone());
                
                let mut steps = 0;
                while vm.step() {
                    steps += 1;
                    if steps > 5000 { break; } // infinite loop protection

                    if !expected.is_empty() && vm.get_state().output.len() >= expected.len() {
                        // For stream levels, we can stop early if we have enough output
                        // But for RAX levels, we might need to continue until RET.
                        // Actually, if it's a stream level (length > 1 or specific id), we stop.
                        if expected.len() > 1 || level.id == "06_Unconditional" {
                            break;
                        }
                    }
                }

                // Validation
                let state = vm.get_state();
                let output_correct = if expected.len() > 1 || level.id == "06_Unconditional" {
                    // Expecting Stream
                     state.output.len() >= expected.len() && &state.output[0..expected.len()] == expected.as_slice()
                } else {
                    // Expecting RAX Return
                    let rax = vm.get_register(vm::Register::RAX);
                    rax == expected[0]
                };

                if !output_correct {
                    let message = format!("Failed Test Case #{}: Input {:?} -> Expected {:?}, Got (RAX={}, Stream={:?})", idx+1, test_in, expected, vm.get_register(vm::Register::RAX), state.output);
                    return Ok(SimulationResult {
                        vm_state: state,
                        success: false,
                        message,
                    });
                }
            }
        }
    }

    // Single Run (Visualization)
    let mut vm = vm::VM::new(program, labels, input);
    
    let mut steps = 0;
    while vm.step() {
        steps += 1;
        if steps > 2000 { return Err("Time Limit Exceeded (Visualization)".to_string()); }
    }
    
    Ok(SimulationResult {
        vm_state: vm.get_state(),
        success: true,
        message: "Simulation Complete".to_string(),
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
    .plugin(tauri_plugin_opener::init())
    .invoke_handler(tauri::generate_handler![run_simulation, get_levels])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
