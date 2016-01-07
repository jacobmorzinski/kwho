import sun.security.krb5.Credentials;
import sun.security.krb5.PrincipalName;
import sun.security.krb5.internal.ccache.FileCredentialsCache;

public class kdemo {
    public static void main(String[] args) {

	if (true) {
	    Credentials creds = null;
	    try {
		double r = Math.random();
		System.out.format("r is %1.2f%n", r);
		if (r < 0.5) {
		    // creds = Credentials.acquireDefaultCreds();
		} else {
		    creds = Credentials.acquireTGTFromCache(null, System.getenv("KRB5CCNAME"));
		}
	    }
	    catch (Exception e) {
		System.err.println(e);
	    }
	    System.out.println(creds);
	}
    }
}
