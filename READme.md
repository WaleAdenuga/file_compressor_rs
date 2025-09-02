==================================================================
Use this program to compress the size of PDF files and JPGs using Ghostscript.
There are two forms: GUI and CLI

For CLI:
    <> is a required argument, [] is an optional argument, Current supported file types are PDF and JPG
    
    If an output file name is not provided, the program will use the input file name with a suffix '_compressed' added before the extension.
    Quality is a value from 0 to 100, with 0 the least compression and 100 for the most compression.
    If a value for quality is not provided, it'll compress by standard metrics. Please note quality cannot be negative. Note a higher value for quality means a higher degree for compression

    Usage with cargo:
        cargo run -- cli <input_file_path> [output_file_name] [quality]

    Usage with PATH: 

        cmd_compressor <input_file_path> [output_file_name] [quality]
    
            You can use short or long flags for the arguments, e.g., -f or --file_type
    
        Example: cmd_compressor -i .\\Downloads\\input.pdf -o output_name -q 50
                 will return a 50% level compressed file named output_name.pdf in the source directory of the input file i.e .\\Downloads\\output_name.pdf
    

        Example: cmd_compressor -i .\\Downloads\\input.jpg
                 will return a compressed file named input_compressed.jpg in the source directory of the input file i.e .\\Downloads\\input_compressed.jpg

For GUI:
    You can navigate like you would any other gui application. This is still under development.

    Usage with cargo:
        cargo run -- gui
    Usage with PATH:
        cmd_compressor gui
    
==================================================================