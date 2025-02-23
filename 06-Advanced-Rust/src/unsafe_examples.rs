pub mod unsafe_examples {
    pub fn unsafe_functions() {
        let list: Vec<u8> = vec![1, 2, 3];

        // Safe rust. Bound checks before access. Panic when out of bounds
        let _x: u8 = list[0];

        // Safe rust. Bound checks before access. None if out of bounds
        let _x: Option<&u8> = list.get(0);

        // Unsafe rust. No bound checks
        // Slight performance increases but potentially undefined behavior (Bound checks can prevent loop unroll optimizations. In these cases performance increases might be significant)
        unsafe {
            let _x: &u8 = list.get_unchecked(0);
        }
    }

    pub fn unsafe_raw_pointers() {
        let mut hello: String = String::from("Hello World");

        // Construct a raw pointer for hello (memory address)
        let a = &raw mut hello;

        let b = unsafe { &*a };
        drop(hello);

        // Undefined behavior.
        println!("{b}");
    }

    pub fn unsafe_union_field_access() {
        // Union behaves like an enum but it has no information about it's type.
        // Unions memory footprint depends on the largest field
        union Foo<'a> {
            a: &'a str,
            b: i64,
        }

        let f: Foo = Foo { a: "Hallo" };

        let x = unsafe { f.b };
        // Undefined behavior. The Memory allocated for Foo as a &str is interpreted as i64
        println!("{x}");
    }

    // Side note:
    // Memory allocation of sum types(union/enum) depend on the largest field size
    //
    // The [`_BadExample`] enum always allocates 216 Bytes because of the [`_VeryLargeStruct`] size.
    enum _BadExample {
        Variant1,
        Variant2,
        Variant3,
        Variant4,
        Variant5(_VeryLargeStruct),
    }

    // The [`_GoodExample`] enum always allocates 16 Bytes but has a pointer indirection in case of Variant5
    enum _GoodExample {
        Variant1,
        Variant2,
        Variant3,
        Variant4,
        Variant5(Box<_VeryLargeStruct>),
    }

    struct _VeryLargeStruct {
        f1: String,
        f2: String,
        f3: String,
        f4: String,
        f5: String,
        f6: String,
        f7: String,
        f8: String,
        f9: String,
    }
}
