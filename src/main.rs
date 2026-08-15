use anyhow::{bail, Context, Result};
use sha2::{Digest, Sha256};
use std::{env, fs};

fn main() -> Result<()> {
    let mut args = env::args().skip(1);
    if args.next().as_deref() != Some("envelope") {
        bail!("usage: axiom-zk-bridge envelope --program P --semantic-proof R --input TEXT --output TEXT");
    }
    let program = fs::read(arg(&mut args, "--program")?)?;
    let semantic_proof = fs::read(arg(&mut args, "--semantic-proof")?)?;
    let input = arg(&mut args, "--input")?;
    let output = arg(&mut args, "--output")?;

    println!("AXIOM-DUAL-PROOF/1");
    println!("program.sha256={}", sha256_hex(&program));
    println!("semantic-proof.sha256={}", sha256_hex(&semantic_proof));
    println!("input.commitment={}", sha256_hex(input.as_bytes()));
    println!("output.commitment={}", sha256_hex(output.as_bytes()));
    println!("execution.backend=UNBOUND");
    println!("execution.receipt=ABSENT");
    println!("status=SEMANTIC_PROOF_BOUND_EXECUTION_PENDING");
    Ok(())
}

fn arg(args: &mut impl Iterator<Item = String>, expected: &str) -> Result<String> {
    let flag = args.next().context("missing flag")?;
    if flag != expected {
        bail!("expected {expected}");
    }
    args.next().context("missing value")
}

fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
