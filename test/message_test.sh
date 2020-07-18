#!/usr/bin/env bash


set -eu


test_message() {
    local input="${1}"
    local expected="${2}"
    local actual="$(
        echo "${input}" |
        cargo run --release --bin=galaxy 2>/dev/null)"
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
# TODO(imos): inc is not implemented.
# test_message 'galaxy = ap inc 0' 'galaxy = ((x0 x2) x1)'
# test_message 'galaxy = ap inc 1' 'galaxy = ((x0 x2) x1)'
# test_message 'galaxy = ap inc 100' 'galaxy = ((x0 x2) x1)'
# test_message 'galaxy = ap inc -1' 'galaxy = ((x0 x2) x1)'
# test_message 'galaxy = ap inc -100' 'galaxy = ((x0 x2) x1)'


# #6. Predecessor
# test_message 'galaxy = ap dec 0' 'galaxy = ((x0 x2) x1)'
# test_message 'galaxy = ap dec 1' 'galaxy = ((x0 x2) x1)'
# test_message 'galaxy = ap dec 100' 'galaxy = ((x0 x2) x1)'
# test_message 'galaxy = ap dec -1' 'galaxy = ((x0 x2) x1)'
# test_message 'galaxy = ap dec -100' 'galaxy = ((x0 x2) x1)'


# #7. Sum
test_message 'galaxy = ap ap add 1 2' 'galaxy = 3'
test_message 'galaxy = ap ap add 2 1' 'galaxy = 3'
test_message 'galaxy = ap ap add 0 1' 'galaxy = 1'
test_message 'galaxy = ap ap add 2 3' 'galaxy = 5'
test_message 'galaxy = ap ap add 3 5' 'galaxy = 8'


# #8. Variables
# test_message 'galaxy = ap ap add 0 x0' 'galaxy = x0'
# test_message 'galaxy = ap ap add 0 x1' 'galaxy = x1'
# test_message 'galaxy = ap ap add 0 x2' 'galaxy = x2'
# test_message 'galaxy = ap ap add x0 0' 'galaxy = x0'
# test_message 'galaxy = ap ap add x1 0' 'galaxy = x1'
# test_message 'galaxy = ap ap add x2 0' 'galaxy = x2'
# test_message 'galaxy = ap ap add x0 x1' 'galaxy = ap ap add x1 x0'


# #9. Product
test_message 'galaxy = ap ap mul 4 2' 'galaxy = 8'
test_message 'galaxy = ap ap mul 3 4' 'galaxy = 12'
test_message 'galaxy = ap ap mul 3 -2' 'galaxy = -6'
# test_message 'galaxy = ap ap mul x0 x1' 'galaxy = ap ap mul x1 x0'
# test_message 'galaxy = ap ap mul x0 0' 'galaxy = 0'
# test_message 'galaxy = ap ap mul x0 1' 'galaxy = x0'


# #10. Integer Division
test_message 'galaxy = ap ap div 4 2' 'galaxy = 2'
test_message 'galaxy = ap ap div 4 3' 'galaxy = 1'
test_message 'galaxy = ap ap div 4 4' 'galaxy = 1'
test_message 'galaxy = ap ap div 4 5' 'galaxy = 0'
test_message 'galaxy = ap ap div 5 2' 'galaxy = 2'
test_message 'galaxy = ap ap div 6 -2' 'galaxy = -3'
test_message 'galaxy = ap ap div 5 -3' 'galaxy = -1'
test_message 'galaxy = ap ap div -5 3' 'galaxy = -1'
test_message 'galaxy = ap ap div -5 -3' 'galaxy = 1'
# test_message 'galaxy = ap ap div x0 1' 'galaxy = x0'


# #11. Equality and Booleans
# test_message 'galaxy = ap ap eq x0 x0' 'galaxy = t'
test_message 'galaxy = ap ap eq 0 -2' 'galaxy = f'
test_message 'galaxy = ap ap eq 0 -1' 'galaxy = f'
test_message 'galaxy = ap ap eq 0 0' 'galaxy = t'
test_message 'galaxy = ap ap eq 0 1' 'galaxy = f'
test_message 'galaxy = ap ap eq 0 2' 'galaxy = f'
test_message 'galaxy = ap ap eq 1 -1' 'galaxy = f'
test_message 'galaxy = ap ap eq 1 0' 'galaxy = f'
test_message 'galaxy = ap ap eq 1 1' 'galaxy = t'
test_message 'galaxy = ap ap eq 1 2' 'galaxy = f'
test_message 'galaxy = ap ap eq 1 3' 'galaxy = f'
test_message 'galaxy = ap ap eq 2 0' 'galaxy = f'
test_message 'galaxy = ap ap eq 2 1' 'galaxy = f'
test_message 'galaxy = ap ap eq 2 2' 'galaxy = t'
test_message 'galaxy = ap ap eq 2 3' 'galaxy = f'
test_message 'galaxy = ap ap eq 2 4' 'galaxy = f'
test_message 'galaxy = ap ap eq 19 20' 'galaxy = f'
test_message 'galaxy = ap ap eq 20 20' 'galaxy = t'
test_message 'galaxy = ap ap eq 21 20' 'galaxy = f'
test_message 'galaxy = ap ap eq -19 -20' 'galaxy = f'
test_message 'galaxy = ap ap eq -20 -20' 'galaxy = t'
test_message 'galaxy = ap ap eq -21 -20' 'galaxy = f'


