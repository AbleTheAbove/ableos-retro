1. What is a primary goal of my OS?

    Is it a standard (low end) desktop system? User is dummy, highest priority for hardware and software compatibility.
    Is it a high-end desktop system? User is CAD/CAM engineer, highest priority for performance and certain hardware/software compatibility.
    Is it a real-time oriented system? User is a professional programmer, highest priority for performance, defined response time, easy extendable hardware support and programming control.

2. What platforms my OS is going to support?

    Will it support multiprocessing?
    What kind of multiprocessor platforms? Symmetric? (all processors are exactly the same). Asymmetric? (CPUs may be different in architecture and computing power). Both?
    Will it support only local multiprocessing? (all CPUs are connected through a local bus). Distributed multiprocessing? (CPUs are connected through network-like connection). Both?
    What is the target hardware system? Desktop? (more or less standard hardware set). Customizable (embedded) hardware? (If the latter is an answer you'll likely have to individually support every even compatible processor).

3. Will it be a multitasking OS?

    What kind of multitasking will it provide for applications? Cooperative? (tasks yield CPU when they don't need it, demonstrating good will). Preemptive? (tasks are given a defined amount of CPU time).
    Do I need to protect tasks from each other well?
    What is a relationship between tasks in terms of living space? Do they share the same address space? Completely separated? Both?
    How will different tasks communicate with each other?
    What will be a memory model of space that a task runs in? Should I favor simplicity and speed (memory is cheap) or size (memory is a scarce resource)?
    Do I need to protect system from application tasks?

4. What file system will my OS use?

    Should I favor access time (performance) or reduced storage space (size)?
    Can I use one of already developed and well documented file systems?
    Can I use a cut down version of one of well-known file systems?
    What will be an executable format?

5. What build tools do I need?

    Can I use one of existent compilers and linkers?
    Can I obtain (for free, buy or lease) source code for compilers and linkers?
    Do I have to write my own several tools?
    Do I have to write all tools on my own?This should be by any means avoided.

6. How can I easily support third party soft?

    Can I support already existent and popular software?
    How can I support easy creating of third party applications for my OS? (Libraries)
    How can I support easy creating of third party device drivers?

7. How can I use already written code and information?

    Can I use code that is written by others and works? (Even partially).
    Where can I get different kinds of information? (Set your own information library).
