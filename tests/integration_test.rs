//! Integration tests for SigmaOS-archlinux-tpm2-tools.

use sigmaos_archlinux_tpm2_tools_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
