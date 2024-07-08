# amia

An operating system in RISC-V 64bit, written in Rust

# check prerequisites

## Ubuntu, macOS

```
$ rustup --version && \
makers --version && \
qemu-system-riscv64 --version
```

# run

```
$ makers run
```

Currently, you can enter QEMU monitor by typing 'exit' in the shell session and exiting the shell process.

# kernel objdump

You have to install `llvm-objdump` to do this.

```
$ makers objdump
```
