use itertools::Itertools;

fn main() {
    println!("Hello world");
    let values: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result = values
        .windows(4)
        .find(|window| window == &[1, 2, 3, 4])
        .is_some();

    // dbg!(result)
    println!("Result:: {}", result);
    println!("Done");


    let i = vec![1,2,3,4,5,6,7,8,9,10,11,12,13];
    let i_slice = &i[..];
    let i_result = i_slice.has_prefix_custom(&[9,10,11]);
    println!("has_prefix_custom:: {}", i_result);
    println!("+++");

    let s = vec!["Hello", "World", "some", "Stuff"];
    let s_result = s.as_slice().has_prefix_custom(&["World", "some"]);
    println!("string has_prefix_custom:: {}", s_result);
    println!("+++");
}

trait Prefix<T> {
    fn has_prefix_custom(&self, prefix: &[T]) -> bool;
}

impl<T> Prefix<T> for &[T]
where
    T: PartialEq,
{
    fn has_prefix_custom(&self, prefix: &[T]) -> bool {
        self.iter()
            // .positions(predicate)
            // .pos
            .positions(|v| *v == prefix[0])
            .find(|&index| {
                let range = index..(index + prefix.len());
                self[range] == *prefix
            })
            .is_some()
    }
}
