# number-converter
converts a number into a minimal form of the prime factorization of that number.

number conversion between numbers (just whole numbers) and a prime factorization based system i call nampa (inspired by toki pona's wan tu mute)

it works like this:

take a number {42}

find a set of numbers the are all prime and sum to the number {2 * 3 * 7}

break down any that are larger into the prime factors of that number - 1 {2 * 3 * (2*3+1)}

rephrase to use implicit multiplication and remove the operators {2(3(2(3)1))}

all parentheses after the last number that isnt a one are closing, and visa versa, so they can be made straight {2|3|2|3|1||}

convert 1, 2, 3 to ·, :, ⋮ {:|⋮|:|⋮|·} or {:⋮:⋮·}

trailing lines are implied

bonus: if a number can be represented in a way other than its prime factorization like {2(2(2))1: 9} or {3(3)}, then the one can be moved out one more bracket. so 17 is 2|2|2|2||1 ({:|:|:|:||·} or {}:::: ·}) -> 2(2(2(2))1) but 2(2(2))1 is 
not correct so -> 2(2(2()))1

to convert back:

take a number {:⋮:⋮·}

insert lines between every character {:|⋮|:|⋮|·}

replace lines with opening parens when before the last non 1 character {:(⋮(:(⋮)·}

close parens {:(⋮(:(⋮)·))}

replace with numbers {2(3(2(3)1))}

replace 2 and 3 with 2* and 3*, and 1 with +1 {2*(3*(2*(3)+1))}

evaluate left to right instead of the order of operations {2\*3\*7} -> {2\*21} -> {42}

voila!
