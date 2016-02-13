CFLAGS = -Wall -g
VALGRIND_OPTS = \
    --quiet --leak-check=full \
    --show-reachable=no --show-possibly-lost=no \
    --trace-children=yes --error-exitcode=1
