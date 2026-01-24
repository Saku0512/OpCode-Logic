pub mod levels;
pub mod vm;
pub mod x86_asm;
pub mod x86_runtime;

use serde::Serialize;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize)]
struct SimulationResult {
    vm_state: vm::VmState,
    success: bool,
    message: String,
    execution_log: Vec<String>, // 実行ログを追加
}

#[tauri::command]
fn get_levels() -> Vec<levels::Level> {
    levels::get_levels()
}

fn level_dir_for_id(level_id: &str) -> Option<&'static str> {
    match level_id {
        "01_Mov&Call" => Some("stages/1.The-Accumulator/Phase1-Registers&ALU/01_Mov&Call"),
        "02_Addition" => Some("stages/1.The-Accumulator/Phase1-Registers&ALU/02_Addition"),
        "03_Subtraction" => Some("stages/1.The-Accumulator/Phase1-Registers&ALU/03_Subtraction"),
        "04_TheXORTrick" => Some("stages/1.The-Accumulator/Phase1-Registers&ALU/04_TheXORTrick"),
        "05_Inc&Dec" => Some("stages/1.The-Accumulator/Phase1-Registers&ALU/05_Inc&Dec"),
        "06_Unconditional" => Some("stages/1.The-Accumulator/Phase2-Flags&Jumps/06_Unconditional"),
        "07_ZeroFlag" => Some("stages/1.The-Accumulator/Phase2-Flags&Jumps/07_ZeroFlag"),
        "08_SignFlag" => Some("stages/1.The-Accumulator/Phase2-Flags&Jumps/08_SignFlag"),
        "09_Comparison" => Some("stages/1.The-Accumulator/Phase2-Flags&Jumps/09_Comparison"),
        "10_Countdown" => Some("stages/1.The-Accumulator/Phase3-LoopStructures/10_Countdown"),
        "11_Accumulate3" => Some("stages/1.The-Accumulator/Phase3-LoopStructures/11_Accumulate3"),
        "12_TheAccumulator" => Some("stages/1.The-Accumulator/BOSS/12_TheAccumulator"),
        // GRAND STAGE 02: The Stack
        "13_Push&Pop" => Some("stages/2.The-Stack/Phase1-StackBasics/13_Push&Pop"),
        "14_SwapTwo" => Some("stages/2.The-Stack/Phase1-StackBasics/14_SwapTwo"),
        "15_Duplicate" => Some("stages/2.The-Stack/Phase1-StackBasics/15_Duplicate"),
        "16_Reverse3" => Some("stages/2.The-Stack/Phase2-StackAsBuffer/16_Reverse3"),
        "17_ReverseUntil0" => Some("stages/2.The-Stack/Phase2-StackAsBuffer/17_ReverseUntil0"),
        "18_SumFromStack" => Some("stages/2.The-Stack/Phase2-StackAsBuffer/18_SumFromStack"),
        "19_SafePop" => Some("stages/2.The-Stack/Phase3-StackAlgorithms/19_SafePop"),
        "20_RPN_AddOnly" => Some("stages/2.The-Stack/Phase3-StackAlgorithms/20_RPN_AddOnly"),
        "21_Sort3" => Some("stages/2.The-Stack/Phase3-StackAlgorithms/21_Sort3"),
        "22_Rotate3" => Some("stages/2.The-Stack/Phase3-StackAlgorithms/22_Rotate3"),
        "23_MinMaxFromStack" => {
            Some("stages/2.The-Stack/Phase3-StackAlgorithms/23_MinMaxFromStack")
        }
        "24_TheStackMachine" => Some("stages/2.The-Stack/BOSS/24_TheStackMachine"),
        _ => None,
    }
}

fn read_stage_file(rel_path: &str) -> Result<String, String> {
    // プロジェクトルートからの相対パスで読み込む
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop(); // src-tauri から出る
    path.push(rel_path);
    fs::read_to_string(&path).map_err(|e| format!("Failed to read file: {} (path: {:?})", e, path))
}

