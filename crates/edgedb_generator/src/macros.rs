#[macro_export]
macro_rules! generate_query_module {
	($module_name:ident, $query:expr) => {
		::quote::quote! {
			const A: u8 = 100;
			A
		}
	};
}

fn s() {
	let a = generate_query_module!(unused, "");
}
