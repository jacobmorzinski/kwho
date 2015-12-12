// vim:shiftwidth=4:tabstop=4:expandtab

import sun.security.krb5.Credentials;
import sun.security.krb5.PrincipalName;
import sun.security.krb5.internal.ccache.FileCredentialsCache;

public class kwho {
    public static void main(String[] args) {

        String cache = System.getenv("KRB5CCNAME");
        if (cache != null && (cache.length() >= 5)
                && cache.substring(0,5).equalsIgnoreCase("FILE:")) {
            cache = new String(cache.substring(5));
        }

        //assumes unix-style file credendials cache
        FileCredentialsCache fcc = FileCredentialsCache.acquireInstance(null, cache);
        PrincipalName princ = fcc.getPrimaryPrincipal();
        String[] nameStrings = princ.getNameStrings();
        StringBuffer temp = new StringBuffer(nameStrings[0]);
        for (int i=1; i<nameStrings.length; i++)
            temp.append("/" + nameStrings[i]);
        System.out.println(temp);
    }
}

