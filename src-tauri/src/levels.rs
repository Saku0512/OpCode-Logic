use serde::{Deserialize, Serialize};

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
            description: "Read bytes from stdin (syscall 0), write them to stdout (syscall 1)."
                .to_string(),
            test_cases: vec![
                (vec![123], vec![123]),
                (vec![0], vec![0]),
                (vec![-55], vec![-55]),
            ],
        },
        Level {
            id: "02_Addition".to_string(),
            name: "Addition".to_string(),
            description: "Read bytes, add 1 to each byte, write result.".to_string(),
            test_cases: vec![
                (vec![10, 20], vec![11, 21]),
                (vec![5, 5], vec![6, 6]),
                (vec![-1, 0], vec![0, 1]),
            ],
        },
        Level {
            id: "03_Subtraction".to_string(),
            name: "Subtraction".to_string(),
            description: "Read bytes, subtract 1 from each byte, write result.".to_string(),
            test_cases: vec![
                (vec![10, 20], vec![9, 19]),
                (vec![0], vec![-1]),
                (vec![-128], vec![127]),
            ],
        },
        Level {
            id: "04_TheXORTrick".to_string(),
            name: "The XOR Trick".to_string(),
            description: "Read bytes, XOR each byte with 0x20, write result.".to_string(),
            test_cases: vec![
                (vec![('a' as i64)], vec![('A' as i64)]),
                (vec![('Z' as i64)], vec![('z' as i64)]),
                (vec![0], vec![0x20]),
            ],
        },
        Level {
            id: "05_Inc&Dec".to_string(),
            name: "Inc & Dec".to_string(),
            description: "Read bytes. Inc even-indexed bytes, dec odd-indexed bytes, write result."
                .to_string(),
            test_cases: vec![
                (vec![10, 10, 10, 10], vec![11, 9, 11, 9]),
                (vec![0], vec![1]),
            ],
        },
        Level {
            id: "06_Unconditional".to_string(),
            name: "Unconditional".to_string(),
            description: "Same as stage 01, but use JMP to structure the control flow.".to_string(),
            test_cases: vec![
                (vec![1, 2, 3], vec![1, 2, 3]),
                (vec![-55], vec![-55]),
            ],
        },
        Level {
            id: "07_ZeroFlag".to_string(),
            name: "Zero Flag".to_string(),
            description: "Read bytes, replace 0x00 with 0x20 (space), write result.".to_string(),
            test_cases: vec![
                (vec![0, 1, 0], vec![0x20, 1, 0x20]),
                (vec![5], vec![5]),
            ],
        },
        Level {
            id: "08_SignFlag".to_string(),
            name: "Sign Flag".to_string(),
            description: "Read bytes, output '+' if first byte is non-negative, otherwise '-'.".to_string(),
            test_cases: vec![
                (vec![0], vec![('+' as i64)]),
                (vec![-1], vec![('-' as i64)]),
            ],
        },
        Level {
            id: "09_Comparison".to_string(),
            name: "Comparison".to_string(),
            description:
                "Read bytes, compare consecutive bytes, output '+', '=', '-' markers (length N-1)."
                    .to_string(),
            test_cases: vec![
                (
                    vec![1, 2, 2, 1],
                    vec![('+' as i64), ('=' as i64), ('-' as i64)],
                ),
                (vec![10], vec![]),
            ],
        },
        Level {
            id: "10_Countdown".to_string(),
            name: "Countdown".to_string(),
            description: "Read one ASCII digit, output a countdown from it to '0'.".to_string(),
            test_cases: vec![
                (vec![('3' as i64)], vec![('3' as i64), ('2' as i64), ('1' as i64), ('0' as i64)]),
                (vec![('0' as i64)], vec![('0' as i64)]),
            ],
        },
        Level {
            id: "11_Accumulate3".to_string(),
            name: "Accumulate 3".to_string(),
            description: "Read bytes, sum the first 3 bytes (u8), output 1 byte result.".to_string(),
            test_cases: vec![
                (vec![1, 2, 3], vec![6]),
                (vec![10, 10, 10], vec![30]),
            ],
        },
        Level {
            id: "12_TheAccumulator".to_string(),
            name: "The Accumulator (BOSS)".to_string(),
            description:
                "Read bytes and transform: A-Z -> a-z, 0-9 -> increment (wrap 9->0), others unchanged."
                    .to_string(),
            test_cases: vec![
                (
                    vec![('A' as i64), ('z' as i64), ('9' as i64), ('!' as i64)],
                    vec![('a' as i64), ('z' as i64), ('0' as i64), ('!' as i64)],
                ),
                (vec![('B' as i64), ('0' as i64)], vec![('b' as i64), ('1' as i64)]),
            ],
        },
    ]
}

pub fn get_level(id: &str) -> Option<Level> {
    get_levels().into_iter().find(|l| l.id == id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_levels() {
        let levels = get_levels();
        assert!(!levels.is_empty());
        assert!(levels.len() >= 12);
    }

    #[test]
    fn test_get_level_by_id() {
        assert!(get_level("01_Mov&Call").is_some());
        assert!(get_level("02_Addition").is_some());
        assert!(get_level("12_TheAccumulator").is_some());
        assert!(get_level("nonexistent").is_none());
    }

    #[test]
    fn test_level_01_mov_call() {
        let level = get_level("01_Mov&Call").unwrap();
        assert_eq!(level.id, "01_Mov&Call");
        assert_eq!(level.test_cases.len(), 3);
        assert_eq!(level.test_cases[0], (vec![123], vec![123]));
    }

    #[test]
    fn test_level_02_addition() {
        let level = get_level("02_Addition").unwrap();
        assert_eq!(level.test_cases.len(), 3);
        assert_eq!(level.test_cases[0], (vec![10, 20], vec![11, 21]));
    }

    #[test]
    fn test_level_05_inc_dec() {
        let level = get_level("05_Inc&Dec").unwrap();
        assert_eq!(
            level.test_cases[0],
            (vec![10, 10, 10, 10], vec![11, 9, 11, 9])
        );
    }

    #[test]
    fn test_all_levels_have_test_cases() {
        let levels = get_levels();
        for level in levels {
            assert!(
                !level.test_cases.is_empty(),
                "Level {} has no test cases",
                level.id
            );
        }
    }
}
