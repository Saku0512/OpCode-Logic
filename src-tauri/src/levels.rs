use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Level {
    pub id: String,
    pub name: String,
    pub description: String,
    // (Input Registers RDI, RSI...) -> (Expected Output RAX or Stream)
    // If output vector has >1 element, it expects STREAM output (via OUT).
    // If 1 element, it expects RAX return. 
    // Wait, simpler: Both check logic.
    // If level expects stream, checking "OutputQueue".
    // If level expects return, checking "RAX".
    // We can infer mode from expected data, or just check both.
    pub test_cases: Vec<(Vec<i64>, Vec<i64>)>, 
}

pub fn get_levels() -> Vec<Level> {
    vec![
        Level {
            id: "01_Mov&Call".to_string(),
            name: "Mov & Call".to_string(),
            description: "Read input (RDI), and return it (RAX).".to_string(),
            test_cases: vec![
                (vec![123], vec![123]),
                (vec![0], vec![0]),
                (vec![-55], vec![-55]),
            ],
        },
        Level {
            id: "02_Addition".to_string(),
            name: "Addition".to_string(),
            description: "Read A (RDI) and B (RSI), return A + B (RAX).".to_string(),
            test_cases: vec![
                (vec![10, 20], vec![30]),
                (vec![5, 5], vec![10]),
                (vec![-10, 20], vec![10]),
            ],
        },
        Level {
            id: "03_Subtraction".to_string(),
            name: "Subtraction".to_string(),
            description: "Read A (RDI) and B (RSI), return A - B (RAX).".to_string(),
            test_cases: vec![
                (vec![30, 10], vec![20]),
                (vec![10, 20], vec![-10]),
                (vec![0, 0], vec![0]),
            ],
        },
        Level {
            id: "04_TheXORTrick".to_string(),
            name: "The XOR Trick".to_string(),
            description: "Return 0 (RAX). Try using XOR.".to_string(),
            test_cases: vec![
                (vec![123], vec![0]),
                (vec![999], vec![0]),
            ],
        },
        Level {
            id: "05_Inc&Dec".to_string(),
             name: "Inc & Dec".to_string(),
            description: "Read input A (RDI). Output A+1 and A-1 (Stream Output via OUT).".to_string(),
            test_cases: vec![
                (vec![10], vec![11, 9]),
                (vec![0], vec![1, -1]),
            ],
        },
        Level {
            id: "06_Unconditional".to_string(),
            name: "Unconditional".to_string(),
            description: "Output 1 infinitely (Stream). (Test checks for first 5 outputs of 1).".to_string(),
            test_cases: vec![
                (vec![0], vec![1, 1, 1, 1, 1]), 
            ],
        },
         Level {
            id: "07_ZeroFlag".to_string(),
            name: "Zero Flag".to_string(),
            description: "Read A (RDI). If 0, return 1. Else return 0 (RAX).".to_string(),
            test_cases: vec![
                (vec![0], vec![1]),
                (vec![5], vec![0]),
                (vec![-1], vec![0]),
            ],
        },
         Level {
            id: "08_SignFlag".to_string(),
            name: "Sign Flag".to_string(),
            description: "Read A (RDI). If A < 0, return 1. Else return 0 (RAX).".to_string(),
            test_cases: vec![
                (vec![-5], vec![1]),
                (vec![0], vec![0]),
                (vec![10], vec![0]),
            ],
        },
         Level {
            id: "09_Comparison".to_string(),
            name: "Comparison".to_string(),
            description: "Read A (RDI), B (RSI). Return the larger value (RAX).".to_string(),
            test_cases: vec![
                (vec![10, 20], vec![20]),
                (vec![50, 40], vec![50]),
                (vec![30, 30], vec![30]),
            ],
        },
         Level {
            id: "10_Countdown".to_string(),
            name: "Countdown".to_string(),
             description: "Read N (RDI). Output N, N-1... 1 (Stream).".to_string(),
            test_cases: vec![
                (vec![3], vec![3, 2, 1]),
                (vec![5], vec![5, 4, 3, 2, 1]),
            ],
        },
         Level {
            id: "11_Accumulate3".to_string(),
            name: "Accumulate 3".to_string(),
            description: "Read 3 inputs (RDI, RSI, RDX). Return sum (RAX).".to_string(),
             test_cases: vec![
                (vec![1, 2, 3], vec![6]),
                (vec![10, 10, 10], vec![30]),
            ],
        },
        Level {
            id: "12_TheAccumulator".to_string(),
            name: "The Accumulator (BOSS)".to_string(),
            description: "Read inputs using 'IN' until 0 is encountered. Return the total sum in RAX.".to_string(),
            test_cases: vec![
                (vec![1, 2, 3, 0], vec![6]),
                (vec![10, -5, 20, 0], vec![25]),
                (vec![0], vec![0]),
            ],
        },
    ]
}

pub fn get_level(id: &str) -> Option<Level> {
    get_levels().into_iter().find(|l| l.id == id)
}
