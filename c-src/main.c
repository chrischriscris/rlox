#include "chunk.h"
#include "common.h"
#include "debug.h"

int main(int argc, const char *argv[]) {
    Chunk chunk;

    Chunk_init(&chunk);
    Chunk_write(&chunk, OP_RETURN);

    disassemble_chunk(&chunk, "test chunk");

    Chunk_free(&chunk);

    return 0;
}
