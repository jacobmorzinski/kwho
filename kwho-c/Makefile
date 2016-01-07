
PROG	= kwho
OBJS    = kwho.o
CC	= gcc -Wall
CFLAGS	= -g -I/usr/athena/include -DKRB5_KRB4_COMPAT=1 -DHAVE_UNISTD_H=1 -DHAVE_ARPA_INET_H=1
CFLAGS	= -g -I/usr/athena/include -DHAVE_UNISTD_H=1 -DHAVE_ARPA_INET_H=1
LDFLAGS	= -L/usr/athena/lib -Wl,-R/usr/athena/lib
LIBS	= -lkrb4 -ldes425 -lkrb5 -lk5crypto -lcom_err -lkrb5support -lresolv
LIBS	= -lkrb5
LIBS	= -lkrb5 -lcom_err
prefix	= /mit/jmorzins

all: $(PROG)

$(PROG): $(OBJS)
	$(CC) $(LDFLAGS) -o $@ $(OBJS) $(LIBS)

.c.o:
	$(CC) $(CFLAGS) -c $<

clean:
	rm -f $(PROG) *.o *~

install:
	install -c -m 755 -s $(PROG) $(DESTDIR)$(prefix)/bin/
	install -c -m 644 $(PROG).1 $(DESTDIR)$(prefix)/man/man1/
