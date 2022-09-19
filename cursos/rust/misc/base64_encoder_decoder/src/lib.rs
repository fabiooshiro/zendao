extern crate base64;

use base64::{encode, decode};

const _DATA: [i32; 325] = [1, 62, 69, 137, 91, 70, 17, 44, 36, 100, 142, 2, 73, 132, 183, 172, 29, 65, 80, 168, 78, 253, 43, 46, 90, 71, 72, 237, 77, 26, 248, 71, 206, 98, 21, 168, 80, 87, 41, 62, 1, 67, 132, 38, 189, 159, 169, 89, 63, 136, 255, 38, 234, 204, 99, 221, 63, 235, 243, 216, 253, 122, 4, 172, 15, 1, 0, 2, 5, 104, 181, 220, 11, 160, 178, 193, 188, 38, 239, 137, 143, 58, 55, 130, 106, 212, 176, 121, 28, 101, 90, 189, 249, 121, 50, 233, 32, 71, 254, 88, 225, 61, 115, 233, 221, 147, 79, 247, 157, 128, 111, 228, 130, 244, 1, 128, 157, 76, 91, 247, 153, 201, 106, 178, 181, 141, 123, 200, 59, 204, 194, 150, 239, 207, 44, 153, 141, 207, 72, 149, 88, 242, 104, 8, 120, 105, 177, 236, 47, 145, 159, 53, 167, 199, 92, 174, 121, 187, 209, 236, 181, 111, 3, 11, 183, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
/* ini program id */
20, 203, 129, 219, 238, 100, 150, 21, 4, 219, 96, 66, 28, 135, 52, 114, 101, 84, 75, 151, 130, 139, 64, 75, 154, 136, 111, 161, 86, 66, 175, 81,
/* end program id */
174, 3, 239, 22, 74, 241, 0, 151, 45, 39, 192, 159, 181, 42, 86, 14, 186, 0, 136, 16, 208, 241, 92, 42, 10, 100, 108, 49, 39, 44, 68, 5, 1, 4, 4, 1, 2, 0, 3, 56, 
/* sha256 do metodo initialize */
175, 175, 109, 31, 13, 152, 155, 237,
/* fim sha256 do metodo initialize */
172, 30, 63, 19, 4, 116, 204, 20, 178, 254, 207, 227, 237, 183, 72, 92, 184, 24, 231, 228, 121, 123, 176, 88, 248, 245, 48, 198, 71, 107, 165, 238, 232, 3, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 115, 108, 117, 103];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let content_base64 = String::from("AT5FiVtGESwkZI4CSYS3rB1BUKhO/SsuWkdI7U0a+EfOYhWoUFcpPgFDhCa9n6lZP4j/JurMY90/6/PY/XoErA8BAAIFaLXcC6Cywbwm74mPOjeCatSweRxlWr35eTLpIEf+WOE9c+ndk0/3nYBv5IL0AYCdTFv3mclqsrWNe8g7zMKW788smY3PSJVY8mgIeGmx7C+RnzWnx1yuebvR7LVvAwu3AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAUy4Hb7mSWFQTbYEIchzRyZVRLl4KLQEuaiG+hVkKvUa4D7xZK8QCXLSfAn7UqVg66AIgQ0PFcKgpkbDEnLEQFAQQEAQIAAzivr20fDZib7awePxMEdMwUsv7P4+23SFy4GOfkeXuwWPj1MMZHa6Xu6AMAAAAAAAAEAAAAc2x1Zw==");
        println!("{:?}", &decode(content_base64).unwrap()[..])
    }
}
