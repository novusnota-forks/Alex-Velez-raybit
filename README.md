# Raybit!

The official (*non-official*) Brainfuck bindings for Raylib!

# What is Raybit?

**Raybit** or also known as **Rayfuck**, is a modified version of the esoteric programming language, **Brainfuck**. More specifically, it is a super set of ***non-looping*** **Brainfuck**, that is, it can run most normal **Brainfuck** code as long as it doesn't require memory strip values to loop (*decrementing from 0 to 255, or incrementing from 255 to 0*). It has all the bells and whistles of normal Brainfuck (*all 8 of 'em!*) plus a very special extra command, `@`.

This interpreter also includes 3 more commands for debugging purposes, `?`, `#`, `!`.

### Specifics
* The memory strip is grown dynamically, thus you must "*explore*" a cell before it can be accessed.
* Cell values are stored as signed 32-bit integers. So both negative and positive values are possible.

## Why is Raybit?

No one asked for this. But... so in conclusion, this will advance the modern world beyond human comprehension!

## Commands

All your files and folders are presented as a tree in the file explorer. You can switch from one to another by clicking a file in the tree.
|Command| Functionality |
|:--:|--|
| > | Increment the memory pointer (moving it to the right 1 cell). |
| < | Decrement the memory pointer (moving it to the left 1 cell). "*take it back now y'all!*". |
| + | Increment the value stored at the current cell. "*one hop this time!*" |
| - | Decrement the value stored at the current cell. |
| [ | If the current cell value is zero, then jump forward to the matching `]`. |
| ] | If the current cell value is nonzero, then jump back to the matching `[`. |
| , | Accept one character of input, storing its ASCII value in the current cell. |
| . | Output the character corresponding to the value at the current cell. |
| @ | Calls a Raylib function, using the value at the current cell as an id. All Raylib functions are mapped in descending order according to the Raylib Cheat sheet starting at `-1`. "*cha cha real smooth!*"  |
| ? | Prints the pointer and value at pointer cell to the console. |
| # | Prints the entire current memory layout. |
| ! | Stops the program. |

## More on `@`

The `@` command is really cool!
* Functions that return values will also return the values to the left of the current cell in the memory. For example, the `WindowShouldClose` function returns a `bool`, so in **Raybit**, calling the  `WindowShouldClose` function returns the corresponding `boolean` value, (`1` or `0`), to the left of the current cell. 

We will first move right and decrement the second cell to `-3`, since that value corresponds to `WindowShouldClose`. Then we can call the function.
```
> --- @
```
```
Memory Cells
-------------
[0][-3][0][0][0][0][0]
    ^
Memory Pointer
```

If `WindowShouldClose` is triggered and returns `true`, then our memory layout will look like this:
```
Memory Cells
-------------
[1][-3][0][0][0][0][0]
    ^
Memory Pointer
```

Even more with `@`!

* Functions that require argument values will use the values in cells to the right of the current cell in the memory. For example, `InitWindow` requires 3 input values for `width`, `height`, and `title`, so in **Raybit**, calling the `InitWindow` function uses the 3 cell values to the right of the current cell in corresponding argument order. For `integers` and `booleans`, those values can be inputted into the corresponding cells normally, but for all other data types, such as `String` or `Color`, the cell value will actually represent a pointer to the cell where its information is held.

For example, a `String` pointer will point to the first value of the string and will be null terminated. This example represents a string `ABC` being stored at the pointer value `3`.
```
Memory Cells
-------------
[0][1][2][3 ][4 ][5 ][6]
[0][0][0][65][66][67][0]
          ^
Memory Pointer
```

We will first decrement the first cell to `-1`, since that value corresponds to `InitWindow`. Then we will move to the second, and third cells, and fill them up with the width and height that we want to pass in respectively. Then we move to the next cell to define the pointer for our title string. In this case, we will store the string at position `4`. We can then move to position `4` and begin filling in ASCII values into cells sequentially for our string (*remember, strings are null terminated*). Lastly, we can move back to the first cell and call the function.

*In this example, I have used shorthand for brevity*:
```
- > +800 > +400 > +4 > +65 <<<< @
```
*Real code*:
```
-
>>++++++++++++++++++++[<++++++++++++
++++++++++++++++++++++++++++>-]>++++
++++++++++++++++[<++++++++++++++++++
++>-]++++>>+++++[<+++++++++++++>-]<
<<<< @
```
At the end of our program, our memory layout will look like this:
```
Memory Cells
-------------
[-1][800][400][4][65][0]
 ^
Memory Pointer
```
Once `@` is called, it will initialize a `800x400` window with the title `A`.

## Example

