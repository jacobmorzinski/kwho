
PROG	= kwho
OBJS    = kwho.o
CC	= gcc -Wall
CFLAGS	= $(shell krb5-config --cflags)
LIBS	= $(shell krb5-config --libs)
prefix	= /mit/jmorzins

all: $(PROG)

$(PROG): $(OBJS)
	$(CC) -o $@ $(OBJS) $(LIBS)

.c.o:
	$(CC) $(CFLAGS) -c $<

clean:
	rm -f $(PROG) *.o *~

install:
	install -c -m 755 -s $(PROG) $(DESTDIR)$(prefix)/bin/
	install -c -m 644 $(PROG).1 $(DESTDIR)$(prefix)/man/man1/
