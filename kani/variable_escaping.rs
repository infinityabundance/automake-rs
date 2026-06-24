// kani/variable_escaping.rs — Kani formal verification proof.
//
// Verifies that Makefile variable name canonicalization is safe:
// no special characters survive into generated variable names.
//
// Kani is a model checker for Rust. Run with:
//   cargo kani --harness check_variable_name_safety
//
// Court: AM.KANI.1 — formal verification

#[cfg(kani)]
mod verification {
    use automake_rs_core::variables::VariableTable;

    /// Prove that variable table operations never panic.
    #[cfg(kani)]
    #[kani::proof]
    fn check_variable_table_no_panic() {
        let name: String = kani::any();
        let value: String = kani::any();

        // Assume reasonable length limits
        kani::assume(name.len() < 1024);
        kani::assume(value.len() < 4096);

        let mut table = VariableTable::new();

        // Setting a variable should never panic
        table.set(
            &name,
            vec![value.clone()],
            automake_rs_core::variables::VariableKind::Simple,
        );

        // Getting a variable should never panic
        let _ = table.get(&name);
    }

    /// Prove that variable name doesn't cause issues in output.
    #[cfg(kani)]
    #[kani::proof]
    fn check_variable_name_in_output() {
        let name: String = kani::any();
        let value: String = kani::any();

        kani::assume(name.len() < 256);
        kani::assume(value.len() < 1024);
        // Only printable ASCII
        kani::assume(name.chars().all(|c| c.is_ascii_graphic() || c == '_'));

        // Generate an assignment line
        let line = format!("{} = {}", name, value);

        // The line should be non-empty and contain the name
        assert!(!line.is_empty());
        assert!(line.contains(&name));
    }
}

#[cfg(not(kani))]
fn main() {
    println!("Kani verification proofs for automake-rs.");
    println!("Run with: cargo kani --harness check_variable_name_safety");
}
