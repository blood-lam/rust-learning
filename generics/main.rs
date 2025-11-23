struct A; // Concreate type A
struct S(A); // Concreate type S that contains A
struct SGen<T>(T); // Generic struct SGGen that can contain any type T

fn reg_fn(_s: S) {} // Regular function that takes S

fn gen_spec_t(_s: SGen<A>) {} // Generic function specialized for SGen<A>

fn gen_spec_i32(_s: SGen<i32>) {} // Generic function specialized for SGen<i32>

fn generic<T>(_s: SGen<T>) {} // Generic function that can take SGen of any type T

fn main() {
    reg_fn(S(A)); // Call regular function with S
    gen_spec_t(SGen(A)); // Call generic function specialized for SGen<A>
    gen_spec_i32(SGen(6)); // Call generic function specialized for SGen<i

    generic::<char>(SGen('a')); // Call generic function with SGen<char>
    generic(SGen('A')); // Call generic function with SGen<char> (type inferred)

    let integer = sum(5, 10);
    let float = sum(5.0, 10.0);
    println!("Integer sum: {}, Float sum: {}", integer, float);

    let point_int: Point<i8> = Point { x: 5, y: 10 };
    let point_float: Point<f32> = Point { x: 1.0, y: 4.0 };
    println!("Point Int: ({}, {}), Point Float: ({:?}, {:?})", point_int.x, point_int.y, point_float.x, point_float.y);

    let tus = Tupse { first: 5, second: 10.4 };
    println!("Tupse: ({}, {})", tus.first, tus.second);

    // let arrays: Array<i32, 5> = Array { elements: [1, 2, 3, 4, 5] };
    let arrays: [Array<i8, 3>; 3] = [
        Array { elements: [1, 2, 3] },
        Array { elements: [4, 1, 6] },
        Array { elements: [7, 8, 9] },
    ];
    println!("Array elements: {:?}", arrays.iter().map(|a| &a.elements).collect::<Vec<_>>());
}

fn sum<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

struct Point<T> {
    x: T,
    y: T,
}

struct Tupse<T, U> {
    first: T,
    second: U,
}

struct Array<T, const N: usize> {
    elements: [T; N],
}
