use std::fmt::Write;

use ::sx::Sx;

pub fn pretty(sx: &Sx) -> String {
    let mut buf = String::new();
    do_pretty(sx, &mut buf, 0);
    return buf;
}

fn do_pretty(sx: &Sx, buf: &mut String, offset: usize) {
    let s = sx.to_string();
    if s.len() < 80 {
        indent(s.as_ref(), buf, offset);
        return;
    }

    match sx {
        Sx::List(sxs) => {
            let mut i = 0;
            let len = sxs.len();
            indent("(", buf, offset);
            for sub_sx in sxs.iter() {
                match i {
                    0 => {
                        do_pretty(sub_sx, buf, 0);
                    },

                    _ => {
                        do_pretty(sub_sx, buf, offset + 1);
                    }
                }

                if i < len - 1 {
                    buf.write_char('\n').expect("write failed");
                }

                i += 1;
            }

            buf.write_char(')').expect("write failed");
        },

        Sx::Vector(sxs) => {
            let mut i = 0;
            let len = sxs.len();
            indent("[", buf, offset);
            for sub_sx in sxs.iter() {
                match i {
                    0 => {
                        do_pretty(sub_sx, buf, 0);
                    },

                    _ => {
                        do_pretty(sub_sx, buf, offset + 1);
                    }
                }

                if i < len - 1 {
                    buf.write_char('\n').expect("write failed");
                }

                i += 1;
            }

            buf.write_char(']').expect("write failed");
        },

        _ => indent(s.as_ref(), buf, offset)
    }
}

fn indent(s: &str, buf: &mut String, offset: usize) {
    buf.write_str(format!("{:indent$}{}", "", s, indent=offset).as_ref()).expect("write failed");
}
