# Practical class to learn the Rust basics

This repo contains two labs made by Samuel Tardieu to learn the Rust programming language :

* `tp-rust-VM/` is a project where we created a simple Virtual Machine in Rust. This VM has **16 virtual registers** and a **virtual memory of 4096 bytes**. It is able to perform some basics operations such as `move_if`, `store`, `load`, `loadimm` and `sub`. By combining these simple operations, it is possible to process more complexe instructions such as `push`, `pop`, `mult`, `fact` and others.

    * The VM code is located in `src/machine.rs`
    * Test files are located in `tests/`.<br> 
    `tests/assignment.rs` is the easiest test file,<br> 
    `tests/basic_operations.rs` is more challenging,<br> 
    `tests/complex_execution.rs` is the most difficult test file.<br> 
    To be 100% functional, the VM must pass all the tests.
    
    Below are the commands to test the VM :
    ```shell
    $ cargo build
    $ cargo test --test assignment
    $ cargo test --test basic_operations
    $ cargo test --test complex_execution
    ```

