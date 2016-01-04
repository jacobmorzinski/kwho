(ns kwho.core
  (:gen-class))

(require '[clojure.string :as str])

(import [sun.security.krb5 Credentials
                           PrincipalName]
        [sun.security.krb5.internal.ccache FileCredentialsCache])


(defn get_principal
  "Get principal from $KRB5CCNAME - assumes it is type FILE:"
  []
  (if-some [krb5ccname (System/getenv "KRB5CCNAME")]
    (let [cachefile (str/replace krb5ccname #"^FILE:" "")]
      (if-some [fcc (FileCredentialsCache/acquireInstance nil cachefile)]
        (->> fcc
            (.getPrimaryPrincipal)
            (.getNameStrings)
            (str/join "/"))
        (format "No credentials cache found (ticket cache FILE:%s)"
                cachefile)))
    "Uh-oh, KRB5CCNAME is not set.  Quitting."))


(defn -main
  "Handle any commandline invocation logic."
  [& args]
  (println (get_principal)))

