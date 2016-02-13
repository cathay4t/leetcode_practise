include ../Makefile.inc

EXEC = two_sum

all: $(EXEC)

check: $(EXEC)
	valgrind $(VALGRIND_OPTS) ./$(EXEC)
