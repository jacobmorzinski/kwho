/*
 * kwho.c
 *
 * Print a short version of the name of your default principal.
 * (This program is a stripped-down derivative of klist.)
 * 
 */

#include <krb5.h>
#ifdef KRB5_KRB4_COMPAT
#include <kerberosIV/krb.h>
#endif
#include <com_err.h>
#include <stdlib.h>
#ifdef HAVE_UNISTD_H
#include <unistd.h>
#endif
#include <string.h>
#include <stdio.h>
#include <time.h>
/* Need definition of INET6 before network headers, for IRIX.  */
#if defined(HAVE_ARPA_INET_H)
#include <arpa/inet.h>
#endif

#ifndef _WIN32
#define GET_PROGNAME(x) (strrchr((x), '/') ? strrchr((x), '/')+1 : (x))
#else
#define GET_PROGNAME(x) max(max(strrchr((x), '/'), strrchr((x), '\\')) + 1,(x))
#endif

#ifndef _WIN32
#include <sys/socket.h>
#include <netdb.h>
#endif

extern int optind;

char *progname;
krb5_int32 now;

krb5_context kcontext;

void do_ccache (char *);

#ifdef KRB5_KRB4_COMPAT
void do_v4_ccache (char *);
#endif /* KRB5_KRB4_COMPAT */

/*
 * The reason we start out with got_k4 and got_k5 as zero (false) is
 * so that we can easily add dynamic loading support for determining
 * whether Kerberos 4 and Keberos 5 libraries are available
 */

static int got_k5 = 0; 
static int got_k4 = 0;

static int default_k5 = 1;
#ifdef KRB5_KRB4_COMPAT
static int default_k4 = 0;
#else
static int default_k4 = 0;
#endif

static void usage()
{
#define KRB_AVAIL_STRING(x) ((x)?"available":"not available")

    fprintf(stderr, "Usage: %s [-5] [-4] [cache_name]\n", progname); 
    fprintf(stderr, "\t-5 Kerberos 5 (%s)\n", KRB_AVAIL_STRING(got_k5));
    fprintf(stderr, "\t-4 Kerberos 4 (%s)\n", KRB_AVAIL_STRING(got_k4));
    fprintf(stderr, "\t   (Default is %s%s%s%s)\n",
	    default_k5?"Kerberos 5":"",
	    (default_k5 && default_k4)?" and ":"",
	    default_k4?"Kerberos 4":"",
	    (!default_k5 && !default_k4)?"neither":"");
    exit(1);
}

int
main(argc, argv)
    int argc;
    char **argv;
{
    int c;
    char *name;
    int use_k5 = 0, use_k4 = 0;

    got_k5 = 1;
#ifdef KRB5_KRB4_COMPAT
    got_k4 = 1;
#endif

    progname = GET_PROGNAME(argv[0]);

    name = NULL;
    while ((c = getopt(argc, argv, "45")) != -1) {
	switch (c) {
	case '4':
	    if (!got_k4)
	    {
#ifdef KRB5_KRB4_COMPAT
		fprintf(stderr, "Kerberos 4 support could not be loaded\n");
#else
		fprintf(stderr, "This was not built with Kerberos 4 support\n");
#endif
		exit(3);
	    }
	    use_k4 = 1;
	    break;
	case '5':
	    if (!got_k5)
	    {
		fprintf(stderr, "Kerberos 5 support could not be loaded\n");
		exit(3);
	    }
	    use_k5 = 1;
	    break;
	default:
	    usage();
	    break;
	}
    }

    if (argc - optind > 1) {
	fprintf(stderr, "Extra arguments (starting with \"%s\").\n",
		argv[optind+1]);
	usage();
    }

    name = (optind == argc-1) ? argv[optind] : 0;

    if (!use_k5 && !use_k4)
    {
	use_k5 = default_k5;
	use_k4 = default_k4;
    }

    if (!use_k5)
	got_k5 = 0;
    if (!use_k4)
	got_k4 = 0;

    now = time(0);

    if (got_k5)
    {
	krb5_error_code retval;
	retval = krb5_init_context(&kcontext);
	if (retval) {
	    com_err(progname, retval, "while initializing krb5");
	    exit(1);
	}

	do_ccache(name);
    } else {
#ifdef KRB5_KRB4_COMPAT
	do_v4_ccache(name);
#endif /* KRB4_KRB5_COMPAT */
    }

    return 0;
}    

