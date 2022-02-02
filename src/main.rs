use std::time::SystemTime;

fn main() {
    let iterations = 1_000_000_000;

    println!("Approximating PI using {} iterations...", iterations);

    let c_start = SystemTime::now();
    let pi_c = unsafe { pi_c(iterations) };
    let c_end = c_start.elapsed().unwrap();

    println!("--- C ---\nstart: {}\nduration: {}\nresult: {}\n",
        humantime::format_rfc3339(c_start),
        humantime::format_duration(c_end),
        pi_c);

    let asm_start = SystemTime::now();
    let pi_asm = unsafe { pi_asm(iterations) };
    let asm_end = asm_start.elapsed().unwrap();

    println!("--- x86_64 ---\nstart: {}\nduration: {}\nresult: {}\n",
        humantime::format_rfc3339(asm_start),
        humantime::format_duration(asm_end),
        pi_asm);

    let rust_start = SystemTime::now();
    let pi_rust = pi_rust(iterations);
    let rust_end = rust_start.elapsed().unwrap();

    println!("--- Rust ---\nstart: {}\nduration: {}\nresult: {}\n",
        humantime::format_rfc3339(rust_start),
        humantime::format_duration(rust_end),
        pi_rust);
}

fn pi_rust(iterations: u64) -> f64 {
    let mut pi: f64 = 3.0;
    let mut i: f64 = 2.0;
    
    for _ in 0..iterations {
        pi += 4.0 / (i * (i + 1.0) * (i + 2.0));
        i += 2.0;
        pi -= 4.0 / (i * (i + 1.0) * (i + 2.0));
        i += 2.0;
    }

    pi
}

#[link(name="pi_c")]
extern "C" {
    fn pi_c(iterations: cty::uint64_t) -> cty::c_double;
}
#[link(name="pi_asm")]
extern "C" {
    fn pi_asm(iterations: cty::uint64_t) -> cty::c_double;
}
