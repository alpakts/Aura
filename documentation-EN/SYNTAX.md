# 📘 Aura (.aur) Programming Language Syntax Guide

This document describes the rules and usage supported in the current version of the **Aura** programming language. Aura is a high-performance, 64-bit compiled language with native MVC capabilities.

## 1. Variable Declaration
Variables are defined using the `var` keyword. Aura uses Type Inference for initial assignments.

```aura
var x = 10
var name = "Alper"
var active = 1
```

## 2. Arrays
Arrays are 64-bit structures defined with square brackets `[]`.

```aura
var numbers = [10, 20, 30]
print(numbers[0]) // Prints 10

var users = [user1, user2] // Array of class instances
```

## 3. Object Oriented Programming (OOP)
Aura supports classes with fields and methods. All instances are handled as 64-bit pointers.

```aura
class User {
    var id;
    var name;

    func init(uId, uName) {
        this.id = uId;
        this.name = uName;
    }

    func sayHi() {
        print_str("Hi, I am ");
        print_str(this.name);
    }
}

var u = new User();
u.init(1, "Aura AI");
u.sayHi();
```

## 4. Web & MVC Engine (Built-in)
Aura has a native high-performance template engine for web applications.

### File reading
```aura
var tpl = read_file("views/index.html");
```

### Template Rendering
Use `render` to bind a single object to an HTML template using `{model.field}` tags.

```aura
var html = render(tpl, userInstance);
```

### List Rendering (AuraView Engine)
Use `render_list` to render an array of objects recursively. It replaces a specific tag with the rendered items using an item template.

```aura
// render_list(mainTpl, targetTag, dataArray, itemTpl)
var listHtml = render_list(tpl, "{users_list}", users, itemTpl);
```

## 5. Printing
* `print(value)`: Prints numbers (i64).
* `print_str(text)`: Prints strings or pointers.

## 6. Control Flow
Standard `if`, `else if`, `else`, `while`, and `for` (C-style) loops are supported.

## 7. Memory & Architecture
* **64-Bit:** All integers and pointers are 64-bit (`i64`).
* **Low Level:** Compiles directly to LLVM IR and then to native machine code via Clang.
