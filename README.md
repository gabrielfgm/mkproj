# mkproj

A (very!) simple command-line application for initiating a project directory for writing academic papers. Takes one argument -- the desired name of 
the directory -- and creates a new folder of said name with sub folders for
a written draft, data, code and literature. A latex article file named 'draft.tex' is placed in the draft folder if the --tex (-t) flag is included. The code reflects the -- perhaps idiosyncratic  -- way I prefer to organize my projects. Written in Rust using the clap crate to handle command line options.