#[tauri::command]
fn get_level_explanation(level_id: String) -> Result<String, String> {
    let dir = level_dir_for_id(level_id.as_str())
        .ok_or_else(|| format!("Stage files not found for level: {}", level_id))?;
    let md = format!("{}/{}.md", dir, level_id);
    read_stage_file(&md)
}

#[tauri::command]
fn get_level_ini(level_id: String) -> Result<String, String> {
    let dir = level_dir_for_id(level_id.as_str())
        .ok_or_else(|| format!("Stage files not found for level: {}", level_id))?;
    let p = format!("{}/ini.asm", dir);
    read_stage_file(&p)
}

#[tauri::command]
fn get_level_collect(level_id: String) -> Result<String, String> {
    let dir = level_dir_for_id(level_id.as_str())
        .ok_or_else(|| format!("Stage files not found for level: {}", level_id))?;
    let p = format!("{}/collect.asm", dir);
    read_stage_file(&p)
}

#[tauri::command]
fn run_simulation(
    code: &str,
    syntax: String,
    input: Vec<i64>,
    level_id: Option<String>,
) -> Result<SimulationResult, String> {
    let syntax_enum = match syntax.as_str() {
        "Intel" => vm::Syntax::Intel,
        "Att" => vm::Syntax::Att,
        _ => return Err("Invalid syntax type".to_string()),
    };

    // If level_id is provided, verify against ALL test cases
    if let Some(lid) = level_id {
        if let Some(level) = levels::get_level(&lid) {
            for (idx, (test_in, expected)) in level.test_cases.iter().enumerate() {
                println!("\n=== TEST CASE #{} ===", idx + 1);
                println!("Input: {:?}", test_in);
                println!("Expected: {:?}", expected);
                let run =
                    x86_runtime::run_x86_64(code, syntax_enum.clone(), test_in.clone(), 20_000)?;

                // Validation
                let state = run.state;
                let execution_log = run.execution_log;
                let rax = *state.registers.get(&vm::Register::RAX).unwrap_or(&0);

                println!("Final RAX: {}", rax);
                println!("Final Output: {:?}", state.output);
                println!("Expected: {:?}", expected);

                let output_correct = if !state.output.is_empty() {
                    // Prioritize Stream check if output was produced
                    state.output.len() >= expected.len()
                        && &state.output[0..expected.len()] == expected.as_slice()
                } else if expected.len() == 1 {
                    // Fallback to RAX if no stream output was produced but we expect 1 value
                    // BUT: if we exited via sys_exit (60), RAX is 60 (or status),
                    // so we should be careful if we are not expecting 60.
                    if state.exited && rax == 60 && expected[0] != 60 {
                        false
                    } else {
                        rax == expected[0]
                    }
                } else {
                    false
                };

                if !output_correct {
                    let message = format!("Failed Test Case #{}: Input {:?} -> Expected {:?}, Got (RAX={}, Stream={:?})", idx+1, test_in, expected, rax, state.output);
                    println!("TEST FAILED: {}", message);
                    return Ok(SimulationResult {
                        vm_state: state,
                        success: false,
                        message,
                        execution_log,
                    });
                } else {
                    println!("TEST PASSED");
                }
            }
        }
    }

    // Single Run (Visualization)
    println!("\n=== SINGLE RUN (Visualization) ===");
    println!("Input: {:?}", input);
    let run = x86_runtime::run_x86_64(code, syntax_enum.clone(), input, 50_000)?;
    let state = run.state;
    let execution_log = run.execution_log;
    let rax = *state.registers.get(&vm::Register::RAX).unwrap_or(&0);
    println!("Final RAX: {}", rax);
    println!("Final Output: {:?}", state.output);

    Ok(SimulationResult {
        vm_state: state,
        success: true,
        message: "Simulation Complete".to_string(),
        execution_log,
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            run_simulation,
            get_levels,
            get_level_explanation,
            get_level_ini,
            get_level_collect
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
