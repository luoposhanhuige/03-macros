use anyhow::Result;

fn main() -> Result<()> {
    // let v = my_vec![1, 2, 3];
    // let v: Vec<i32> = my_vec![];
    // let v = vec![1, 2, 3];
    let v: Vec<i32> = my_vec![
        "1".parse()?,
        "2".parse()?,
        "3".parse()?,
        "4".parse()?,
        "5".parse()?,
        "6".parse()? // what does .parse()? do?
                     // It converts a string slice to an integer, returning a Result type.
                     // so, what the result of "6".parse()? looks like?
                     // It will be Ok(6) if the string is a valid integer, or an error if it is not.
                     // so, the v has a vec of OK(1), OK(2),...OK(6)
                     // the compiler will automatically convert the Vec<Result<i32, _>> to Vec<i32> if all are Ok
    ];
    println!("{:?}", v); // Output: [1, 2, 3]
    Ok(())
}

// my_vec! = my_vec![1, 2, 3] // Vec<i32>
#[macro_export]
macro_rules! my_vec {
    () => (
        $crate::vec::Vec::new()
    );
    ($elem:expr; $n:expr) => (
        $std::vec::from_elem($elem, $n)
    );
    ($($x:expr),+ $(,)?) => {
        {
            // let mut temp_vec = Vec::new();
            // $(
            //     temp_vec.push($x);
            // )*
            // temp_vec
            <[_]>::into_vec(Box::new([$($x),*]))
        }
    };
}

// why does  <[_]>::into_vec(Box::new([$($x),*])) equals to the above commented code?
// The expression `<[_]>::into_vec(Box::new([$($x),*]))` creates a boxed slice from the elements and then converts it into a `Vec`.
// This is a more concise way to create a `Vec` from a list of expressions.
// what is <[_]>?
// The type `<[_]>` is a type that represents a slice of an array with an unknown length.
// It is a way to refer to a slice type without specifying the length of the array.
// what is Box::new([$($x),*])?
// `Box::new([$($x),*])` creates a boxed array containing the elements `$x`.
// The `Box` type is a smart pointer that allocates memory on the heap.
// The `[$($x),*]` syntax creates an array with the elements `$x`.
// what is <[_]>::into_vec?
// The method `<[_]>::into_vec` is a method that converts a boxed slice into a `Vec`.
// It takes ownership of the boxed slice and returns a `Vec` containing the same elements.
// why is <[_]>::into_vec(Box::new([$($x),*])) more efficient than the above commented code?
// The expression `<[_]>::into_vec(Box::new([$($x),*]))` is more efficient than the commented code because it avoids the overhead of repeatedly calling `push` on a `Vec`.
// Instead, it allocates the memory for the entire array at once and then converts it into a `Vec`, which is more efficient in terms of memory allocation and performance.
// it allocates the memory for the entire array at once, how does is make it?
// Allocating memory for the entire array at once is more efficient because it reduces the number of memory allocations and copies that would occur if you were to push elements one by one.
// When you use `Box::new([$($x),*])`, it allocates a contiguous block of memory for all the elements in the array at once.
// This means that the memory is allocated in a single operation, which is generally faster than multiple smaller allocations.
// Additionally, when you convert this boxed array into a `Vec` using `<[_]>::into_vec`, it takes ownership of the memory and creates the `Vec` without needing to reallocate or copy the elements.
// This approach minimizes the overhead associated with dynamic memory management, leading to better performance, especially for larger collections of elements.
