SRC_DIR  := src
OBJ_DIR  := build
BIN_DIR  := .

EXE := $(BIN_DIR)/ray-tracer
SRC := $(wildcard $(SRC_DIR)/*c)
OBJ := $(SRC:$(SRC_DIR)/%.c=$(OBJ_DIR)/%.o)

CPPFLAGS := -Isrc -MMD -MP # -MMD and -MP flags required to detect header changes
CFLAGS   := -Wall
LDFLAGS  := -L/usr/local/lib -Wl,-rpath=/usr/local/lib
LDLIBS   := -lm -lcglm

.PHONY: all clean

all: $(EXE)

$(EXE): $(OBJ) | $(BIN_DIR)
	$(CC) $(LDFLAGS) $^ $(LDLIBS) -o $@

$(OBJ_DIR)/%.o: $(SRC_DIR)/%.c | $(OBJ_DIR)
	$(CC) $(CPPFLAGS) $(CFLAGS) -c $< -o $@

$(BIN_DIR) $(OBJ_DIR):
	mkdir -p $@

clean:
	@$(RM) -rv $(EXE) $(OBJ_DIR)

-include $(OBJ:.o=.d)
