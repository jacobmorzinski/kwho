(defproject kwho "0.1.0"
  :description "FIXME: write description"
  :url "https://github.com/jacobmorzinski/kwho"
  :license {:name "The MIT License"
            :url "http://opensource.org/licenses/MIT"}
  :dependencies [[org.clojure/clojure "1.6.0"]]
  :plugins [[lein-bin "0.3.5"]] #_ "https://github.com/jacobmorzinski/lein-bin"
  :bin {:name "kwho.bat"} #_ "I wish I could use a conditional..."
  :main ^:skip-aot kwho.core
  :target-path "target/%s"
  :jar-exclusions [#"(^|\\|/)[._](.*\.|)s[a-w][a-z]$"] #_ "vim .swp files"
  :profiles {:uberjar {:aot :all}})
