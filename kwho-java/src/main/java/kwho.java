// vim:shiftwidth=4:tabstop=4:expandtab

import sun.security.krb5.PrincipalName;
import sun.security.krb5.internal.ccache.FileCredentialsCache;

public class kwho {
    public static void main(String[] args) {


        String cache = System.getenv("KRB5CCNAME");
        if (cache == null) {
            System.err.println("Uh-oh, KRB5CCNAME is not set.  Quitting.");
            return;
        }

        // The FileCredentialsCache does not want to see the "FILE:" prefix
        if ((cache.length() >= 5)
                && cache.substring(0,5).equalsIgnoreCase("FILE:")) {
            cache = cache.substring(5);
        }

        //assumes credendials cache of type "FILE:"
        FileCredentialsCache fcc = FileCredentialsCache.acquireInstance(null, cache);
        if (fcc == null) {
            System.err.println(
                    "No credentials cache found (ticket cache " + cache + ")");
            return;
        }
        PrincipalName princ = fcc.getPrimaryPrincipal();
        String[] nameStrings = princ.getNameStrings();
        StringBuffer temp = new StringBuffer(nameStrings[0]);
        for (int i=1; i<nameStrings.length; i++) {
            temp.append("/");
            temp.append(nameStrings[i]);
        }
        System.out.println(temp);
    }
}

