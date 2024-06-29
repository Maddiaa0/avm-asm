<img align="right" width="150" height="150" top="100" src="./assets/readme.jpg">
## Avm Asm

A simple assembler for Aztec virtual machine programs, inspired by x86, huff and mips assemblers.

## Syntax / Features
### Quirks
All statements must be terminated with a `;`. Even macro definitions.

### Indirect
Avm opcodes support an indirect addressing mode, this can be activated by annotating the opcode with a bang `!`. The default mode will be direct.

**Direct**
```asm
add 1 2 3;
```

**Indirect**
```asm
add! 1 2 3;
```

### Labels
Labels within the Avm are static and will be evaluated at compile time, to define a label, use the `<name>:` syntax (common in many assemblers).

Jumping to a label can be performed with `@<name>`

```asm
    add 1 2 3;
    jump @label;
    sub 1 2 3;
label:
    add 1 2 3;
```

### Macros
Macros are defined with the `.macro` prefix. They are not feature complete, in that there is not yet support to template macros with arguments.

Macros are encapsulated by a pair of curly brackets.

When invoking a macro, you must prefix it with `$`. For example:

```asm
.macro first_macro {
    add 1 2 3;
    sub 1 2 3;
};

.macro second_macro {
    $first_macro;
    add 1 2 3;
};

$second_macro;
```

### Tagged Opcodes
When working with opcodes that reason about the underlying types (a consequence of a tagged memory design) we can define types in a variety of ways.

*As the underlying type in the bytecode*
```asm
cast 1 2 3;
```

*As the type that it represents*
```asm
cast u16 2 3;
```

The two statements above are equivalent.

### Hex literals
The Set opcode requires that you write a constant value to be written into a memory address, some of these types are larger than are supported as a numeric literal
by the compiler, the solution is to use an explicit hex literal when dealing with large values.

This allows you to set a hex value into the memory location 2. 

(Error handling is wack so the compiler will panic if your literal overflows your specified type tag)

```asm
set ff 0x30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd46 2;
```

### Constants
You can define constants anywhere in a file that will be replaced with the assigned value. 

Constants are defined as follows:
```asm
.const name = 0x1234;
```

Constants can be used within the bytecode in the same way a macro is invoked
```
.const name = 0x1234;

add $name 2 3 // equivalent to add 0x1234 2 3
```


## Warning
This assembler has no guard rails implemented, it will let you write invalid bytecode.
The Avm does not have a final spec do not try and use this