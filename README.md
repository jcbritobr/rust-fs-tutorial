# Rust fs module and filesystem tutorial

Rust standard library has a rich set of abstractions for multiplatform file and directory operations.
This tutorial shows the basics of how files are managed by Unix/Linux systems, and how to use Rust APIs to work with files, directories, paths and links.

* **Files** - Files are just an amount of bytes. They can be a text, video, audio, image, or any other digital information. Is the application that does the interpretation of the file and its contents. There is a Unix/Linux philosophy that ***every system resource is a file***.

1. Regular files - We use to store binary or text data.
2. Directories - Files wich contains references to other files.
3. Block device files - Hard disks, tape drives, USB cameras.
4. Character device files - A terminal, a keyboard, a printer, a sound card.
5. Named pipes - In-memory inter-process communication machanism.
6. Unix domain sockets - Is a form of inter-process communication.
7. Links - Hard links and symbolic links.

All resources above are files.

* **System Calls**

Is aprogrammatic way in which a computer program requests a service from the kernel of the operation system on which it is executed.
The same system calls used to open, read, write and close regular files can also be used in any other type of files such as block devices, links.

* **File Descriptors**

A file descriptor is a handle to a file. Opening a file returns a descriptor, and other operations such as reading, writing and closing use the file descriptor.
