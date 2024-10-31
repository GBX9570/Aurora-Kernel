# Aurora Kernel
Aurora is a rust-based Kernel for the Cosmix Operating System - but can be used for any operating system.

# Licensing
The Aurora Kernel is licensed under the GPL 3.0 license, which in short means:
1. You must use the GPL 3.0 License if you plan on using this for use in other projects.

2. You cannot use this project to make close sourced or proprietary software, or make this project closed source or proprietary.

3. You must retain the original copyright notices as listed in this file and/or in the licenses file, including credit to the original contributers.

For a comprehensive list of the GPL 3.0 license, check in the licenses file above or go to https://www.gnu.org/licenses/gpl-3.0.en.html.

As of the terms of GPL 3.0, we cannot require you give credit in any use of this code outside of this project, however we politely request that you do to help support and continue the Aurora and Cosmix project.

# What can it do?
The Aurora Kernel has support for:
* Hardware Interrupts (eg, keyboard inputs)
* Displaying text to the screen
* Basic Functions of the Rust Standard Library (eg, `println!`)
* Serial output when running on Qemu (using `cargo test`)
* Exception handling (Eg, CPU Exceptions, Double Faults)
* Paging
* Heap Collection/Allocation
* Async and Await

# What comes with it?
Literally nothing - just the kernel, no terminal or anything - that is all yours to make.

# Credits
https://os.phil-opp.com/
Most or all of the code in this kernel was written in reference to the 'Writing an OS in Rust' guide, without it, this would not be possible

The Contributors - in contrib.txt

# How do I run it/edit it?
It can be edited like any other rust project, with a main.rs, lib.rs, etc etc in /src/
To run it, simply just git clone the folder and open it in terminal. Then, type `cargo run` (requires Rust Nightly from after 2020, and various components)
