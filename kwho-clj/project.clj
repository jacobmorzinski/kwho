(defproject kwho "0.1.0-SNAPSHOT"
  :description "FIXME: write description"
  :url "https://github.com/jacobmorzinski/kwho"
  :license {:name "The MIT License"
            :url "http://opensource.org/licenses/MIT"}
  :dependencies [[org.clojure/clojure "1.6.0"]]
  :main ^:skip-aot kwho.core
  :target-path "target/%s"
  :profiles {:uberjar {:aot :all}})
