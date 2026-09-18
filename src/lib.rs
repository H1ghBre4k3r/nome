#[macro_export]
macro_rules! construct_args {
    ($arg:ident { $($name:ident = $value:expr),+ $(,)? }) => {
        $arg::__Args { $($name: $value),+, ..Default::default() }
    };
}

#[macro_export]
macro_rules! func {
    ($d:tt $ident:ident, |$($name:ident: $type:ty),+ $(,)?| { $($stmt:stmt)* }) => {
        mod $ident {
            use super::*;

            #[derive(Default)]
            pub struct __Args {
                $(pub $name: $type),+
            }
        }

        macro_rules! $ident {
            () => {
                $ident(Default::default());
            };
            ($d($fargs:tt)+) => {
                $ident($crate::construct_args!($ident { $d($fargs)+ }));
            };
        }

        fn $ident(__arg: $ident::__Args) {
            let $ident::__Args { $($name),+ } = __arg;
            $($stmt)*
        }
    };
}