Here is a **Raybit** example for creating a basic window!
```
->>+++++++++++++++++[<++++++++++++++++++++++++++++++++++++++++++++++++++>-]<++++++>>++++++++++++++++[<
++++++++++++++++++++++++++++++>-]<++>++++>>++++[<+++++++++++++++++++++++++++++>-]>+++++++[<+++++++++++
++++>-]>++++[<+++++++++++++++++++++++++++++>-]>+++++++++[<++++++++++++>-]>++++++++++[<++++++++++>-]<+>
<<<<<<<<<@>>>>>>>>>>-------------------------------------------------------->++++++++++++>>+++++[<++++
+>-]>+++++[<+++++>-]>+++++[<+++++>-]>+++++++++++++++[<+++++++++++++++++>-]>++++++++++[<+++++++++++++++
+++++>-]>++++++++++[<++++++++++++++++++++>-]>++++++++++[<++++++++++++++++++++>-]>+++++++++++++++[<++++
+++++++++++++>-]>+++++++++++++++++++[<-------------------->-]>+++++[<+++++>-]<+>>++++++++++[<+++++++++
++++++++++>-]>++++++++++[<++++++++++++++++++++>-]>++++[<+++++>-]>++++[<++++>-]>++++++[<+++++++++++>-]<
+>>++++++++++[<+++++++++++>-]<+>>++++++++++[<+++++++++++>-]>++++++++[<+++++++++++++>-]<->>++++++[<++++
+++++++++++++++>-]>++++++++[<++++++++++++>-]<+>>+++++++++[<+++++++++++++>-]<->>+++++++++[<++++++++++++
+>-]<-->>+++[<+++++++++++>-]>++++[<++++++++>-]>+++++++++[<++++++++++>-]<->>++++++++++[<+++++++++++>-]<
+>>+++++++++[<+++++++++++++>-]>++++[<++++++++>-]>+++++++++[<+++++++++++>-]>++++++[<+++++++++++++++++++
>-]>++++++++++[<++++++++++>-]<+>>++++++++[<++++++++++++>-]<+>>+++++++++[<+++++++++++++>-]<->>+++++++++
+[<++++++++++>-]<+>>++++++++++[<++++++++++>-]>++++[<++++++++>-]>+++++++++++[<+++++++++++>-]>++++++++++
[<+++++++++++>-]<+>>+++++++++[<+++++++++++++>-]>++++++[<+++++++++++++++++++>-]>++++[<++++++++>-]>+++++
+[<+++++++++++++++++>-]>+++++++[<+++++++++++++++>-]>++++++[<+++++++++++++++++++>-]>+++++++++[<++++++++
+++++>-]<-->>+++++++++[<+++++++++++++>-]<->>++++[<++++++++>-]>+++++++[<+++++++++++++++++>-]>+++++++[<+
++++++++++++++>-]>++++++++++[<+++++++++++>-]>++++++++++[<++++++++++>-]>++++++++++[<+++++++++++>-]<+>>+
++++++[<+++++++++++++++++>-]>+++[<+++++++++++>-]<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
<<[@+@>>>>>>>>>>@<<<<<<<<<<--@++++++++++++++++++++++++++++++++++++++++++++++++++++++@<[>+@++++++++++++
+++++++++++++++++++++++++++++++++++++++++++<-]>-----------------------------------------------------]
```
Pretty self-explanatory right?
Just kidding! Here is a break down of the code with comments :)
I'll warn you though, it's quite *vertical*! 
```
// cell 0 [-1]
-

// cell 1 [856]
>>+++++++++++++++++
[
<
++++++++++
++++++++++
++++++++++
++++++++++
++++++++++
>-
]
<++++++>

// cell 2 [482]
> ++++++++++++++++
[
<
++++++++++
++++++++++
++++++++++
>-
]
<++>

// cell 3 [4]
++++>

// cell 5 [116]
> ++++
[
<
++++++++++
++++++++++
+++++++++
>-
]

// cell 6 [105]
> +++++++
[
<
++++++++++
+++++
>-
]

// cell 7 [116]
> ++++
[
<
++++++++++
++++++++++
+++++++++
>-
]

// cell 8 [108]
> +++++++++
[
<
++++++++++
++
>-
]

// cell 9 [101]
> ++++++++++
[
<
++++++++++
>-
]
<+>

// cell 0 [-1] (init)
<<<<<<<<<@


// cell 9 [0]
>>>>>>>>>

// cell 10 [-56]
>
----------
----------
----------
----------
----------
------

// cell 11 [12]
> ++++++++++ ++

// cell 12 [25]
>> +++++ [<+++++>-]

// cell 13 [25]
> +++++ [<+++++>-]

// cell 14 [25]
> +++++ [<+++++>-]

// cell 15 [255]
> +++++++++++++++ [<+++++++++++++++++>-]

// cell 16 [200]
> ++++++++++ [<++++++++++++++++++++>-]

// cell 17 [200]
> ++++++++++ [<++++++++++++++++++++>-]

// cell 18 [200]
> ++++++++++ [<++++++++++++++++++++>-]

// cell 19 [255]
> +++++++++++++++ [<+++++++++++++++++>-]

// cell 20 [-380]
>+++++++++++++++++++ [<-------------------->-]

// cell 21 [26]
>+++++[<+++++>-]<+>

// cell 22 [190]
>++++++++++[<+++++++++++++++++++>-]

// cell 23 [200]
>++++++++++[<++++++++++++++++++++>-]

// cell 24 [20]
>++++[<+++++>-]

// cell 25 [16]
>++++[<++++>-]

// cell 26-65 ["Congrats! You created your first window!"]
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

// cell 10 [-56]
<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

[
    // cell 10 [-56] (begin drawing)
    @

    // cell 10 [-55] (clear background)
    +@

    // cell 20 [-380] (draw text)
    >>>>>>>>>>@<<<<<<<<<<

    // cell 10 [-57] (end drawing)
    --@

    // cell 10 [-3] (call window_should_close)
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++@

    // cell 9 [0/1]
    <
    [
        // cell 10 [-3]
        >

        // cell 10 [-2] (close window)
        +@
        
        // cell 10 [53]
        ++
        ++++++++++
        ++++++++++
        ++++++++++
        ++++++++++
        ++++++++++
        +++

        // cell 9 [0]
        <-
    ]
    
    // cell 10 [-56]
    >
    ----------
    ----------
    ----------
    ----------
    ----------
    ---
]
```
