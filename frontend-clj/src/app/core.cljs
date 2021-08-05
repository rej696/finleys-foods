;; ^:figwheel-hooks tells figwheel to use figwheel hooks in this namespace
(ns ^:figwheel-hooks app.core
  (:require
    [reagent.dom :as r.dom]
    [app.app :refer [app]]))

(defn mount []
  (r.dom/render [app] (js/document.getElementById "root")))

;; ^:after-load is a figwheel hook telling figwheel that we want to run this function after each compile
(defn ^:after-load re-render []
  (mount))

(defonce start-up (do (mount) true))
