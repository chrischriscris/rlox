SRC_DIR := src
OBJ_DIR := obj
BIN_DIR := .
EXE := clox
SRC := $(wildcard $(SRC_DIR)/*.c)
OBJ := $(SRC:$(SRC_DIR)/%.c=$(OBJ_DIR)/%.o)
CFLAGS := -Wall

.PHONY: all clean
all: $(EXE)

$(EXE): $(OBJ)
	$(CC) $^ -o $(BIN_DIR)/$@

$(BIN_DIR) $(OBJ_DIR):
	mkdir -p $@

$(OBJ_DIR)/%.o: $(SRC_DIR)/%.c | $(OBJ_DIR)
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	@$(RM) -rv $(OBJ_DIR) $(BIN_DIR)/$(EXE)

-include $(OBJ:.o=.d)
