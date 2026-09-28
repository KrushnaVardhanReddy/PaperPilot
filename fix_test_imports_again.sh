sed -i '/mod tests {/a \    use tempfile::tempdir;' paperpilot-cli/src/commands/group_b.rs
sed -i '/mod tests {/a \    use tempfile::tempdir;\n    use std::sync::{Arc, Mutex};' paperpilot-cli/src/commands/group_c.rs
