fn main() {
    let number_target = 3;
    let mut numbers = vec![1,4,5,6,7,8,9,11,12,15,16,17,18,20];

    binary_search_fn(&number_target, &mut numbers);
}

fn binary_search_fn(number_target: &i32, numbers: &mut Vec<i32>){
    let middle_index = {
        if numbers.len() as i32 / 2 - 1 < 0 {
            numbers.len() / 2
        } else {
            numbers.len() / 2 - 1
        }
    };
    let middle_number = numbers.get(middle_index)
        .unwrap_or(&-1);
    if middle_number == &-1{
        println!("number not found");
        return;
    }

    if number_target == middle_number{
        println!("number found")
    } else if number_target < middle_number {

        let numbers = &numbers[..middle_index];
        let mut numbers = Vec::from(numbers);
        binary_search_fn(number_target, &mut numbers);

        
    } else if number_target > middle_number {
        
        let numbers = &numbers[middle_index + 1..];
        let mut numbers = Vec::from(numbers);
        binary_search_fn(number_target, &mut numbers);
    }
}
