# File Sorter

The File Sorter is a Rust script that helps you organize files in a directory by sorting them into corresponding folders based on their file extensions.

## Getting Started

To use the File Sorter script, follow these steps depending on your operating system:

### Windows

1. Download the executable:
   - Go to the [Releases](https://github.com/sugeodj/filesorter/releases) page.
   - Download the `windows_file_sorter.exe` executable from the latest release. Choose the appropriate executable for your operating system.

2. Running the Script:
   - Place the `windows_file_sorter.exe` executable in the directory you want to sort files in.
   - Double-click on the `windows_file_sorter.exe` executable to run it.

   The script will automatically sort the files in the directory into corresponding folders based on their file extensions.

### MacOS/Linux

1. Download the zipped folder:
   - Go to the [Releases](https://github.com/sugeodj/filesorter/releases) page.
   - Download the `linux&macos_file_sorter.zip` file from the latest release. Choose the appropriate executable for your operating system.

2. Running the Script:
   - Extract the `linux&macos_file_sorter.zip` folder.
   - Place the `file_sorter` executable in the directory you want to sort files in.
   - Double-click on the `file_sorter` executable to run it.

   The script will automatically sort the files in the directory into corresponding folders based on their file extensions.

## Example

Suppose you have a directory called `Documents` with the following files:

- `resume.pdf`
- `cover_letter.pdf`
- `notes.txt`
- `image.jpg`
- `image2.jpg`
- `presentation.pptx`

After running the File Sorter script, the `Documents` directory will be organized as follows:

<pre>
Documents/
├── pdf/
│   │── resume.pdf
│   └── cover_letter.pdf
├── txt/
│   └── notes.txt
├── jpg/
│   │── image.jpg
│   └── image2.jpg
└── pptx/
    └── presentation.pptx
</pre>

## Limitations

- The script works only on files in the same directory as the script. Subdirectories are not processed.
- Existing files or directories will not be overwritten or deleted.
- Files with unrecognized or missing file extensions will be placed in a folder called `other`.

## Conclusion

The File Sorter script is a useful tool for organizing files based on their file extensions. Follow the instructions above to quickly sort and tidy up your files, making them easier to manage and locate.

If you encounter any issues or have suggestions for improvement, please feel free to contribute to the repository.

Happy organizing!
