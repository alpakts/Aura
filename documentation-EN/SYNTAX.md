# 📘 .aa Programming Language Syntax Guide

This document describes the rules and usage supported in the current version of the **.aa** programming language.

## 1. Variable Declaration
Variables are defined using the `var` keyword. Type specification is not needed (Type Inference).

```aa
var x = 10
var name = "Alice"
var active = 1
```

## 2. Arrays
Arrays are defined with square brackets `[]` and accessed using an index `[i]`.

```aa
var numbers = [10, 20, 30]

print(numbers[0]) // Prints 10
var x = numbers[1] + 5
```

## 3. Mathematical Operations
Standard operations and operator precedence are supported.

* `+`, `-`, `*`, `/`

```aa
var a = (10 + 5) * 2
```

## 4. Printing to Screen
There are two distinct printing functions:

* `print(value)`: Prints numbers or numeric expressions.
* `print_str(text)`: Prints text (String Literals or String Variables).

```aa
print(100)               // Prints number
print_str("Hello")       // Prints text

var message = "Hi"
print_str(message)       // Prints variable content
```

## 5. Conditional Statements (If / Else If / Else)
Classic `if` structure is supported. Extended `else if` chains can be used.

```aa
var grade = 75

if (grade > 90) {
    print_str("Excellent")
} else if (grade > 50) {
    print_str("Passed")
} else {
    print_str("Failed")
}
```

## 6. Loops

### While Loop
Runs as long as the condition is true.

```aa
var i = 0
while (i < 5) {
    print(i)
    i = i + 1
}
```

### For Loop
C-style `for` loop is supported: `for (init; condition; step)`.

```aa
for (var k = 0; k < 10; k = k + 1) {
    print(k)
}
```

## 7. Functions
Functions are defined with `func`, can take parameters, and return values using `return`.

```aa
func add(x, y) {
    return x + y
}

var result = add(10, 20)
print(result) // 30
```

### Function Tips:
* Variables defined inside a function are local (Local Scope).
* String parameters can be passed to functions (must be printed with `print_str`).

```aa
func greet(name) {
    print_str("Hello")
    print_str(name)
}

greet("John")
```

## 8. Comments
Single-line comments start with `//`.

```aa
// This is a comment line
var x = 1 // Can also be written here
```