# #12. Strict Less-Than
test_message 'galaxy = ap ap lt 0 -1' 'galaxy = f'
test_message 'galaxy = ap ap lt 0 0' 'galaxy = f'
test_message 'galaxy = ap ap lt 0 1' 'galaxy = t'
test_message 'galaxy = ap ap lt 0 2' 'galaxy = t'
test_message 'galaxy = ap ap lt 1 0' 'galaxy = f'
test_message 'galaxy = ap ap lt 1 1' 'galaxy = f'
test_message 'galaxy = ap ap lt 1 2' 'galaxy = t'
test_message 'galaxy = ap ap lt 1 3' 'galaxy = t'
test_message 'galaxy = ap ap lt 2 1' 'galaxy = f'
test_message 'galaxy = ap ap lt 2 2' 'galaxy = f'
test_message 'galaxy = ap ap lt 2 3' 'galaxy = t'
test_message 'galaxy = ap ap lt 2 4' 'galaxy = t'
test_message 'galaxy = ap ap lt 19 20' 'galaxy = t'
test_message 'galaxy = ap ap lt 20 20' 'galaxy = f'
test_message 'galaxy = ap ap lt 21 20' 'galaxy = f'
test_message 'galaxy = ap ap lt -19 -20' 'galaxy = f'
test_message 'galaxy = ap ap lt -20 -20' 'galaxy = f'
test_message 'galaxy = ap ap lt -21 -20' 'galaxy = t'


# #16. Negate
test_message 'galaxy = ap neg 0' 'galaxy = 0'
test_message 'galaxy = ap neg 1' 'galaxy = -1'
test_message 'galaxy = ap neg -1' 'galaxy = 1'
test_message 'galaxy = ap neg 2' 'galaxy = -2'
test_message 'galaxy = ap neg -2' 'galaxy = 2'


# #17. Function Application
# test_message 'galaxy = ap inc ap inc 0' 'galaxy = 2'
# test_message 'galaxy = ap inc ap inc ap inc 0' 'galaxy = 3'
# test_message 'galaxy = ap inc ap dec x0' 'galaxy = x0'
# test_message 'galaxy = ap dec ap inc x0' 'galaxy = x0'
# test_message 'galaxy = ap dec ap ap add x0 1' 'galaxy = x0'
test_message 'galaxy = ap ap add ap ap add 2 3 4' 'galaxy = 9'
test_message 'galaxy = ap ap add 2 ap ap add 3 4' 'galaxy = 9'
test_message 'galaxy = ap ap add ap ap mul 2 3 4' 'galaxy = 10'
test_message 'galaxy = ap ap mul 2 ap ap add 3 4' 'galaxy = 14'
# test_message 'galaxy = inc' 'galaxy = ap add 1'
# test_message 'galaxy = dec' 'galaxy = ap add ap neg 1'


# #18. S Combinator
# test_message 'galaxy = ap ap ap s x0 x1 x2' 'galaxy = ap ap x0 x2 ap x1 x2'
# test_message 'galaxy = ap ap ap s add inc 1' 'galaxy = 3'
test_message 'galaxy = ap ap ap s mul ap add 1 6' 'galaxy = 42'


# #19. C Combinator
# test_message 'galaxy = ap ap ap c x0 x1 x2' 'galaxy = ap ap x0 x2 x1'
test_message 'galaxy = ap ap ap c add 1 2' 'galaxy = 3'


# #20. B Combinator
# test_message 'galaxy = ap ap ap b x0 x1 x2' 'galaxy = ap x0 ap x1 x2'
# test_message 'galaxy = ap ap ap b inc dec x0' 'galaxy = x0'


# #21. True (K Combinator)
test_message 'galaxy = ap ap t x0 x1' 'galaxy = x0'
test_message 'galaxy = ap ap t 1 5' 'galaxy = 1'
test_message 'galaxy = ap ap t t i' 'galaxy = t'
# test_message 'galaxy = ap ap t t ap inc 5' 'galaxy = t'
# test_message 'galaxy = ap ap t ap inc 5 t' 'galaxy = 6'


# #22. False
test_message 'galaxy = ap ap f x0 x1' 'galaxy = x1'
# test_message 'galaxy = f' 'galaxy = ap s t'


# #24. I Combinator
test_message 'galaxy = ap i x0' 'galaxy = x0'
test_message 'galaxy = ap i 1' 'galaxy = 1'
test_message 'galaxy = ap i i' 'galaxy = i'
test_message 'galaxy = ap i add' 'galaxy = add'
# test_message 'galaxy = ap i ap add 1' 'galaxy = ap add 1'


# #25. Cons (or Pair)
# test_message 'galaxy = ap ap ap cons x0 x1 x2' 'galaxy = ap ap x2 x0 x1'


# #26. Car (First)
test_message 'galaxy = ap car ap ap cons x0 x1' 'galaxy = x0'
# test_message 'galaxy = ap car x2' 'galaxy = ap x2 t'


# #27. Cdr (Tail)
test_message 'galaxy = ap cdr ap ap cons x0 x1' 'galaxy = x1'
# test_message 'galaxy = ap cdr x2' 'galaxy = ap x2 f'


# #28. Nil (Empty List)
# 気になる
# test_message 'galaxy = ap nil x0' 'galaxy = t'


# #29. Is Nil (Is Empty List)
# test_message 'galaxy = ap isnil nil' 'galaxy = t'
# test_message 'galaxy = ap isnil ap ap cons x0 x1' 'galaxy = f'


# #31. Vector
# test_message 'galaxy = vec' 'galaxy = cons'


# #34. Multiple Draw
# test_message 'galaxy = ap multipledraw nil' 'galaxy = nil'
# test_message 'galaxy = ap multipledraw ap ap cons x0 x1' 'galaxy = ap ap cons ap draw x0 ap multipledraw x1'


# #37. Is 0
# test_message 'galaxy = ap ap ap if0 0 x0 x1' 'galaxy = x0'
# test_message 'galaxy = ap ap ap if0 1 x0 x1' 'galaxy = x1'
