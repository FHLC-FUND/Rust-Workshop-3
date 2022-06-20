fn main(){
    let mut a:Vec<u8> = [1,2,3,4,5].to_vec();
    let id = a.len() - 1;
    let mut index0 = 0;
    let mut index1 = 1;
    loop{
        if index1 <= id {
            if a[index0] != a[index1]{
                index0 += 1;
                index1 += 1;
            }
            else{
                a.swap_remove(index0);
                a.sort();
            }
        }
        else{
            println!("{:?}", a);
            break;
        }
    }
}