- [ ] Our encoding of line information is hilariously wasteful of memory. Given that a
      series of instructions o!en correspond to the same source line, a natural solution is
      something akin to run-length encoding of the line numbers.
      Devise an encoding that compresses the line information for a series of instructions
      on the same line. Change writeChunk() to write this compressed form, and
      implement a getLine() function that, given the index of an instruction,
      determines the line where the instruction occurs.
      Hint: It’s not necessary for getLine() to be particularly e"icient. Since it is only
      called when a runtime error occurs, it is well o" the critical path where
      performance matters

- [ ] Because OP_CONSTANT only uses a single byte for its operand, a chunk may only
      contain up to 256 di"erent constants. That’s small enough that people writing realworld code will hit that limit. We could use two or more bytes to store the operand,
      but that makes every constant instruction take up more space. Most chunks won’t
      need that many unique constants, so that wastes space and sacrifices some locality
      in the common case to support the rare case.
      To balance those two competing aims, many instruction sets feature multiple
      instructions that perform the same operation but with operands of di"erent sizes.
      Leave our existing one-byte OP_CONSTANT instruction alone, and define a
      second OP_CONSTANT_LONG instruction. It stores the operand as a 24-bit
      number, which should be plenty.
      Implement this function:
      void writeConstant(Chunk\* chunk, Value value, int line) {
          // Implement me...
      }
      It adds value to chunk’s constant array and then writes an appropriate
      instruction to load the constant. Also add support to the disassembler for
      OP_CONSTANT_LONG instructions.
      Defining two instructions seems to be the best of both worlds. What sacrifices, if
      any, does it force on us?
