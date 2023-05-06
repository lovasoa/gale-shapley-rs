# gale-shapley-rs

The code implements the Gale-Shapley algorithm for solving the stable marriage problem, which is a classic problem in game theory and economics.
The problem involves matching a set of men and women into stable marriages,
where a stable marriage is defined as one where there are no two people of opposite genders who would both prefer each other over their current partners.

## Algorithm

The algorithm works as follows: first, each man proposes to the woman he prefers the most. If the woman is not engaged, she accepts the proposal and the two become engaged.
If the woman is already engaged to another man, she compares her current partner with the new proposal and chooses the man she prefers. The man who was rejected becomes free to propose to his next preferred woman. This process continues until all men are engaged, at which point the algorithm terminates and the current engagements are returned as the stable marriage.

The algorithm is guaranteed to terminate and find a stable marriage for any input, and the output is also guaranteed to be unique.


## Command-line usage

```sh
$ ./galeshapley
Joe: Jane Isabelle
Jack: Isabelle Jane
Isabelle: Joe Jack
Jane: Joe Jack

Joe: Jane
Jack: Isabelle
```

## Programmatic usage

### Implementation
The GaleShapley struct represents the algorithm itself, and it has several methods that implement the different steps of the algorithm.
The `init` method is used to initialize the data structures needed for the algorithm, such as the men and women preferences.
The `find_stable_marriage` method runs the algorithm and returns the final stable marriage.

### Example

```rs
let men_preferences = vec![vec![0, 1], vec![0, 1]]; // both men prefer woman 0
let women_preferences = vec![vec![1, 0], vec![1, 0]]; // both women prefer man 1
        
let stable_marriages: Vec<(Man, Woman)> = GaleShapley::init(men_preferences, women_preferences)
            .find_stable_marriage()
            .collect();
```
