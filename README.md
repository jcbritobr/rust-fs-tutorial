# Rust fs module and filesystem tutorial

Rust standard library has a rich set of abstractions for multiplatform file and directory operations.
This tutorial shows the basics of how files are managed by Unix/Linux systems, and how to use Rust APIs to work with files, directories, paths and links.

* **Files**

Files are just an amount of bytes. They can be a text, video, audio, image, or any other digital information. Is the application that does the interpretation of the file and its contents. There is a Unix/Linux philosophy that ***every system resource is a file***.

1. Regular files - We use to store binary or text data.
2. Directories - Files wich contains references to other files.
3. Block device files - Hard disks, tape drives, USB cameras.
4. Character device files - A terminal, a keyboard, a printer, a sound card.
5. Named pipes - In-memory inter-process communication machanism.
6. Unix domain sockets - Is a form of inter-process communication.
7. Links - Hard links and symbolic links.

All resources above are files.

* **System Calls**

Is a programmatic way in which a computer program requests a service from the kernel of the operation system on which it is executed.
The same system calls used to open, read, write and close regular files can also be used in any other type of files such as block devices, links.

* **File Descriptors**

A file descriptor is a handle to a file. Opening a file returns a descriptor, and other operations such as reading, writing and closing use the file descriptor. File operations are performed by processes. A process perform this operations by invoking system calls on the kernel. Eche process has a limit on the number of files it can open.

* Examples

1. [File Operations](src/files.rs)
2. [Directory Operations](src/dir.rs)
3. [Link Operations](src/link.rs)
4. [The rstat tool](src/rstat.rs)

* Test rstat tool
```
$ cargo test
```

* Build rstat tool

The rstat is a tool that extracts information about rust source code. It uses all APIs from all examples.
```
$ cargo build --bin rstat
```

* Run rstat
```
$ cargo run --bin rstat -- -m src src
$ Summary stats: SrcStats { number_of_files: 6, number_of_lines: 323, loc: 277, comments: 3, blanks: 43 }
```

* Build all examples and tools
```
$ cargo build
```

## References
***
1. [System calls](https://en.wikipedia.org/wiki/System_call)
2. [Practical System Programming for Rust Developers](https://www.amazon.com.br/Practical-System-Programming-Rust-Developers-ebook/dp/B08MBCQ5L1)