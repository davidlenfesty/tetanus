# (Simulated) Hello world!

I wanted to run *my own* code in a simulated vexriscv while I was playing around with it,
so I figured Rust would be the easiest way to set up the toolchain. (I've compiled riscv gcc
before, and it wasn't much fun).

90% of this code is copied from [this article](https://craigjb.com/2020/01/22/ecp5/), all I did
was write the UART bits. (address was found in one of the demo examples, don't remember which,
just ripgrep for `uart_write` if you want to find it).

