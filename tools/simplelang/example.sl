import with (a: i32, b: i32) cfunc

with (a: i32, b: i32) func: {
    return cfunc(a, b) + b;
}