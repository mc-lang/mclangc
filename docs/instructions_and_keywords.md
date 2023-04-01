# Instructions And Keywords

This explains all of the instructions and keywords

## Instructions

Instructions modify the stack, the stack is an array, i will be using typescript to show how each instruction works

```ts
let stack: number[] = [];
```

### PushInt ("Any Number")

PushInt pushes one number on the stack.

Usage:

```forth
420
```

How it works:

```ts
// num = 420
stack.push(num);
console.assert(stack == [420]);
```

### PushStr ("Any String")

PushStr pushes 2 things on the stack, string length and string pointer

Usage:

```forth
"Henlo world!\n"
```

How it works:

```ts
stack.push(str_len);
stack.push(str_ptr);
```

### Print ("print")

Print just prints a number from the top of the stack into stdout

Usage:

```forth
69 print
```

How it works:

```ts
num = stack.pop()
console.log(num);

```

### Dup ("dup")

Dup duplicates the top most number on the stack

Usage:

```forth
69 dup
```

How it works:

```ts
stack = [12];
a = stack.pop();
stack.push(a);
stack.push(a);
console.assert(stack == [12, 12]);
```

### Drop ("drop")

Drop removes the top most number on the stack

Usage:

```forth
69 drop
```

How it works:

```ts
stack = [69];

stack.pop();

console.assert(stack == []);
```

### Rot ("rot")

Rot moves the third number from the top of the stack and moves it to the top

Usage:

```forth
1 2 3 rot
```

How it works:

```ts
stack = [1, 2, 3];

let a = stack.pop();
let b = stack.pop();
let c = stack.pop();
stack.push(b);
stack.push(a);
stack.push(c);

console.assert(stack == [3, 1, 2]);
```

### Over ("over")

Over takes the second number from the top of the stack and copies it to the top

Usage:

```forth
1 2 over
```

How it works:

```ts
stack = [1, 2];

let a = stack.pop();
let b = stack.pop();
stack.push(b);
stack.push(a);
stack.push(b);

console.assert(stack == [1, 2, 1]);
```

### Swap ("swap")

Swap swaps the first and second numbers from the stack

Usage:

```forth
1 2 stack
```

How it works:

```ts
stack = [1, 2];

let a = stack.pop();
let b = stack.pop();
stack.push(a);
stack.push(b);

console.assert(stack == [2, 1]);
```

### Plus ("+")
### Minus ("-")
### Mul ("*")
### Equals ("=")
### Gt (">")
### Lt ("<")
### NotEquals ("!=")
### Le ("<=")
### Ge (">=")
### Band ("band")
### Bor ("bor")
### Shr ("shr")
### Shl ("shl")
### DivMod ("divmod")
### Mem ("mem")
### Load8 ("@8")
### Store8 ("!8")
### Syscall0 ("syscall0")
### Syscall1 ("syscall1")
### Syscall2 ("syscall2")
### Syscall3 ("syscall3")
### Syscall4 ("syscall4")
### Syscall5 ("syscall5")
### Syscall6 ("syscall6")
### None ("None")

## Keywords

### If ("if")
### Else ("else")
### End ("end")
### While ("while")
### Do ("do")
### Macro ("macro")
### Include ("include")