void do_ccache(name)
   char *name;
{
    krb5_ccache cache = NULL;
    krb5_cc_cursor cur;
    krb5_creds creds;
    krb5_principal princ;
    krb5_flags flags;
    krb5_error_code code;
    /* exit_status is set back to 0 if a valid tgt is found */
    int	exit_status = 1;
	    
    if (name == NULL) {
	if ((code = krb5_cc_default(kcontext, &cache))) {
	    com_err(progname, code, "while getting default ccache");
	    exit(1);
	    }
    } else {
	if ((code = krb5_cc_resolve(kcontext, name, &cache))) {
	    com_err(progname, code, "while resolving ccache %s", name);
	    exit(1);
	}
    }
 
    flags = 0;				/* turns off OPENCLOSE mode */
    if ((code = krb5_cc_set_flags(kcontext, cache, flags))) {
	if (code == KRB5_FCC_NOFILE) {
	    com_err(progname, code, "(ticket cache %s:%s)",
		    krb5_cc_get_type(kcontext, cache),
		    krb5_cc_get_name(kcontext, cache));
#ifdef KRB5_KRB4_COMPAT
	    if (name == NULL)
		do_v4_ccache(0);
#endif
	} else {
	    com_err(progname, code,
		    "while setting cache flags (ticket cache %s:%s)",
		    krb5_cc_get_type(kcontext, cache),
		    krb5_cc_get_name(kcontext, cache));
	}
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

    if ((code = krb5_cc_start_seq_get(kcontext, cache, &cur))) {
	com_err(progname, code, "while starting to retrieve tickets");
	exit(1);
    }

    /*
     * Test to see if ticket is valid and not expired.
     *
     * This is valid, if timestamp is okay:
     *  Default principal: jmorzins@ATHENA.MIT.EDU
     *  Service principal: krbtgt/ATHENA.MIT.EDU@ATHENA.MIT.EDU
     *
     * This is valid, if timestamp is okay:
     *  Default principal: jmorzins/admin@ATHENA.MIT.EDU
     *  Service principal: kadmin/admin@ATHENA.MIT.EDU
     * 
     */
    while (!(code = krb5_cc_next_cred(kcontext, cache, &cur, &creds))) {
	if (exit_status && creds.server->length == 2
	    &&
	    strcmp(creds.server->realm.data, princ->realm.data) == 0
	    &&
	    ( (strcmp((char *)creds.server->data[0].data, "krbtgt") == 0 &&
	       strcmp((char *)creds.server->data[1].data,
		      princ->realm.data) == 0 )
	      ||
	      (strcmp((char *)creds.server->data[0].data, "kadmin") == 0 &&
	       strcmp((char *)creds.server->data[1].data, "admin") == 0 ) )
	    && 
	    creds.times.endtime > now)
	    exit_status = 0;
	krb5_free_cred_contents(kcontext, &creds);
    }
    if (code == KRB5_CC_END) {
	if ((code = krb5_cc_end_seq_get(kcontext, cache, &cur))) {
	    com_err(progname, code, "while finishing ticket retrieval");
	    exit(1);
	}
	flags = KRB5_TC_OPENCLOSE;	/* turns on OPENCLOSE mode */
	if ((code = krb5_cc_set_flags(kcontext, cache, flags))) {
	    com_err(progname, code, "while closing ccache");
	    exit(1);
	}
#ifdef KRB5_KRB4_COMPAT
	if (name == NULL)
	    do_v4_ccache(0);
#endif
	exit(exit_status);
    } else {
	com_err(progname, code, "while retrieving a ticket");
	exit(1);
    }	
}


#ifdef KRB5_KRB4_COMPAT
void
do_v4_ccache(name)
    char * name;
{
    char    pname[ANAME_SZ];
    char    pinst[INST_SZ];
    char    prealm[REALM_SZ];
    char    *file;
    int     k_errno;
    CREDENTIALS c;
    int     header = 1;

    if (!got_k4)
	return;

    file = name?name:(char *)tkt_string();

    /* 
     * Since krb_get_tf_realm will return a ticket_file error, 
     * we will call tf_init and tf_close first to filter out
     * things like no ticket file.  Otherwise, the error that 
     * the user would see would be 
     * klist: can't find realm of ticket file: No ticket file (tf_util)
     * instead of
     * klist: No ticket file (tf_util)
     */

    /* Open ticket file */
    k_errno = tf_init(file, R_TKT_FIL);
    if (k_errno) {
	fprintf(stderr, "%s: %s\n", progname, krb_get_err_text (k_errno));
	exit(1);
    }
    /* Close ticket file */
    (void) tf_close();

    /* 
     * We must find the realm of the ticket file here before calling
     * tf_init because since the realm of the ticket file is not
     * really stored in the principal section of the file, the
     * routine we use must itself call tf_init and tf_close.
     */
    if ((k_errno = krb_get_tf_realm(file, prealm)) != KSUCCESS) {
	fprintf(stderr, "%s: can't find realm of ticket file: %s\n",
		progname, krb_get_err_text (k_errno));
	exit(1);
    }

    /* Open ticket file */
    if ((k_errno = tf_init(file, R_TKT_FIL))) {
	fprintf(stderr, "%s: %s\n", progname, krb_get_err_text (k_errno));
	exit(1);
    }
    /* Get principal name and instance */
    if ((k_errno = tf_get_pname(pname)) ||
	(k_errno = tf_get_pinst(pinst))) {
	fprintf(stderr, "%s: %s\n", progname, krb_get_err_text (k_errno));
	exit(1);
    }

    /* 
     * You may think that this is the obvious place to get the
     * realm of the ticket file, but it can't be done here as the
     * routine to do this must open the ticket file.  This is why 
     * it was done before tf_init.
     */
       
    /* Print minimal principal.  Include instance, exclude realm. */
    printf("%s%s%s\n", pname, (pinst[0] ? "." : ""), pinst);
    while ((k_errno = tf_get_cred(&c)) == KSUCCESS) {
	if (header) {
	    header = 0;
	}
    }
    if (header && k_errno == EOF) {
	printf("No tickets in file.\n");
	exit(1);
    }
}
#endif /* KRB4_KRB5_COMPAT */
