mod utils;

use proc_macro::TokenStream;
use utils::struct_field_impler;

#[proc_macro_derive(AddRef)]
pub fn add_ref_derive(input: TokenStream) -> TokenStream {
    let trait_generics = "<Output = T> + Copy";
    let prefix = "type Output = Self;";
    struct_field_impler(input, "std::ops::Add", trait_generics, "add", prefix, "", true)
}

#[proc_macro_derive(SubRef)]
pub fn sub_ref_derive(input: TokenStream) -> TokenStream {
    let trait_generics = "<Output = T> + Copy";
    let prefix = "type Output = Self;";
    struct_field_impler(input, "std::ops::Sub", trait_generics, "sub", prefix, "", true)
}

#[proc_macro_derive(MulRef)]
pub fn mul_ref_derive(input: TokenStream) -> TokenStream {
    let trait_generics = "<Output = T> + Copy";
    let prefix = "type Output = Self;";
    struct_field_impler(input, "std::ops::Mul", trait_generics, "mul", prefix, "", true)
}

#[proc_macro_derive(DivRef)]
pub fn div_ref_derive(input: TokenStream) -> TokenStream {
    let trait_generics = "<Output = T> + Copy";
    let prefix = "type Output = Self;";
    struct_field_impler(input, "std::ops::Div", trait_generics, "div", prefix, "", true)
}

#[proc_macro_derive(AddAssignRef)]
pub fn add_assign_ref_derive(input: TokenStream) -> TokenStream {
    let trait_generics = " + Copy";
    struct_field_impler(input, "std::ops::AddAssign", trait_generics, "add_assign", "", "", false)
}

#[proc_macro_derive(SubAssignRef)]
pub fn sub_assign_ref_derive(input: TokenStream) -> TokenStream {
    let trait_generics = " + Copy";
    struct_field_impler(input, "std::ops::SubAssign", trait_generics, "sub_assign", "", "", false)
}

#[proc_macro_derive(MulAssignRef)]
pub fn mul_assign_ref_derive(input: TokenStream) -> TokenStream {
    let trait_generics = " + Copy";
    struct_field_impler(input, "std::ops::MulAssign", trait_generics, "mul_assign", "", "", false)
}

#[proc_macro_derive(DivAssignRef)]
pub fn div_assign_ref_derive(input: TokenStream) -> TokenStream {
    let trait_generics = " + Copy";
    struct_field_impler(input, "std::ops::DivAssign", trait_generics, "div_assign", "", "", false)
}