/**
 * Created by Jacob on 1/4/2016.
 */

import sun.security.krb5.PrincipalName;
import sun.security.krb5.internal.ccache.FileCredentialsCache;
import kotlin.text.Regex

fun main(args: Array<String>) {
    var cache: String? = System.getenv("KRB5CCNAME")
    // The FileCredentialsCache does not want to see the "FILE:" prefix
    cache = cache?.replace(Regex("^FILE:"), "")
    val fcc: FileCredentialsCache? = FileCredentialsCache.acquireInstance(null, cache)
    if (fcc == null) {
        println("No credentials cache found (ticket cache FILE:$cache)")
        return
    }
    val princ: PrincipalName = fcc.primaryPrincipal
    val nameStrings: Array<out String> = princ.nameStrings
    val output: String = nameStrings.joinToString("/")
    println(output)
}