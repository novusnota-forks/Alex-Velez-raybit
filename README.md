# Raybit!

The official (*non-official*) **Brainfuck** bindings for **Raylib**!

# What is Raybit? or... BitBit?

**BitBit** is a modified version of the esoteric programming language, **Brainfuck**. More specifically, it is a super set of **Brainfuck**, meaning, it can run all normal **Brainfuck** code but also includes some extra functionally. It has all the bells and whistles of normal Brainfuck (*all 8 of 'em!*) plus 4 very special extra commands, `+-`, `-+`, `<>`, `><`. These commands allow **BitBit** to call functions from libraries with a predefined function map. Since there are 4 function call commands, this allows **BitBit** to access a maximum of 1024 possible external functions!

**Raybit**, or also known as **Rayfuck**, is simply the bindings for **Raylib** in **BitBit**.

This interpreter also includes 7 more commands for debugging purposes, `?`, `#`, `$`, `!`, `{`, `}`, `|`.

### Some Specifics
* The memory strip is grown dynamically, thus you must "*explore*" a cell before it can be accessed.
* This interpreter implements a looping memory pointer (*decrementing from cell 0 to the last cell, or incrementing from the last cell to cell 0`*).
* This interpreter includes single-line comments with `//`.
* A <code>&nbsp;</code> (*space*) or `|` character can be used to separate commands for the interpreter.
* `Panic!` occurs when trying to access invalid memory or an invalid function.
* The default word size in **BitBit** is 1 byte. However, this can be modified in the compiler. The examples use a word size of 2.

## Why is Raybit?
No one asked for this. But... so in conclusion, this will advance the modern world beyond human comprehension!

In all seriousness, this is an experimental hobby project to test interpretation and compilation of a base-256 system within the domains of computer architecture and numerical computation.

As a personal hobby project, at its core, this project extends the boundaries of traditional programming languages by introducing function calls within the framework of **Brainfuck**.

This project focuses on modeling the intricacies of numerical computation as observed in conventional computing systems. **BitBit** is a theoretical exploration of fundamental principles of computer architecture and organization.

## Commands
| Command | Functionality                                                               |
| :-----: | :-------------------------------------------------------------------------- |
|   `>`   | Increment the memory pointer right 1 cell.                                  |
|   `<`   | Decrement the memory pointer left 1 cell.                                   |
|   `+`   | Increment the value stored at the current cell.                             |
|   `-`   | Decrement the value stored at the current cell.                             |
|   `[`   | If the current cell value is zero, then jump forward to the matching `]`.   |
|   `]`   | If the current cell value is nonzero, then jump back to the matching `[`.   |
|   `,`   | Accept one character of input, storing its ASCII value in the current cell. |
|   `.`   | Output the character corresponding to the value at the current cell.        |
|  `+-`   | Flip up! Calls an `rcore` function.                                         |
|  `-+`   | Flip down! Calls `rshapes` & `rmodels` functions.                           |
|  `<>`   | Flip left! Calls `rtextures` & `rtext` functions.                           |
|  `><`   | Flip right! Calls an `raudio` function.                                     |
|   `?`   | Prints the pointer and value at current cell to the console.                |
|   `#`   | Prints the entire current memory layout horizontally.                       |
|   `$`   | Prints the entire current memory layout vertically.                         |
|   `!`   | Stops the program.                                                          |
|  `//`   | Single-line comment.                                                        |
|   `{`   | Begin multi-line comment.                                                   |
|   `}`   | End multi-line comment.                                                     |
|  `\|`   | Spacer. Used to space commands for parser                                   |

## BitBit Data Types
In strip memory, all data types must somehow be represented using only an array of unsigned 8-bit integers. So to fix this, **BitBit** uses separate models for different types of data.

### `Boolean`:
`Boolean` has a bit-size of `8`. Thus, you only need one cell to store it. Normally its value is either `1` or `0` to represent `true` or `false` respectively, but any number greater than `0` is also read as `true`.

Value `true` at pointer position `2`.
Value `false` at pointer position `3`.
Value `true` at pointer position `4`.
```brainfuck
Memory Cells
-------------
[0][0][1][0][9][0][0]
       ^
Memory Pointer
```

### `String`:
For most values, the byte-size of a particular object is predefined based on its type. A `Color` type will always have a byte-size of `4` (*4 cells*) to store each field, `r`, `g`, `b`, `a`, since they are all `Uint8`. However, `String` has a dynamic byte-size. Meaning, it can be stored in any amount of cells depending on the length of the `String`. Thus, the ASCII value of each character in the `String` is stored in seperate consecutive cells, left-to-right order, with a null terminator at the end.

Value `"ABC"` at pointer position `2`.
```brainfuck
Memory Cells
-------------
[0][0][65][66][67][0][0]
       ^
Memory Pointer
```

### `Integer`:
`Integer` is actually a class of several types defining an `Integer` of varying byte-sizes. **BitBit** uses `base-256` to represent an `Integer`. `Signed` types use signed 2's complement. `Integer` is stored in big endian order, left-to-right.

#### `Unsigned Integer`:

$b = \text{Byte Size}$

$\text{Max Value} = {256^b} - 1$

| $\text{Type}$ | $\text{Bit-size}$ | $\text{Byte-size}$ | $\text{Max Value}$ |
| :------------ | :---------------- | :----------------- | :----------------- |
| `Uint8`       | $8$               | $1$                | $255$              |
| `Uint16`      | $16$              | $2$                | $65535$            |
| `Uint24`      | $24$              | $3$                | $16777215$         |
| `Uint32`      | $32$              | $4$                | $4294967295$       |

The value, $v$ of an `Unsigned Integer` to a decimal base is calculated by the sum of each digit in the `Integer` multiplied with its corresponding $256^n$.

$$
\begin{aligned}
w &= \text{width of unsigned integer} \\
\vec{U} &= \left[x_0\right]\left[x_1\right]\left[x_2\right]\dots\left[x_{w-1}\right] \\
v &=\sum_{i=0}^{w - 1}\left(\vec{U}_i \cdot 256^i \right)
\end{aligned}
$$

| `base-10`    | $256^0$ | $256^1$ | $256^2$ | $256^3$ | $256^4$ |
| :----------- | :------ | :------ | :------ | :------ | :------ |
| $0$          | $0$     | $0$     | $0$     | $0$     | $0$     |
| $1$          | $1$     | $0$     | $0$     | $0$     | $0$     |
| $255$        | $255$   | $0$     | $0$     | $0$     | $0$     |
| $256$        | $0$     | $1$     | $0$     | $0$     | $0$     |
| $65545$      | $9$     | $0$     | $1$     | $0$     | $0$     |
| $234897$     | $145$   | $149$   | $3$     | $0$     | $0$     |
| $4295098368$ | $0$     | $0$     | $2$     | $0$     | $1$     |

A `Uint16` value of `65545` at pointer position `2`.
```brainfuck
Memory Cells
-------------
[0][0][9][0][1][0][0]
       ^
Memory Pointer
```
#### `Signed Integer`:

$b = \text{Byte Size}$

$\text{Max Value} = \frac{256^b}{2} - 1$

| $\text{Type}$ | $\text{Bit-size}$ | $\text{Byte-size}$ | $\text{Max Value}$ |
| :------------ | :---------------- | :----------------- | :----------------- |
| `Int8`        | $8$               | $1$                | $127$              |
| `Int16`       | $16$              | $2$                | $32767$            |
| `Int24`       | $24$              | $3$                | $8388607$          |
| `Int32`       | $32$              | $4$                | $2147483647$       |

The value of a negative `Signed Integer` to a decimal base is calculated by adding it to the corresponding `Unsigned` maximum value.

$$
\begin{aligned}
w &= \text{width of signed integer} \\
\vec{T} &= \left[x_0\right]\left[x_1\right]\left[x_2\right]\dots\left[x_{w-1}\right] \\
v &= \sum_{i=0}^{w-2}{\left(\vec{T}_i \cdot 256^i\right)} + \left(-\vec{T}_{w-1} \cdot 256^{w-1}\right)
\end{aligned}
$$

<!-- \left(-\vec{T}_{w-1} \cdot 256^{w-1}\right) +  -->

| `base-10`     | $256^0$ | $256^1$ | $256^2$ | $256^3$ | $256^4$ |
| :------------ | :------ | :------ | :------ | :------ | :------ |
| $0$           | $0$     | $0$     | $0$     | $0$     | $0$     |
| $1$           | $1$     | $0$     | $0$     | $0$     | $0$     |
| $-1$          | $255$   | $255$   | $255$   | $255$   | $255$   |
| $-256$        | $0$     | $255$   | $255$   | $255$   | $255$   |
| $-65545$      | $247$   | $255$   | $254$   | $255$   | $255$   |
| $-234897$     | $111$   | $106$   | $252$   | $255$   | $255$   |
| $-4295098368$ | $0$     | $0$     | $254$   | $255$   | $254$   |

A `Int16` value of `-65545` at pointer position `1`.
```brainfuck
Memory Cells
-------------
[0][247][255][254][255][255][0]
     ^
Memory Pointer
```
### `Float`:
`Float` is actually a class of several types defining a `Float` of varying byte-sizes and precisions. Floating-point representation encodes rational numbers of  
the form $v = x \cdot 256^y$. `Float` is useful for performing computations involving very large numbers, $|v| >> 0$, numbers very close to zero, $0 < |v| << 1$, and  
more generally as an approximation to real arithmetic. `Float` vaguely models the `IEEE 754` standard.

##### Numerical Form: $(-1)^s \cdot M \cdot 256^E$  
* Sign byte $s$ determines whether number is negative or positive  
* Mantissa $M$ normally a fractional value in range $[1.0, 2.0)$
* Exponent $E$ weights value by power of $256$

##### Encoding:  
* $\text{s}$ is sign byte $s$
* $\text{exp}$ field encodes $E\ni\text{exp}\not={E}$
* $\text{frac}$ field encodes $M\ni\text{frac}\not={M}$

#### `Float24`
```brainfuck
Bit Size: s = 8, exp = 8, frac = 8, bias = 0
-------------
[s][exp][frac]
[ ][   ][    ]
-------------
Total Bit Size: 24
```
#### `Float32`
```brainfuck
Bit Size: s = 8, exp = 8, frac = 16, bias = 0
-------------
[s][exp][frac]
[ ][   ][ ][ ]
-------------
Total Bit Size: 32
```
#### `Float40`
```brainfuck
Bit Size: s = 8, exp = 8, frac = 24, bias = 0
-------------
[s][exp][frac   ]
[ ][   ][ ][ ][ ]
-------------
Total Bit Size: 40
```
#### `Float64`
```brainfuck
Bit Size: s = 8, exp = 16, frac = 40, bias = 255
-------------
[s][exp ][frac         ]
[ ][ ][ ][ ][ ][ ][ ][ ]
-------------
Total Bit Size: 64
```
#### `Float88`
```brainfuck
Bit Size: s = 8, exp = 16, frac = 64, bias = 255
-------------
[s][exp ][frac                  ]
[ ][ ][ ][ ][ ][ ][ ][ ][ ][ ][ ]
-------------
Total Bit Size: 88
```

### Calculation of `Float`:

$v_{256} = \text{value of floating-point number in base-256}$

$v_{10} = \text{value of floating-point number in base-10}$

$\text{sign} = (-1)^s$

> *Because the sign is determined by a whole cell, this also has the potential to represent several types of abstract numerical values in a single byte for values 2 to 255.*

#### Normalized Values: $\text{exp}\not={\vec{0}}\space\And\space\text{exp}\not={\overrightarrow{255}}$

* Exponent coded as a biased value, dependant on the number of exponent bytes:

$
\begin{aligned}
\vec{e} &= \text{exp}\\
k &= \text{width of } \vec{e} \\
bias &= 256^{k - 1} - 1 \\
E &= \text{uint(}\vec{e}\text{)} - bias
\end{aligned}
$

* Mantissa $M$ coded with implied leading $1$:

$
\begin{aligned}
M  &=  1 + 0.[x]...[x] \\
\overrightarrow{[x]} &= \text{frac} \rightarrow v_{10} - \lfloor v_{10} \rfloor
\end{aligned}
$

$
\begin{aligned}
&\bullet\space\text{Minimum: frac}=\vec{0}&\implies&M=1.0\\
&\bullet\space\text{Maximum: frac}=\overrightarrow{255}&\implies&M=2.0-\epsilon
\end{aligned}
$

$
\begin{aligned}
\vec{F} &= \text{frac} \\
w &= \text{width of }\vec{F} \\
M &= 1 + \sum_{i=1}^{w - 1}\frac{\vec{F}_{i-1}}{256^{i}}
\end{aligned}
$

Therefore, the value of a normalized `Float` is calculated as $v$ where:

$$
\begin{aligned}
v_{10} &= (–1)^s \cdot M \cdot 256^E \\
v_{10} &= (–1)^s
\cdot 
(
    1 +
    \sum_{i=1}^{|\vec{F}|-1}
    \frac{\vec{F}_{i-1}}{256^i}
)
\cdot 
256^{
    (
        \sum_{i=0}^{|\vec{e}|-1}
        \vec{e}_i \cdot 256^i
    )
    -(256^{|\vec{e}|-1} - 1)
}
\end{aligned}
$$

#### Denormalized Values: $exp = \vec{0}$
* Exponent is explicitly coded as:

$$
\begin{aligned}
\vec{e} &= \text{exp}\\
k &= \text{width of } \vec{e} \\
bias &= 256^{k - 1} - 1 \\
E &= 1 - bias
\end{aligned}
$$

* Mantissa $M$ coded with implied leading $0$:

$$
\begin{aligned}
M &= 0.[x]...[x] \\
\overrightarrow{[x]} &= \text{frac} \rightarrow v_{10} - \lfloor v_{10} \rfloor  
\end{aligned}
$$

$$
\begin{aligned}
&\bullet\space\text{frac}=\vec{0}&\implies&v_{10}=0\\
&\bullet\space\text{frac}\not=\vec{0}&\implies&|v_{10}|\in(0,1)
\end{aligned}
$$

$$
\begin{aligned}
\vec{F} &= \text{frac} \\
w &= \text{width of }\vec{F} \\\space\\
M &= \sum_{i=1}^{w - 1}\frac{\vec{F}_{i-1}}{256^{i}}
\end{aligned}
$$

Therefore, the value of a denormalized `Float` is calculated as $v$ where:

$$
\begin{aligned}
v_{10} &= (–1)^s \cdot M \cdot 256^E \\
v_{10} &= {(–1)^s} \cdot {(\sum_{i=1}^{|\vec{F}|-1}\frac{\vec{F}_{i-1}}{256^{i}})} \cdot {256^{(1-(256^{|\vec{e}|-1}-1))}}
\end{aligned}
$$

#### Special Values: $\text{exp} = \overrightarrow{255}$

$$
\begin{aligned}
&\text{frac}=\vec{0}&\implies\space &v_{10} = \pm\infty \\
&\text{frac}\not=\vec{0}&\implies\space &v_{10} = NaN \space
\end{aligned}
$$

### `Structs`:
A `Struct` is simply defined as a collection of fields. It consists of an array containing its fields in order. The fields are a `Pointer` to the data, except for `Boolean` and `Uint8` as those can be stored in the field cells directly. Therefore, with the default word size, `2` (16-bit), all fields require `2` cells to store each `Pointer` or value.

### Pointers
A `Pointer` in **BitBit** points to the cells where their respective information is held. The default word-size for **BitBit** is `2` or 16-bits. So pointers in **BitBit** actually require `2` cells to hold the value of the pointer that points to the cell where specific data is stored at.
* This interpreter can be configured to run with different word-sizes.

## Raylib Function Map
All **Raylib** functions are mapped in ascending order according to the *[Raylib Cheatsheet](https://www.raylib.com/cheatsheet/cheatsheet.html)* starting at `0` and organized based on modules.
| Command | Modules               |
| :-----: | :-------------------- |
|  `+-`   | `rcore`               |
|  `-+`   | `rshapes` & `rmodels` |
|  `<>`   | `rtextures` & `rtext` |
|  `><`   | `raudio`              |

So calling flip up, `+-`, with the current cell value at `15` will call the `MaximizeWindow` function.
```
>+++[<+++++>-]<  // cell 0 [15]
+-               // flip up
```
***Flip commands are awesome!***

## Function Output
In **BitBit**, functions that return values, will return those values to the left of the current cell in the memory. For example, the `WindowShouldClose` function returns a `bool`. So in **Raybit**, calling the  `WindowShouldClose` function returns the corresponding `Boolean` value, (`1` or `0`), to the left of the current cell. For all other data types, such as `String` or `Color`, their values are output in right-to-left order.

First we will move right and increment the second cell to `2`, since that value corresponds to `WindowShouldClose`. Then we flip up with `+-`, since the `WindowShouldClose` function is in the `rcore` module.
```brainfuck
> ++ +-
```
```brainfuck
Memory Cells
-------------
[0][2][0][0][0][0][0]
    ^
Memory Pointer
```

If `WindowShouldClose` is triggered and returns `true`, then our memory layout will look like this:
```brainfuck
Memory Cells
-------------
[1][2][0][0][0][0][0]
    ^
Memory Pointer
```
## Function Input
Functions that require argument values, will use the values in cells to the right of the current cell as `Pointers`. For example, `InitWindow` requires 3 input values for `width`, `height`, and `title`. Therefore with a default word-size of `2`, 16-bit, calling the `InitWindow` function uses the 6 cell values to the right of the current cell in left-to-right argument order as `Pointers`. For `Boolean`, `Uint8`, and `Uint16` data types, their value can be inputted into an argument cell normally, since their size is equal to or less then the default word-size.

We will first keep cell `0` at value `0`, since that value corresponds to `InitWindow`. Then we will move to and fill cells `1`-`4` with the `width` and `height` that we want to pass in respectively in `base-256`. Then we move to the next cell to define the `Pointer` for our `title`. In this case, we will store the `String` at memory position `7`. We can then move to position `7` and begin filling in ASCII values into cells sequentially for our string (*remember, `Strings` are null terminated*). Lastly, we can move back to the first cell and call the function with `+-`, since `InitWindow` is in the `rcore` module.

`InitWindow(width: Uint8, height: Uint8, title: &String)`

```brainfuck
>>++++[<++++++++>-]+++>>++++++
++++++[<++++++++++++>-]+>+++++
++>>>+++++[<+++++++++++++>-]
<<<<<<<< +-
```
At the end of our program, our memory layout will look like this:
```brainfuck
Memory Cells
-------------
[0][32][3][144][1][7][0][65][0]
 ^
Memory Pointer
```
Once `+-` is called, it will initialize an `800x400` window with the title `A`. 


## Example

Here is a **Raybit** example for creating a basic window!
```brainfuck
>>++++++++[<+++++++++++>-]+++>>+++++++++++++++[<+++++++++++++++>-]<+>+>+++++++>>>+++++++++[<++++++
+++++++>-]<->>+++++++[<+++++++++++++++>-]>+++++++++[<+++++++++++++>-]<->>+++++++++[<++++++++++++>-
]>++++++++++[<++++++++++>-]<+>>>>+++++[<+++++++++++>-]>++++[<++++>-]<+>>>+++++[<+++++>-]>+++++[<++
+++>-]>+++++[<+++++>-]>+++++++++++++++[<+++++++++++++++++>-]>++++++++++[<++++++++++++++++++++>-]>+
+++++++++[<++++++++++++++++++++>-]>++++++++++[<++++++++++++++++++++>-]>+++++++++++++++[<++++++++++
+++++++>-]>+++++++++[<+++++++++++++>-]>+++++[<+++++++>-]>>++++++++++[<+++++++++++++++++++>-]>>++++
++++++[<++++++++++++++++++++>-]>>++++[<+++++>-]>+++[<+++++++>-]>>++++++[<+++++++++++>-]<+>>+++++++
+++[<+++++++++++>-]<+>>++++++++++[<+++++++++++>-]>++++++++[<+++++++++++++>-]<->>++++++[<++++++++++
+++++++++>-]>++++++++[<++++++++++++>-]<+>>+++++++++[<+++++++++++++>-]<->>+++++++++[<+++++++++++++>
-]<-->>+++[<+++++++++++>-]>++++[<++++++++>-]>+++++++++[<++++++++++>-]<->>++++++++++[<+++++++++++>-
]<+>>+++++++++[<+++++++++++++>-]>++++[<++++++++>-]>+++++++++[<+++++++++++>-]>++++++[<+++++++++++++
++++++>-]>++++++++++[<++++++++++>-]<+>>++++++++[<++++++++++++>-]<+>>+++++++++[<+++++++++++++>-]<->
>++++++++++[<++++++++++>-]<+>>++++++++++[<++++++++++>-]>++++[<++++++++>-]>+++++++++++[<+++++++++++
>-]>++++++++++[<+++++++++++>-]<+>>+++++++++[<+++++++++++++>-]>++++++[<+++++++++++++++++++>-]>++++[
<++++++++>-]>++++++[<+++++++++++++++++>-]>+++++++[<+++++++++++++++>-]>++++++[<+++++++++++++++++++>
-]>+++++++++[<+++++++++++++>-]<-->>+++++++++[<+++++++++++++>-]<->>++++[<++++++++>-]>+++++++[<+++++
++++++++++++>-]>+++++++[<+++++++++++++++>-]>++++++++++[<+++++++++++>-]>++++++++++[<++++++++++>-]>+
+++++++++[<+++++++++++>-]<+>>+++++++[<+++++++++++++++++>-]>+++[<+++++++++++>-]<<<<<<<<<<<<<<<<<<<<
<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<< +- >>>>>>>>>>>>>>[ +- - +- >>>>>>>>>>> <> 
<<<<<<<<<<<++ +- ------------------------------------------------------ +- <-[>+++++++++++++++++++
++++++++++++++++++++++++++++++++++++<+]>--]+ +- 
```
Pretty self-explanatory right?
Just kidding! Here is a break down of the code with comments :)
Here I surround flips with the `|` character to signify a command trigger.
I'll warn you though, it's quite *vertical*! 
```brainfuck
//==================================================================================
// Program main entry point & Initialization 
//==================================================================================

// cell 0 [0] : InitWindow() ID
>

// cell 1-2 [88][3] : width : [856]b10 = [88][3]b256
>++++++++[<+++++++++++>-]
+++>

// cell 3-4 [226][1] : height : [482]b10 = [226][1]b256
>+++++++++++++++[<+++++++++++++++>-]<+>
+>

// cell 5-6 [7][0] : title ptr : [7]b10 = [7][0]b256
+++++++>
>

// cell 7-12 ["title"][0] : title
>+++++++++[<+++++++++++++>-]<->
>+++++++[<+++++++++++++++>-]
>+++++++++[<+++++++++++++>-]<->
>+++++++++[<++++++++++++>-]
>++++++++++[<++++++++++>-]<+>
>

// cell 13 [0] : WindowShouldClose() output space
>

// cell 14 [55] : BeginDrawing() ID
>+++++[<+++++++++++>-]

// cell 15-16 [17][0] : color ptr
>++++[<++++>-]<+>
>

// cell 17-20 [r][g][b][a] : background : rgba(25, 25, 25, 255)
>+++++[<+++++>-]
>+++++[<+++++>-]
>+++++[<+++++>-]
>+++++++++++++++[<+++++++++++++++++>-]

// cell 21-24 [r][g][b][a] : text color : rgba(200, 200, 200, 255)
> ++++++++++ [<++++++++++++++++++++>-]
> ++++++++++ [<++++++++++++++++++++>-]
> ++++++++++ [<++++++++++++++++++++>-]
>+++++++++++++++[<+++++++++++++++++>-]

// cell 25 [117] : DrawText() ID
>+++++++++[<+++++++++++++>-]

// cell 26-27 [35][0] : text ptr : [35]b10 = [35][0]b256
>+++++[<+++++++>-]
>

// cell 28-29 [190][0] : text x pos : [190]b10 = [190][0]b256
>++++++++++[<+++++++++++++++++++>-]
>

// cell 30-31 [200][0] : text y pos : [200]b10 = [200][0]b256
>++++++++++[<++++++++++++++++++++>-]
>

// cell 32 [20] : font size : [20]b10 = [20][0]b256
>++++[<+++++>-]

// cell 33-34 [21][0] : color ptr : [21]b10 = [21][0]b256
>+++[<+++++++>-]
>

// cell 35-75 ["Congrats! You created your first window!"][0] : text
>++++++[<+++++++++++>-]<+>
>++++++++++[<+++++++++++>-]<+>
>++++++++++[<+++++++++++>-]
>++++++++[<+++++++++++++>-]<->
>++++++[<+++++++++++++++++++>-]
>++++++++[<++++++++++++>-]<+>
>+++++++++[<+++++++++++++>-]<->
>+++++++++[<+++++++++++++>-]<-->
>+++[<+++++++++++>-]
>++++[<++++++++>-]
>+++++++++[<++++++++++>-]<->
>++++++++++[<+++++++++++>-]<+>
>+++++++++[<+++++++++++++>-]
>++++[<++++++++>-]
>+++++++++[<+++++++++++>-]
>++++++[<+++++++++++++++++++>-]
>++++++++++[<++++++++++>-]<+>
>++++++++[<++++++++++++>-]<+>
>+++++++++[<+++++++++++++>-]<->
>++++++++++[<++++++++++>-]<+>
>++++++++++[<++++++++++>-]
>++++[<++++++++>-]
>+++++++++++[<+++++++++++>-]
>++++++++++[<+++++++++++>-]<+>
>+++++++++[<+++++++++++++>-]
>++++++[<+++++++++++++++++++>-]
>++++[<++++++++>-]
>++++++[<+++++++++++++++++>-]
>+++++++[<+++++++++++++++>-]
>++++++[<+++++++++++++++++++>-]
>+++++++++[<+++++++++++++>-]<-->
>+++++++++[<+++++++++++++>-]<->
>++++[<++++++++>-]
>+++++++[<+++++++++++++++++>-]
>+++++++[<+++++++++++++++>-]
>++++++++++[<+++++++++++>-]
>++++++++++[<++++++++++>-]
>++++++++++[<+++++++++++>-]<+>
>+++++++[<+++++++++++++++++>-]
>+++[<+++++++++++>-]

// cell 0 [0] : call InitWindow()
<<<<<<<<<< <<<<<<<<<<
<<<<<<<<<< <<<<<<<<<<
<<<<<<<<<< <<<<<<<<<<
<<<<<<<<<< <<<<<
|+-|

// cell 14 [55] : BeginDrawing() ID
>>>>>>>>>> >>>>

//==================================================================================
// Main game loop
//==================================================================================
[
    //==================================================================================
    // Update
    //==================================================================================
    // TODO: Update your variables here
    //==================================================================================

    //==================================================================================
    // Draw
    //==================================================================================

    // cell 14 [55] : call BeginDrawing()
    |+-|

    // cell 14 [54] : ClearBackground() ID
    -

    // cell 14 [54] : call ClearBackground()
    |+-|

    // cell 25 [117] : DrawText() ID
    >>>>>>>>>> >

    // cell 25 [117] : call DrawText()
    |<>|

    // cell 14 [56] : EndDrawing() ID
    <<<<<<<<<< <
    ++

    // cell 14 [56] : call EndDrawing()
    |+-|

    //==================================================================================
    // Detect window close button or ESC key
    //==================================================================================

    // cell 14 [2] : WindowShouldClose() ID
    ---------- ----------
    ---------- ----------
    ---------- ----

    // cell 14 [2] : call WindowShouldClose()
    |+-|

    // cell 13 [0/1] : WindowShouldClose() output
    <

    // cell 13 [255/0] : flip boolean
    -

    // if cell 13 true (continue)
    [
        // cell 14 [2]
        >

        // cell 14 [57] : BeginDrawing() ID + WindowShouldClose() ID : 55 + 2
        ++++++++++ ++++++++++
        ++++++++++ ++++++++++
        ++++++++++ +++ ++

        // cell 13 [255] : true
        <

        // cell 13 [0] : false (exit if)
        +
    ]

    // cell 14 [57/2] : (BeginDrawing() ID + WindowShouldClose() ID) / WindowShouldClose() ID
    >

    // cell 14 [55/0] : BeginDrawing() ID / (exit main loop)
    --
]

//==================================================================================
// De-Initialization : (Close window and OpenGL context)
//==================================================================================

// cell 14 [1] : CloseWindow() ID
+

// cell 14 [1] : call CloseWindow()
|+-|
```

#### Why the fuck is your README so long?

¯\\_\_(ツ)__/¯