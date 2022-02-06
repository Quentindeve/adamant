global port_out
port_out:
    outb di, si

global port_in
port_in:
    in di, ax
    ret