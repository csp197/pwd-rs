`pwd-rs` is a Rust based implementation of the Unix `pwd` program.

NAME

     pwd-rs – return working directory name

SYNOPSIS
     
     ./pwd-rs [-L | -P]

DESCRIPTION
     
     The pwd-rs utility writes the absolute pathname of the current working directory to the standard output.

     Unix based operating systems provide `pwd`, which should ideally be identical to this utility.

     The options are as follows:

     -L      Display the logical current working directory.

     -P      Display the physical current working directory (all symbolic links resolved).

     If no options are specified, the -L option is assumed.

ENVIRONMENT

     Environment variables used by pwd-rs:

     PWD      Logical current working directory.

EXIT STATUS
     
     The pwd-rs utility exits 0 on success, and >0 if an error occurs.

EXAMPLES
     
     Show current working directory with symbolic links resolved:

           $ /bin/pwd -P
           /usr/home/fernape

     Show the logical current directory.  Then use file(1) to inspect the /home directory:

           $ /bin/pwd
           /home/fernape
           $ file /home
           /home: symbolic link to usr/home

HISTORY
     
     The pwd command appeared in Version 5 AT&T UNIX.
     
     This pwd-rs utility is not available anywhere unless you clone this repo and install it yourself.

BUGS
     
     In csh(1) the command dirs is always faster because it is built into that shell.  However, it can give a different answer in the rare case that the current directory or a
     containing directory was moved after the shell descended into it.

     The -L option does not work unless the PWD environment variable is exported by the shell.

(This README is absolutely adapted from `pwd`'s man page.)