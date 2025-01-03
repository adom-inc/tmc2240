macro_rules! write_1_to_clear {
    ($register_name:ident, $($field_name:ident),* $(,)?) => (
        paste::paste! {
            impl $register_name {
                $(
                    pub fn $field_name(&self) -> bool {
                        self.[<_ $field_name>]()
                    }

                    pub fn [<clear_ $field_name>](&mut self) {
                        self.[<_set_ $field_name>](true);
                    }
                )*

            }
        }
    );
}

pub(crate) use write_1_to_clear;
