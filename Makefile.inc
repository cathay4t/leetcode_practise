CFLAGS = -Wall -g

VALGRIND_OPTS = \
    --quiet --leak-check=full \
    --show-reachable=no --show-possibly-lost=no \
    --trace-children=yes --error-exitcode=1

all: $(EXEC) check_mem_leak check_time

check_mem_leak: $(EXEC)
	valgrind $(VALGRIND_OPTS) ./$(EXEC) 1
	echo

check_time: $(EXEC)
	time -p ./$(EXEC)

clean:
	rm -f $(EXEC) vgcore*
