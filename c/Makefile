
clox: main.o
	cc -o clox main.o chunk.o memory.o debug.o value.o

main.o: chunk.o memory.o common.h debug.o value.o

value.o: value.h value.c

debug.o: debug.c debug.h

chunk.o: chunk.c chunk.h common.h

memory.o: memory.c memory.h common.h

.PHONY: clean
clean:
	rm -fv *.o clox
