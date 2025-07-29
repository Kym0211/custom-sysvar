use solana_program::sysvar::greet::Greet;

pub fn process_instruction_with_greet() -> u64 {
    let greet = Greet::default();
    println!("{}", greet.greeting);
    0
}

#[test]
fn test_process_instruction_with_greet() {
    // let greet = Greet { greeting: "Gm Gm".to_string() };
    let output = process_instruction_with_greet();
    assert_eq!(output, 0);
}