CC = gcc
CFLAGS = -Wall -Wextra -I./include -O2
LDFLAGS = -shared

SRC_DIR = src
OBJ_DIR = obj
LIB_DIR = lib

SRCS = $(wildcard $(SRC_DIR)/*.c)
OBJS = $(SRCS:$(SRC_DIR)/%.c=$(OBJ_DIR)/%.o)
TARGET = $(LIB_DIR)/libmetamark.a

.PHONY: all clean test

all: $(TARGET)

$(TARGET): $(OBJS)
	@mkdir -p $(LIB_DIR)
	ar rcs $@ $^

$(OBJ_DIR)/%.o: $(SRC_DIR)/%.c
	@mkdir -p $(OBJ_DIR)
	$(CC) $(CFLAGS) -c $< -o $@

test: $(TARGET)
	$(CC) $(CFLAGS) tests/test_parser.c -o test_parser -L$(LIB_DIR) -lmetamark
	./test_parser

clean:
	rm -rf $(OBJ_DIR) $(LIB_DIR) test_parser

# Dependencies
$(OBJ_DIR)/lexer.o: $(SRC_DIR)/lexer.c include/metamark.h
$(OBJ_DIR)/ast.o: $(SRC_DIR)/ast.c include/metamark.h
$(OBJ_DIR)/metadata.o: $(SRC_DIR)/metadata.c include/metamark.h 