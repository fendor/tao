# Tao

Tao is a statically-typed functional programming language.

## Example

```
def len A = |xs of [A]| match xs in
	| [] => 0
	| [_, ...tail] => 1 + tail:len
```

See `examples/` for more example programs.

## Features

- Functional and pure
- Currying
- Static type system
- Hindley-Milney type inference
- Complex types (lists, tuples, functions, data types)
- Useful error messages
- Type parameters
- Bytecode compilation
- Pattern matching
- Sum types (incomplete)
- Monadic I/O (incomplete)

## Status

Tao is currently in heavy development and many aspects (particularly the compiler backend) are very unfinished.
In addition, the compile codebase is undergoing relatively rapid changes.

### What Works

- Type inference
- Recursive definitions
- Declaration of data types
- Generics
- Bytecode compilation
- Bytecode VM execution
- Pattern-matching
- Common expression constructs (`if`, `match`, `let`, etc.)

### What Doesn't Work

- Trait system
- Data types
- HKTs
- Standard library / prelude

## Type System

Tao's type system is ML-like and supports functions, lists, tuples, primitives, sum types and product types.

## Declarations

Tao supports top-level type, data structures, and value definition declarations.
Below are some examples of these.

*Recursive function*

```
def factorial = |x|
	if x = 0
	then 1
	else x * factorial(x - 1)
```

*Type alias*

```
type NonEmpty A = (A, List A)
```

*Sum type*

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
	.name String
	.age  Num
	.address Maybe Num
```

## Error Messages

Tao aims to have useful error messages. Below are a few examples.

```
Error: Type mismatch between 'Num' and 'Str'
-> line 1, column 2
   1 | (x -> x + 3)("test")
        ^           ^^^^^^
```

```
Error: Cannot fully infer type A in [A] -> Num
-> line 1, column 5
   1 | def len A = |xs of [A]| match xs in
           ---
   5 | def main = []:len
                     ^^^
Hint: Specify all missing types
```

```
Error: No such binding 'bar' in the current scope
-> line 1, column 22
   1 | let foo = 5 in foo + bar
                            ^^^﻿
```

## Compiler Architecture

Tao's implementation is largely a learning exercise for me.
As a result, I'm avoiding the use of pre-made compiler components as much as possible for now.

Tao's compiler is written in Rust and is composed of several distinct stages that follow the traditional 'pipeline' compiler architecture closely.
These stages are listed below. Note that many are unfinished.

1) **Lexing**: Turns an input string into a series of disassociated tokens
2) **Parsing**: Turns the context-free grammar into an abstract syntax tree (AST)
3) **Type inference**: Converts the AST into a fully-typed higher-level intermediate representation (HIR) via a brief representation useful for type inference
4) **Soundness checking**: Ensures that various aspects of the program are well-formed. This includes pattern exhaustion checks, consistency of recursive values, etc.
5) **Instantiation**: Takes the HIR representation and converts it into a MIR representation where generic functions are instantiated with concrete types
6) **Optimisation**: Analysis is performed on the MIR and optimisations that are most easily made in tree form are made
7) **Code generation**: The MIR is converted into a low-level bytecode that may be executed by the bytecode VM. Later stages may transpile to other languages or even to native machine code.

