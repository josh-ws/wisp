(define > (lambda (a b) (< b a)))
(define <= (lambda (a b) (not (< b a))))
(define >= (lambda (a b) (not (< a b))))

(define zero? (lambda (x) (= x 0)))
(define positive? (lambda (x) (< 0 x)))
(define negative? (lambda (x) (< x 0)))

(define abs (lambda (x) (if (negative? x) (- 0 x) x)))
(define square (lambda (x) (* x x)))
(define cube (lambda (x) (* x (square x))))

(define max (lambda (a b) (if (< a b) b a)))
(define min (lambda (a b) (if (< a b) a b)))

(define map
  (lambda (f xs)
    (if (null? xs) '() (cons (f (car xs)) (map f (cdr xs))))))

(define filter
  (lambda (pred xs)
    (if (null? xs) '() 
        (if (pred (car xs))
            (cons (car xs) (filter pred (cdr xs)))
            (filter pred (cdr xs))))))

(define fold-left
  (lambda (f acc xs)
    (if (null? xs) acc (fold-left f (f acc (car xs)) (cdr xs)))))

(define not (lambda (x) (if x #f #t)))

(define equal?
  (lambda (a b)
    (if (atom? a)
        (if (atom? b) (eq? a b) #f)
        (if (atom? b) #f
            (if (equal? (car a) (car b))
                (equal? (cdr a) (cdr b))
                #f)))))
