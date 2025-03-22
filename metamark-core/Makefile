CC = gcc
CFLAGS = -Wall -Wextra -I./include -g
LDFLAGS = 

SRC_DIR = src
TEST_DIR = tests
BUILD_DIR = build

SRCS = $(wildcard $(SRC_DIR)/*.c)
OBJS = $(SRCS:$(SRC_DIR)/%.c=$(BUILD_DIR)/%.o)
TEST_SRCS = $(wildcard $(TEST_DIR)/*.c)
TEST_OBJS = $(TEST_SRCS:$(TEST_DIR)/%.c=$(BUILD_DIR)/%.o)

TARGET = $(BUILD_DIR)/libmetamark.a
TEST_TARGET = $(BUILD_DIR)/test_metamark

.PHONY: all clean test

all: $(TARGET)

$(BUILD_DIR):
	mkdir -p $(BUILD_DIR)

$(TARGET): $(OBJS) | $(BUILD_DIR)
	ar rcs $@ $^

$(BUILD_DIR)/%.o: $(SRC_DIR)/%.c | $(BUILD_DIR)
	$(CC) $(CFLAGS) -c $< -o $@

$(BUILD_DIR)/%.o: $(TEST_DIR)/%.c | $(BUILD_DIR)
	$(CC) $(CFLAGS) -c $< -o $@

$(TEST_TARGET): $(TEST_OBJS) $(TARGET) | $(BUILD_DIR)
	$(CC) $(CFLAGS) $^ -o $@ $(LDFLAGS)

test: $(TEST_TARGET)
	./$(TEST_TARGET)

clean:
	rm -rf $(BUILD_DIR)

# Dependencies
$(BUILD_DIR)/lexer.o: $(SRC_DIR)/lexer.c include/metamark.h
$(BUILD_DIR)/ast.o: $(SRC_DIR)/ast.c include/metamark.h
$(BUILD_DIR)/metadata.o: $(SRC_DIR)/metadata.c include/metamark.h 