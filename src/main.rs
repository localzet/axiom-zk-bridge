use anyhow::{bail, Context, Result};
use sha2::{Digest, Sha256};
use std::{env, fs};
fn main() -> Result<()> {
    let mut a = env::args().skip(1);
    if a.next().as_deref() != Some("commit") {
        bail!("usage: axiom-zk-bridge commit --program P --input TEXT --output TEXT");
    }
    let p = arg(&mut a, "--program")?;
    let i = arg(&mut a, "--input")?;
    let o = arg(&mut a, "--output")?;
    let program = fs::read(p)?;
    println!("AXIOM-ZK-ENVELOPE/1");
    println!("program.sha256={}", hex(&hash(&program)));
    println!("input.sha256={}", hex(&hash(i.as_bytes())));
    println!("output.sha256={}", hex(&hash(o.as_bytes())));
    println!("backend=UNBOUND");
    println!("proof=ABSENT");
    Ok(())
}
fn arg(a: &mut impl Iterator<Item = String>, n: &str) -> Result<String> {
    let f = a.next().context("missing flag")?;
    if f != n {
        bail!("expected {n}");
    }
    a.next().context("missing value")
}
fn hash(b: &[u8]) -> [u8; 32] {
    Sha256::digest(b).into()
}
fn hex(h: &[u8; 32]) -> String {
    h.iter().map(|b| format!("{b:02x}")).collect()
}
