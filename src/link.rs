use std::process::Command;

pub enum Linker {
    Clang,
    Gcc,
    Ld,
}

impl Linker {
    pub fn link_obj(&self, obj: &str, output: &str) {
        let status = Command::new(self.name()).args(&[obj, "-o", output]).status().expect("Failed to link");

        if status.success() {
            println!("Linked {} to {}", obj, output);
        } else {
            eprintln!("Linking failed with status: {}", status);
        }
    }

    fn name(&self) -> &str {
        match self {
            Linker::Clang => "clang",
            Linker::Gcc => "gcc",
            Linker::Ld => "ld",
        }
    }
}
