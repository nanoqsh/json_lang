## Json Lang
Do you want to program in JSON? This simple executor allows you to do that.

### Constuctions
The instruction block is an JSON array. It can list instructions that will be performed sequentially. The last operation is the result of the block expression. If the block is empty, then the result is `undefined`.
```json
[]
```

Numbers are just JSON numbers:
```json
12
```

To read a variable specify a regular JSON string. If the variable is not set, then it has the value `undefined`.
```json
"x"
```

To create a string, you need to specify the `str` key:
```json
{ "str": "hello" }
```

The `let` block allows you to assign names to values:
```json
{
    "let": {
        "x": 1,
        "y": 2
    }
}
```
Variables have their scope limited of a function call or global scope. So, creating a new variable inside a function will not change a more global variable.

To print a value, specify the `print` key:
```json
{
    "print": { "str": "Hello, World!" }
}
```

Arithmetic operators like `+`, `-`, `*`, `/` or the comparison operator `==` can be expressed as:
```json
{ "+": [12, "x"] }
```
The result of this expression will be a sum of value `12` and the variable `x`. If an operation cannot be evaluated, its value will be `undefined`. Note that the JSON array here is not a block of instructions, but simply a short notation of two operans instead of `{ "left": 12, "right": "x" }`.

How to create a function? Just:

```json
{ "fn": 7 }
```
This creates an anonymous function that returns `7`. Let's call it by passing an empty map of parameters, since this function does not take any arguments:
```json
{
    "call": { "fn": 7 },
    "pars": {}
}
```
The `call` key specifies a function to call. The `pars` key specifies a function parameters, if it's empty then the key can be omitted.

What if you need to give a name to a function? So, let's use the `let` block:
```json
{
    "let": {
        "function_name": { "fn": 7 }
    }
}
```

Then call it by name:
```json
{ "call": "function_name" }
```

Good. Let's write some more complex calculation. For example, calculate `1 + x`:
```json
{
    "fn": { "+": [1, "x"] }
}
```
If you simply call this function without parameters, then the value of `x` will be determined in an outer scope. But we can set this value when calling by passing it as a parameter:
```json
{
    "call": {
        "fn": { "+": [1, "x"] }
    },
    "pars": { "x": 2 }
}
```

The conditional operator will return the `then` value if the `if` value is non-zero or non-empty string, otherwise `else`:
```json
{
    "if": { "==": [1, 1] },
    "then": 1,
    "else": 2
}
```
