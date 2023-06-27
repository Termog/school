## Task
Develop a special program that asks for the user's full name and putting this information into a text file. If such a name is present in the file, then give out about this message. After entering the program, you must complete work and reporting on the limits of its use (temporary or number of starts). According to the launch limit, the program should offer purchase its full version or uninstall yourself. At program update on exit to that computer and check past usage limits (i.e. do not allow them to be exceeded in total). The installer, the program and the uninstaller (program It registers in the system, and you know where to look and how to “hack” it). Two versions of the program are running (can be combined in one):  
A) Time limited (time limit to make no more than 3 minutes to
could be tracked at the time of delivery of reaching the limit).  
B) Start-limited (the limit on the number of starts should also have been
visual, for example - 4-5).

## Building
This program is windows specific!   
to build the installer first build the binary with  
```cargo build```  
then install cargo-wix with  
```cargo install cargo-wix```  
and run  
```cargo wix```  
