use dioxus::prelude::*;

fn get_class(id: u8) -> &'static str {
    match id {
        0 | 3 | 6 | 27 | 30 | 33 | 54 | 57 | 60 => "tsb lsb rdb bdb",
        1 | 4 | 7 | 28 | 31 | 34 | 55 | 58 | 61 => "tsb  bdb",
        2 | 5 | 29 | 32 | 56 | 59 => "tsb bdb ldb",
        8 | 35 | 62 => "tsb rsb bdb ldb",
        9 | 12 | 15 | 36 | 39 | 42 | 63 | 66 | 69 => "lsb rdb bdb",
        10 | 13 | 16 | 37 | 40 | 43 | 64 | 67 | 70 => "bdb",
        11 | 14 | 38 | 41 | 65 | 68 => "bdb ldb",
        18 | 21 | 24 | 45 | 48 | 51 => "lsb rdb",
        20 | 23 | 47 | 50 => "ldb",
        17 | 44 | 71 => "rsb ldb bdb",
        26 | 53 => "rsb ldb",
        72 | 75 | 78 => "bsb lsb rdb",
        73 | 76 | 79 => "bsb",
        74 | 77 => "bsb ldb",
        80 => "bsb rsb ldb",
        _ => "",
    }
}

#[derive(Props, PartialEq)]
pub struct CellProps {
    index: u8,
    value: u8,
}

pub fn LockCell(cx: Scope<CellProps>) -> Element {
    let class = get_class(cx.props.index);
    let id = cx.props.index;
    // Conditionally display the value or an empty string
    let value = if cx.props.value != 0 {
        cx.props.value.to_string()
    } else {
        String::new()
    };

    cx.render(rsx!(
        div {
            class: class,
            id: "{id}",
            "{value}"
        }
    ))
}

pub fn FreeCell(cx: Scope<CellProps>) -> Element {
    let class = get_class(cx.props.index);
    let id = cx.props.index;
    let value = use_state(cx, || "".to_string());

    cx.render(rsx!(
        div {
            class: class,
            id: "{id}",
            style: "color: green", // TODO: better color
            "{value}"
        }
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use dioxus_ssr::render_lazy;
    use rand::Rng;
    use regex::Regex;

    #[test]
    fn test_lock_cell() {
        // Test with a non-zero value
        let rendered_lock_cell = render_lazy(rsx!(LockCell { index: 0, value: 5 }));
        assert!(rendered_lock_cell.contains('5'));

        // Test with a zero value
        let rendered_lock_cell_zero = render_lazy(rsx!(LockCell { index: 1, value: 0 }));
        // Adjust this based on whether you expect to render "0" or an empty string
        assert!(!rendered_lock_cell_zero.contains('0'));
    }

    #[test]
    fn test_free_cell() {
        // Assuming FreeCell starts with an empty value
        let rendered_free_cell = render_lazy(rsx!(FreeCell { index: 0, value: 0 }));

        // Test with a zero value
        assert!(rendered_free_cell.contains('0'));
    }

    #[test]
    fn test_free_cell_classes() {
        let re = Regex::new(r#"<div[^>]*class="([^"]*)""#).unwrap();

        for id in 0..81 {
            let rendered = render_lazy(rsx!(FreeCell {
                index: id,
                value: 0
            }));
            let caps = re.captures(&rendered).unwrap();
            let class_attr = &caps[1];

            assert_eq!(class_attr, get_class(id));
        }
    }

    #[test]
    fn test_lock_cell_classes() {
        let mut rng = rand::thread_rng();

        let re = Regex::new(r#"<div[^>]*class="([^"]*)""#).unwrap();

        for id in 0..81 {
            let value: u8 = rng.gen_range(1..=9);
            let rendered = render_lazy(rsx!(LockCell {
                index: id,
                value: value,
            }));
            let caps = re.captures(&rendered).unwrap();
            let class_attr = &caps[1];

            assert_eq!(class_attr, get_class(id));
        }
    }
}
