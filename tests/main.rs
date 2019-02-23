#![feature(proc_macro_hygiene)]
#![feature(stmt_expr_attributes)]
#![feature(custom_attribute)]

#[macro_use] extern crate with_clone;

#[derive(Debug)]
struct CloneTracker(u32);

impl std::clone::Clone for CloneTracker {
    fn clone(&self) -> CloneTracker {
        CloneTracker(self.0 + 1)
    }
}

#[test]
fn it_works() {
    let x = CloneTracker(0);
   
    // test expression
    #[clone(x)]
    assert_eq!(x.0, 1);
    
    // ensure original is still zero
    assert_eq!(x.0, 0);

    // test closure
    let closure =
        #[clone(x)]
        move || assert_eq!(x.0, 1);
    closure();

    // test multiple
    let y = x.clone();
    #[clone(x, y)] {
        assert_eq!(x.0, 1);
        assert_eq!(y.0, 2);
    }

    // test mut
    #[clone(mut x)] {
        x.0 = 4;
        assert_eq!(x.0, 4);
    }
    // ensure original is still zero
    assert_eq!(x.0, 0);

    // test loop
    #[clone(mut x)]
    loop {
        if x.0 > 5 { break; }
        x.0 = x.0 + 1;
    }

    // test match arm
    match x {
        CloneTracker(_) =>
            #[clone(x)]
            assert_eq!(x.0, 1)
    }
}


/* attempt at a non-proc-macro version with worse syntax
macro_rules! clone_rules {
    ( $( $var:ident ),+ => $body:expr ) => (
        {
            $( let $var = $var.clone(); )+
            $body
        }
    )
}*/
