# Rust: Raw Pointer Vector Modification Bug
This repository demonstrates a common error in Rust: modifying a vector through a raw pointer after modifying the vector's length or capacity. Modifying the vector after obtaining the raw pointer leads to undefined behavior.

The `bug.rs` file contains the buggy code.  The `bugSolution.rs` shows the corrected approach that avoids unsafe operations.

**How to reproduce**:
1. Clone this repository.
2. Navigate to the repository directory.
3. Run `rustc bug.rs && ./bug` (this may crash or produce unexpected output).
4. Run `rustc bugSolution.rs && ./bugSolution` (this will run without problems).