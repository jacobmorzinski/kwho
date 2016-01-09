/*
 * kwho.c
 *
 * Print a short version of the name of your default principal.
 * (This program is a stripped-down derivative of klist.)
 * 
 */

#include <krb5.h>
#include <com_err.h>
#include <string.h>
#include <stdio.h>

#define GET_PROGNAME(x) (strrchr((x), '/') ? strrchr((x), '/')+1 : (x))

char *progname;
krb5_context kcontext;
void do_ccache ();

int
main(argc, argv)
    int argc;
    char **argv;
{
    krb5_error_code code;

    progname = GET_PROGNAME(argv[0]);

    if ((code = krb5_init_context(&kcontext))) {
      com_err(progname, code, "while initializing krb5");
      exit(1);
    }
    do_ccache();

    return 0;
}

void do_ccache()
{
    krb5_ccache cache = NULL;
    krb5_principal princ;
    krb5_error_code code;
	    
    if ((code = krb5_cc_default(kcontext, &cache))) {
      com_err(progname, code, "while getting default ccache");
      exit(1);
    }

    if ((code = krb5_cc_get_principal(kcontext, cache, &princ))) {
	com_err(progname, code, "while retrieving principal name");
	exit(1);
    }

    /*
     * Print minimal default principal. Include the instance,
     * exclude realm.  Be aware that this code does not do the
     * full sanity-checking that the stock krb5_unparse_name
     * routine (from krb5/src/lib/krb5/krb/unparse.c) does.
     *
     * Prints slashes between components.
     */       
    int i=0;
    printf("%s", krb5_princ_component(kcontext, princ, i)->data);
    for (i=1; i<krb5_princ_size(kcontext, princ); i++) {
      printf("/%s", krb5_princ_component(kcontext, princ, i)->data);
    }
    printf("\n");

}

