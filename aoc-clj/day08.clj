(ns day08
  (:require [clojure.string :as str]
            [clojure.set :as set]))

(def input (-> (slurp "input_08.txt")
               (str/split #"\n")))

(defn infer-mapping [sample]
  (let [counts (for [s sample] [(count s) s])
        counts-map (zipmap (map count sample) sample)
        result (merge (zipmap sample (map #(get {2 1 4 4 7 8 3 7} (count %) (count %)) sample)))
        get-string-for-count (fn [nums] (->> (map counts-map nums) (str/join)))
        union (get-string-for-count #{2 4 3})
        eight (get-string-for-count #{7})
        diff (set/difference (set eight) (set union))
        fives (filter (fn [[ct _]] (= ct 5)) counts)
        sixes (filter (fn [[ct _]] (= ct 6)) counts)

        ;; There's got to be a better way to do this mess
        two (->
              (filter (fn [[ct val]]
                        (= (set diff)
                           (set/intersection (set val) (set diff))))
                      fives)
              (first)
              (last))
        five (->
               (filter (fn [[ct val]]
                         (= (set eight)
                            (set/union (set two) (set val))))
                       fives)
               (first)
               (last))
        three (-> (filter (fn [[_ v]]
                            (and (not= v two) (not= v five)))
                          fives)
                  (first)
                  (last))
        six (-> (filter (fn [[_ val]]
                          (= (set eight)
                             (set/union (set (get-string-for-count #{3})) (set val))))
                        sixes)
                (first)
                (last))
        zero (-> (filter (fn [[_ v]]
                           (and (not= v six)
                                (= (set diff)
                                   (set/intersection (set v) (set diff)))))
                         sixes)
                 (first)
                 (last))
        nine (-> (filter (fn [[_ v]]
                           (and (not= v zero) (not= v six)))
                         sixes)
                 (first)
                 (last))]
    (merge result {two 2 three 3 five 5 six 6 nine 9 zero 0})))

(defn generate-digits [[sample readings]]
  (let [mapping (->> (infer-mapping sample)
                     (reduce-kv (fn [acc k v] (assoc acc (str/join (sort k)) v)) {}))]
    (map (fn [val]
           (get {2 1 4 4 7 8 3 7}
                (count val)
                (get mapping (str/join (sort val))))) readings)))

(comment (apply +
                (for [data (map #(str/split % #"\|") input)]
                  (-> (generate-digits (map #(str/split % #" ") data))
                      (str/join)
                      (Integer/parseInt)))))