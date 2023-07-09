# File Sorter

The **File Sorter** is a Rust script that helps you organize files in a directory by sorting them into corresponding folders based on their file extensions. This README provides instructions on how to use the script to organize your files efficiently.

## Prerequisites

To use the File Sorter script, you'll need to have the following:

- Rust installed on your machine: If Rust is not installed, please visit the [Rust official website](https://www.rust-lang.org/tools/install) and follow the instructions to install Rust.

## Getting Started

To get started with the File Sorter script, follow these steps:

1. Clone the repository to your local machine:
   $ git clone <repository-url>
   
2. Build the script using Cargo:
   $ cd <repository-directory>
   $ cargo build

3. Locate the compiled script:
The compiled script will be located at `target/debug/file_sorter` within the repository directory. Please ensure that you have downloaded this file before proceeding.

4. Running the Script:
- Place the `file_sorter` executable in the directory you want to sort files in.
- Double-click on the `file_sorter` executable to run it.

The script will automatically sort the files in the directory into corresponding folders based on their file extensions.

## Example

Let's consider an example to illustrate how the File Sorter works:

Suppose you have a directory called `Documents` with the following files:

- `resume.pdf`
- `notes.txt`
- `image.jpg`
- `presentation.pptx`

After running the File Sorter script, the `Documents` directory will be organized as follows:

Documents/
├── pdf/
│ └── resume.pdf
├── txt/
│ └── notes.txt
├── jpg/
│ └── image.jpg
└── pptx/
└── presentation.pptx

The script creates separate folders for each unique file extension and moves the files into their respective folders.

## Limitations

- The File Sorter script works only on the files present in the same directory as the script itself. Subdirectories are not processed.
- The script will not overwrite or delete any existing files or directories.
- If the script encounters a file with an unrecognized or missing file extension, it will be placed in a folder called `other`.

## Conclusion

The File Sorter script is a useful tool for organizing files based on their file extensions. By following the instructions in this README, you can quickly sort and tidy up your files, making them easier to manage and locate.

If you encounter any issues or have suggestions for improvement, please feel free to open an issue or contribute to the repository.

Happy organizing!

