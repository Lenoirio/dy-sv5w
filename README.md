# dy-sv5w
A crate for the UART-mode of a DY-SV5W voice module (and the ones with the same UART protocol).
This is a [no-std] crate which uses a trait to handle I/O. 
See the two examples on how to implement such an interface to your UART.
Although primarily designed for embedded usage, it uses ASYNC to profit from async-frameworks like Embassy.

*Hint*: The module sometimes needs (e.g., after issuing play()) a short delay before it accepts the next command.

Most of the commands are fire-and-forget commands. This means there is no ack/nack sent from the module thus it's not possible to provide the API user with success information.