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


## Warning
This assembler has no guard rails implemented, it will let you write invalid bytecode.
The Avm does not have a final spec do not try and use this