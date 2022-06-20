use std::io;
#[warn(dead_code)]
#[warn(non_snake_case)]
// Sườn thông tin cho mọi người dễ làm
fn main() {
    //Vec
    let mut _softmark:Vec<String> = Vec::new();
    let mut _mark:Vec<String> = Vec::new();
    let mut _name:Vec<String> = Vec::new();
    let mut _findname:Vec<String> = Vec::new();
    //Add Data
    let _data0 = add("Bob", 4);
    _name.push("Bob".to_string());
    _softmark.push("4".to_string());
    _mark.push("4".to_string());
    _softmark.sort();

    let _data1 = add("Alice", 4);
    _name.push("Alice".to_string());
    _softmark.push("4".to_string());
    _mark.push("4".to_string());
    _softmark.sort();

    let _data2 = add("Tom", 5);
    _name.push("Tom".to_string());
    _softmark.push("5".to_string());
    _mark.push("5".to_string());
    _softmark.sort();

    //Search (Case 2)
    let id = _softmark.len() - 1;
    let mut index0 = 0;
    let mut index1 = 1;
    loop{
        if index1 >= id {
            println!("{:?}", _softmark);
            break;
        }
        else {
            if _softmark[index0] != _softmark[index1]{
                index0 += 1;
                index1 += 1;
            }
            else{
                _softmark.swap_remove(index0);
                _softmark.sort();
            }
        }
    }

    //Case 3
    let mut line = String::new();
        println!("Nhập số điểm để ra list: ");
        io::stdin().read_line(&mut line).unwrap();
    
    let mut index = 0;
    let ids = _name.len() - 1;
    loop{
        if index > ids {
            println!("{:?}", _findname);
            break;
        }
        else {
            if _mark[index] != line{ index += 1 }
            else {
                _findname.push((_name[index]).to_string());
                index += 1;
            }
        }
    }
}
fn add(ten: &str, diem: u32) -> School{
    let data = School {
        name : ten.to_string(),
        mark: diem,
    };
    return data;
}
pub struct School {
    name: String,
    mark: u32,
}

impl School {
    pub fn new(agee: u32, student: String) -> Self {
        Self{ name: student, mark: agee}
    }
}
