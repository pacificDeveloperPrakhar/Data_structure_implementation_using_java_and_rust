use std::vec::*;
fn main()
{
let mut  students=vec![1,2];
 let mut cookies=vec![1,2,3];
 let result=distribute_cookie(&mut students,&mut cookies);
 println!("{}",result);
}
pub fn distribute_cookie(students:&mut Vec<i64>,cookies:&mut Vec<i64>)->i64
{
 students.sort();
 cookies.sort();
 let mut ptr=0;
 let mut ptr1=0;
 let mut count=0;
 while (ptr<students.len())&&(ptr1<cookies.len())
 {
    while (ptr1<cookies.len())&&(cookies[ptr1]<students[ptr])
    {
        ptr1+=1;
    }
    ptr+=1;
    ptr1+=1;
    if ptr1<cookies.len()
    {
        count+=1;
    }
 }
 return count;
}