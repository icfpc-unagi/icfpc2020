#!/usr/bin/env bash


set -eu


test_message() {
    local input="${1}"
    local expected="${2}"
    local actual="$(
        echo "${input}" |
        cargo run --release --bin=evaluate_text 2>/dev/null |
        cargo run --release --bin=convert_lisp_to_text 2>/dev/null
    )"
    if [ "${expected}" != "${actual}" ]; then
        echo "### TEST FAILURE ###"
        echo "Input:"
        echo "${input}"
        echo
        echo "Expected: ${expected}"
        echo "Actual: ${actual}"
        return 1
    else
        echo "PASSED: ${input}"
    fi
}


# #5. Successor
test_message 'ap inc 0' '1'
test_message 'ap inc 1' '2'
test_message 'ap inc 2' '3'
test_message 'ap inc 3' '4'
test_message 'ap inc 300' '301'
test_message 'ap inc 301' '302'
test_message 'ap inc -1' '0'
test_message 'ap inc -2' '-1'
test_message 'ap inc -3' '-2'


# #6. Predecessor
test_message 'ap dec 1' '0'
test_message 'ap dec 2' '1'
test_message 'ap dec 3' '2'
test_message 'ap dec 4' '3'
test_message 'ap dec 1024' '1023'
test_message 'ap dec 0' '-1'
test_message 'ap dec -1' '-2'
test_message 'ap dec -2' '-3'


# #7. Sum
test_message 'ap ap add 1 2' '3'
test_message 'ap ap add 2 1' '3'
test_message 'ap ap add 0 1' '1'
test_message 'ap ap add 2 3' '5'
test_message 'ap ap add 3 5' '8'


# #8. Variables
# test_message 'ap ap add 0 x0' 'x0'
# test_message 'ap ap add 0 x1' 'x1'
# test_message 'ap ap add 0 x2' 'x2'
# test_message 'ap ap add x0 0' 'x0'
# test_message 'ap ap add x1 0' 'x1'
# test_message 'ap ap add x2 0' 'x2'
# test_message 'ap ap add x0 x1' 'ap ap add x1 x0'


# #9. Product
test_message 'ap ap mul 4 2' '8'
test_message 'ap ap mul 3 4' '12'
test_message 'ap ap mul 3 -2' '-6'
# test_message 'ap ap mul x0 x1' 'ap ap mul x1 x0'
# test_message 'ap ap mul x0 0' '0'
# test_message 'ap ap mul x0 1' 'x0'


# #10. Integer Division
test_message 'ap ap div 4 2' '2'
test_message 'ap ap div 4 3' '1'
test_message 'ap ap div 4 4' '1'
test_message 'ap ap div 4 5' '0'
test_message 'ap ap div 5 2' '2'
test_message 'ap ap div 6 -2' '-3'
test_message 'ap ap div 5 -3' '-1'
test_message 'ap ap div -5 3' '-1'
test_message 'ap ap div -5 -3' '1'
# test_message 'ap ap div x0 1' 'x0'


# #11. Equality and Booleans
# test_message 'ap ap eq x0 x0' 't'
test_message 'ap ap eq 0 -2' 'f'
test_message 'ap ap eq 0 -1' 'f'
test_message 'ap ap eq 0 0' 't'
test_message 'ap ap eq 0 1' 'f'
test_message 'ap ap eq 0 2' 'f'
test_message 'ap ap eq 1 -1' 'f'
test_message 'ap ap eq 1 0' 'f'
test_message 'ap ap eq 1 1' 't'
test_message 'ap ap eq 1 2' 'f'
test_message 'ap ap eq 1 3' 'f'
test_message 'ap ap eq 2 0' 'f'
test_message 'ap ap eq 2 1' 'f'
test_message 'ap ap eq 2 2' 't'
test_message 'ap ap eq 2 3' 'f'
test_message 'ap ap eq 2 4' 'f'
test_message 'ap ap eq 19 20' 'f'
test_message 'ap ap eq 20 20' 't'
test_message 'ap ap eq 21 20' 'f'
test_message 'ap ap eq -19 -20' 'f'
test_message 'ap ap eq -20 -20' 't'
test_message 'ap ap eq -21 -20' 'f'


