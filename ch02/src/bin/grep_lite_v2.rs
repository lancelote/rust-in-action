fn main() {
    let context_line_num = 1;
    let needle = "oo";
    let found_needle = format!("\x1b[0;31m{}\x1b[0m", needle);
    let haystack = "\
Every face, every shop,
bedroom window, public-house, and dark square is a picture
feverishly turned--in search of what? It is the same with books.
What do we seek
through millions of pages?";

    let mut match_line_indexes: Vec<usize> = Vec::new();
    let mut contexts: Vec<Vec<(usize, String)>> = Vec::new();

    for (i, line) in haystack.lines().enumerate() {
        if line.contains(needle) {
            match_line_indexes.push(i);
            let context = Vec::with_capacity(2 * context_line_num + 1);
            contexts.push(context);
        }
    }

    if match_line_indexes.is_empty() {
        return;
    }

    for (i, line) in haystack.lines().enumerate() {
        for (j, match_line_index) in match_line_indexes.iter().enumerate() {
            let lower_bound = match_line_index.saturating_sub(context_line_num);
            let upper_bound = match_line_index + context_line_num;

            if (i >= lower_bound) && (i <= upper_bound) {
                let line_as_string;

                if i == *match_line_index {
                    line_as_string = line.replace(needle, found_needle.as_str());
                } else {
                    line_as_string = String::from(line);
                }

                let context_item = (i, line_as_string);
                contexts[j].push(context_item);
            }
        }
    }

    for context in contexts.iter() {
        for &(i, ref line) in context.iter() {
            let line_num = i + 1;
            println!("{}: {}", line_num, line);
        }
        println!();
    }
}
