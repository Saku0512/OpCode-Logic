use opcode_logic_lib::levels;
use opcode_logic_lib::vm::Syntax;
use opcode_logic_lib::x86_runtime;

use std::fs;
use std::path::PathBuf;

fn print_usage_and_exit() -> ! {
    eprintln!(
        "Usage:\n  stage_runner --level-id <ID> --asm <path> [--syntax Intel|Att] [--max-instructions N]\n"
    );
    std::process::exit(2);
}

fn main() {
    let mut level_id: Option<String> = None;
    let mut asm_path: Option<PathBuf> = None;
    let mut syntax = Syntax::Intel;
    let mut max_instructions: usize = 50_000;

    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--level-id" => level_id = args.next(),
            "--asm" => asm_path = args.next().map(PathBuf::from),
            "--syntax" => {
                let s = args.next().unwrap_or_else(|| {
                    eprintln!("Missing value for --syntax");
                    print_usage_and_exit();
                });
                syntax = match s.as_str() {
                    "Intel" => Syntax::Intel,
                    "Att" => Syntax::Att,
                    other => {
                        eprintln!("Unknown syntax: {}", other);
                        print_usage_and_exit();
                    }
                };
            }
            "--max-instructions" => {
                let s = args.next().unwrap_or_else(|| {
                    eprintln!("Missing value for --max-instructions");
                    print_usage_and_exit();
                });
                max_instructions = s.parse().unwrap_or_else(|_| {
                    eprintln!("Invalid number for --max-instructions: {}", s);
                    print_usage_and_exit();
                });
            }
            "-h" | "--help" => print_usage_and_exit(),
            other => {
                eprintln!("Unknown arg: {}", other);
                print_usage_and_exit();
            }
        }
    }

    let level_id = level_id.unwrap_or_else(|| {
        eprintln!("Missing --level-id");
        print_usage_and_exit();
    });
    let asm_path = asm_path.unwrap_or_else(|| {
        eprintln!("Missing --asm");
        print_usage_and_exit();
    });

    let level = levels::get_level(&level_id).unwrap_or_else(|| {
        eprintln!("Unknown level id: {}", level_id);
        std::process::exit(2);
    });

    let code = fs::read_to_string(&asm_path).unwrap_or_else(|e| {
        eprintln!("Failed to read asm file {}: {}", asm_path.display(), e);
        std::process::exit(2);
    });

    let mut failures = 0usize;
    for (idx, (test_in, expected)) in level.test_cases.iter().enumerate() {
        let run =
            match x86_runtime::run_x86_64(&code, syntax.clone(), test_in.clone(), max_instructions)
            {
                Ok(r) => r,
                Err(e) => {
                    eprintln!(
                        "[{}] FAIL: runtime error for input {:?}: {}",
                        idx + 1,
                        test_in,
                        e
                    );
                    failures += 1;
                    continue;
                }
            };

        let state = run.state;
        if let Some(err) = state.error.clone() {
            eprintln!(
                "[{}] FAIL: vm error for input {:?}: {}",
                idx + 1,
                test_in,
                err
            );
            failures += 1;
            continue;
        }

        let got = if !state.output.is_empty() {
            state.output.clone()
        } else {
            // Mirror app behavior: treat RAX as output only if no stream output exists.
            let rax = *state
                .registers
                .get(&opcode_logic_lib::vm::Register::RAX)
                .unwrap_or(&0);
            if state.exited && rax == 60 && expected.len() == 1 && expected[0] != 60 {
                vec![]
            } else {
                vec![rax]
            }
        };

        let ok = if !got.is_empty() {
            got.len() >= expected.len() && &got[0..expected.len()] == expected.as_slice()
        } else {
            expected.is_empty()
        };

        if ok {
            eprintln!("[{}] PASS", idx + 1);
        } else {
            eprintln!(
                "[{}] FAIL: input {:?}\n  expected: {:?}\n  got:      {:?}",
                idx + 1,
                test_in,
                expected,
                got
            );
            failures += 1;
        }
    }

    if failures != 0 {
        eprintln!(
            "FAILED {} / {} test case(s).",
            failures,
            level.test_cases.len()
        );
        std::process::exit(1);
    }
}
