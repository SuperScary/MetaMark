# Compiler settings
ifeq ($(OS),Windows_NT)
    CC = gcc
    EXE = .exe
    RM = del /F /Q
    MKDIR = mkdir
    RMDIR = rmdir /S /Q
else
    CC = gcc
    EXE =
    RM = rm -f
    MKDIR = mkdir -p
    RMDIR = rm -rf
endif

# Directories
SRC_DIR = src
OBJ_DIR = obj
BIN_DIR = bin
TEST_DIR = tests

# Source files
SRCS = $(wildcard $(SRC_DIR)/*.c)
OBJS = $(SRCS:$(SRC_DIR)/%.c=$(OBJ_DIR)/%.o)

# Test files
TEST_SRCS = $(wildcard $(TEST_DIR)/*.c)
TEST_OBJS = $(TEST_SRCS:$(TEST_DIR)/%.c=$(OBJ_DIR)/%.o)

# Command files for testing
CMD_SRCS = $(SRC_DIR)/commands.c
CMD_OBJS = $(CMD_SRCS:$(SRC_DIR)/%.c=$(OBJ_DIR)/%.o)

# Target executables
TARGET = $(BIN_DIR)/mmk$(EXE)
TEST_TARGET = $(BIN_DIR)/mmk_test$(EXE)

# Compiler flags
CFLAGS = -Wall -Wextra -I./include -I../metamark-core/include
LDFLAGS = -L../metamark-core/build -lmetamark

# Default target
all: $(TARGET) $(TEST_TARGET)

# Create directories
$(OBJ_DIR):
	$(MKDIR) $(OBJ_DIR)

$(BIN_DIR):
	$(MKDIR) $(BIN_DIR)

# Compile source files
$(OBJ_DIR)/%.o: $(SRC_DIR)/%.c | $(OBJ_DIR)
	$(CC) $(CFLAGS) -c $< -o $@

# Compile test files
$(OBJ_DIR)/%.o: $(TEST_DIR)/%.c | $(OBJ_DIR)
	$(CC) $(CFLAGS) -c $< -o $@

# Link the main executable
$(TARGET): $(OBJS) | $(BIN_DIR)
	$(CC) $(OBJS) -o $@ $(LDFLAGS)

# Link the test executable
$(TEST_TARGET): $(TEST_OBJS) $(CMD_OBJS) | $(BIN_DIR)
	$(CC) $(TEST_OBJS) $(CMD_OBJS) -o $@ $(LDFLAGS)

# Clean build files
clean:
	$(RMDIR) $(OBJ_DIR)
	$(RMDIR) $(BIN_DIR)

# Run tests
test: $(TEST_TARGET)
	./$(TEST_TARGET)

# Phony targets
.PHONY: all clean test 