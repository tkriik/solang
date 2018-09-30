(def run
  (fn (x)
    (if (= x 0)
      0
      (if (= x 1)
        1
        (+ (run (- x 1))
           (run (- x 2)))))))

(trace "run 25" (run 25))
