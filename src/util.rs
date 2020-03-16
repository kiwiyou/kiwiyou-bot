/// Created with method [`split_args`]
pub struct SplitArgs<'a> {
    inner: &'a str,
}

impl<'a> Iterator for SplitArgs<'a> {
    type Item = &'a str;

    // Assume the first character is NOT a whitespace
    fn next(&mut self) -> Option<Self::Item> {
        let end_index = self
            .inner
            .char_indices()
            .find(|(_, letter)| letter.is_whitespace())
            .map_or(self.inner.len(), |(index, _)| index);
        let (arg, inner) = self.inner.split_at(end_index);
        self.inner = inner.trim_start();
        if arg.is_empty() {
            None
        } else {
            Some(arg)
        }
    }
}

impl<'a> SplitArgs<'a> {
    pub fn rest(&self) -> &'a str {
        self.inner
    }
}

/// Creates an iterator over the arguments, separated by unicode whitespace characters.
///
/// Equivalent to [`str::split_whitespace`], except that you can get the part of the text not yet parsed.
pub fn split_args(args_text: &str) -> SplitArgs {
    SplitArgs {
        inner: args_text.trim_start(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_full_arg_parse() {
        let text = "she gives\tsuper\n cider\rpepsi    resha";
        assert_eq!(
            vec!["she", "gives", "super", "cider", "pepsi", "resha"],
            split_args(text).collect::<Vec<_>>()
        )
    }

    #[test]
    fn test_partial_arg_parse() {
        let text = "she gives\tsuper\n cider\rpepsi    resha";
        let mut args = split_args(text);
        args.next();
        args.next();
        args.next();
        assert_eq!("cider\rpepsi    resha", args.rest())
    }
}
