# dy-sv5w
Library for the UART-mode of a DY-SV5W voice module.
This is a [no-std] crate which uses a trait to handle I/O. 
See the two examples on how to implement such an interface to your UART.
Although primarily designed for embedded usage, it uses ASYNC to make profit of async-framework like Embassy.

