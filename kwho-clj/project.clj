(defproject kwho "0.1.0"
  :description "FIXME: write description"
  :url "https://github.com/jacobmorzinski/kwho"
  :license {:name "The MIT License"
            :url "http://opensource.org/licenses/MIT"}
  :dependencies [[org.clojure/clojure "1.6.0"]]
  :plugins [[lein-bin "0.3.6-SNAPSHOT"]] #_ "https://github.com/Raynes/lein-bin"
  :bin {:name "kwho.bat"} #_ "I wish I could use a conditional..."
  :main ^:skip-aot kwho.core
  :target-path "target/%s"
  :profiles {:uberjar {:aot :all}})
