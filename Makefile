
OBJS = sha256.o nostril.o
HEADERS = hex.h random.h config.h sha256.h

all: nostril

%.o: %.c config.h
	@echo "cc $<"
	@$(CC) -c $< -o $@

nostril: $(HEADERS) $(OBJS)
	$(CC) $(OBJS) -lsecp256k1 -o $@ 

config.h: configurator                                                          
	./configurator > $@                                                     

configurator: configurator.c                                                    
	$(CC) $< -o $@

clean:
	rm -f nostril *.o

tags: fake
	ctags *.c *.h

.PHONY: fake
