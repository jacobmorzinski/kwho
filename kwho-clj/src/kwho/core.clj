(ns kwho.core
  (:gen-class))

(require '[clojure.string :as str])

(import [sun.security.krb5 Credentials
                           PrincipalName]
        [sun.security.krb5.internal.ccache FileCredentialsCache])


(defn -main
  "Handle any commandline invocation logic."
  [& args]
  (println (get_principal)))

(defn get_principal
  "Get principal from $KRB5CCNAME - assumes it is type FILE:"
  []
  (->> (System/getenv "KRB5CCNAME")
       (#(str/replace % #"^FILE:" ""))
       (FileCredentialsCache/acquireInstance nil)
       (.getPrimaryPrincipal)
       (.getNameStrings)
       (str/join "/")))


