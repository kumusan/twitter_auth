mod token;
mod get_list;

use token::get_token;
use get_list::get_list;

fn main() {
    let token = get_token();
    let res = get_list(&token);
    for data in res {
        println!("{:?}", data["text"])
    }
}
