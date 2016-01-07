#!/bin/sh

# Warning; only works with new awks.

if [ -x /usr/bin/nawk ] ; then
  awk=/usr/bin/nawk
elif [ -x /usr/bin/gawk ] ; then
  awk=/usr/bin/gawk
else
  awk=/bin/awk			# Hope for the best...
fi

exec "$awk" '
BEGIN {
  klist="/usr/athena/bin/klist"; 
  while( (klist | getline) > 0 ) {
    if (/^([Dd]efault )?[Pp]rincipal: /) {
      sub(/^([Dd]efault )?[Pp]rincipal: /, "");
      sub(/@.*$/, "");
      print $0;
      break;
    }
  }
  close( klist )
}
'
