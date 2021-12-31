#[cfg(test)]
mod test {
    use crate::{reduce, parse, Element, read_magnitude, add_direction};

    fn add(input: &mut Vec<Element>, input2: &mut Vec<Element>) {
        input.insert(0, Element::Open);
        input.append(input2);
        input.push(Element::Close);
    }

    fn add_list(input: Vec<&str>) -> Vec<Element> {
        let mut res = parse(input[1]);
        for i in 2 .. input.len() - 1 {
            res.insert(0, Element::Open);
            res.append(&mut parse(input[i]));
            res.push(Element::Close);

            reduce(&mut res);
        }

        res
    }

    fn add_list_iters(input: &Vec<&str>, iters: usize) -> Vec<Element> {
        let mut res = parse(input[1]);
        for i in 2 .. 2 + iters {
            res.insert(0, Element::Open);
            res.append(&mut parse(input[i]));
            res.push(Element::Close);

            reduce(&mut res);
        }

        res
    }

    fn fmt(input: &Vec<Element>) -> String {
        let mut out = String::new();
        for i in 0 .. input.len() - 1 {
            let el = input[i];
            let n = input[i + 1];

            match el {
                Element::Open => out.push('['),
                Element::Close => {
                    out.push(']');
                    if n != Element::Close {
                        out.push(',');
                    }
                },
                Element::Num(num) => {
                    out.push_str(num.to_string().as_str());
                    if n != Element::Close {
                        out.push(',');
                    }
                }
            }
        }

        out
    }

    fn iter_once(input: &mut Vec<Element>) {
        let mut pos = 0;
        let mut depth = 0;

        while pos < input.len() {
            let el = input[pos];
            pos += 1;

            match el {
                // depth change
                Element::Open => depth += 1,
                Element::Close => depth -= 1,

                // number
                Element::Num(num) => {
                    if depth > 4 {
                        // 4 deep - check for explosion!
                        if let Element::Num(n) = input[pos] {
                            // we are in a pair. explode
                            add_direction(input, pos - 1, false, num);
                            add_direction(input, pos, true, n);

                            input.drain(pos - 2 ..= pos + 1);
                            input.insert(pos - 2, Element::Num(0));

                            break;
                        }
                    } else {
                        if num >= 10 {
                            // split
                            let mut to_insert: Vec<Element> = Vec::new();
                            let float = (num as f32) / 2.0;
                            to_insert.push(Element::Open);
                            to_insert.push(Element::Num(float.floor() as u8));
                            to_insert.push(Element::Num(float.ceil() as u8));
                            to_insert.push(Element::Close);

                            input.splice(pos - 1 ..= pos - 1, to_insert);
                            break;
                        }
                    }
                }
            }
        }

        if pos == input.len() {
            return;
        }
    }

    #[test]
    fn explode_a() {
        let mut a = parse("[[[[[9,8],1],2],3],4]");
        reduce(&mut a);
        let b = parse("[[[[0,9],2],3],4]");
        assert_eq!(a, b);
    }

    #[test]
    fn explode_b() {
        let mut a = parse("[7,[6,[5,[4,[3,2]]]]]");
        reduce(&mut a);
        let b = parse("[7,[6,[5,[7,0]]]]");
        assert_eq!(a, b);
    }

    #[test]
    fn explode_c() {
        let mut a = parse("[[6,[5,[4,[3,2]]]],1]");
        reduce(&mut a);
        let b = parse("[[6,[5,[7,0]]],3]");
        assert_eq!(a, b);
    }

    #[test]
    fn explode_d() {
        let mut a = parse("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
        iter_once(&mut a);
        let b = parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
        assert_eq!(a, b);
    }

    #[test]
    fn explode_e() {
        let mut a = parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
        reduce(&mut a);
        let b = parse("[[3,[2,[8,0]]],[9,[5,[7,0]]]]");
        assert_eq!(a, b);
    }

    #[test]
    fn reduce_a() {
        let a = add_list("
[1,1]
[2,2]
[3,3]
[4,4]
        ".lines().collect());
        let b = parse("[[[[1,1],[2,2]],[3,3]],[4,4]]");
        assert_eq!(a, b);
    }

    #[test]
    fn reduce_b() {
        let a = add_list("
[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
        ".lines().collect());
        let b = parse("[[[[3,0],[5,3]],[4,4]],[5,5]]");
        assert_eq!(a, b);
    }

    #[test]
    fn reduce_c() {
        let a = add_list("
[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
[6,6]
        ".lines().collect());
        let b = parse("[[[[5,0],[7,4]],[5,5]],[6,6]]");
        assert_eq!(a, b);
    }

    #[test]
    fn reduce_d() {
        let mut a = parse("[[[[4,3],4],4],[7,[[8,4],9]]]");
        let mut b = parse("[1,1]");
        add(&mut a, &mut b);
        reduce(&mut a);
        
        let result = parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
        assert_eq!(a, result);
    }

    #[test]
    fn largetest_a() {
        let a: Vec<&str> = "
[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]
        ".lines().collect();
        let b: Vec<&str> = "
[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]
[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]
[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]
[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]
[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]
[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]
[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]
[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]
[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]
        ".lines().collect();

        for i in 1 ..= b.len() - 2 {
            let sa = fmt(&add_list_iters(&a, i));
            if sa != b[i] {
                println!("fail at index {}", i);
                let mut sa2 = parse(a[1]);
                let mut sb2 = parse(a[2]);
                add(&mut sa2, &mut sb2);
                println!("{}", fmt(&sa2));
                println!("--------");

                println!("{}", sa);
                println!("{}\n", b[i]);
                assert!(fmt(&add_list_iters(&a, i)) == b[i]);
            }
        }
    }

    #[test]
    fn largetest_b() {
        let a = add_list("
[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
        ".lines().collect());
        let b = parse("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]");
        assert_eq!(a, b);
    }

    #[test]
    fn magnitude() {
        let lines: Vec<&str> = "
[[1,2],[[3,4],5]]
[[[[0,7],4],[[7,8],[6,0]]],[8,1]]
[[[[1,1],[2,2]],[3,3]],[4,4]]
[[[[3,0],[5,3]],[4,4]],[5,5]]
[[[[5,0],[7,4]],[5,5]],[6,6]]
[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]
        ".lines().collect();

        let magnitudes = [143, 1384, 445, 791, 1137, 3488];
        for i in 1 .. lines.len() - 1 {
            let mut line = parse(lines[i]);
            reduce(&mut line);
            assert_eq!(magnitudes[i - 1], read_magnitude(&line, &mut 1));
        }
    }

    #[test]
    fn add_a() {
        let mut a = parse("[1,2]");
        let mut b = parse("[[3,4],5]");
        add(&mut a, &mut b);
        assert_eq!(a, parse("[[1,2],[[3,4],5]]"));
    }
}
