#![allow(non_snake_case)]

const CMD: [&str; 256] = [
    "EOS", // End Of Statement
    "VAR", // Creates a new variable in stack
    "USE", // Includes other LO files
    "OBJ", // Creates an Object Template in Heap
    "FUN", // Creates a Function Code in Heap
    "NEW", // Heap allocate
    "DEL", // Heap deallocate
    "OPN", // Opens file in heap
    "SAV", // Saves file from heap
    "PUT", // User Output
    "GET", // User Input
    "MOV", // Move variables
    "AND", // Bitwise AND
    "OOR", // Bitwise OR
    "NOT", // Bitwise NOT
    "XOR", // Bitwise XOR
    "LBS", // Left Bitshift
    "RBS", // Right Bitshift
]

fn assemble(lo: &str) -> Vec<u8> {
    let statements = lo.split('\n');
    let vm: Vec<u8> = vec![];
    
    vm
}

fn disassemble(vm: &[u8]) -> String {
    let statements = vm.split(|byte| byte==&0);
    let lo: String = String::from("");

    lo
}