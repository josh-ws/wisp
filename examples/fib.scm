; prints first 20 fibonacci numbers

(define fib
  (lambda (n)
    (if (< n 2)
        n
        (+ (fib (- n 1)) (fib (- n 2))))))

(define range
  (lambda (a b)
    (if (< b a)
        '()
        (cons a (range (+ a 1) b)))))

(map fib (range 1 20))
