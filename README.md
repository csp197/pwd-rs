`pwd-rs` is a Rust based implementation of the Unix `pwd` program.

NAME

     pwd-rs – return working directory name

SYNOPSIS
     
     ./pwd-rs [-L | -P]

DESCRIPTION
     
     The pwd utility writes the absolute pathname of the current working directory to the standard output.

     Some shells may provide a builtin pwd command which is similar or identical to this utility.  Consult the builtin(1) manual page.

     The options are as follows:

     -L      Display the logical current working directory.

     -P      Display the physical current working directory (all symbolic links resolved).

     If no options are specified, the -L option is assumed.

ENVIRONMENT

     Environment variables used by pwd:

     PWD      Logical current working directory.

EXIT STATUS
     
     The pwd utility exits 0 on success, and >0 if an error occurs.

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

BUGS
     
     In csh(1) the command dirs is always faster because it is built into that shell.  However, it can give a different answer in the rare case that the current directory or a
     containing directory was moved after the shell descended into it.

     The -L option does not work unless the PWD environment variable is exported by the shell.

(This README was absolutely adapted from `pwd`'s man page.)