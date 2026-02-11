pub fn main() {
    // sorted array
    let mut arr = vec![1, 3, 5, 6];

    let val = 0;

    // find insertion range
    let (start_idx, end_index) = find_range(&arr, val, 0, arr.len() - 1);

    // insert at returned second index
    insert_element(&mut arr,val,start_idx,end_index);

    println!("Array after insertion: {:?}", start_idx);
}

pub fn find_range(arr: &Vec<i32>, val: i32, start: usize, end: usize) -> (usize, usize) {
    if (start + 1) >= end {
        return (start, end);
    }

    let mid = start + ((end - start) / 2);

    if arr[mid] < val {
        find_range(arr, val, mid, end)
    } else {
        find_range(arr, val, start, mid)
    }
}
pub fn search_idx(arr:&mut)
pub fn insert_element(arr:&mut Vec<i32>,val:i32,start:usize,end:usize)->()
{
    let mut stack:Vec<i32>=vec![];
    let mut  idx=arr.len();
    while arr.is_empty()!=true&&idx!=end
    {
     idx-=1;
     stack.push(arr.pop().unwrap());
    }
    arr.push(val);
    while stack.is_empty()!=true
    {
        arr.push(stack.pop().unwrap());
    }
}
