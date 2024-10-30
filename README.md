# Aurora Kernel
Aurora is a rust-based Kernel for the Cosmix Operating System - but can be used for any operating system - with the request that credits is given to the Cosmix team.

# What can it do?
The Aurora Kernel has support for:
Hardware Interrupts (eg, keyboard inputs)
Displaying text to the screen
Basic Functions of the Rust Standard Library (eg, `println!`)
Serial output when running on Qemu (using `cargo test`)
Exception handling (Eg, CPU Exceptions, Double Faults)
Paging
Heap Collection/Allocation
Async and Await

# What comes with it?
Literally nothing - just the kernel, no terminal or anything - that is all yours to make.

# Credits
https://os.phil-opp.com/
Most or all of the code in this kernel was written in reference to the 'Writing an OS in Rust' guide, without it, this would not be possible

The Contributors - in contrib.txt

# How do I run it/edit it?
It can be edited like any other rust project, with a main.rs, lib.rs, etc etc in /src/
To run it, simply just git clone the folder and open it in terminal. Then, type `cargo run` (requires Rust Nightly from after 2020, and various components)
