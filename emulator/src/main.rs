use emulator::vm::cpu::Cpu;

static PROGRAM: &str = "
    MOVE  0xBAD     R0
    MOVE  0xCAFE    R1
    ADD   R0 R1     R2
    STORE R2        0x2
";

fn main() {
    let mut vm = Cpu::default();
    println!("New CPU created");
    vm.load(PROGRAM);
    println!("CODE [{}] Loaded", PROGRAM);
    println!("CPU state:\n{}", vm);
    println!("Starting emulation...");
    vm.run(true);
}
