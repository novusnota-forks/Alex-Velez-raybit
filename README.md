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
* A <code>&nbsp;</code> (*space*) or `|` character can be used to space commands for the interpreter.
* `Panic!` occurs when trying to access invalid memory or an invalid function.

## Why is Raybit?
No one asked for this. But... so in conclusion, this will advance the modern world beyond human comprehension!

## Commands
| Command | Functionality                                                                                 |
| :-----: | :-------------------------------------------------------------------------------------------- |
|   `>`   | Increment the memory pointer (moving it to the right 1 cell).                                 |
|   `<`   | Decrement the memory pointer (moving it to the left 1 cell). "*take it back now y'all!*".     |
|   `+`   | Increment the value stored at the current cell. "*one hop this time!*"                        |
|   `-`   | Decrement the value stored at the current cell.                                               |
|   `[`   | If the current cell value is zero, then jump forward to the matching `]`.                     |
|   `]`   | If the current cell value is nonzero, then jump back to the matching `[`.                     |
|   `,`   | Accept one character of input, storing its ASCII value in the current cell.                   |
|   `.`   | Output the character corresponding to the value at the current cell. "*cha cha real smooth!*" |
|  `+-`   | Flip up! Calls an `rcore` function.                                                           |
|  `-+`   | Flip down! Calls `rshapes` & `rmodels` functions.                                             |
|  `<>`   | Flip left! Calls `rtextures` & `rtext` functions.                                             |
|  `><`   | Flip right! Calls an `raudio` function.                                                       |
|   `?`   | Prints the pointer and value at current cell to the console.                                  |
|   `#`   | Prints the entire current memory layout horizontally.                                         |
|   `$`   | Prints the entire current memory layout vertically.                                           |
|   `!`   | Stops the program.                                                                            |
|  `//`   | Single-line comment.                                                                          |
|   `{`   | Begin multi-line comment.                                                                     |
|   `}`   | End multi-line comment.                                                                       |
|  `\|`   | Spacer. Used to space commands for parser                                                     |

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

#### `Unsigned Integer`
* `Uint8`:
	* Bit-size: `8`
	* Max: `256^1 - 1` = `255`
* `Uint16`
	* Bit-size: `16`
	* Max: `256^2 - 1` = `65535`
* `Uint24`
	* Bit-size: `24`
	* Max: `256^3 - 1` = `16777215`
* `Uint32`
	* Bit-size: `32`
	* Max: `256^4 - 1` = `4294967295`

The value of an `Unsigned Integer` to a decimal base is calculated by the sum of each digit in the `Integer` multiplied with its corresponding `256^n`.
| `base-10`  | `256^0` | `256^1` | `256^2` | `256^3` | `256^4` |
| :--------- | :------ | :------ | :------ | :------ | :------ |
| 0          | 0       | 0       | 0       | 0       | 0       |
| 1          | 1       | 0       | 0       | 0       | 0       |
| 255        | 255     | 0       | 0       | 0       | 0       |
| 256        | 0       | 1       | 0       | 0       | 0       |
| 65545      | 9       | 0       | 1       | 0       | 0       |
| 234897     | 145     | 149     | 3       | 0       | 0       |
| 4295098368 | 0       | 0       | 2       | 0       | 1       |

A `Uint16` value of `65545` at pointer position `2`.
```brainfuck
Memory Cells
-------------
[0][0][9][0][1][0][0]
       ^
Memory Pointer
```
#### `Signed Integer`:
* `Int8`:
	* Bit-size: `8`
	* Max: `((256^1) / 2) - 1` = `127`
* `Int16`
	* Bit-size: `16`
	* Max: `(256^2) / 2) - 1` = `32767`
* `Int24`
	* Bit-size: `24`
	* Max: `(256^3) / 2) - 1` = `8388607`
* `Int32`
	* Bit-size: `32`
	* Max: `(256^4) / 2) - 1` = `2147483647`

The value of a negative `Signed Integer` to a decimal base is calculated by adding it to the corresponding `Unsigned` maximum value.
| `base-10`   | `256^0` | `256^1` | `256^2` | `256^3` | `256^4` |
| :---------- | :------ | :------ | :------ | :------ | :------ |
| 0           | 0       | 0       | 0       | 0       | 0       |
| 1           | 1       | 0       | 0       | 0       | 0       |
| -1          | 255     | 255     | 255     | 255     | 255     |
| -256        | 0       | 255     | 255     | 255     | 255     |
| -65545      | 247     | 255     | 254     | 255     | 255     |
| -234897     | 111     | 106     | 252     | 255     | 255     |
| -4295098368 | 0       | 0       | 254     | 255     | 254     |

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
the form `v = x * 2^y`. `Float` is useful for performing computations involving very large numbers, `|v| >> 0`, numbers very close to zero, `0 < |v| << 1`, and  
more generally as an approximation to real arithmetic. `Float` vaguely models the `IEEE 754` standard.

##### Numerical Form: `(-1)^s * M * 256^E`  
* Sign byte `s` determines whether number is negative or positive  
* Significand `M` normally a fractional value in range `[1.0, 2.0)`
* Exponent `E` weights value by power of `2`

##### Encoding:  
* `s` is sign byte `s`  
* `exp` field encodes `E`  (but is not equal to `E`)  
* `frac` field encodes `M` (but is not equal to `M`)

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

#### Normalized Values: `exp ≠ [0]...[0] & exp ≠ [255]...[255]`
* Exponent coded as a biased  value:  `E = exp - bias`
* `bias = 256^(k - 1) - 1`, where k is number of exponent bytes
* Significand `M` coded with implied leading `1`:  `M  =  [1].[x]...[x]`
	* `[x][x][x]...[x]`: bytes of `frac`
	* Minimum when `frac = [0]...[0]` $\rightarrow$ `M = 1.0`
	* Maximum when `frac = [255]...[255]` $\rightarrow$ `M = 2.0 – ε`

Therefore, the value of a normalized `Float` is calculated as `v` where:
`E = exp – bias`
`v = (–1)^s * M * 256^E` 
#### Denormalized Values: `exp = [0]...[0]`
* Exponent coded as: `E = 1 – bias`
* Significand `M` coded with implied leading `0`:  `M = 0.[x]...[x]` 
	* `[x][x][x]...[x]`: bytes of `frac`
* `frac = [0]...[0]`  $\rightarrow$ $v = 0$
* `frac ≠ [0]...[0]`  $\rightarrow$ $0 < |v| << 1$

Therefore, the value of a denormalized `Float` is calculated as `v` where:
`E = 1 – bias`
`v = (–1)^s * M * 256^E`
#### Special Values: `exp = [255]...[255]`
* `frac = [0]...[0]`  $\rightarrow$ $v = \pm\infty$
* `frac ≠ [0]...[0]`  $\rightarrow$ $v = NaN$

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

*In this example, I have used shorthand for brevity*:
```brainfuck
> +32 > +3 > +144 > +1 > +7 >> +65
<<<<<<< +-
```
*Real code*:
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
Once `+-` is called, it will initialize a `800x400` window with the title `A`. 


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
Here I use surround flips with the `|` character to signify a command trigger.
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

>If only github provided a place in the repository to lay out the roadmap of my project, show the current status, and document the software better... oh well!

¯\\_\_(ツ)__/¯