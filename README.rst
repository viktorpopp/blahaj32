========
Blåhaj64
========

.. figure:: https://c.tenor.com/h6T7YUwIn5UAAAAd/tenor.gif
    :width: 320

Blåhaj64 is a smol little RISC-based ISA, optimized for... well, nothing really, but it is meant to
be very easy to understand :3

Registers
=========

There are 32 64-bit registers, labelled `x0` through `x31`. Each register is also assigned a
symbolic name to make the usage more obvious and consistent.

======== ============= ===========
Register Symbolic Name Description
======== ============= ===========
x0       zero          Always zero
x1-31                  Reserved
======== ============= ===========

Instructions
============

Since Blåhaj64 uses a reduced instruction set, the instruction formats are pretty simple, and there
are not that many instructions. An instruction can be 16, 32, 48 or 64 bits wide.

Instruction Formats
-------------------

+-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------+
|                 | Bit                                                                                                                                                 |
| Format          +----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+---+---+---+---+---+---+---+---+---+---+
|                 | 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
+-----------------+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+---+---+---+---+---+---+---+---+---+---+
| Register        | Reserved                         | rs2                    | rs1                    |                        | Reserved      | Opcode                |
+-----------------+----------------------------------+------------------------+------------------------+ rd                     +-----------+---+-----------------------+
| Large Immediate | imm[19:4]                                                                          |                        | imm[3:0]  | Opcode                    |
+-----------------+------------------------------------------------------------------------------------+------------------------+-----------+---------------------------+

Instructions
------------

movui
^^^^^

Move upper immediate into register

Encoding
********

Type: Large Immediate
Opcode: ``0001011``

Description
***********

Moves the given sign-extended immediate into ``rd``.
