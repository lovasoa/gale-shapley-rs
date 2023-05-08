# gale-shapley-rs

The code implements the Gale-Shapley algorithm for solving the stable marriage problem, which is a classic problem in game theory and economics.
The problem involves matching a set of men and women into stable marriages,
where a stable marriage is defined as one where there are no two people of opposite genders who would both prefer each other over their current partners.

## Algorithm

The algorithm works as follows: first, each man proposes to the woman he prefers the most. If the woman is not engaged, she accepts the proposal and the two become engaged.
If the woman is already engaged to another man, she compares her current partner with the new proposal and chooses the man she prefers. The man who was rejected becomes free to propose to his next preferred woman. This process continues until all men are engaged, at which point the algorithm terminates and the current engagements are returned as the stable marriage.

The algorithm is guaranteed to terminate and find a stable marriage for any input, and the output is also guaranteed to be unique.


## Command-line usage

### Solve a problem in textual form

Run the program from the terminal

```sh
$ ./galeshapley
```

Type your problem in the following format: 

```
Joe: Jane Isabelle
Jack: Isabelle Jane
Isabelle: Joe Jack
Jane: Joe Jack
```

Then leave a blank line, and wait for the reponse in the form

```
Joe: Jane
Jack: Isabelle
```

### Compute statistics

Computes the probability of a man to gets his first choice as a stable mariage in a problem of N men and N women.

run

```
./galeshapley 500
```

and it will display the results as it computes them

```
Solving problems with 500 men and 500 women with random preferences.
Success rate for the first man (got first choice / total samples) and 95% confidence interval :
   844   /  5658   = 14.90  ± 0.9  %
```

## Programmatic usage

### Implementation

The [`GaleShapley` struct](https://github.com/lovasoa/gale-shapley-rs/blob/main/src/lib.rs#L4-L15) represents the algorithm itself, and it has several methods that implement the different steps of the algorithm.
The `init` method is used to initialize the data structures needed for the algorithm, such as the men and women preferences.
The `find_stable_marriage` method runs the algorithm and returns the final stable marriage.

The implementation lets the user drive the algorithm on their own and can return to user code after each proposal round.

It also provides a [`has_stable_mariage_with(m: Man, w: Woman)`](https://github.com/lovasoa/gale-shapley-rs/blob/main/src/lib.rs#L113) method,
that allows computing whether a given mariage is in the solution faster than if we were to compute the entire solution and then extract that information from it.

### Example

```rs
let men_preferences = vec![vec![0, 1], vec![0, 1]]; // both men prefer woman 0
let women_preferences = vec![vec![0, 1], vec![1, 0]]; // woman 0 prefers man 0, woman 1 prefers man 1
        
let stable_marriages: Vec<(Man, Woman)> = GaleShapley::init(men_preferences, women_preferences)
            .find_stable_marriage()
            .collect();
```
