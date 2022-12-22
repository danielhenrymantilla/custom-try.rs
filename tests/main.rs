use ::custom_try::custom_try;

#[custom_try]
const _: () = {
    fn foo()
    {}
};

#[custom_try]
const _: () = {
    macro_rules! r#try {( $e:expr ) => ( $e? )}

    fn foo()
      -> Option<i32>
    {
        None?
    }
};

const _: () = {
    macro_rules! noöp {( $e:expr ) => ( $e? )}

    #[custom_try(noöp)]
    fn foo()
      -> Option<i32>
    {
        None?
    }

    #[custom_try(noöp!)]
    fn bar()
      -> Option<i32>
    {
        None?
    }

    #[custom_try(noöp,)]
    fn baz()
      -> Option<i32>
    {
        None?
    }

    #[custom_try(noöp!,)]
    fn quux()
      -> Option<i32>
    {
        None?
    }
};

const _: () = {
    macro_rules! wtf {( $e:expr ) => ( () )}

    #[custom_try(wtf!)]
    fn sus() {
        ඞ??????????????????????????
    }
};
