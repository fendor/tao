# Tao

Tao is a statically-typed functional programming language.

## Example

```
# Quicksort
def sort = l ->
    if l:len <= 1
    then l
    else
        let mid = l:nth(l:len / 2) in
        l:filter(x -> x < mid):sort ++
        l:filter(x -> x = mid) ++
        l:filter(x -> x > mid):sort

[45, 75, 98, 24, 10, 12, 60, 32, 17, 41]:sort
```

See `examples/` for more example programs.

## Features

- Functional and pure
- Currying
- Static type system
- Hindley-Milney type inference
- Complex types (lists, tuples, functions)
- Useful error messages
- Bytecode compilation
- Pattern matching (incomplete)
- Sum types (incomplete)
- Type parameters (incomplete)
- Monadic I/O (incomplete)

## Static typing

Tao has a static type system and type inference.
It supports complex types such as functions and lists.
Below are some examples of types that can be represented in Tao.

- `Num` / `String` / `Bool`

- `Num -> Num` / `String -> Bool -> Num` / `(Num -> Num) -> Bool`

- `List Num` / `List String`

- `Num -> List Num -> Bool`

- `(Num, Num)` / `(Str, (Bool, Num -> Num))`

## Declarations

Tao supports top-level type, data structure, and value definition declarations.
Note that many of these are not yet supported.
Below are some examples of these.

*Recursive function*

```
def factorial = x ->
	if x = 0
	then 1
	else x * factorial(x - 1)
```

*Maybe type*

```
data Maybe A =
	| Just A
	| Nil
```

*Cons list type*

```
data List A =
	| Item (A, List A)
	| Nil
```

*Record type*

```
data Person =
	.name String,
	.age  Num,
	.address Maybe Num,
```

*Type alias*

```
type NonEmpty A = (A, List A)
```

## Error Messages

Tao aims to have useful error messages. Below are a few examples.

```
Error: Type mismatch between 'Num' and 'String'
-> line 1, column 2
   1 | (x -> x + 3)("test")
        ^           ^^^^^^
```

```
Error: No such binding 'bar' in the current scope
-> line 1, column 22
   1 | let foo = 5 in foo + bar
                            ^^^
```
