########
Blåhaj32
########

.. figure:: https://c.tenor.com/h6T7YUwIn5UAAAAd/tenor.gif
    :width: 320

Blåhaj32 is a smol little RISC-based ISA, and hardware collection, optimized for... well, nothing
really, but it is meant to be very easy to understand :3

*********
Registers
*********

There are 32 registers, named ``r0`` through ``r31``. Each register is 32 bits wide and fully mutable,
except one, called ``r0``, also referred to as the zero register. It is hardwired to always be zero
and cannot be mutated.

=========== =============
Register    Symbolic Name
=========== =============
``r0``      ``zero``
``r1..r31`` Reserved
=========== =============

************
Instructions
************

All instructions are currently 32 bits wide.

Instruction Formats
===================

The following instruction formats exist:

+----------+-----------------------------------------------------------------------------------------------------------------------------------------------------+
|          | Bit                                                                                                                                                 |
| Format   +----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+---+---+---+---+---+---+---+---+---+---+
|          | 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
+----------+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+---+---+---+---+---+---+---+---+---+---+
| Register | ``f7``                           | ``f3``       | ``rs2``                | ``rs1``                     | ``rd``             | ``opcode``            |
+----------+----------------------------------+--------------+------------------------+-----------------------------+--------------------+-----------------------+

