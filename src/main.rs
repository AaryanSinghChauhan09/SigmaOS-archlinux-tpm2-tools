//! SigmaOS-archlinux-tpm2-tools — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_archlinux_tpm2_tools_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-archlinux-tpm2-tools: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-archlinux-tpm2-tools: ok");
}
