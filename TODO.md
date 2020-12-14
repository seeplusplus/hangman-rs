1. establish hangman client interface to standardize interface between different hangman implementations e.g., terminal, ncurses, amethyst-2d, or amethyst-3d
2. lib: expose config with all game settings i.e., max guess numbers etc so that each client gets passed this from the runtime
  + pull this out of game\_state 
  + should make unit testing easier
3. change game state internal implementations to not rely on ascii
  + we do this because we want Hello to be the same word as hello, instead change a guess for "h" to check if character is ascii then update the search logic to find 
    either "h" or "H"
