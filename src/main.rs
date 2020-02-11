struct SomeThing {
    tiny: i8,
}

fn main() {
    println!("\n=== Rust data is immutable by default & must be declared mutable  ===");

    // Recall from borrowing-rust that a function an take ownership.
    //  In this case, immutable_object takes ownership of the SomeThing object which
    //  will be deallocated after the function ends.
    let im_s = SomeThing { tiny: 74 };
    immutable_object(im_s);

    // Reallocate another instance of SomeThing, and let the function borrow it.
    let im_s = SomeThing { tiny: 74 };
    println!(
        "\n- Before calling immutable_borrowed_object, value is {}",
        im_s.tiny
    );
    immutable_borrowed_object(&im_s);
    println!(
        "- After calling immutable_borrowed_object, value is still {}",
        im_s.tiny
    );

    // Just like the call to immutable_object above, mutable_object will take
    //  ownership of the SomeThing instance.
    mutable_object(im_s);

    // Declare a mutable instance of SomeThing.  Immutable instances cannot be
    //  borrowed as mutable.
    let mut m_s = SomeThing { tiny: 74 };
    println!(
        "\n- Before calling mutable_borrowed_object, value is {}",
        m_s.tiny
    );
    mutable_borrowed_object(&mut m_s);
    println!(
        "- After calling mutable_borrowed_object, value is {}",
        m_s.tiny
    );

    println!("\nPassing a mutable object as an immutable, borrowed reference works fine");
    immutable_borrowed_object(&m_s);
    println!(
        "- After calling immutable_borrowed_object, value is still {}",
        m_s.tiny
    );

    println!("");
}

#[allow(unused_variables)] // don't warn about x being unused
fn immutable_object(x: SomeThing) {
    // Uncomment to cause compilation error, E0594: 'x' is not declared as mutable.
    // x.tiny += 1;
}

#[allow(unused_variables)] // don't warn about x being unused
fn immutable_borrowed_object(x: &SomeThing) {
    // Uncomment to cause compilation error, E0594: `x` is a `&` reference, so the data
    //  it refers to cannot be written
    // x.tiny += 1;
}

fn mutable_object(mut ms: SomeThing) {
    ms.tiny += 1;
}

fn mutable_borrowed_object(ms: &mut SomeThing) {
    ms.tiny += 1;
}
