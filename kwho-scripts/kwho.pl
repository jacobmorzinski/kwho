#!/usr/athena/bin/perl -w

# Goal: run klist, see what principals I'm signed in with.

# klist
# [...]
# -|Ticket cache: FILE:/tmp/krb5cc_jmorzins.extra
# -|Default principal: jmorzins/extra@ATHENA.MIT.EDU
# [...]
# -|Kerberos 4 ticket cache: /tmp/tkt_jmorzins.extra
# -|Principal: jmorzins.extra@ATHENA.MIT.EDU

use strict;
require 5.006;		       # v5.6 required for open( , "-|", )

my $klist = "/usr/athena/bin/klist";

open(KLIST, "-|", $klist) or die "Fatal error; can't read tickets.\n";
while ( <KLIST> ) {
  if (s/^(?:default )?principal: (.*)@.*$/$1/i) {
    print;
    last;
  }
}
















if (close(KLIST)) {
  1;				# no-op.  (klist succeeded)
} else {
  if ($? == -1) {
    print "failed to execute: $!\n";
  }
  elsif ($? & 127) {
    printf "child died with signal %d, %s coredump\n",
      ($? & 127),  ($? & 128) ? "with" : "without";
  }
  else {
    printf "child exited with value %d\n", $? >> 8;
  }
}



# 
# # 'use POSIX' would let me do semi-portable tests on the exit status
# # of klist, but it doubles the load time of the script.
# 
# if (close (KLIST)) {
#   1; 				# no-op
# } else {
#   if (WIFEXITED($?))  {
#     print "$klist had exit status ", WEXITSTATUS($?), "\n";
#   } elsif (WIFSIGNALED($?)) {
#     print "$klist terminated by signal ", WTERMSIG($?), "\n";
#   } elsif (WIFSTOPPED($?)) {
#     print "$klist stopped by signal ", WSTOPSIG($?), "\n";
#   } else {
#     print "Warning: program received an unexpected error status.\n";
#   }
# }
# 
