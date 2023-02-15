# Vending Machine
The program should have the following features:
- The user should be able to select a product from a list of available options.
- The program should display the price of the selected product.
- The user should be prompted to enter the required amount of money.
- If the user enters an amount that is less than the price of the selected product, the program should display an error message.
- If the user enters an amount that is equal to or greater than the price of the selected product, the program should display a message indicating that the product has been dispensed and any change due to the user.

I uses a Product struct to represent each available product, and stores them in a vector. The program then prompts the user to select a product, and reads their selection and payment from the command line. Finally, the program displays an appropriate message based on the amount of payment provided.

## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
