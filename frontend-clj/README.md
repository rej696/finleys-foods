# Demo Project for ClojureScript Frontend
This version uses the clj build tool with figwheel main to provide hot reloading.

## [Get clj tool](https://clojure.org/guides/getting_started)
### Linux
1. Ensure that the following dependencies are installed: `bash`, `curl`, `rlwrap`, and `Java`.
2. Use the `linux-install` script to download and run the installation, which will set up the `clj` and `clojure` executables
```
curl -O https://download.clojure.org/install/linux-install-1.10.3.814.sh
chmod +x linux-install-1.10.3.814.sh
sudo ./linux-install-1.10.3.814.sh
```

### [Windows](https://github.com/clojure/tools.deps.alpha/wiki/clj-on-Windows)
Follow Link

## clj and figwheel-main
set up project deps.edn
```
{:paths
 ["src" "resources"]
 
 :deps {org.clojure/clojurescript {:mvn/version "1.10.764"}
        com.bhauman/figwheel-main {:mvn/version "0.2.12"}
        reagent/reagent           {:mvn/version "1.0.0"}}

 :aliases
 {:dev {:main-opts ["-m"      "figwheel.main"
                    "--build" "dev"
                    "--repl"]}}}
```

run figwheel as follows
`clj -M:dev`

## clj and vim-fireplace
set up ~/.clojure/deps.edn as follows
```
{:aliases {
    :cider-clj {:extra-deps {org.clojure/clojure {:mvn/version "1.10.1"}
                             cider/cider-nrepl   {:mvn/version "0.25.9"}}
                :main-opts ["-m" "nrepl.cmdline" "--middleware" "[cider.nrepl/cider-middleware]"]}
    
    :cider-cljs {:extra-deps {org.clojure/clojure       {:mv/version "1.10.1"}
                              org.clojure/clojurescript {:mvn/version "1.10.339"}
                              cider/cider-nrepl         {:mvn/version "0.25.9"}
                              cider/piggieback          {:mvn/version "0.5.2"}}
                 :main-opts ["-m" "nrepl.cmdline" "--middleware"
                             "[cider.nrepl/cider-middleware,cider.piggieback/wrap-cljs-repl]"]}}}

```

set up project deps.edn
```
{:paths
 ["src"]
 
 :deps {org.clojure/clojure {:mvn/version "1.10.3"}}}
```

Run nrepl as follows
- `clj -M:cider-clj` starts a clojure nrepl
- `clj -M:cider-cljs` starts a clojurescript nrepl

### Vim-Fireplace commands
- `:FireplaceConnect` connects to a repl
- `cqp` (or `:Eval`) presents repl prompt
- `cqc` opens repl subwindow
- `cpp` evaluates form under cursor
- `cqq` opens repl subwindow and pastes form under cursor

### Vim-SEXP commands
- `>)` barf to the right
- `<(` barf to the left
- `<)` slurp from the right
- `>(` slurp from the left

## Useful Links

- [Start a ClojureScript App from Scratch](https://betweentwoparens.com/start-a-clojurescript-app-from-scratch)
- [Reagent Introduction](https://reagent-project.github.io/index.html)
- [Reagent Github](https://github.com/reagent-project/reagent/tree/v1.0.0)
- [Reagent Tutorials](https://cljdoc.org/d/reagent/reagent/1.0.0/doc/documentation-index)
- [Reagent API/Docs](https://reagent-project.github.io/docs/master/index.html)
- [cljs-http Github](https://github.com/r0man/cljs-http)

