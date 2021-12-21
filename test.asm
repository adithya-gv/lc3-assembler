.orig x3000

;;put your code here

; Load the argument A into Register 0 as the value to be added.
LD R0, A 

; Load the argument A into Register 1 as the counter.
LD R1, A

; Clear R2 to store answer in R2
AND R2, R2, #0

; Set R3 to be the value of -1 just for ease
AND R3, R3, #0
ADD R3, R3, #1 ; R3 = 1
NOT R3, R3 ; Flip the bits
ADD R3, R3, #1 ; Add one. R3 = -1

; Add A to R2 A times. Decrement R1 by R3 = -1 at each step. 
; If R1 is positive, we haven't looped enough, branch back to loop. 
LOOP    ADD R2, R2, R0 ; R2 = R2 + R0
        ADD R1, R1, R3 ; R1 = R1 - 1;
        BRp LOOP

;Store the result (in R2) into Answer
ST R2, ANSWER

HALT


A .fill 9

ANSWER .fill 1

.end