# Using the complete list of possible preference configurations for a given n and the resulting matchings,
# the aim of this script is to look at the probability of success for a random woman

# Number of women's success for each configuration
gale_3$successes_women <- ifelse(gale_3$woman_1_mariage == gale_3$woman_1_preference_1, 1, 0) +
  ifelse(gale_3$woman_2_mariage == gale_3$woman_2_preference_1, 1, 0) +
  ifelse(gale_3$woman_3_mariage == gale_3$woman_3_preference_1, 1, 0)

n <- 3
# List of blocks (each block groups configurations where men's prefs are fixed)
blocks <- c(1:(factorial(n))**(n-1))
# Length of each block
length_block <- factorial(n)**n
success_blocks <-c()
# Computation of probability of success for a random woman in each block
for(i in blocks){
  range_i <- c(1:length_block)+(i-1)*length_block
  success_blocks[i] <- mean(gale_3$successes_women[range_i])/3
}
summary(success_blocks)