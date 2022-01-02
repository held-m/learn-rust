fn main() {
    println!("Hello, world!");
    
    let sorted_array: [usize; 650000] = [0;650000];
    let number = 9;
    binary_search(sorted_array, number);
}

fn binary_search(sorted_array: [usize; 650000], required_number: usize) {
    

    let mut slice_end: usize = sorted_array.len();
    let mut binary_index: usize = slice_end / 2;
    let mut count: usize = 0;

    loop {

        if count >= sorted_array.len() {
            break;
        }
       
        if sorted_array[binary_index] == required_number {
            println!("number {} has index {}", required_number, binary_index);
            break;
        }

        if sorted_array[binary_index] > required_number {
            slice_end = binary_index;
            binary_index = binary_index / 2;
            println!("Try index: {}", binary_index);
        }

        if sorted_array[binary_index] < required_number {
            binary_index = (slice_end - binary_index) / 2 + binary_index;
            println!("Try index: {}", binary_index);
        }

        count += 1;

    }

}

