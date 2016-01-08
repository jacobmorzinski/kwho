package demo

import sun.security.krb5.PrincipalName;
import sun.security.krb5.internal.ccache.FileCredentialsCache;
import kotlin.collections.joinToString
import kotlin.text.Regex
import kotlin.text.replace

fun main(args: Array<String>) {
    var cache: String? = System.getenv("KRB5CCNAME")
    // The FileCredentialsCache does not want to see the "FILE:" prefix
    cache = cache?.replace(Regex("^FILE:"), "")
    val fcc: FileCredentialsCache? = FileCredentialsCache.acquireInstance(null, cache)
    if (fcc == null) {
        return
    }
    val princ: PrincipalName = fcc.primaryPrincipal
    val nameStrings: Array<out String> = princ.nameStrings
    val output: String = nameStrings.joinToString("/")
    println(output)
}
