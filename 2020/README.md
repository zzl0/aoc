## Introduction

https://adventofcode.com/2020


## Notes

* Day 17: Conway Cubes
* Day 18: Operation Order
* Day 19: Monster Messages


### Day 21: Allergen Assessment


You don't speak the local language, so you can't read any ingredients lists. However, sometimes,
allergens are listed in a language you do understand. You should be able to use this information
to determine *which ingredient contains which allergen* and work out which foods are safe to
take with you on your trip.

Constraints:
1. Each allergen is found in exactly one ingredient.
2. Each ingredient contains zero or one allergen.
3. Allergens aren't always marked.

Example foods:
```
mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)
```

Since there is a global 1-1 mapping between ingredients and allergens. So if an allergen appears in
a food, then the corresponding ingredient must in the ingredients of the food. E.g., in above example,
`dairy` is in both first and second foods, then the corresponding ingredient must in both ingredients
as well, since the `mxmxvkd` is the only common ingredient, then we have found an mapping
(`dairy` -> `mxmxvkd`). We can also use this rules to eliminate possible values for each allergen.
