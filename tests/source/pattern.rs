// rustfmt-normalize_comments: true
fn main() {
    let z = match x {
        "pat1" => 1,
        ( ref  x, ref  mut  y /*comment*/) => 2,
    };

    if let <  T as  Trait   > :: CONST = ident {
        do_smth();
    }

    let Some ( ref   xyz  /*   comment!   */) = opt;

    if let  None  =   opt2 { panic!("oh noes"); }

    let foo@bar (f) = 42;
    let a::foo ( ..) = 42;
    let [ ] = 42;
    let [a..,     b,c ] = 42;
    let [ a,b,c.. ] = 42;
    let [a,    b, c, d..,e,f,     g] = 42;
    let foo {   } = 42;
    let foo {..} = 42;
    let foo { x, y: ref foo,     .. } = 42;
    let foo { x, yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy: ref foo,     .. } = 42;
    let foo { x,       yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy: ref foo,      } = 42;
    let foo { x, yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy: ref foo,     .. };
    let foo { x,       yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy: ref foo,      };
}

impl<'a,'b> ResolveGeneratedContentFragmentMutator<'a,'b> {
    fn mutate_fragment(&mut self, fragment: &mut Fragment) {
        match **info {
            GeneratedContentInfo::ContentItem(
                ContentItem::Counter(
                    ref counter_name,
                    counter_style
                )
            ) => {}}}
}
