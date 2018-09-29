use ::sx::Sx;

pub fn pretty_print(sx: &Sx) {
    do_pretty_print(sx, 0);
    print!("\n");
}

fn do_pretty_print(sx: &Sx, offset: usize) {
    let s = sx.to_string();
    if s.len() < 80 {
        indent(s.as_ref(), offset);
        //print!("\n");
        return;
    }

    match sx {
        Sx::List(sxs) => {
            let mut i = 0;
            let len = sxs.len();
            indent("(", offset);
            for sub_sx in sxs.iter() {
                match i {
                    0 => {
                        do_pretty_print(sub_sx, 0);
                    },

                    _ => {
                        do_pretty_print(sub_sx, offset + 1);
                    }
                }

                if i < len - 1 {
                    print!("\n");
                }

                i += 1;
            }

            print!(")");
        },

        Sx::Vector(sxs) => {
            let mut i = 0;
            let len = sxs.len();
            indent("[", offset);
            for sub_sx in sxs.iter() {
                match i {
                    0 => {
                        do_pretty_print(sub_sx, 0);
                    },

                    _ => {
                        do_pretty_print(sub_sx, offset + 1);
                    }
                }

                if i < len - 1 {
                    print!("\n");
                }

                i += 1;
            }

            print!("]");
        },

        _ => indent(s.as_ref(), offset)
    }
}

fn indent(s: &str, offset: usize) {
    print!("{:indent$}{}", "", s, indent=offset);
}
