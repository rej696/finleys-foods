(ns app.app
  (:require-macros [cljs.core.async.macros :refer [go]])
  (:require [reagent.core :as r]
            [cljs.core.async :refer [<!]]
            [cljs-http.client :as http]))

;-----------------------------------------------------
; get time from server
;-----------------------------------------------------

(def time-api "http://127.0.0.1:5000/api/v1/time")

(def current-time (r/atom nil))

(defn get-time [endpoint]
  (go (let [response (<! (http/get endpoint {:with-credentials? false}))]
        (prn (str "GET " time-api " status: "(:status response)))
        (prn (str "GET " time-api " response: " (:time (:body response))))
        (reset! current-time (:time (:body response))))))

(defn title [text]
  [:h1.site__title
   [:span.site__title-text text]])

(defn app
  []
  (get-time time-api)
  (fn []
    [:div
    ;; [title "Hello Clojurians!"]
     [title (str "Time...\n" @current-time)]
     [:div
      [:input.btn {:type "button" :value "Update Time!"
                   :on-click (fn [] (get-time time-api))}]]]))
