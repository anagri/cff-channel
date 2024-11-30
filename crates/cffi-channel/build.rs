use std::process::Command;

fn main() {
  // Get the manifest directory (where Cargo.toml lives)
  let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("Failed to get CARGO_MANIFEST_DIR");
  // Tell cargo to watch the csrc directory for changes
  println!("cargo:rerun-if-changed=csrc");

  // Determine the make command based on platform
  let cmd = format!("make -C '{}' -f Makefile.win.mk ci.build", manifest_dir);
  let (make_cmd, make_args) = if cfg!(windows) {
    (
      "pwsh",
      vec!["-NoProfile", "-NonInteractive", "-Command", &cmd],
    )
  } else {
    ("make", vec!["-C", &manifest_dir, "-f", "Makefile", "ci.build"])
  };

  // Run the make command
  eprintln!("Running command: {}", cmd);
  let status = Command::new(make_cmd)
    .args(&make_args)
    .status()
    .expect("Failed to execute make command");

  if !status.success() {
    panic!("Make command failed with status: {}", status);
  }
}
