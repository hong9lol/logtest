// // #[cfg(test)]
// // mod tests {
// //     use super::*;

// //     #[test]
// //     fn it_works() {
// //         let result = add(2, 2);
// //         assert_eq!(result, 4);
// //     }
// // }

// // // use proc_macro::TokenStream;

// // // #[proc_macro]
// // // pub fn my_macro(input: TokenStream) -> TokenStream {
// // //     // 매크로 구현 코드
// // //     input
// // // }

// // extern crate proc_macro;
// // use proc_macro::TokenStream;
// // use quote::quote;
// // use syn::{parse_macro_input, LitStr};

// // #[proc_macro]
// // pub fn concat_to_static_bytes(input: TokenStream) -> TokenStream {
// //     // 입력 문자열들 파싱 (쉼표로 구분된 문자열을 기대)
// //     // let input = parse_macro_input!(input as syn::punctuated::Punctuated<LitStr, syn::token::Comma>);
// //     // let mut combined_string = String::new();

// //     // // 문자열을 합침
// //     // for lit in input {
// //     //     combined_string.push_str(&lit.value());
// //     // }
// //     let combined_string = "test string";
// //     // 합친 문자열을 바이트 배열로 변환
// //     let mut byte_array: Vec<u8> = combined_string.bytes().collect();
// //     let _l = byte_array.len();
// //     // let a = combined_string.as_bytes().try_into().unwrap();
// //     println!("byte_array: {:?}", byte_array);
// //     // 바이트 배열을 반환하는 코드 생성
// //     let a = combined_string.to_string();
// //     // let hex_array: Vec<String> = combined_string
// //     //     .as_bytes()
// //     //     .iter()
// //     //     .map(|byte| format!("{:02x}", byte));
// //     // input.
// //     // let a: = input.into();
// //     // let mut byte_array: [u8; 32] = [0; 32]; // 고정 길이 배열을 0으로 초기화
// //     // byte_array[..32].copy_from_slice(a);

// //     // let input_string = input.to_string();
// //     // let length = input_string.len();
// //     #[link_section = "__DATA,my_custom5"]
// //     static MY_DATA: [u8; 14] = *b"Hello, world!\0";
// //     println!("{:?}", MY_DATA);
// //     #[link_section = "__DATA,my_custom4"]
// //     #[used]
// //     static EXG2: [u8; 8] = *b"TEST!!!\0";
// //     println!("{:?}", EXG2);
// //     let _a = EXG2.to_vec().len();

// //     let output = quote! {
// //         #[link_section = "__DATA,my_custom4"]
// //         #[used]
// //         static mut COMPILED_STRING: [u8; #_l] = [0; #_l];
// //         let mut byte_array: [u8; 32] = [0; 32];
// //         let input_bytes = #combined_string.as_bytes();
// //         unsafe {COMPILED_STRING[..#_l].copy_from_slice(input_bytes);
// //             println!("{:?}",COMPILED_STRING);
// //         }
// //     // println!("{:?}", byte_array);
// //     // static COMPILED_STRING: [u8; #byte_array.len()] = [#(#byte_array),*];
// //         // {
// //         //     #[link_section = ".my_custom_section"]
// //         //     static COMPILED_STRING: [u8; #byte_array.len()] = [#(#byte_array),*];
// //         //     &COMPILED_STRING
// //         // }

// //         // println!("{:?}", #_a);
// //         // #_a
// //     };

// //     output.into()
// // }

// use proc_macro::TokenStream;
// use quote::quote;
// use syn::{parse_macro_input, LitStr};

// #[proc_macro]
// pub fn define_variable_with_section(input: TokenStream) -> TokenStream {
//     // let section_name: LitStr = parse_macro_input!(input as LitStr);

//     // #[link_section = "__DATA,QWJESADI"]
//     // static MY_VAR: [u8; 2] = *b"44";
//     // println!("{:?}", MY_VAR);

//     // #[link_section = "__DATA,my_custom5"]
//     // static MY_DATA: [u8; 14] = *b"Hello, world!\0";
//     // println!("{:?}", MY_DATA);
//     // println!("{:?}", section_name.);

//     let combined_string = "test string";
//     let byte_array: Vec<u8> = combined_string.bytes().collect();
//     let _l = byte_array.len();
//     // #[link_section = "__DATA,ETQRQW"]
//     // static mut COMPILED_STRING: [u8; #_l] = [0; #_l];

//     // let _l = 2;
//     // #[link_section = "__DATA,ETQRQW"]
//     // let expanded = quote! {
//     //     fn #func_name() {
//     //         println!("This is the function: {}", stringify!(#func_name));

//     //         #[link_section = "__DATA,ETQRQW"]
//     //         static mut COMPILED_STRING: [u8; #_l] = [0; #_l];
//     //         let input_bytes = #combined_string.as_bytes();
//     //         unsafe {COMPILED_STRING[..#_l].copy_from_slice(input_bytes);}

//     //     }
//     // };
//     // let input_bytes = #combined_string.as_bytes();
//     // unsafe {#MY_VAR2[..#_l].copy_from_slice(input_bytes);
//     // }
//     //             println!("{:?}",COMPILED_STRING);
//     //         }
//     let func_name: LitStr = parse_macro_input!(input as LitStr);

//     let expanded = quote! {
//         fn #func_name() {

//             // #[link_section = "__DATA,ETQRQW"]
//             // static mut COMPILED_STRING: [u8; #_l] = [0; #_l];
//             // let input_bytes = #combined_string.as_bytes();
//             // unsafe {COMPILED_STRING[..#_l].copy_from_slice(input_bytes);}
//             println!("This is the function: {}", stringify!(#func_name));
//         }
//     };
//     expanded.into()
// }
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident};

#[proc_macro]
pub fn generate_complex_function(input: TokenStream) -> TokenStream {
    // let arg: Ident = parse_macro_input!(input as Ident);

    let combined_string = "test string!!! wow it is working!";
    let byte_array: Vec<u8> = combined_string.bytes().collect();
    let _l = byte_array.len();

    let expanded = quote! {
        // fn log_info(#arg: i32) {
            pub const fn string_to_u8_array(r: Option<[u8; #_l]>, n: usize) -> [u8; #_l] {
                if let Some(mut _r) = r {
                    if n == 0 {
                        return _r;
                    } else {
                        _r[#_l - n] = #combined_string.as_bytes()[#_l - n];
                        return string_to_u8_array(Some(_r), n - 1);
                    }
                } else {
                    let mut result:[u8; #_l] = [0; #_l];
                    result[#_l - n] = #combined_string.as_bytes()[#_l - n];
                    return string_to_u8_array(Some(result), n - 1);
                }

            }

            #[link_section = "__DATA,SYM_TABLE"]
            static COMPILED_STRING: [u8; #_l] = string_to_u8_array(None, #_l);
            println!("{:?}", COMPILED_STRING);
            // }
    };

    expanded.into()
}
