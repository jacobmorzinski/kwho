/* 
 * kwho3 v1 (c) Jacob Morzinski
 */

import java.security.Principal;
import java.util.HashMap;
import java.util.Iterator;
import java.util.LinkedList;
import java.util.Set;

import javax.security.auth.Subject;
import javax.security.auth.login.AppConfigurationEntry;
import javax.security.auth.login.Configuration;
import javax.security.auth.login.LoginContext;
import javax.security.auth.login.LoginException;

public class kwho3 {

	/**
	 * @param args
	 */
	public static void main(String[] args) {

		LoginContext lc = null;
		
		HashMap<String, LinkedList<AppConfigurationEntry>> configMap =
			new HashMap<String, LinkedList<AppConfigurationEntry>>();
		initializeConfigMap(configMap);
		Configuration conf = new SampleConfiguration(configMap);


		try {
			lc = new LoginContext("Kerberos", null, new MyCallbackHandler(), conf);
		} catch (LoginException le) {
		    System.err.println("Cannot create LoginContext. "
		        + le.getMessage());
		    System.exit(-1);
		} catch (SecurityException se) {
		    System.err.println("Cannot create LoginContext. "
		        + se.getMessage());
		    System.exit(-1);
		}

		try {
			// attempt authentication
			lc.login();
		} catch (LoginException le) {
			System.err.println("Authentication failed:");
			System.err.println("  " + le.getMessage());
			//			le.printStackTrace();
			return;
		}

		Subject subj = lc.getSubject();

	    String name;
	    Principal p;
	    Set<Principal> princSet  = subj.getPrincipals();
	    Iterator<Principal> iter = princSet.iterator();
	    if (iter.hasNext()) {
	    	p = iter.next();
	    	name = p.getName().split("@",2)[0];
	    	System.out.print(name);
//	    	iter = princSet.iterator();  // for testing: reset iterator
	    	while (iter.hasNext()) {
		    	p = iter.next();
		    	name = p.getName().split("@",2)[0];
		    	System.out.print("," + name);
	    	}
	    }
	    System.out.println();
	}

	/**
	 * Generate a map of settings, which will be passed 
	 * as the initial parameters to a Configuration object.
	 * 
	 * The map is equivalent to a config file that says:
	 * 
	 * <pre>
	 * Kerberos {
	 *  com.sun.security.auth.module.Krb5LoginModule required useTicketCache=true;
	 * }
	 * </pre>
	 * 
	 * @param config - a map allocated by the caller (non-<code>null</code>)
	 */
	private static void initializeConfigMap(
			HashMap<String, LinkedList<AppConfigurationEntry>> config) {
		
		if (config == null) {
			throw (new NullPointerException(
					"initializeConfigMap requires a non-null config"));
		}

		String moduleClass =
			"com.sun.security.auth.module.Krb5LoginModule";

		AppConfigurationEntry.LoginModuleControlFlag controlFlag = 
			AppConfigurationEntry.LoginModuleControlFlag.REQUIRED;
		
		HashMap<String, String> options =
			new HashMap<String, String>();
		//		options.put("debug", "true");
		options.put("useTicketCache", "true");
		String ccache = System.getenv("KRB5CCNAME");
		if (ccache != null) {
		    options.put("ticketCache", ccache);
		}
		options.put("doNotPrompt", "true"); //don't get new tickets
		
		AppConfigurationEntry ace = 
			new AppConfigurationEntry(moduleClass, controlFlag, options);
		
		LinkedList<AppConfigurationEntry> configEntries =
			new LinkedList<AppConfigurationEntry>();
		configEntries.add(ace);
		
		config.put("Kerberos", configEntries);
	}

}
