pub fn closest(arr: &Vec<i32>, val: i32,mut s:usize,mut e:usize) -> usize {

    while s < e {
        let mid = (e - s) / 2 + s;

        let l_c = (arr[mid] - val).abs();
        let r_c = (arr[mid + 1] - val).abs();

        if l_c > r_c {
            s = mid + 1;
        } else {
            e = mid;
        }
    }
    return s;
}

pub fn aggressive_cows(arr:&mut Vec<i32>,mut cows:usize)->i32
{
    arr.sort();
    let mut e=arr.len()-1;
    let mut s=0;
    let mut ans=arr[arr.len()-1]-arr[0];
    cows-=2;
    while (e>s)&&cows>0
    {
        let mid=(arr[e]-arr[s])/2+arr[s];
        cows-=1;
        let idx=closest(arr,mid,s,e);
        let l_res=(arr[s]-arr[idx]).abs();
        let r_res=(arr[e]-arr[idx]).abs();
        ans=*[ans,l_res,r_res].iter().max().unwrap();
        if l_res>r_res
        {
            e=idx-1;
        }
        else
        {
            s=idx+1;
        }
    }
    return ans;

}

pub fn main() {
    // example stall positions (sorted)
    let mut arr = vec![4,2,1,3,6];

    // number of cows
    let cows = 3;

    let result = aggressive_cows(&mut arr, cows);

    println!("Minimum distance achieved = {}", result);
}