# #12. Strict Less-Than
test_message 'ap ap lt 0 -1' 'f'
test_message 'ap ap lt 0 0' 'f'
test_message 'ap ap lt 0 1' 't'
test_message 'ap ap lt 0 2' 't'
test_message 'ap ap lt 1 0' 'f'
test_message 'ap ap lt 1 1' 'f'
test_message 'ap ap lt 1 2' 't'
test_message 'ap ap lt 1 3' 't'
test_message 'ap ap lt 2 1' 'f'
test_message 'ap ap lt 2 2' 'f'
test_message 'ap ap lt 2 3' 't'
test_message 'ap ap lt 2 4' 't'
test_message 'ap ap lt 19 20' 't'
test_message 'ap ap lt 20 20' 'f'
test_message 'ap ap lt 21 20' 'f'
test_message 'ap ap lt -19 -20' 'f'
test_message 'ap ap lt -20 -20' 'f'
test_message 'ap ap lt -21 -20' 't'


# #16. Negate
test_message 'ap neg 0' '0'
test_message 'ap neg 1' '-1'
test_message 'ap neg -1' '1'
test_message 'ap neg 2' '-2'
test_message 'ap neg -2' '2'


# #17. Function Application
test_message 'ap inc ap inc 0' '2'
test_message 'ap inc ap inc ap inc 0' '3'
# test_message 'ap inc ap dec x0' 'x0'
# test_message 'ap dec ap inc x0' 'x0'
# test_message 'ap dec ap ap add x0 1' 'x0'
test_message 'ap ap add ap ap add 2 3 4' '9'
test_message 'ap ap add 2 ap ap add 3 4' '9'
test_message 'ap ap add ap ap mul 2 3 4' '10'
test_message 'ap ap mul 2 ap ap add 3 4' '14'
# test_message 'inc' 'ap add 1'
# test_message 'dec' 'ap add ap neg 1'


# #18. S Combinator
# test_message 'ap ap ap s x0 x1 x2' 'ap ap x0 x2 ap x1 x2'
test_message 'ap ap ap s add inc 1' '3'
test_message 'ap ap ap s mul ap add 1 6' '42'


# #19. C Combinator
# test_message 'ap ap ap c x0 x1 x2' 'ap ap x0 x2 x1'
test_message 'ap ap ap c add 1 2' '3'


# #20. B Combinator
# test_message 'ap ap ap b x0 x1 x2' 'ap x0 ap x1 x2'
# test_message 'ap ap ap b inc dec x0' 'x0'


# #21. True (K Combinator)
test_message 'ap ap t x0 x1' 'x0'
test_message 'ap ap t 1 5' '1'
test_message 'ap ap t t i' 't'
test_message 'ap ap t t ap inc 5' 't'
test_message 'ap ap t ap inc 5 t' '6'


# #22. False
test_message 'ap ap f x0 x1' 'x1'
# test_message 'f' 'ap s t'


# #24. I Combinator
test_message 'ap i x0' 'x0'
test_message 'ap i 1' '1'
test_message 'ap i i' 'i'
test_message 'ap i add' 'add'
# test_message 'ap i ap add 1' 'ap add 1'


# #25. Cons (or Pair)
# test_message 'ap ap ap cons x0 x1 x2' 'ap ap x2 x0 x1'


# #26. Car (First)
test_message 'ap car ap ap cons x0 x1' 'x0'
# test_message 'ap car x2' 'ap x2 t'


# #27. Cdr (Tail)
test_message 'ap cdr ap ap cons x0 x1' 'x1'
# test_message 'ap cdr x2' 'ap x2 f'


# #28. Nil (Empty List)
test_message 'ap nil x0' 't'


# #29. Is Nil (Is Empty List)
test_message 'ap isnil nil' 't'
test_message 'ap isnil ap ap cons x0 x1' 'f'


# #31. Vector
# test_message 'vec' 'cons'


# #34. Multiple Draw
# test_message 'ap multipledraw nil' 'nil'
# test_message 'ap multipledraw ap ap cons x0 x1' 'ap ap cons ap draw x0 ap multipledraw x1'


# #37. Is 0
test_message 'ap ap ap if0 0 x0 x1' 'x0'
test_message 'ap ap ap if0 1 x0 x1' 'x1'
