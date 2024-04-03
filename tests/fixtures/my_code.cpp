#include <iostream>

unsigned long long factorial(int n) {
    if (n == 0)
        return 1;
    else
        return n * factorial(n - 1);
}

int main() {
    int number;
    std::cout << "Enter a positive integer: ";
    std::cin >> number;
    std::cout << "Factorial of " << number << " = " << factorial(number);
    std::cout << std::endl;
    return 0;
}
