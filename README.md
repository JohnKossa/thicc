# thicc
Rust library that converts strings into their equivalent thicc counterparts

Turns string like ""This is an example string!"" into ""丅卄工丂 工丂 卂𠘨 乇乂卂从尸乚乇 丂丅尺工𠘨厶!""

It's built as a crate so the functions can more easily be shared to other rust modules.

Provides str_to_thicc and char_to_thicc functions, called with a char and with a &str respectively.

Replacements are case-insensitive.

## Full replacement alphabet

The quick brown fox jumps over the lazy dog.

丅卄乇 㔿凵工匚长 乃尺口山𠘨 下口乂 丁凵从尸丂 口リ乇尺 丅卄乇 乚卂乙丫 刀口厶.
