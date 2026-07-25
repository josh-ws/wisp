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
