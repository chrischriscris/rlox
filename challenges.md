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
      contain up to 256 di"erent constants. That’s small enough that people writing real
      world code will hit that limit. We could use two or more bytes to store the operand,
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

- [ ] Our VM’s stack has a fixed size, and we don’t check if pushing a value overflows it.
      This means the wrong series of instructions could cause our interpreter to crash or
      go into undefined behavior. Avoid that by dynamically growing the stack as
      needed. What are the costs and benefits of doing so

- [ ] To interpret OP_NEGATE, we pop the operand, negate the value, and then push
      the result. That’s a simple implementation, but it increments and decrements
      stackTop unnecessarily, since the stack ends up the same height in the end. It
      might be faster to simply negate the value in place on the stack and leave
      stackTop alone. Try that and see if you can measure a performance di!erence.
      Are there other instructions where you can do a similar optimization?
