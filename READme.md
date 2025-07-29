==================================================================
Use this program to compress the size of PDF files and JPGs using Ghostscript
    
<> is a required argument, [] is an optional argument, Current supported file types are PDF and JPG
    
If an output file name is not provided, the program will use the input file name with a suffix '_compressed' added before the extension.
    
Usage: cmd_compressor -f --<file_type> -i <input_file_path> [-o output_file_name]
    You can use short or long flags for the arguments, e.g., -f or --file_type
    
    Example: cmd_compressor -f --pdf -i .\\Downloads\\input.pdf -o output
                 will return a compressed file named output.pdf in the source directory of the input file i.e .\\Downloads\\output.pdf
    

    Example: cmd_compressor -f --jpg -i .\\Downloads\\input.jpg
                 will return a compressed file named input_compressed.jpg in the source directory of the input file i.e .\\Downloads\\input_compressed.jpg
    
==================================================================
