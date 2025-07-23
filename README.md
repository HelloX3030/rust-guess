# rust-guess

Guessing Game Project Roadmap
Phase 1: Setup and Basics

    Goal: Get familiar with Rust project structure, basic input/output, and control flow.

    Create a new Rust project with cargo new guessing_game

    Write a simple program to:

        Print a welcome message.

        Read user input from the terminal.

        Print the input back to the user.

Focus:

    println! macro

    std::io::stdin().read_line()

    Basic variable binding and mutability

Phase 2: Implement the Guessing Game Core Loop

    Goal: Implement the game logic loop.

    Generate a random secret number between 1 and 100 (learn to use rand crate).

    Ask the user to input a guess.

    Read user input and convert it to a number.

    Use match or if to compare the guess with the secret number.

    Print hints ("Too small!", "Too big!", "You win!") accordingly.

    Loop until the user guesses correctly.

Focus:

    match statement usage for handling guess comparison.

    Crate usage and dependencies (rand).

    Looping constructs (loop, break).

Phase 3: Error Handling & Input Validation

    Goal: Make the program robust against invalid input.

    Handle cases where the user inputs non-numeric or invalid data.

    Use Result and pattern matching to handle parsing errors gracefully.

    Prompt the user again instead of crashing when input is invalid.

Focus:

    Result<T, E> enum handling with match.

    Error handling best practices.

    Avoiding unwrap(); prefer graceful recovery.

Phase 4: Code Cleanup and Modularity

    Goal: Organize code for readability and maintainability.

    Split the code into functions:

        A function to get and validate user input.

        A function to compare the guess and secret number.

        Main game loop that calls these functions.

    Consider using comments and docstrings for clarity.

Focus:

    Functions and parameters in Rust.

    Code organization and reuse.

    Documentation basics (/// comments).

Phase 5: Add Enhancements (Optional)

    Add a counter for number of guesses and display it when the user wins.

    Allow the user to choose difficulty level (range of secret number).

    Add a replay option after winning.

    Explore using enums for game state or user commands (like quit).

Focus:

    More complex control flow and state handling.

    Enums and pattern matching.

    User experience improvements.

Final Deliverables

    A fully working, user-friendly guessing game.

    Clean, well-commented Rust code.

    Demonstrated ability to handle user input, errors, and control flow idiomatically.